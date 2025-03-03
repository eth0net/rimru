use gpui::prelude::*;
use gpui::{Window, div};

pub(crate) struct MainPane;

impl Render for MainPane {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex_grow()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .text_xl()
            .child(format!("Hello, {}!", "world"))
    }
}
