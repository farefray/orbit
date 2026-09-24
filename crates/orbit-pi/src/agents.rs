//! Read-only pi-subagents observations. These are reported tool facts, not a
//! second agent runtime: inspecting a child never resumes or consumes its result.

use std::{cell::Cell, rc::Rc};

use gpui::{div, prelude::*, px, AnyElement, ClipboardItem, ElementId, FontWeight, Pixels};
use serde_json::Value;

use crate::{theme::Theme, transcript::ToolCall};

/// Coordinates belong to one Transcript and are cleared when it is rebuilt.
pub(crate) type AgentSelection = Rc<Cell<Option<(usize, usize)>>>;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AgentDetails {
    pub name: String,
    pub description: String,
    pub kind: String,
    pub status: String,
    /// Only an active tool progress event establishes liveness, never history.
    pub live: bool,
    pub activity: Option<String>,
    pub model: Option<String>,
    pub tags: Vec<String>,
    pub id: Option<String>,
    pub run_id: Option<String>,
    pub tools: Option<u64>,
    pub tokens: Option<String>,
    pub duration_secs: Option<u64>,
    pub turns: Option<u64>,
    pub max_turns: Option<u64>,
    pub cost: Option<f64>,
    pub error: Option<String>,
    pub outcome: Option<String>,
}

impl AgentDetails {
    /// Match the extension's structured AgentDetails shape, not tool names or
    /// prose. Optional malformed fields do not hide otherwise useful facts.
    pub fn from_details(value: &Value) -> Option<Self> {
        let text = |key| {
            value
                .get(key)
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
        };
        Some(Self {
            name: text("displayName")?,
            kind: text("subagentType")?,
            description: text("description").unwrap_or_default(),
            status: text("status")?,
            live: false,
            activity: text("activity"),
            model: text("modelName"),
            tags: value
                .get("tags")
                .and_then(Value::as_array)
                .map(|tags| {
                    tags.iter()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect()
                })
                .unwrap_or_default(),
            id: text("agentId"),
            run_id: text("runId"),
            tools: value.get("toolUses").and_then(Value::as_u64),
            tokens: text("tokens"),
            // Spinner frames and subsecond clocks must not invalidate every row.
            duration_secs: value
                .get("durationMs")
                .and_then(Value::as_u64)
                .map(|ms| ms / 1000),
            turns: value.get("turnCount").and_then(Value::as_u64),
            max_turns: value
                .get("maxTurns")
                .and_then(Value::as_u64)
                .filter(|n| *n > 0),
            // Zero also means unknown model pricing in pi-subagents.
            cost: value
                .get("cost")
                .and_then(Value::as_f64)
                .filter(|n| n.is_finite() && *n > 0.),
            error: text("error"),
            outcome: text("taskOutcome"),
        })
    }

    pub fn state_label(&self) -> String {
        let label = match self.status.as_str() {
            "queued" => tr!("agents.queued"),
            "running" => tr!("agents.running"),
            "completed" => tr!("agents.completed"),
            "steered" => tr!("agents.steered"),
            "aborted" | "stopped" => tr!("agents.stopped"),
            "error" => tr!("agents.failed"),
            // The spawn tool has returned, not the child. Never paint a check.
            "background" => tr!("agents.background"),
            "unknown" => tr!("agents.unknown"),
            other => format!("{} ({other})", tr!("agents.unknown")),
        };
        if !self.live && matches!(self.status.as_str(), "running" | "queued") {
            tr!("agents.last_state", state = label)
        } else {
            label
        }
    }

    pub fn tone(&self, theme: Theme) -> gpui::Hsla {
        match self.status.as_str() {
            "error" => theme.del_red,
            "completed" => theme.ok_green,
            _ => theme.text_2,
        }
    }

    pub fn summary(&self) -> String {
        let mut parts = vec![self.state_label()];
        if let Some(secs) = self.duration_secs {
            parts.push(format!("{}:{:02}", secs / 60, secs % 60));
        }
        if let Some(tools) = self.tools {
            parts.push(tr!("agents.tools", count = tools));
        }
        if let Some(tokens) = &self.tokens {
            parts.push(tokens.clone());
        }
        parts.join(" · ")
    }
}

/// A fixed-height addition to the ordinary tool card; raw arguments/output
/// remain available through its existing disclosure.
pub(crate) fn summary_row(
    agent: &AgentDetails,
    key: (usize, usize),
    selection: AgentSelection,
    theme: Theme,
) -> AnyElement {
    div()
        .h(px(30.))
        .px(px(10.))
        .flex()
        .items_center()
        .gap(px(8.))
        .border_t_1()
        .border_color(theme.border)
        .text_size(theme.ui_px(11.))
        .text_color(agent.tone(theme))
        .child(div().flex_1().min_w_0().truncate().child(agent.summary()))
        .child(
            div()
                .id(ElementId::NamedInteger(
                    "inspect-agent".into(),
                    (key.0 as u64) << 32 | key.1 as u64,
                ))
                .debug_selector(move || format!("inspect-agent-{}-{}", key.0, key.1))
                .flex_none()
                .px(px(6.))
                .py(px(3.))
                .rounded(px(4.))
                .text_color(theme.text)
                .cursor_pointer()
                .hover(|s| s.bg(theme.overlay_strong))
                .child(tr!("agents.inspect"))
                .on_click(move |_, _, cx| {
                    selection.set(Some(key));
                    cx.stop_propagation();
                    cx.refresh_windows();
                }),
        )
        .into_any_element()
}

/// Shared with the layout calculation so narrow windows keep a usable chat.
pub(crate) fn inspector_width(available: Pixels) -> Pixels {
    px((f32::from(available) * 0.45).clamp(240., 460.))
}

pub(crate) fn inspector(
    tool: &ToolCall,
    selection: AgentSelection,
    width: Pixels,
    theme: Theme,
) -> Option<AnyElement> {
    let agent = tool.facts.agent.as_ref()?;
    let mut body = div()
        .id("agent-inspector-scroll")
        .flex_1()
        .min_w_0()
        .min_h_0()
        .whitespace_normal()
        .overflow_y_scroll()
        .p(px(16.))
        .flex()
        .flex_col()
        .gap(px(14.))
        .text_size(theme.ui_px(13.))
        .text_color(theme.text);
    body = body
        .child(
            div()
                .flex_none()
                .font_weight(FontWeight::SEMIBOLD)
                .child(agent.name.clone()),
        )
        .child(
            div()
                .flex_none()
                .text_color(agent.tone(theme))
                .child(agent.summary()),
        )
        .child(
            div()
                .text_color(theme.text_3)
                .text_size(theme.ui_px(11.))
                .flex_none()
                .child(tr!("agents.snapshot_hint")),
        );
    let mut facts = vec![(tr!("agents.type"), agent.kind.clone())];
    if let Some(model) = &agent.model {
        facts.push((tr!("session.detail_model"), model.clone()));
    }
    if !agent.tags.is_empty() {
        facts.push((tr!("agents.configuration"), agent.tags.join(" · ")));
    }
    if let Some(id) = &agent.id {
        facts.push((tr!("agents.id"), id.clone()));
    }
    if let Some(id) = &agent.run_id {
        facts.push((tr!("agents.run_id"), id.clone()));
    }
    if let Some(turns) = agent.turns {
        facts.push((
            tr!("agents.turns"),
            agent
                .max_turns
                .map_or_else(|| turns.to_string(), |max| format!("{turns} / {max}")),
        ));
    }
    if let Some(cost) = agent.cost {
        let cost = if cost < 0.0001 {
            "<$0.0001".into()
        } else {
            format!("~${cost:.4}")
        };
        facts.push((tr!("agents.estimated_cost"), cost));
    }
    if let Some(outcome) = &agent.outcome {
        facts.push((tr!("agents.outcome"), outcome.clone()));
    }
    body = body.children(facts.into_iter().map(|(label, value)| {
        div()
            .flex_none()
            .flex()
            .gap(px(8.))
            .child(
                div()
                    .w(px(100.))
                    .flex_none()
                    .text_color(theme.text_3)
                    .child(label),
            )
            .child(div().flex_1().min_w_0().whitespace_normal().child(value))
    }));
    if !agent.description.is_empty() {
        body = body.child(text_section(
            "agent-description",
            tr!("agents.task"),
            &agent.description,
            theme,
        ));
    }
    if let Some(activity) = &agent.activity {
        body = body.child(text_section(
            "agent-activity",
            tr!("agents.activity"),
            activity,
            theme,
        ));
    }
    if let Some(prompt) = tool
        .args
        .as_ref()
        .and_then(|a| a.get("prompt"))
        .and_then(Value::as_str)
    {
        body = body.child(text_section(
            "agent-prompt",
            tr!("agents.prompt"),
            prompt,
            theme,
        ));
    }
    if let Some(error) = &agent.error {
        body = body.child(text_section(
            "agent-error",
            tr!("settings.last_error"),
            error,
            theme,
        ));
    }
    if let Some(output) = &tool.output {
        let text = output
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| output.to_string());
        body = body.child(text_section(
            "agent-output",
            tr!("agents.reported_output"),
            &text,
            theme,
        ));
    }
    Some(
        div()
            .id("agent-inspector")
            .debug_selector(|| "agent-inspector".into())
            .w(width)
            .h_full()
            .overflow_hidden()
            .flex_none()
            .min_h_0()
            .flex()
            .flex_col()
            .border_l_1()
            .border_color(theme.border)
            .bg(theme.bg_sidebar)
            .child(
                div()
                    .h(px(44.))
                    .flex_none()
                    .pl(px(12.))
                    .pr(px(if crate::platform::draws_window_controls() {
                        crate::platform::WINDOW_CONTROLS_W + 6.
                    } else {
                        12.
                    }))
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_size(theme.ui_px(13.))
                            .text_color(theme.text)
                            .child(tr!("agents.inspector")),
                    )
                    .child(
                        div()
                            .id("close-agent-inspector")
                            .debug_selector(|| "close-agent-inspector".into())
                            .flex_none()
                            .px(px(6.))
                            .py(px(4.))
                            .rounded(px(4.))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.overlay_strong))
                            .text_size(theme.ui_px(12.))
                            .text_color(theme.text_2)
                            .child(tr!("settings.close"))
                            .on_click(move |_, _, cx| {
                                selection.set(None);
                                cx.refresh_windows();
                            }),
                    ),
            )
            .child(body)
            .into_any_element(),
    )
}

fn text_section(id: &'static str, title: String, text: &str, theme: Theme) -> AnyElement {
    // Bounded paint, full copy. No file reads, Markdown image loads, or tool
    // execution are triggered by inspecting untrusted child output.
    const PREVIEW_CHARS: usize = 12_000;
    let mut chars = text.chars();
    let preview: String = chars.by_ref().take(PREVIEW_CHARS).collect();
    let truncated = chars.next().is_some();
    let copy = text.to_owned();
    div()
        .flex_none()
        .flex()
        .flex_col()
        .gap(px(6.))
        .min_w_0()
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .child(div().font_weight(FontWeight::SEMIBOLD).child(title))
                .child(
                    div()
                        .id(id)
                        .cursor_pointer()
                        .text_color(theme.text_3)
                        .child(tr!("menu.copy"))
                        .on_click(move |_, _, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(copy.clone()));
                        }),
                ),
        )
        .child(
            div()
                .whitespace_normal()
                .text_color(theme.text_2)
                .child(preview),
        )
        .when(truncated, |section| {
            section.child(
                div()
                    .text_color(theme.text_3)
                    .child(tr!("agents.preview_limited")),
            )
        })
        .into_any_element()
}

#[cfg(test)]
#[path = "agents_ui_tests.rs"]
mod ui_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn unrelated_or_incomplete_details_are_not_agents() {
        for value in [
            json!({"status":"running"}),
            json!({"displayName":"Scout","subagentType":4,"status":"running"}),
            Value::Null,
        ] {
            assert!(AgentDetails::from_details(&value).is_none());
        }
    }

    #[test]
    fn optional_malformed_fields_do_not_hide_identity() {
        let agent = AgentDetails::from_details(&json!({"displayName":"Scout","subagentType":"scout","status":"running","cost":"bad","tags":["thinking: high",4],"toolUses":-1})).unwrap();
        assert_eq!(
            (agent.cost, agent.tools, agent.tags),
            (None, None, vec!["thinking: high".to_owned()])
        );
    }

    #[test]
    fn spinner_and_subsecond_updates_are_coalesced() {
        let base = json!({"displayName":"Scout","subagentType":"scout","status":"running","durationMs":1200,"spinnerFrame":1});
        let mut next = base.clone();
        next["durationMs"] = json!(1280);
        next["spinnerFrame"] = json!(2);
        assert_eq!(
            AgentDetails::from_details(&base),
            AgentDetails::from_details(&next)
        );
    }

    #[test]
    fn unknown_pricing_is_not_a_free_run() {
        let agent = AgentDetails::from_details(
            &json!({"displayName":"Scout","subagentType":"scout","status":"completed","cost":0}),
        )
        .unwrap();
        assert_eq!(agent.cost, None);
    }

    #[test]
    fn future_states_are_preserved_not_reported_as_completed() {
        let agent = AgentDetails::from_details(
            &json!({"displayName":"Scout","subagentType":"scout","status":"waiting_for_review"}),
        )
        .unwrap();
        assert_eq!(agent.status, "waiting_for_review");
    }
}
