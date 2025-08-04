use std::{fmt::Display, ops::Range};

use gpui::{
    ClickEvent, FocusHandle, MouseButton, Pixels, Point, UniformList, px, relative, uniform_list,
};

use crate::{
    game::mods::{ModIssues, ModMetaData},
    project::Project,
    settings::Settings,
    theme::colors,
    ui::{TextInput, TextInputEvent, prelude::*},
};

use super::ModListItem;

pub struct ModList {
    project: Entity<Project>,
    settings: Entity<Settings>,
    text_input: Entity<TextInput>,
    focus_handle: FocusHandle,
    list_name: SharedString,
    list_type: ModListType,
    search_text: SharedString,
    case_sensitive: bool,
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

        let settings = project.read_with(cx, |project, _| project.settings());

        let text_input = TextInput::new(cx);
        text_input.update(cx, |input, _| {
            input.placeholder("Search mods...");
        });

        cx.subscribe(&text_input, |list, _input, event, cx| match event {
            TextInputEvent::ContentChanged { content } => {
                list.search_text = content.into();
                list.smart_case(cx);
            }
        })
        .detach();

        Self {
            project,
            settings,
            text_input,
            focus_handle,
            list_name,
            list_type,
            search_text: "".into(),
            case_sensitive: false,
            mouse_down: false,
        }
    }

    // todo: move search to new line - possible setting to toggle advanced search on new line
    // todo: add search controls (case sensitive, regex, etc.)
    fn render_header(&mut self, cx: &mut Context<Self>) -> Div {
        let mods = self.mods_for_list_type(cx).len();
        let filtered_mods = self.filtered_mods_for_list_type(cx).len();
        let inactive_order = self
            .project
            .read_with(cx, |project, _| project.inactive_mods_order());

        let separate_search_bar = self
            .settings
            .read_with(cx, |settings, _| settings.separate_search_bar());

        let mods_str = match mods == filtered_mods {
            true => mods.to_string(),
            false => format!("{filtered_mods} / {mods}"),
        };

        // todo: don't do this every render
        let buttons = match self.list_type {
            ModListType::Active => {
                vec![
                    IconButton::from_name("sort", IconName::Sort)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.project.update(cx, |project, _| {
                                project.sort_active_mods();
                            });
                        }))
                        .tooltip(Tooltip::text("Sort active mods")),
                    IconButton::from_name("save", IconName::Save)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.project.update(cx, |project, cx| {
                                project.save_mods_config(cx);
                            });
                        }))
                        .tooltip(Tooltip::text("Save mod order to game")),
                    IconButton::from_name("reload", IconName::Reload)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.project.update(cx, |project, cx| {
                                project.load_mods_config(cx);
                                project.apply_mods_config();
                            });
                        }))
                        .tooltip(Tooltip::text("Reload mod order from game")),
                    IconButton::from_name("reset", IconName::Reset)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.project.update(cx, |project, _| {
                                project.apply_mods_config();
                            });
                        }))
                        .tooltip(Tooltip::text("Restore loaded mod order")),
                    IconButton::from_name("clear", IconName::Clear)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.project.update(cx, |project, _| {
                                project.clear_active_mods();
                            });
                        }))
                        .tooltip(Tooltip::text("Clear mod order")),
                ]
            }
            ModListType::Inactive => {
                vec![
                    IconButton::from_name("sort", IconName::Sort)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.project.update(cx, |project, _| {
                                project.cycle_inactive_mods_order();
                                project.cache_mods();
                            });
                        }))
                        .tooltip(Tooltip::text(format!(
                            "Sort inactive mods by {inactive_order}"
                        ))),
                    IconButton::from_name("supported", IconName::Supported)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.project.update(cx, |project, _| {
                                project.toggle_supported_mods_only();
                            });
                        }))
                        .icon_color(Hsla::from(rgba({
                            if self
                                .project
                                .read_with(cx, |project, _| project.show_supported_mods_only())
                            {
                                colors::SUCCESS_TEXT
                            } else {
                                colors::TEXT
                            }
                        })))
                        .tooltip(Tooltip::text("Show only supported mods")),
                    IconButton::from_name("reload", IconName::Reload)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.project.update(cx, |project, cx| {
                                project.load_mods(cx);
                            });
                        }))
                        .tooltip(Tooltip::text("Reload installed mods")),
                ]
            }
        };

        div()
            .flex()
            .flex_col()
            .gap_2()
            .px_2()
            .py_2()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_center()
                            .items_start()
                            .pt_0p5()
                            .child(format!("{} ({})", self.list_name, mods_str)),
                    )
                    .when(!separate_search_bar, |this| {
                        this.child(
                            div()
                                .flex()
                                .flex_row()
                                .flex_grow()
                                .items_center()
                                .gap_2()
                                .child(self.text_input.clone())
                                .child(
                                    div().flex().flex_row().items_center().gap_1().child(
                                        IconButton::from_name(
                                            "case sensitive",
                                            IconName::CaseSensitive,
                                        )
                                        .on_click(cx.listener(|this, _, _, _| {
                                            this.case_sensitive = !this.case_sensitive;
                                        }))
                                        .icon_color(Hsla::from(rgba({
                                            match self.case_sensitive {
                                                true => colors::SUCCESS_TEXT,
                                                false => colors::TEXT,
                                            }
                                        })))
                                        .tooltip(Tooltip::text("Toggle case sensitivity")),
                                    ),
                                ),
                        )
                    })
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_center()
                            .items_start()
                            .children(buttons),
                    ),
            )
            .when(separate_search_bar, |this| {
                this.child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(self.text_input.clone())
                        .child(
                            div().flex().flex_row().items_center().gap_1().child(
                                IconButton::from_name("case sensitive", IconName::CaseSensitive)
                                    .on_click(cx.listener(|this, _, _, _| {
                                        this.case_sensitive = !this.case_sensitive;
                                    }))
                                    .icon_color(Hsla::from(rgba({
                                        match self.case_sensitive {
                                            true => colors::SUCCESS_TEXT,
                                            false => colors::TEXT,
                                        }
                                    })))
                                    .tooltip(Tooltip::text("Toggle case sensitivity")),
                            ),
                        ),
                )
            })
    }

    // todo: preload images for visible mods in this list
    fn render_list(&self, cx: &mut Context<Self>) -> UniformList {
        let mods = self.filtered_mods_for_list_type(cx);
        uniform_list(
            self.list_name.clone(),
            mods.len(),
            cx.processor(move |this, range: Range<usize>, window, cx| {
                let mut items = Vec::with_capacity(range.end - range.start);
                for ix in range {
                    let mod_meta = cx.new(|_| mods[ix].clone());
                    let mod_issues = this.project.read_with(cx, |project, _| {
                        project.issues_for_mod(&mods[ix].id).cloned()
                    });
                    items.push(this.render_entry(mod_meta, mod_issues, window, cx));
                }
                items
            }),
        )
        .flex_grow()
    }

    fn render_entry(
        &self,
        mod_meta: Entity<ModMetaData>,
        mod_issues: Option<ModIssues>,
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

        let item =
            ModListItem::new(id.clone(), mod_meta.clone(), mod_issues).toggle_state(is_selected);

        div()
            .id(id)
            .cursor_pointer()
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .hover(|style| style.bg(bg_hover_color).border_color(border_hover_color))
            .on_drag(dragged_selection, |selection, click_offset, _window, cx| {
                cx.new(|_| DraggedModListItemView {
                    mod_meta: selection.selected.clone(),
                    click_offset,
                })
            })
            .drag_over::<DraggedSelection>(|style, _, _, _| {
                style.bg(rgba(colors::DROP_TARGET_BACKGROUND))
            })
            .on_drop(
                cx.listener(move |this, selection: &DraggedSelection, _, cx| {
                    this.drag_onto(selection, mod_id.clone(), cx);
                }),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.mouse_down = true;
                    cx.propagate();
                }),
            )
            .on_click(cx.listener(move |this, event: &ClickEvent, _, cx| {
                let mod_meta_entity = mod_meta.clone();
                let mod_meta = mod_meta_entity.read(cx);
                match event.down.button {
                    MouseButton::Left => match event.down.click_count {
                        1 => {
                            log::debug!("select {mod_meta:?}");
                            this.project.update(cx, {
                                move |project, cx| {
                                    let mod_meta = mod_meta_entity.read(cx);
                                    project.select_mod(mod_meta);
                                }
                            });
                        }
                        2 => {
                            log::debug!("toggle {mod_meta:?}");
                            this.project.update(cx, move |project, cx| {
                                let mod_meta = mod_meta_entity.read(cx);
                                project.toggle_mod(mod_meta);
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

    // todo: support case-insensitive search - case sensitive toggle + smart activation
    // todo: support selecting which fields to search (name, id, description, etc.)
    // todo: support regex search
    fn filtered_mods_for_list_type(&self, cx: &mut Context<Self>) -> Vec<ModMetaData> {
        let search = self.search_text.to_string();
        let is_inactive = self.list_type == ModListType::Inactive;
        let show_supported_only = is_inactive
            && self
                .project
                .read_with(cx, |project, _| project.show_supported_mods_only());
        let game_version = if show_supported_only {
            self.project
                .read_with(cx, |project, _| project.game_version())
        } else {
            None
        };

        self.mods_for_list_type(cx)
            .iter()
            .filter(|mod_meta| {
                // Search filter
                (search.is_empty()
                    || if self.case_sensitive {
                        mod_meta.name.contains(&search) || mod_meta.id.contains(&search)
                    } else {
                        let search_lower = search.to_lowercase();
                        mod_meta.name.to_lowercase().contains(&search_lower)
                            || mod_meta.id.to_lowercase().contains(&search_lower)
                    })
                // Supported mods filter (only for inactive list)
                && (!show_supported_only
                    || match &game_version {
                        Some(version) => mod_meta.supported_versions.contains(version),
                        None => true,
                    })
            })
            .cloned()
            .collect()
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

    fn smart_case(&mut self, cx: &mut Context<Self>) {
        if self
            .settings
            .read_with(cx, |settings, _cx| settings.smart_search())
        {
            let search = self.search_text.to_string();
            if !search.is_empty() {
                let is_case = search.chars().any(|c| c.is_uppercase());
                if self.case_sensitive != is_case {
                    log::debug!("smart case toggle: {}", is_case);
                    self.case_sensitive = !self.case_sensitive;
                }
            }
        }
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
