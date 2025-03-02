use gpui::{ParentElement, Render, Styled, div, rgb};

pub struct Rimru;

impl Render for Rimru {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x0d1117))
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", "world"))
    }
}
