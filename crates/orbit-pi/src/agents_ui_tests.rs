use super::*;
use gpui::{size, Context, Modifiers, Render, Window};
use serde_json::json;

struct InspectorTestView {
    tool: ToolCall,
    selection: AgentSelection,
    width: Pixels,
}

impl Render for InspectorTestView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::for_id(crate::theme::ThemeId::Orbit);
        let mut root = div().size_full().flex();
        root = root.child(div().flex_1().min_w_0().child(summary_row(
            self.tool.facts.agent.as_ref().unwrap(),
            (0, 0),
            self.selection.clone(),
            theme,
        )));
        if self.selection.get().is_some() {
            root = root.children(inspector(
                &self.tool,
                self.selection.clone(),
                self.width,
                theme,
            ));
        }
        root
    }
}

fn tool_fixture() -> ToolCall {
    ToolCall {
        name: "Agent".into(),
        summary: String::new(),
        path: None,
        added: 0,
        removed: 0,
        id: Some("call-1".into()),
        args: Some(json!({"prompt":"Inspect collision handling"})),
        output: Some(json!("3 tool uses...")),
        failed: false,
        facts: crate::transcript::ToolFacts {
            agent: AgentDetails::from_details(&json!({
                "displayName":"Scout", "subagentType":"scout", "status":"running",
                "description":"Inspect collision handling", "toolUses":3,
                "modelName":"test-model", "durationMs":1200
            }))
            .map(Box::new),
            ..Default::default()
        },
    }
}

#[gpui::test]
fn agents_inspect_opens_and_closes_native_panel(cx: &mut gpui::TestAppContext) {
    let selection: AgentSelection = Rc::new(Cell::new(None));
    // Use a real root entity: add_empty_window + draw would replace click
    // listeners with Empty during the automatic mouse-down repaint.
    let (_, cx) = cx.add_window_view(|_, _| InspectorTestView {
        tool: tool_fixture(),
        selection: selection.clone(),
        width: px(400.),
    });
    cx.simulate_resize(size(px(1000.), px(700.)));
    cx.run_until_parked();
    let button = cx.debug_bounds("inspect-agent-0-0").unwrap().center();
    cx.simulate_click(button, Modifiers::none());
    assert_eq!(selection.get(), Some((0, 0)));
    assert_eq!(
        cx.debug_bounds("agent-inspector").unwrap().size.width,
        px(400.)
    );
    let close = cx.debug_bounds("close-agent-inspector").unwrap().center();
    cx.simulate_click(close, Modifiers::none());
    assert_eq!(selection.get(), None);
}

#[gpui::test]
fn agents_inspector_fits_narrow_and_wide_docks(cx: &mut gpui::TestAppContext) {
    let (view, cx) = cx.add_window_view(|_, _| InspectorTestView {
        tool: tool_fixture(),
        selection: Rc::new(Cell::new(Some((0, 0)))),
        width: px(400.),
    });
    for width in [240., 400., 460.] {
        cx.simulate_resize(size(px(width + 360.), px(700.)));
        view.update(cx, |view, cx| {
            view.width = px(width);
            cx.notify();
        });
        cx.run_until_parked();
        let panel = cx.debug_bounds("agent-inspector").unwrap();
        let close = cx.debug_bounds("close-agent-inspector").unwrap();
        assert_eq!(panel.size.width, px(width));
        assert!(
            close.left() >= panel.left() && close.right() <= panel.right(),
            "close button overflows at {width}: {close:?}"
        );
    }
}
