use gpui::prelude::*;
use gpui::{div, rgb};

mod main_pane;
mod status_bar;
mod title_bar;

pub struct Rimru;

impl Render for Rimru {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x0d1117))
            .text_color(rgb(0xffffff))
            .child(cx.new(|_| title_bar::TitleBar {}))
            .child(cx.new(|_| main_pane::MainPane {}))
            .child(cx.new(|_| status_bar::StatusBar {}))
    }
}
