use gpui::{Context, Entity, IntoElement, Window, div, prelude::*};
use mod_details::ModDetails;
use mod_list::ModList;

use crate::project::Project;

mod mod_details;
mod mod_list;

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
            active_list: cx.new(|_| ModList::new_active(project.clone())),
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
            .child(self.inactive_list.clone())
            .child(self.active_list.clone())
            .child(self.details_pane.clone())
    }
}
