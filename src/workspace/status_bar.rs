use gpui::{Context, IntoElement, Pixels, Window, div, prelude::*, px, rgba};

use crate::theme::colors;

pub struct StatusBar;

impl StatusBar {
    #[cfg(not(target_os = "windows"))]
    pub fn height(window: &mut Window) -> Pixels {
        (1.5 * window.rem_size()).max(px(34.0))
    }

    #[cfg(target_os = "windows")]
    pub fn height(_window: &mut Window) -> Pixels {
        // todo(windows) instead of hard coded size report the actual size to the Windows platform API
        px(32.0)
    }
}

impl Render for StatusBar {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let height = Self::height(window);

        div()
            .w_full()
            .h(height)
            .flex()
            .flex_row()
            .items_center()
            .p_2()
            .border_t_1()
            .border_color(rgba(colors::BORDER))
            .text_sm()
            .child("status bar is wip".to_string())
    }
}
