use std::{fmt::Display, fs};

use gpui::{Entity, MouseButton, Stateful, UniformList, relative, uniform_list};

use crate::{game::mods::meta::ModMetaData, project::Project, theme::colors, ui::prelude::*};

// todo: add list actions for refresh / sort etc
pub struct ModList {
    project: Entity<Project>,
    list_name: SharedString,
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
        let list_name = SharedString::from(list_type.to_string());
        Self {
            project,
            list_name,
            list_type,
        }
    }

    fn render_header(&mut self, cx: &mut Context<Self>) -> Div {
        let mods = self.mods_for_list_type(cx);

        let buttons = match self.list_type {
            ModListType::Active => {
                vec![
                    // IconButton::from_name("sort", IconName::Sort),
                    IconButton::from_name("save", IconName::Save).on_click({
                        let project = self.project.clone();
                        move |_, _, cx| {
                            project.update(cx, |project, _| {
                                project.save_mods_config();
                            });
                        }
                    }),
                    IconButton::from_name("reload", IconName::Reload).on_click({
                        let project = self.project.clone();
                        move |_, _, cx| {
                            project.update(cx, |project, _| {
                                project.load_mods_config();
                                project.apply_mods_config();
                            });
                        }
                    }),
                    IconButton::from_name("reset", IconName::Reset).on_click({
                        let project = self.project.clone();
                        move |_, _, cx| {
                            project.update(cx, |project, _| {
                                project.apply_mods_config();
                            });
                        }
                    }),
                    IconButton::from_name("clear", IconName::Clear).on_click({
                        let project = self.project.clone();
                        move |_, _, cx| {
                            project.update(cx, |project, _| {
                                project.clear_active_mods();
                            });
                        }
                    }),
                ]
            }
            ModListType::Inactive => {
                vec![IconButton::from_name("reload", IconName::Reload).on_click({
                    let project = self.project.clone();
                    move |_, _, cx| {
                        project.update(cx, |project, cx| {
                            project.load_mods(cx);
                        });
                    }
                })]
            }
        };

        div()
            .flex()
            .flex_row()
            .justify_center()
            .items_center()
            .gap_4()
            .px_2()
            .py_1()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_start()
                    .pt_0p5()
                    .child(format!("{} ({})", self.list_name, mods.len())),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_start()
                    .children(buttons),
            )
    }

    // todo: add search bar to filter mods in this list
    // todo: preload images for visible mods in this list
    fn render_list(&self, cx: &mut Context<Self>) -> UniformList {
        let mods = self.mods_for_list_type(cx);
        uniform_list(cx.entity().clone(), self.list_name.clone(), mods.len(), {
            move |this, range, _window, _cx| {
                let mut items = Vec::with_capacity(range.end - range.start);
                for ix in range {
                    items.push(this.render_entry(&mods[ix]));
                }
                items
            }
        })
        .flex_grow()
    }

    fn render_entry(&self, mod_meta: &ModMetaData) -> Stateful<Div> {
        let id = SharedString::from(format!("{}-{}", self.list_name, mod_meta.id));
        let mod_name = mod_meta.name.clone();
        div()
            .id(id)
            .cursor_pointer()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .px_2()
            .on_click({
                let mod_meta = mod_meta.clone();
                let project = self.project.clone();
                move |event, _window, cx| match event.down.button {
                    MouseButton::Left => match event.down.click_count {
                        1 => {
                            log::debug!("select {mod_meta:?}");
                            project.update(cx, {
                                let mod_meta = mod_meta.clone();
                                move |project, _| {
                                    project.select_mod(&mod_meta);
                                }
                            });
                        }
                        2 => {
                            log::debug!("toggle {mod_meta:?}");
                            project.update(cx, {
                                let mod_meta = mod_meta.clone();
                                move |project, _| {
                                    project.toggle_mod(&mod_meta);
                                }
                            });
                        }
                        _ => {}
                    },
                    MouseButton::Right => {
                        log::debug!("context menu {mod_meta:?}");
                    }
                    _ => {
                        log::debug!("unhandled click {mod_meta:?}")
                    }
                }
            })
            // todo: highlight selected mod
            .child(
                IconButton::from_name(
                    SharedString::from(format!("{mod_name}-source")),
                    mod_meta.source.icon_name(),
                )
                .style(ButtonStyle::Transparent),
            )
            .child({
                let id = format!("{mod_name}-icon");
                let icon_path = mod_meta.icon_file_path();
                let icon_source = match fs::metadata(&icon_path) {
                    Ok(_) => icon_path.into(),
                    Err(_) => IconName::Unknown.into(),
                };
                IconButton::new(SharedString::from(id), icon_source).style(ButtonStyle::Transparent)
            })
            .child(
                div()
                    .flex_grow()
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(mod_name),
            )
            // todo: indicate if mod is incompatible with game version
            // todo: indicate if the mod has any load order conflicts
            .child(div())
    }

    fn mods_for_list_type(&self, cx: &mut Context<Self>) -> Vec<ModMetaData> {
        self.project
            .read_with(cx, |project, _| match self.list_type {
                ModListType::Active => project.active_mods(),
                ModListType::Inactive => project.inactive_mods(),
            })
    }
}

impl Render for ModList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .h_full()
            .w(relative(0.3))
            .border_r_1()
            .border_color(rgba(colors::BORDER))
            .text_sm()
            .child(self.render_header(cx))
            .child(self.render_list(cx))
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
