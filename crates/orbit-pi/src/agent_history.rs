//! Disk-only child-session inspection. This view has no RPC client, composer,
//! session-switch callback, or executable tool actions.
use crate::{sessions::SessionInfo, theme};
use gpui::{
    div, list, prelude::*, px, App, ClipboardItem, Context, FocusHandle, ListAlignment, ListState,
    Render, Task, Window,
};
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
    rc::Rc,
};

const MAX_BYTES: u64 = 8 * 1024 * 1024;
const MAX_TEXT: usize = 12_000;

#[derive(Clone, Debug)]
struct SavedMessage {
    role: String,
    text: String,
}
#[derive(Default)]
struct Snapshot {
    messages: Vec<SavedMessage>,
    limited: bool,
}

fn read_snapshot(path: &Path) -> std::io::Result<Snapshot> {
    let file = File::open(path)?;
    let mut snapshot = Snapshot {
        limited: file.metadata()?.len() > MAX_BYTES,
        ..Default::default()
    };
    let mut entries = HashMap::new();
    let mut leaf = None;
    for line in BufReader::new(file.take(MAX_BYTES)).lines() {
        let Ok(v) = serde_json::from_str::<Value>(&line?) else {
            continue;
        };
        if let Some(id) = v["id"].as_str().filter(|_| v["type"] != "session") {
            leaf = Some(id.to_owned());
            entries.insert(id.to_owned(), v);
        }
    }
    // Follow the saved active branch, not every abandoned branch in the file.
    let mut seen = HashSet::new();
    while let Some(id) = leaf.take() {
        if !seen.insert(id.clone()) {
            snapshot.limited = true;
            break;
        }
        let Some(v) = entries.get(&id) else {
            snapshot.limited = true;
            break;
        };
        leaf = v["parentId"].as_str().map(str::to_owned);
        if v["type"] != "message" {
            continue;
        }
        let message = &v["message"];
        let Some(role) = message["role"].as_str().filter(|role| *role != "system") else {
            continue;
        };
        let content = &message["content"];
        let text = if let Some(s) = content.as_str() {
            s.to_owned()
        } else {
            content
                .as_array()
                .into_iter()
                .flatten()
                .map(|b| {
                    if let Some(text) = b["text"].as_str().or_else(|| b["thinking"].as_str()) {
                        text.to_owned()
                    } else if b["type"] == "toolCall" {
                        format!(
                            "{}\n{}",
                            b["name"].as_str().unwrap_or("toolCall"),
                            b["arguments"]
                        )
                    } else {
                        b["type"].as_str().unwrap_or_default().to_owned()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        };
        let limited: String = text.chars().take(MAX_TEXT).collect();
        snapshot.limited |= limited.len() != text.len();
        snapshot.messages.push(SavedMessage {
            role: role.into(),
            text: limited,
        });
    }
    snapshot.messages.reverse();
    Ok(snapshot)
}

type CloseHistory = Box<dyn Fn(&mut Window, &mut App)>;

pub(crate) struct AgentHistory {
    session: SessionInfo,
    messages: Rc<Vec<SavedMessage>>,
    list: ListState,
    loading: bool,
    limited: bool,
    error: Option<String>,
    task: Option<Task<()>>,
    focus: FocusHandle,
    focus_pending: bool,
    pub header_leading: f32,
    close: CloseHistory,
}

impl AgentHistory {
    pub fn new(session: SessionInfo, close: CloseHistory, cx: &mut Context<Self>) -> Self {
        let mut view = Self {
            session,
            messages: Rc::new(Vec::new()),
            list: ListState::new(0, ListAlignment::Top, px(400.)),
            loading: false,
            limited: false,
            error: None,
            task: None,
            focus: cx.focus_handle(),
            focus_pending: true,
            header_leading: 12.,
            close,
        };
        view.reload(cx);
        view
    }

    pub fn path(&self) -> &Path {
        &self.session.path
    }

    pub fn focus(&self, window: &mut Window) {
        window.focus(&self.focus);
    }

    fn reload(&mut self, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }
        self.loading = true;
        let path = self.session.path.clone();
        self.task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move { read_snapshot(&path) })
                .await;
            // The entity may have been closed while disk I/O was pending.
            this.update(cx, |view, cx| {
                view.loading = false;
                match result {
                    Ok(snapshot) => {
                        view.error = None;
                        view.limited = snapshot.limited;
                        view.messages = Rc::new(snapshot.messages);
                        view.list.reset(view.messages.len());
                    }
                    Err(error) => {
                        view.error = Some(error.to_string());
                    }
                }
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }
}

impl Render for AgentHistory {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.focus_pending {
            window.focus(&self.focus);
            self.focus_pending = false;
        }
        let theme = *theme::get(cx);
        let messages = self.messages.clone();
        div()
            .id("agent-history")
            .debug_selector(|| "agent-history".into())
            .track_focus(&self.focus)
            .flex_1()
            .min_w_0()
            .h_full()
            .min_h_0()
            .flex()
            .flex_col()
            .bg(theme.bg_main)
            .text_color(theme.text)
            .text_size(theme.ui_px(13.))
            .child(
                div()
                    .h(px(44.))
                    .flex_none()
                    .flex()
                    .items_center()
                    .gap(px(12.))
                    .pl(px(self.header_leading))
                    .pr(px(if crate::platform::draws_window_controls() {
                        crate::platform::WINDOW_CONTROLS_W + 12.
                    } else {
                        12.
                    }))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .truncate()
                            .child(self.session.title.clone()),
                    )
                    .child(
                        div()
                            .id("refresh-agent-history")
                            .cursor_pointer()
                            .child(tr!("agents.refresh_history"))
                            .on_click(cx.listener(|view, _, _, cx| view.reload(cx))),
                    )
                    .child(
                        div()
                            .id("close-agent-history")
                            .debug_selector(|| "close-agent-history".into())
                            .cursor_pointer()
                            .child(tr!("settings.close"))
                            .on_click(cx.listener(|view, _, window, cx| (view.close)(window, cx))),
                    ),
            )
            .child(
                div()
                    .px(px(16.))
                    .py(px(8.))
                    .flex_none()
                    .whitespace_normal()
                    .text_color(theme.text_3)
                    .child(tr!("agents.history_hint")),
            )
            .when(self.loading, |el| {
                el.child(div().px(px(16.)).child(tr!("agents.loading_history")))
            })
            .when(
                !self.loading && self.error.is_none() && self.messages.is_empty(),
                |el| {
                    el.child(
                        div()
                            .px(px(16.))
                            .text_color(theme.text_3)
                            .child(tr!("agents.history_empty")),
                    )
                },
            )
            .when(self.limited, |el| {
                el.child(
                    div()
                        .px(px(16.))
                        .text_color(theme.text_3)
                        .child(tr!("agents.history_limited")),
                )
            })
            .when_some(self.error.clone(), |el, error| {
                el.child(div().px(px(16.)).text_color(theme.del_red).child(error))
            })
            .child(
                div().flex_1().min_h_0().child(
                    list(self.list.clone(), move |ix, _, _| {
                        let message = &messages[ix];
                        let copy = message.text.clone();
                        div()
                            .px(px(18.))
                            .py(px(12.))
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap(px(8.))
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .text_color(theme.text_3)
                                    .child(message.role.clone())
                                    .child(
                                        div()
                                            .id(("copy-agent-history", ix))
                                            .cursor_pointer()
                                            .child(tr!("agents.copy_preview"))
                                            .on_click(move |_, _, cx| {
                                                cx.write_to_clipboard(ClipboardItem::new_string(
                                                    copy.clone(),
                                                ))
                                            }),
                                    ),
                            )
                            .child(div().whitespace_normal().child(message.text.clone()))
                            .into_any_element()
                    })
                    .h_full(),
                ),
            )
    }
}

#[cfg(test)]
#[path = "agent_history_tests.rs"]
mod tests;
