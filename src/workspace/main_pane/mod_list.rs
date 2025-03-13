use std::fmt::Display;

use gpui::{
    Context, Entity, IntoElement, MouseButton, SharedString, TextOverflow, Window, div, prelude::*,
    rgb, uniform_list,
};

use crate::{project::Project, theme::colours};

pub struct ModList {
    project: Entity<Project>,
    list_type: ModListType,
    // mods: Vec<Mod>, // todo: cache list
}

impl ModList {
    pub fn new_active(project: Entity<Project>) -> Self {
        Self::new(ModListType::Active, project)
    }

    pub fn new_inactive(project: Entity<Project>) -> Self {
        Self::new(ModListType::Inactive, project)
    }

    pub fn new(list_type: ModListType, project: Entity<Project>) -> Self {
        Self { project, list_type }
    }
}

impl Render for ModList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let project = self.project.clone();
        let list_name = SharedString::from(self.list_type.to_string());
        let mods = project.read_with(cx, |project, _| match self.list_type {
            ModListType::Active => project.active_mods(),
            ModListType::Inactive => project.inactive_mods(),
        });

        div()
            .flex()
            .flex_col()
            .h_full()
            .w_1_4()
            .border_r_1()
            .border_color(rgb(colours::BORDER))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .px_2()
                    .py_1()
                    .child(self.list_type.to_string()),
            )
            .child(
                uniform_list(cx.entity().clone(), list_name.clone(), mods.len(), {
                    let mods = mods.clone();
                    let list_name = list_name.clone();
                    move |_this, range, _window, _cx| {
                        let mut items = Vec::new();
                        for ix in range {
                            let mod_meta = mods[ix].clone();
                            let mod_name = mod_meta.name.clone();
                            items.push(
                                div()
                                    .id((list_name.clone(), ix))
                                    .cursor_pointer()
                                    .px_2()
                                    .text_overflow(TextOverflow::Ellipsis("..."))
                                    .on_click({
                                        let mod_meta = mod_meta.clone();
                                        let project = project.clone();
                                        move |event, _window, cx| {
                                            match event.down.button {
                                                MouseButton::Left => {
                                                    match event.down.click_count {
                                                        1 => {
                                                            // Select
                                                            log::debug!("select {mod_meta:?}");
                                                            project.update(cx, {
                                                                let mod_meta = mod_meta.clone();
                                                                move |project, _| {
                                                                    project.select_mod(&mod_meta);
                                                                }
                                                            });
                                                        }
                                                        2 => {
                                                            // Activate/deactivate
                                                            log::debug!("toggle {mod_meta:?}");
                                                            project.update(cx, {
                                                                let mod_meta = mod_meta.clone();
                                                                move |project, _| {
                                                                    project.toggle_mod(&mod_meta);
                                                                }
                                                            });
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                MouseButton::Right => {
                                                    // Open context menu
                                                    log::debug!("context menu {mod_meta:?}");
                                                }
                                                _ => {
                                                    log::debug!("unhandled click {mod_meta:?}")
                                                }
                                            }
                                        }
                                    })
                                    .child(mod_name),
                            );
                        }
                        items
                    }
                })
                .flex_grow(),
            )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModListType {
    Active,
    Inactive,
}

impl Display for ModListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModListType::Active => write!(f, "Active"),
            ModListType::Inactive => write!(f, "Inactive"),
        }
    }
}
