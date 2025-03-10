use gpui::{Context, IntoElement, SharedString, Window, div, prelude::*};
use mod_details::ModDetails;
use mod_list::ModList;

mod mod_details;
mod mod_list;

pub struct MainPane;

impl Render for MainPane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // this is the list of all mods installed, sourced from the mods directory
        let mods: Vec<(SharedString, SharedString)> = vec![
            ("mod.one".into(), "Mod One".into()),
            ("mod.two".into(), "Mod Two".into()),
            ("mod.three".into(), "Mod Three".into()),
        ];

        // this is the list of active mod ids, sourced from the config or save file
        let active_ids: Vec<SharedString> = vec!["mod.one".into()];

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
            .child(cx.new(|_| ModList::new("Inactive".into(), inactive_mods)))
            .child(cx.new(|_| ModList::new("Active".into(), active_mods)))
            .child(cx.new(|_| ModDetails {}))
    }
}
