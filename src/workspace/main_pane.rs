use gpui::{Context, IntoElement, Window, div, prelude::*};
use mod_details::ModDetails;
use mod_list::ModList;

mod mod_details;
mod mod_list;

pub struct MainPane;

impl Render for MainPane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mods = [
            ("mod.one".to_string(), "Mod One".to_string()),
            ("mod.two".to_string(), "Mod Two".to_string()),
            ("mod.three".to_string(), "Mod Three".to_string()),
        ];

        let active_ids = ["mod.one".to_string()];

        let active_mods: Vec<_> = mods
            .iter()
            .filter(|(id, _)| active_ids.contains(id))
            .cloned()
            .collect();

        let inactive_mods: Vec<_> = mods
            .iter()
            .filter(|(id, _)| !active_ids.contains(id))
            .cloned()
            .collect();

        div()
            .size_full()
            .flex_grow()
            .flex()
            .flex_row()
            .child(cx.new(|_| ModList::new("Inactive".to_string(), inactive_mods)))
            .child(cx.new(|_| ModList::new("Active".to_string(), active_mods)))
            .child(cx.new(|_| ModDetails {}))
    }
}
