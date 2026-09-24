//! Close confirmation for window controls and the app-level Quit command.
//!
//! A close request is always interrupted once so an accidental click cannot
//! tear down live agent processes. Intentional updater shutdowns still call
//! `App::quit` directly and bypass this user-facing prompt.

use super::helpers::*;
use super::*;

impl OrbitApp {
    /// Intercept an OS/window-caption close request and show the confirmation.
    /// Returning `false` keeps the window alive while the user decides.
    pub(crate) fn window_should_close(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.open_quit_confirmation(window, cx);
        false
    }

    /// The menu/keybinding path for Quit. It uses the same prompt as the OS
    /// close button so every user-initiated exit has identical safeguards.
    pub(super) fn on_quit(&mut self, _: &crate::Quit, window: &mut Window, cx: &mut Context<Self>) {
        self.open_quit_confirmation(window, cx);
    }

    fn open_quit_confirmation(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.quit_confirmation_open = true;
        window.focus(&self.quit_confirmation_focus);
        cx.notify();
    }

    fn dismiss_quit_confirmation(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.quit_confirmation_open {
            self.quit_confirmation_open = false;
            self.input.read(cx).focus(window);
            cx.notify();
        }
    }

    pub(super) fn on_quit_dialog_cancel(
        &mut self,
        _: &crate::QuitDialogCancel,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.dismiss_quit_confirmation(window, cx);
    }

    pub(super) fn on_quit_dialog_confirm(
        &mut self,
        _: &crate::QuitDialogConfirm,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // `App::quit` is deliberate here: routing the confirmed choice back
        // through the window close callback would immediately reopen the
        // prompt instead of exiting.
        cx.quit();
    }

    fn has_in_progress_sessions(&self) -> bool {
        self.busy
            || self.transcript.is_streaming()
            || self.is_compacting
            || self.lives.values().any(|session| session.busy)
    }

    /// Blocking confirmation rendered above all workbench surfaces.
    pub(super) fn quit_confirmation_layer(
        &self,
        window: &Window,
        cx: &Context<Self>,
    ) -> Option<AnyElement> {
        if !self.quit_confirmation_open {
            return None;
        }

        let theme = *theme::get(cx);
        let card_width = (window.viewport_size().width - px(48.)).min(px(440.));
        let detail = if self.has_in_progress_sessions() {
            tr!("quit_dialog.running_detail")
        } else {
            tr!("quit_dialog.detail")
        };

        let quiet_button = press(
            div()
                .id("quit-dialog-cancel")
                .group(BUTTON_GROUP)
                .h(px(32.))
                .px(px(14.))
                .rounded(px(8.))
                .border_1()
                .border_color(theme.border)
                .bg(theme.bg_raised)
                .text_color(theme.text_2)
                .cursor_pointer()
                .flex()
                .items_center()
                .justify_center()
                .text_size(theme.ui_px(12.5))
                .font_weight(FontWeight::MEDIUM)
                .hover(|style| style.bg(theme.bg_hover))
                .on_mouse_up(
                    MouseButton::Left,
                    cx.listener(|this, _: &MouseUpEvent, window, cx| {
                        this.dismiss_quit_confirmation(window, cx)
                    }),
                ),
        )
        .child(tr!("view.cancel"));

        let quit_button = press(
            div()
                .id("quit-dialog-confirm")
                .group(BUTTON_GROUP)
                .h(px(32.))
                .px(px(14.))
                .rounded(px(8.))
                .border_1()
                .border_color(theme.border)
                .bg(theme.bg_raised)
                .text_color(theme.crit)
                .cursor_pointer()
                .flex()
                .items_center()
                .justify_center()
                .text_size(theme.ui_px(12.5))
                .font_weight(FontWeight::MEDIUM)
                .hover(move |style| {
                    style
                        .border_color(theme.crit.opacity(0.6))
                        .bg(theme.crit.opacity(0.08))
                })
                .on_mouse_up(MouseButton::Left, |_, _, cx| cx.quit()),
        )
        .child(tr!("menu.quit", app = tr!("app.name")));

        let card = div()
            .id("quit-dialog-card")
            .w(card_width)
            .rounded(px(14.))
            .popover_surface(theme)
            .flex()
            .flex_col()
            .overflow_hidden()
            .occlude()
            .font_family(theme::ui_font_family())
            .child(
                div()
                    .px(px(18.))
                    .pt(px(16.))
                    .pb(px(8.))
                    .text_size(theme.ui_px(14.5))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.text)
                    .child(tr!("quit_dialog.title")),
            )
            .child(
                div()
                    .px(px(18.))
                    .pb(px(16.))
                    .flex()
                    .items_start()
                    .gap(px(12.))
                    .child(embedded_image(crate::app_icon::ASSET, 44.))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .whitespace_normal()
                            .text_size(theme.ui_px(13.))
                            .text_color(theme.text_2)
                            .child(detail),
                    ),
            )
            .child(
                div()
                    .px(px(18.))
                    .py(px(14.))
                    .flex()
                    .justify_end()
                    .gap(px(8.))
                    .child(quiet_button)
                    .child(quit_button),
            );

        Some(
            div()
                .id("quit-dialog-layer")
                .absolute()
                .inset_0()
                .occlude()
                .bg(theme.scrim_modal())
                .px(px(24.))
                .flex()
                .items_center()
                .justify_center()
                .key_context("QuitDialog")
                .track_focus(&self.quit_confirmation_focus)
                .on_action(cx.listener(Self::on_quit_dialog_cancel))
                .on_action(cx.listener(Self::on_quit_dialog_confirm))
                .child(card)
                .into_any_element(),
        )
    }
}
