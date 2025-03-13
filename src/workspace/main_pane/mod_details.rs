use gpui::{Context, Entity, IntoElement, Window, div, prelude::*, rgb};

use crate::{project::Project, theme::colours};

pub struct ModDetails {
    project: Entity<Project>,
}

impl ModDetails {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl Render for ModDetails {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.project.read(cx).selected_mod();
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
                    .id("mod-details")
                    .flex()
                    .flex_col()
                    .gap_1()
                    .px_2()
                    .py_1()
                    .overflow_y_scroll()
                    .when(selected.is_some(), |this| {
                        let mod_meta = selected.unwrap();
                        // todo: display mod image
                        this.child(div().child(mod_meta.name.clone()))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .text_sm()
                                    .text_color(rgb(colours::TEXT_SECONDARY))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .justify_start()
                                            .items_center()
                                            .gap_2()
                                            .child(format!("[ id: {} ]", mod_meta.id.clone()))
                                            .when(mod_meta.steam_app_id.is_some(), |this| {
                                                this.child(format!(
                                                    "[ steam: {} ]",
                                                    mod_meta
                                                        .steam_app_id
                                                        .clone()
                                                        .unwrap_or_default()
                                                ))
                                            }),
                                    )
                                    .child(div().child(format!(
                                        "Authors: {}",
                                        mod_meta.authors.join(", ")
                                    ))),
                            )
                            .child(div().child(mod_meta.description.clone()))
                    }),
            )
    }
}
