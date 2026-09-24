//! Offline replay tests of the pi-subagents AgentDetails wire shape.
use super::*;
use serde_json::json;

fn details(status: &str, id: &str) -> Value {
    json!({
        "displayName": "Scout", "description": "Inspect collision handling",
        "subagentType": "scout", "agentId": id, "status": status,
        "activity": "Reading internal/player.go", "modelName": "test-model",
        "tags": ["thinking: high"], "toolUses": 3, "tokens": "1.2k token",
        "durationMs": 2400, "turnCount": 2, "maxTurns": 8, "cost": 0.003
    })
}

fn start(transcript: &mut Transcript, id: &str) {
    transcript.apply_event(&Event::ToolExecutionStart {
        value: json!({"toolCallId":id,"toolName":"Agent","args":{"prompt":"Inspect the code","subagent_type":"scout"}}),
    });
}

fn progress(id: &str, details: Value) -> Event {
    Event::ToolExecutionUpdate {
        value: json!({"toolCallId":id,"toolName":"Agent","partialResult":{
            "content":[{"type":"text","text":"3 tool uses..."}],"details":details
        }}),
    }
}

#[test]
fn agents_progress_retains_metadata_beside_normalized_output() {
    let mut transcript = Transcript::new();
    start(&mut transcript, "call-1");
    transcript.apply_event(&progress("call-1", details("running", "agent-1")));
    let messages = transcript.messages.borrow();
    let tool = messages[0].tools().next().unwrap();
    let agent = tool.facts.agent.as_ref().unwrap();
    assert_eq!(
        tool.output.as_ref().and_then(Value::as_str),
        Some("3 tool uses...")
    );
    assert_eq!(agent.id.as_deref(), Some("agent-1"));
    assert_eq!(agent.model.as_deref(), Some("test-model"));
    assert_eq!(agent.tools, Some(3));
    assert!(agent.live);
}

#[test]
fn agents_parallel_progress_does_not_overwrite_siblings() {
    let mut transcript = Transcript::new();
    start(&mut transcript, "call-1");
    start(&mut transcript, "call-2");
    transcript.apply_event(&progress("call-2", details("running", "agent-2")));
    transcript.apply_event(&progress("call-1", details("queued", "agent-1")));
    let messages = transcript.messages.borrow();
    let agents: Vec<_> = messages[0]
        .tools()
        .map(|t| {
            let a = t.facts.agent.as_ref().unwrap();
            (a.id.as_deref(), a.status.as_str())
        })
        .collect();
    assert_eq!(
        agents,
        [(Some("agent-1"), "queued"), (Some("agent-2"), "running")]
    );
}

#[test]
fn agents_spinner_only_updates_do_not_invalidate_transcript() {
    let mut transcript = Transcript::new();
    start(&mut transcript, "call-1");
    let mut data = details("running", "agent-1");
    transcript.apply_event(&progress("call-1", data.clone()));
    data["spinnerFrame"] = json!(8);
    data["durationMs"] = json!(2480);
    assert!(!transcript.apply_event(&progress("call-1", data)));
}

#[test]
fn agents_completion_replaces_live_observation() {
    let mut transcript = Transcript::new();
    start(&mut transcript, "call-1");
    transcript.apply_event(&progress("call-1", details("running", "agent-1")));
    transcript.apply_event(&Event::ToolExecutionEnd {
        value: json!({"toolCallId":"call-1","isError":false,"result":{
            "content":[{"type":"text","text":"Found the issue"}],
            "details":details("completed", "agent-1")
        }}),
    });
    let messages = transcript.messages.borrow();
    let agent = messages[0]
        .tools()
        .next()
        .unwrap()
        .facts
        .agent
        .as_ref()
        .unwrap();
    assert_eq!((agent.status.as_str(), agent.live), ("completed", false));
}

#[test]
fn agents_metadata_free_result_message_does_not_erase_known_completion() {
    let mut transcript = Transcript::new();
    start(&mut transcript, "call-1");
    transcript.apply_event(&Event::ToolExecutionEnd {
        value: json!({"toolCallId":"call-1","isError":false,"result":{
            "content":[{"type":"text","text":"Done"}],
            "details":details("completed", "agent-1")
        }}),
    });
    transcript.apply_event(&Event::MessageEnd {
        value: json!({"role":"toolResult","toolCallId":"call-1",
            "content":[{"type":"text","text":"Done"}]}),
    });
    let messages = transcript.messages.borrow();
    assert_eq!(
        messages[0]
            .tools()
            .next()
            .unwrap()
            .facts
            .agent
            .as_ref()
            .unwrap()
            .status,
        "completed"
    );
}

#[test]
fn agents_generic_failure_keeps_identity_but_not_running_status() {
    let mut transcript = Transcript::new();
    start(&mut transcript, "call-1");
    transcript.apply_event(&progress("call-1", details("running", "agent-1")));
    transcript.apply_event(&Event::ToolExecutionEnd {
        value: json!({"toolCallId":"call-1","isError":true,"result":{
            "content":[{"type":"text","text":"Aborted"}]
        }}),
    });
    let messages = transcript.messages.borrow();
    let agent = messages[0]
        .tools()
        .next()
        .unwrap()
        .facts
        .agent
        .as_ref()
        .unwrap();
    assert_eq!(
        (agent.id.as_deref(), agent.status.as_str(), agent.live),
        (Some("agent-1"), "error", false)
    );
}

#[test]
fn agents_reload_restores_metadata_without_claiming_liveness() {
    let mut transcript = Transcript::new();
    transcript.agent_selection.set(Some((0, 0)));
    transcript.load_from(&json!({"messages":[
        {"role":"assistant","content":[{"type":"toolCall","id":"call-1","name":"Agent","arguments":{"prompt":"Inspect"}}]},
        {"role":"toolResult","toolCallId":"call-1","content":[{"type":"text","text":"Started"}],"details":details("running", "agent-1")}
    ]}));
    assert_eq!(transcript.agent_selection.get(), None);
    let messages = transcript.messages.borrow();
    let agent = messages[0]
        .tools()
        .next()
        .unwrap()
        .facts
        .agent
        .as_ref()
        .unwrap();
    assert_eq!((agent.id.as_deref(), agent.live), (Some("agent-1"), false));
}

#[test]
fn agents_process_exit_invalidates_live_claims() {
    let mut transcript = Transcript::new();
    start(&mut transcript, "call-1");
    transcript.apply_event(&progress("call-1", details("running", "agent-1")));
    transcript.apply_event(&Event::ProcessExited);
    let messages = transcript.messages.borrow();
    let agent = messages[0]
        .tools()
        .next()
        .unwrap()
        .facts
        .agent
        .as_ref()
        .unwrap();
    assert!(!agent.live);
}

#[test]
fn agents_new_session_does_not_keep_inspector_selection() {
    let mut transcript = Transcript::new();
    start(&mut transcript, "call-1");
    transcript.agent_selection.set(Some((0, 0)));
    transcript.clear();
    assert_eq!(transcript.agent_selection.get(), None);
}

#[test]
fn agents_final_message_snapshot_preserves_progress_details() {
    let mut transcript = Transcript::new();
    transcript.apply_event(&Event::MessageStart {
        value: json!({"role":"assistant","content":[]}),
    });
    start(&mut transcript, "call-1");
    transcript.apply_event(&progress("call-1", details("running", "agent-1")));
    transcript.apply_event(&Event::MessageEnd {
        value: json!({"role":"assistant","content":[
            {"type":"toolCall","id":"call-1","name":"Agent","arguments":{"prompt":"Inspect"}}
        ]}),
    });
    let messages = transcript.messages.borrow();
    assert_eq!(
        messages[0]
            .tools()
            .next()
            .unwrap()
            .facts
            .agent
            .as_ref()
            .unwrap()
            .id
            .as_deref(),
        Some("agent-1")
    );
}
