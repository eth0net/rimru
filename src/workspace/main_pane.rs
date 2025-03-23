use gpui::{Context, Entity, IntoElement, Window, div, prelude::*};

use crate::{
    project::Project,
    ui::{ModDetails, ModList},
};

pub struct MainPane {
    // todo: subscribe to project changes
    // project: Entity<Project>,
    active_list: Entity<ModList>,
    inactive_list: Entity<ModList>,
    details_pane: Entity<ModDetails>,
}

impl MainPane {
    pub fn new(cx: &mut Context<Self>, project: Entity<Project>) -> Self {
        MainPane {
            // project: project.clone(),
            // todo: add reset action to reset the active list
            // todo: add save action to save the active list
            // todo: add sort action to sort the active list
            active_list: cx.new(|_| ModList::new_active(project.clone())),
            // todo: add refresh action to load new mods while keeping active mods
            // todo: add sort options to sort the inactive list by name, id, date installed, date updated etc
            // todo: add source options to filter the inactive list by official, local, or steam
            inactive_list: cx.new(|_| ModList::new_inactive(project.clone())),
            details_pane: cx.new(|_| ModDetails::new(project.clone())),
        }
    }
}

impl Render for MainPane {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex_grow()
            .flex()
            .flex_row()
            .overflow_hidden()
            .child(self.inactive_list.clone())
            .child(self.active_list.clone())
            .child(self.details_pane.clone())
    }
}
