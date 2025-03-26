use std::fmt::Display;

use gpui::{
    ClickEvent, FocusHandle, MouseButton, Pixels, Point, UniformList, px, relative, uniform_list,
};

use crate::{game::mods::ModMetaData, project::Project, theme::colors, ui::prelude::*};

use super::ModListItem;

// todo: add list action for sort
pub struct ModList {
    project: Entity<Project>,
    focus_handle: FocusHandle,
    list_name: SharedString,
    list_type: ModListType,
    // mods: Vec<Mod>, // todo: cache list
    mouse_down: bool,
}

impl ModList {
    pub fn new_active(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        Self::new(ModListType::Active, project, cx)
    }

    pub fn new_inactive(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        Self::new(ModListType::Inactive, project, cx)
    }

    pub fn new(list_type: ModListType, project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let list_name = SharedString::from(list_type.to_string());
        Self {
            project,
            focus_handle,
            list_name,
            list_type,
            mouse_down: false,
        }
    }

    fn render_header(&mut self, cx: &mut Context<Self>) -> Div {
        let mods = self.mods_for_list_type(cx);

        let buttons = match self.list_type {
            ModListType::Active => {
                vec![
                    // IconButton::from_name("sort", IconName::Sort),
                    IconButton::from_name("save", IconName::Save)
                        .on_click({
                            let project = self.project.clone();
                            move |_, _, cx| {
                                project.update(cx, |project, _| {
                                    project.save_mods_config();
                                });
                            }
                        })
                        .tooltip(Tooltip::text("Save mod order to game")),
                    IconButton::from_name("reload", IconName::Reload)
                        .on_click({
                            let project = self.project.clone();
                            move |_, _, cx| {
                                project.update(cx, |project, _| {
                                    project.load_mods_config();
                                    project.apply_mods_config();
                                });
                            }
                        })
                        .tooltip(Tooltip::text("Reload mod order from game")),
                    IconButton::from_name("reset", IconName::Reset)
                        .on_click({
                            let project = self.project.clone();
                            move |_, _, cx| {
                                project.update(cx, |project, _| {
                                    project.apply_mods_config();
                                });
                            }
                        })
                        .tooltip(Tooltip::text("Restore loaded mod order")),
                    IconButton::from_name("clear", IconName::Clear)
                        .on_click({
                            let project = self.project.clone();
                            move |_, _, cx| {
                                project.update(cx, |project, _| {
                                    project.clear_active_mods();
                                });
                            }
                        })
                        .tooltip(Tooltip::text("Clear mod order")),
                ]
            }
            ModListType::Inactive => {
                vec![
                    IconButton::from_name("reload", IconName::Reload)
                        .on_click({
                            let project = self.project.clone();
                            move |_, _, cx| {
                                project.update(cx, |project, cx| {
                                    project.load_mods(cx);
                                });
                            }
                        })
                        .tooltip(Tooltip::text("Reload installed mods")),
                ]
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

    // todo: add sort by added, updated, name, id etc to inactive list
    // todo: add search bar to filter mods in this list
    // todo: preload images for visible mods in this list
    fn render_list(&self, cx: &mut Context<Self>) -> UniformList {
        let mods = self.mods_for_list_type(cx);
        uniform_list(cx.entity().clone(), self.list_name.clone(), mods.len(), {
            move |this, range, window, cx| {
                let mut items = Vec::with_capacity(range.end - range.start);
                for ix in range {
                    let mod_meta = cx.new(|_| mods[ix].clone());
                    items.push(this.render_entry(mod_meta, window, cx));
                }
                items
            }
        })
        .flex_grow()
    }

    fn render_entry(
        &self,
        mod_meta: Entity<ModMetaData>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let mod_id = mod_meta.read_with(cx, |mod_meta, _| mod_meta.id.clone());

        let is_selected = self.project.read_with(cx, |project, _| {
            project
                .selected_mod()
                .is_some_and(|selected| selected.id == mod_id)
        });

        let bg_color = match is_selected {
            true => rgba(colors::ELEMENT_SELECTED),
            false => Hsla::transparent_black().into(),
        };

        let bg_hover_color = match is_selected {
            true => rgba(colors::ELEMENT_SELECTED),
            false => rgba(colors::ELEMENT_HOVER),
        };

        let border_color =
            if !self.mouse_down && is_selected && self.focus_handle.contains_focused(window, cx) {
                rgba(colors::BORDER_FOCUSED)
            } else {
                bg_color
            };

        let border_hover_color =
            if !self.mouse_down && is_selected && self.focus_handle.contains_focused(window, cx) {
                rgba(colors::BORDER_FOCUSED)
            } else {
                bg_hover_color
            };

        let id = SharedString::from(format!("{}-{}", self.list_name, mod_id));

        let dragged_selection = DraggedSelection {
            selected: mod_meta.read(cx).clone(),
        };

        let item = ModListItem::new(id.clone(), mod_meta.clone()).toggle_state(is_selected);

        div()
            .id(id)
            .cursor_pointer()
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .hover(|style| style.bg(bg_hover_color).border_color(border_hover_color))
            .on_drag(
                dragged_selection,
                move |selection, click_offset, _window, cx| {
                    cx.new(|_| DraggedModListItemView {
                        mod_meta: selection.selected.clone(),
                        click_offset,
                    })
                },
            )
            .drag_over::<DraggedSelection>(move |style, _, _, _| {
                style.bg(rgba(colors::DROP_TARGET_BACKGROUND))
            })
            .on_drop({
                cx.listener(move |this, selection: &DraggedSelection, _, cx| {
                    this.drag_onto(selection, mod_id.clone(), cx);
                })
            })
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.mouse_down = true;
                    cx.propagate();
                }),
            )
            .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                let mod_meta = mod_meta.clone();
                match event.down.button {
                    MouseButton::Left => match event.down.click_count {
                        1 => {
                            log::debug!("select {mod_meta:?}");
                            this.project.update(cx, {
                                move |project, cx| {
                                    let mod_meta = mod_meta.read(cx);
                                    project.select_mod(mod_meta);
                                }
                            });
                        }
                        2 => {
                            log::debug!("toggle {mod_meta:?}");
                            this.project.update(cx, {
                                move |project, cx| {
                                    let mod_meta = mod_meta.read(cx);
                                    project.toggle_mod(mod_meta);
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
            }))
            .child(item)
    }

    fn mods_for_list_type(&self, cx: &mut Context<Self>) -> Vec<ModMetaData> {
        self.project
            .read_with(cx, |project, _| match self.list_type {
                ModListType::Active => project.active_mods(),
                ModListType::Inactive => project.inactive_mods(),
            })
    }

    fn drag_onto(
        &mut self,
        selection: &DraggedSelection,
        target_mod_id: String,
        cx: &mut Context<Self>,
    ) {
        // move dragged mod to other side of target mod
        let source = selection.selected.id.clone();
        let target = target_mod_id.clone();
        self.project.update(cx, |project, _| {
            if let Err(e) = project.move_active_mod(source.clone(), target.clone()) {
                log::error!("error moving {source} to {target}: {e}");
            }
        });
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

struct DraggedSelection {
    selected: ModMetaData,
}

struct DraggedModListItemView {
    mod_meta: ModMetaData,
    click_offset: Point<Pixels>,
}

impl Render for DraggedModListItemView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            // .font(ui_font)
            .pl(self.click_offset.x + px(12.))
            .pt(self.click_offset.y + px(12.))
            .child(
                div()
                    .flex()
                    .gap_1()
                    .items_center()
                    .py_1()
                    .px_2()
                    .rounded_lg()
                    .bg(rgba(colors::BACKGROUND))
                    .text_color(rgba(colors::TEXT))
                    .map(|this| this.child(self.mod_meta.name.clone())),
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
