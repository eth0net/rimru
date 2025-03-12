use gpui::{Context, IntoElement, Window, div, prelude::*};

use crate::game::mods::Mod;

pub struct ModDetails {
    mod_meta: Option<Mod>,
}

impl ModDetails {
    pub fn new(mod_meta: Option<Mod>) -> Self {
        Self { mod_meta }
    }
}

impl Render for ModDetails {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                    .when(self.mod_meta.is_some(), |this| {
                        let mod_meta = self.mod_meta.as_ref().unwrap();
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
