//! Conservative discovery of subagent sessions, independent of display titles.
//! Names alone and pi's generic `parentSession` (also used by clones) are not
//! evidence. All I/O here runs in the session-store worker, never during paint.

use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubagentOrigin {
    pub parent: PathBuf,
    pub kind: String,
}

pub(crate) fn path_key(path: &Path) -> String {
    let text = path.to_string_lossy().into_owned();
    #[cfg(windows)]
    {
        text.replace('\\', "/")
            .trim_start_matches("//?/")
            .to_lowercase()
    }
    #[cfg(not(windows))]
    {
        text
    }
}

#[derive(Default)]
struct ParentEvidence {
    agent_ids: HashSet<String>,
    spawns: Vec<(String, String)>,
}

/// One parent scan per discovery pass, even when it owns dozens of children.
/// Only files already discovered in pi's store may be followed.
pub(crate) struct OriginResolver {
    paths: HashMap<String, PathBuf>,
    parents: HashMap<String, ParentEvidence>,
}

impl OriginResolver {
    pub fn new(paths: &[PathBuf]) -> Self {
        Self {
            paths: paths.iter().map(|p| (path_key(p), p.clone())).collect(),
            parents: HashMap::new(),
        }
    }

    pub fn resolve(
        &mut self,
        parent: &Path,
        name: &str,
        prompt: &str,
        child_context: bool,
    ) -> Option<SubagentOrigin> {
        // evidence-subagents assigns this initial (not current/renamable) name.
        let (kind, short_id) = name.rsplit_once('#')?;
        if kind.is_empty()
            || short_id.len() != 8
            || !short_id.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return None;
        }
        let key = path_key(parent);
        let path = self.paths.get(&key)?;
        let evidence = self.parents.entry(key).or_insert_with(|| scan_parent(path));
        // Settled records identify the agent. While it is running, require both
        // the fork's machine-added child context and an exact parent spawn.
        // Cloning a child does not satisfy this: the child did not spawn itself.
        let recorded = evidence.agent_ids.contains(short_id);
        let spawning = child_context
            && evidence
                .spawns
                .iter()
                .any(|(ty, task)| ty == kind && task == prompt);
        (recorded || spawning).then(|| SubagentOrigin {
            parent: path.clone(),
            kind: kind.into(),
        })
    }
}

fn scan_parent(path: &Path) -> ParentEvidence {
    let mut evidence = ParentEvidence::default();
    let Ok(file) = File::open(path) else {
        return evidence;
    };
    // A conservative fallback, not an unbounded transcript index. If evidence
    // falls beyond the budget the session stays visible as an ordinary session.
    for (index, line) in BufReader::new(file.take(32 * 1024 * 1024))
        .lines()
        .map_while(Result::ok)
        .enumerate()
    {
        let v = match serde_json::from_str::<Value>(&line) {
            Ok(value) => value,
            Err(_) if index == 0 => return evidence,
            Err(_) => continue,
        };
        if index == 0 {
            if v["type"] != "session" {
                return evidence;
            }
            continue;
        }
        if v["type"] == "custom" && v["customType"] == "subagents:record" {
            if let Some(id) = v["data"]["id"]
                .as_str()
                .filter(|id| id.len() >= 8 && id.is_ascii())
            {
                evidence.agent_ids.insert(id[..8].to_owned());
            }
        }
        if v["type"] != "message" {
            continue;
        }
        let message = &v["message"];
        if let Some(details) = crate::agents::AgentDetails::from_details(&message["details"]) {
            if let Some(id) = details.id.filter(|id| id.len() >= 8 && id.is_ascii()) {
                evidence.agent_ids.insert(id[..8].to_owned());
            }
        }
        if message["role"] != "assistant" {
            continue;
        }
        for block in message["content"].as_array().into_iter().flatten() {
            if block["type"] != "toolCall" || block["name"] != "Agent" {
                continue;
            }
            let args = &block["arguments"];
            if let (Some(kind), Some(prompt)) =
                (args["subagent_type"].as_str(), args["prompt"].as_str())
            {
                evidence
                    .spawns
                    .push((kind.into(), prompt.replace('\n', " ")));
            }
        }
    }
    evidence
}

/// Optional versioned origin record for extensions. Bound to the *child's*
/// session id, so verbatim cloned history cannot misclassify the clone.
pub(crate) fn declared_origin(
    entry: &Value,
    session_id: &str,
    parent: &Path,
) -> Option<SubagentOrigin> {
    let data = &entry["data"];
    if entry["type"] != "custom"
        || entry["customType"] != "subagents:session"
        || data["version"] != 1
        || data["sessionId"].as_str()? != session_id
        || path_key(Path::new(data["parentSession"].as_str()?)) != path_key(parent)
    {
        return None;
    }
    let kind = data["agentType"].as_str()?.trim();
    if kind.is_empty() {
        return None;
    }
    Some(SubagentOrigin {
        parent: parent.into(),
        kind: kind.into(),
    })
}

/// Associate descendants with their nearest ordinary-session ancestor. Cycles
/// and missing parents become an orphan group rather than disappearing.
pub(crate) fn family_roots(sessions: &[crate::sessions::SessionInfo]) -> Vec<Option<usize>> {
    let indices: HashMap<_, _> = sessions
        .iter()
        .enumerate()
        .map(|(i, s)| (path_key(&s.path), i))
        .collect();
    // Path compression keeps even a deep imported chain linear, not O(n²)
    // work on every sidebar frame. Some(None) is a known orphan/cycle.
    let mut memo: Vec<Option<Option<usize>>> = vec![None; sessions.len()];
    let mut visited = vec![usize::MAX; sessions.len()];
    for start in 0..sessions.len() {
        if memo[start].is_some() {
            continue;
        }
        let mut current = start;
        let mut chain = Vec::new();
        let root = loop {
            if let Some(root) = memo[current] {
                break root;
            }
            if visited[current] == start {
                break None;
            }
            visited[current] = start;
            chain.push(current);
            let Some(origin) = &sessions[current].subagent else {
                break Some(current);
            };
            let Some(&parent) = indices.get(&path_key(&origin.parent)) else {
                break None;
            };
            current = parent;
        };
        for index in chain {
            memo[index] = Some(root);
        }
    }
    memo.into_iter().map(Option::flatten).collect()
}
