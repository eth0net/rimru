use crate::{
    project::Project,
    ui::{ModDetails, ModList, prelude::*},
};

pub struct MainPane {
    // todo: subscribe to project changes
    // project: Entity<Project>,
    active_list: Entity<ModList>,
    inactive_list: Entity<ModList>,
    details_pane: Entity<ModDetails>,
}

impl MainPane {
    pub fn new(project: Entity<Project>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        MainPane {
            active_list: cx.new(|cx| ModList::new_active(project.clone(), window, cx)),
            inactive_list: cx.new(|cx| ModList::new_inactive(project.clone(), window, cx)),
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
