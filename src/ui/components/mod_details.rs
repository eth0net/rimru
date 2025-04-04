use std::fs;

use gpui::{img, relative};

use crate::{project::Project, theme::colors, ui::prelude::*};

pub struct ModDetails {
    project: Entity<Project>,
}

impl ModDetails {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

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
                    .id("mod-details")
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_2()
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
                                    if let Err(err) = fs::metadata(&image_path) {
                                        log::warn!("failed to load mod image: {}", err);
                                    }
                                    img(image_path).max_h_full().max_w_full()
                                }),
                        )
                        .child(mod_meta.name.clone())
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap_2()
                                .text_sm()
                                .text_color(rgba(colors::TEXT_SECONDARY))
                                .child(mod_meta.id.clone())
                                .child(format!("Authors: {}", mod_meta.authors.join(", "))),
                        )
                        .when(!mod_meta.dependencies.is_empty(), |this| {
                            this.child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .text_sm()
                                    .text_color(rgba(colors::TEXT_SECONDARY))
                                    .child("Depends on:")
                                    .children(
                                        mod_meta.dependencies.keys().map(|id| format!("- {id}",)),
                                    ),
                            )
                        })
                        .child(mod_meta.description.clone())
                    }),
            )
    }
}
