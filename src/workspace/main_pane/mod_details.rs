use gpui::{Context, Entity, IntoElement, Window, div, prelude::*};

use crate::project::Project;

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
                    .flex()
                    .flex_col()
                    .px_2()
                    .py_1()
                    .overflow_x_hidden()
                    .when(selected.is_some(), |this| {
                        let mod_meta = selected.unwrap();
                        this.child(format!(
                            "{} ({})",
                            mod_meta.name.clone(),
                            mod_meta.id.clone()
                        ))
                        .child(format!("Authors: {}", mod_meta.authors.join(", ")))
                        .child(mod_meta.description.clone())
                    }),
            )
    }
}
