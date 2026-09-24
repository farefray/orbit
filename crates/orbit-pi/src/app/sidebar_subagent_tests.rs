use super::{sidebar::*, *};

fn session(name: &str, parent: Option<&str>) -> SessionInfo {
    SessionInfo {
        path: PathBuf::from(format!("/store/{name}.jsonl")),
        id: name.into(),
        cwd: PathBuf::from("/work/game"),
        title: name.into(),
        first_message: "task".into(),
        modified: SystemTime::UNIX_EPOCH,
        subagent: parent.map(|p| crate::session_origin::SubagentOrigin {
            parent: PathBuf::from(format!("/store/{p}.jsonl")),
            kind: "scout".into(),
        }),
    }
}

fn rows(sessions: &[SessionInfo], expanded: HashSet<PathBuf>) -> Vec<SideRow> {
    with_subagent_rows(
        build_sidebar_rows(
            sessions,
            &["/work/game".into()],
            "game",
            &HashSet::new(),
            &HashSet::new(),
            &HashMap::new(),
            &HashSet::new(),
            &None,
            &HashSet::new(),
        ),
        sessions,
        &expanded,
    )
}

#[test]
fn dozen_children_do_not_displace_main_sessions_or_inflate_project_count() {
    let mut sessions: Vec<_> = (0..12)
        .map(|i| session(&format!("child-{i}"), Some("main")))
        .collect();
    sessions.extend([session("main", None), session("older", None)]);
    let rows = rows(&sessions, HashSet::new());
    assert!(matches!(&rows[0], SideRow::Workspace { count: 2, .. }));
    assert!(matches!(&rows[1], SideRow::Session(12)));
    assert!(matches!(
        &rows[2],
        SideRow::Subagents {
            count: 12,
            expanded: false,
            ..
        }
    ));
    assert!(matches!(&rows[3], SideRow::Session(13)));
    assert_eq!(rows.len(), 4);
}

#[test]
fn expanding_a_family_reveals_children_and_nested_descendants() {
    let sessions = [
        session("nested", Some("child")),
        session("child", Some("main")),
        session("main", None),
    ];
    let rows = rows(&sessions, HashSet::from([sessions[2].path.clone()]));
    assert_eq!(
        rows.iter()
            .filter(|r| matches!(r, SideRow::ChildSession(_)))
            .count(),
        2
    );
    assert!(matches!(
        &rows[2],
        SideRow::Subagents {
            count: 2,
            expanded: true,
            ..
        }
    ));
}

#[test]
fn orphan_and_cyclic_children_remain_discoverable_in_a_collapsed_group() {
    let sessions = [
        session("orphan", Some("gone")),
        session("a", Some("b")),
        session("b", Some("a")),
    ];
    let rows = rows(&sessions, HashSet::new());
    assert!(matches!(&rows[0], SideRow::Workspace { count: 0, .. }));
    assert!(matches!(
        &rows[1],
        SideRow::Subagents {
            orphan: true,
            count: 3,
            expanded: false,
            ..
        }
    ));
}

#[test]
fn collapsing_workspace_hides_even_an_expanded_active_family() {
    let sessions = [session("child", Some("main")), session("main", None)];
    let roots = build_sidebar_rows(
        &sessions,
        &["/work/game".into()],
        "game",
        &HashSet::from(["game".into()]),
        &HashSet::new(),
        &HashMap::new(),
        &HashSet::new(),
        &Some(sessions[1].path.clone()),
        &HashSet::new(),
    );
    let rows = with_subagent_rows(roots, &sessions, &HashSet::from([sessions[1].path.clone()]));
    assert_eq!(rows.len(), 2);
    assert!(matches!(&rows[1], SideRow::Session(1)));
}

#[test]
fn deeply_nested_imports_resolve_without_recursion() {
    let mut sessions: Vec<_> = (0..10_000)
        .map(|i| session(&i.to_string(), Some(&(i + 1).to_string())))
        .collect();
    sessions.push(session("10000", None));
    assert!(crate::session_origin::family_roots(&sessions)
        .iter()
        .all(|root| *root == Some(10_000)));
}

#[test]
fn a_child_in_another_workspace_stays_under_its_parent() {
    let mut child = session("child", Some("main"));
    child.cwd = "/work/worktree".into();
    let sessions = [child, session("main", None)];
    let rows = rows(&sessions, HashSet::from([sessions[1].path.clone()]));
    assert!(matches!(rows.last(), Some(SideRow::ChildSession(0))));
}
