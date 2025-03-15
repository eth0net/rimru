use std::fmt::Display;

use gpui::{
    Context, Div, Entity, IntoElement, MouseButton, SharedString, TextOverflow, Window, div, img,
    prelude::*, relative, rgb, uniform_list,
};

use crate::{game::mods::meta::Source, project::Project, theme::colours};

// todo: add list actions for refresh / sort etc
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let project = self.project.clone();
        let list_name = SharedString::from(self.list_type.to_string());
        // todo: move this to callback?
        let mods = project.read_with(cx, |project, _| match self.list_type {
            ModListType::Active => project.active_mods(),
            ModListType::Inactive => project.inactive_mods(),
        });

        fn fake_button(child: impl IntoElement) -> Div {
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_center()
                .border_1()
                .border_color(rgb(colours::BORDER))
                .rounded_md()
                .child(child)
        }

        fn fake_icon(child: impl IntoElement) -> Div {
            fake_button(child).h_6().w_6()
        }

        let buttons = match self.list_type {
            ModListType::Active => {
                vec![
                    // fake_button("reset").id("reset"),
                    // fake_button("sort").id("sort"),
                    fake_button("save").id("save").on_click({
                        let project = self.project.clone();
                        move |_, _, cx| {
                            project.update(cx, |project, _| {
                                project.save_mod_config();
                            });
                        }
                    }),
                ]
            }
            ModListType::Inactive => vec![
                // fake_button("refresh").id("refresh"),
            ],
        };

        div()
            .flex()
            .flex_col()
            .h_full()
            .w(relative(0.3))
            .border_r_1()
            .border_color(rgb(colours::BORDER))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .gap_4()
                    .px_2()
                    .py_1()
                    .child(self.list_type.to_string())
                    .child(div().flex().flex_row().gap_1().text_xs().children(buttons)),
            )
            // todo: add search bar to filter mods in this list
            // todo: preload images for visible mods in this list
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
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .text_overflow(TextOverflow::Ellipsis("..."))
                                    .on_click({
                                        let mod_meta = mod_meta.clone();
                                        let project = project.clone();
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
                                    // todo: indicate if mod is incompatible with game version
                                    // todo: indicate if the mod has any load order conflicts
                                    .child(fake_icon({
                                        // todo: add source icons instead of letters
                                        match mod_meta.source {
                                            Source::Official => "O",
                                            Source::Local => "L",
                                            Source::Steam => "S",
                                            Source::Unknown => "?",
                                        }
                                    }))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .justify_center()
                                            .h_6()
                                            .w_6()
                                            .child({
                                                match std::fs::metadata(mod_meta.icon_file_path()) {
                                                    Ok(_) => img(mod_meta.icon_file_path())
                                                        .max_h_full()
                                                        .into_any(),
                                                    Err(_) => fake_icon("?").into_any(),
                                                }
                                            }),
                                    )
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
