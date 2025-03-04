use gpui::{div, prelude::*};

pub struct ModDetails;

impl Render for ModDetails {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex()
            .flex_col()
            .h_full()
            .w_2_4()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .px_2()
                    .py_1()
                    .child("Details".to_string()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .px_2()
                    .py_1()
                    .child("Details Content".to_string()),
            )
    }
}
