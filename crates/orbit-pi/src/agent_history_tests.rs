use super::*;
use serde_json::json;

struct TempDir(std::path::PathBuf);
impl TempDir {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "orbit-agent-history-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn path(&self) -> &Path {
        &self.0
    }
}
impl Drop for TempDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

fn fixture() -> (TempDir, std::path::PathBuf) {
    let dir = TempDir::new();
    let path = dir.path().join("child.jsonl");
    let entries = [
        json!({"type":"session","id":"child","cwd":"/work/game"}),
        json!({"type":"message","id":"user","parentId":null,"message":{"role":"user","content":"Task"}}),
        json!({"type":"message","id":"abandoned","parentId":"user","message":{"role":"assistant","content":"Old branch"}}),
        json!({"type":"message","id":"active","parentId":"user","message":{"role":"assistant","content":[{"type":"text","text":"Current branch"}]}}),
        json!({"type":"session_info","id":"name","parentId":"active","name":"scout"}),
    ];
    std::fs::write(
        &path,
        entries.iter().map(|v| format!("{v}\n")).collect::<String>(),
    )
    .unwrap();
    (dir, path)
}

#[test]
fn subagent_snapshot_follows_active_branch_without_modifying_the_file() {
    let (_dir, path) = fixture();
    let before = std::fs::read(&path).unwrap();
    let snapshot = read_snapshot(&path).unwrap();
    assert_eq!(
        snapshot
            .messages
            .iter()
            .map(|m| m.text.as_str())
            .collect::<Vec<_>>(),
        ["Task", "Current branch"]
    );
    assert!(!snapshot.limited);
    assert_eq!(std::fs::read(&path).unwrap(), before);
}

#[test]
fn subagent_snapshot_tolerates_a_partial_live_append() {
    use std::io::Write;
    let (_dir, path) = fixture();
    std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .unwrap()
        .write_all(b"{\"type\":")
        .unwrap();
    assert_eq!(read_snapshot(&path).unwrap().messages.len(), 2);
}

#[test]
fn subagent_snapshot_bounds_large_text_and_marks_the_preview() {
    let (_dir, path) = fixture();
    let text = "界".repeat(MAX_TEXT + 1);
    std::fs::write(
        &path,
        json!({"type":"message","id":"x","parentId":null,
        "message":{"role":"assistant","content":text}})
        .to_string(),
    )
    .unwrap();
    let snapshot = read_snapshot(&path).unwrap();
    assert!(snapshot.limited);
    assert_eq!(snapshot.messages[0].text.chars().count(), MAX_TEXT);
}

#[test]
fn subagent_snapshot_reports_missing_files() {
    let dir = TempDir::new();
    assert!(read_snapshot(&dir.path().join("missing.jsonl")).is_err());
}

#[gpui::test]
fn subagent_history_is_a_read_only_native_view_with_close(cx: &mut gpui::TestAppContext) {
    use gpui::{size, Modifiers};
    use std::cell::Cell;
    let (_dir, path) = fixture();
    let before = std::fs::read(&path).unwrap();
    cx.update(|cx| cx.set_global(theme::Theme::for_id(theme::ThemeId::Orbit)));
    let closed = Rc::new(Cell::new(false));
    let close_flag = closed.clone();
    let (_, cx) = cx.add_window_view(|_, cx| {
        AgentHistory::new(
            SessionInfo {
                path: path.clone(),
                id: "child".into(),
                cwd: "/work/game".into(),
                title: "Scout".into(),
                first_message: "Task".into(),
                modified: std::time::SystemTime::UNIX_EPOCH,
                subagent: Some(crate::session_origin::SubagentOrigin {
                    parent: "parent.jsonl".into(),
                    kind: "scout".into(),
                }),
            },
            Box::new(move |_, _| close_flag.set(true)),
            cx,
        )
    });
    cx.simulate_resize(size(px(900.), px(700.)));
    cx.run_until_parked();
    assert!(cx.debug_bounds("agent-history").is_some());
    let close = cx.debug_bounds("close-agent-history").unwrap().center();
    cx.simulate_click(close, Modifiers::none());
    assert!(closed.get());
    assert_eq!(std::fs::read(path).unwrap(), before);
}
