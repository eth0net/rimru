use gpui::prelude::*;
use gpui::{Pixels, Window, div, px, rgb};

pub(crate) struct TitleBar;

impl TitleBar {
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

impl Render for TitleBar {
    fn render(
        &mut self,
        window: &mut Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        let height = Self::height(window);

        div()
            .w_full()
            .h(height)
            .flex()
            .flex_row()
            .items_center()
            .p_2()
            .pl_16()
            .border_b_1()
            .border_color(rgb(0x666666))
            .text_sm()
            .child("rimru".to_string())
    }
}
