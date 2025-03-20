use gpui::{Context, Entity, IntoElement, Window, div, img, prelude::*, relative, rgba};

use crate::{project::Project, theme::colors};

pub struct ModDetails {
    project: Entity<Project>,
}

impl ModDetails {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

// todo: remove details header?
// todo: add placeholder with no selected mod
impl Render for ModDetails {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.project.read(cx).selected_mod();
        div()
            .flex()
            .flex_col()
            .h_full()
            .w(relative(0.4))
            .max_w_full()
            .overflow_x_hidden()
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
                    .id("mod-details")
                    .flex()
                    .flex_col()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .overflow_x_hidden()
                    .overflow_y_scroll()
                    .when_some(selected, |this, mod_meta| {
                        this.child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_center()
                                .w_full()
                                .border_1()
                                .border_color(rgba(colors::BORDER))
                                .child({
                                    // todo: optimise image loading
                                    let image_path = mod_meta.preview_file_path();
                                    if let Err(err) = std::fs::metadata(&image_path) {
                                        log::warn!("failed to load mod image: {}", err);
                                    }
                                    img(image_path).max_h_full().max_w_full()
                                }),
                        )
                        .child(div().child(mod_meta.name.clone()))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .text_sm()
                                .text_color(rgba(colors::TEXT_SECONDARY))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .justify_start()
                                        .items_center()
                                        .gap_2()
                                        .child(format!("[ id: {} ]", mod_meta.id.clone()))
                                        .when_some(
                                            mod_meta.steam_app_id.clone(),
                                            |this, steam_app_id| {
                                                this.child(format!("[ steam: {} ]", steam_app_id))
                                            },
                                        ),
                                )
                                .child(
                                    div()
                                        .child(format!("Authors: {}", mod_meta.authors.join(", "))),
                                ),
                        )
                        .child(div().child(mod_meta.description.clone()))
                    }),
            )
    }
}
