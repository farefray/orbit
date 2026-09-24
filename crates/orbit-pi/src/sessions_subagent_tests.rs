use super::*;
use serde_json::json;

struct Store(PathBuf);
impl Store {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("orbit-origins-{}", next_session_id()));
        fs::create_dir_all(path.join("project")).unwrap();
        Self(path)
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join("project").join(format!("{name}.jsonl"))
    }
    fn put(&self, name: &str, entries: &[Value]) -> PathBuf {
        let path = self.path(name);
        fs::write(
            &path,
            entries.iter().map(|v| format!("{v}\n")).collect::<String>(),
        )
        .unwrap();
        path
    }
    fn parent(&self, record: bool) -> PathBuf {
        let mut entries = vec![
            json!({"type":"session","id":"parent","cwd":"/work/game"}),
            json!({"type":"message","message":{"role":"user","content":"Main task"}}),
            json!({"type":"message","message":{"role":"assistant","content":[{
                "type":"toolCall","name":"Agent","arguments":{"subagent_type":"scout","prompt":"Inspect\nfiles"}
            }]}}),
        ];
        if record {
            entries.push(json!({"type":"custom","customType":"subagents:record","data":{"id":"9ef6e557-abcd","type":"scout"}}));
        }
        self.put("parent", &entries)
    }
    fn child(&self, parent: &Path, context: bool) -> PathBuf {
        self.put("child", &[
            json!({"type":"session","id":"child","cwd":"/work/game","parentSession":parent}),
            json!({"type":"session_info","name":"scout#9ef6e557"}),
            json!({"type":"message","message":{"role":"system","sections":{"preamble":if context { "prefix\n<sub_agent_context>\nchild" } else { "ordinary" }}}}),
            json!({"type":"message","message":{"role":"user","content":"Inspect\nfiles"}}),
            json!({"type":"session_info","name":"Renamed child"}),
        ])
    }
    fn load(&self, path: &Path) -> SessionInfo {
        load_sessions_in(&self.0)
            .into_iter()
            .find(|s| s.path == path)
            .unwrap()
    }
}
impl Drop for Store {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn running_subagent_requires_exact_parent_spawn_and_child_context() {
    let store = Store::new();
    let parent = store.parent(false);
    let child = store.child(&parent, true);
    let session = store.load(&child);
    assert_eq!(session.subagent.unwrap().parent, parent);
    assert_eq!(session.title, "Renamed child");
}

#[test]
fn a_name_and_parent_link_alone_do_not_hide_an_ordinary_session() {
    let store = Store::new();
    let parent = store.parent(false);
    let child = store.child(&parent, false);
    assert!(store.load(&child).subagent.is_none());
}

#[test]
fn settled_record_corroborates_older_child_without_system_context() {
    let store = Store::new();
    let parent = store.parent(true);
    let child = store.child(&parent, false);
    assert!(store.load(&child).subagent.is_some());
}

#[test]
fn cloning_a_subagent_remains_an_ordinary_session() {
    let store = Store::new();
    let parent = store.parent(true);
    let child = store.child(&parent, true);
    let clone = clone_session_file(&child).unwrap();
    assert!(store.load(&clone).subagent.is_none());
    assert!(store.load(&child).subagent.is_some());
}

#[test]
fn unverified_missing_parent_is_not_hidden() {
    let store = Store::new();
    let child = store.child(&store.path("missing"), true);
    assert!(store.load(&child).subagent.is_none());
}

#[test]
fn declared_origin_is_bound_to_child_identity_and_survives_missing_parent() {
    let store = Store::new();
    let parent = store.path("missing");
    let child = store.put(
        "declared",
        &[
            json!({"type":"session","id":"declared","cwd":"/work/game","parentSession":parent}),
            json!({"type":"custom","customType":"subagents:session","data":{
            "version":1,"sessionId":"declared","parentSession":parent,"agentType":"scout"}}),
            json!({"type":"message","message":{"role":"user","content":"Inspect files"}}),
        ],
    );
    assert!(store.load(&child).subagent.is_some());
    assert!(store
        .load(&clone_session_file(&child).unwrap())
        .subagent
        .is_none());
}

#[test]
fn parent_files_outside_the_discovered_store_are_not_followed() {
    let store = Store::new();
    let parent = store.parent(true);
    let external = store.0.join("external.jsonl");
    fs::rename(parent, &external).unwrap();
    let child = store.child(&external, true);
    assert!(store.load(&child).subagent.is_none());
}

#[test]
fn unrelated_spawn_prompt_does_not_prove_child_ownership() {
    let store = Store::new();
    let parent = store.parent(false);
    let child = store.child(&parent, true);
    let raw = fs::read_to_string(&child)
        .unwrap()
        .replace("Inspect\\nfiles", "Other task");
    fs::write(&child, raw).unwrap();
    assert!(store.load(&child).subagent.is_none());
}
