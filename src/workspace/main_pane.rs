use gpui::{MouseButton, Window, div, prelude::*, rgb, uniform_list};

pub struct MainPane;

impl MainPane {
    fn mod_list(
        cx: &mut gpui::Context<'_, Self>,
        name: String,
        mods: Vec<(String, String)>,
    ) -> impl gpui::IntoElement {
        div()
            .id("mod-list")
            .flex()
            .flex_col()
            .h_full()
            .w_1_4()
            .border_r_1()
            .border_color(rgb(0x30363d))
            .child(
                div()
                    .id("mod-list-header")
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .px_2()
                    .py_1()
                    // .border_b_1()
                    // .border_color(rgb(0x30363d))
                    .child(name),
            )
            .child(
                uniform_list(cx.entity().clone(), "mods", mods.len(), {
                    move |_this, range, _window, _cx| {
                        let mut items = Vec::new();
                        for ix in range {
                            let mod_meta = &mods[ix];
                            let mod_name = mod_meta.1.clone();
                            items.push(
                                div()
                                    .id(ix)
                                    .cursor_pointer()
                                    .px_2()
                                    .on_click({
                                        let mod_meta = mod_meta.clone();
                                        move |event, _window, _cx| {
                                            // log::debug!("click {mod_meta:?} {event:?}");
                                            match event.down.button {
                                                MouseButton::Left => {
                                                    match event.down.click_count {
                                                        1 => {
                                                            // Select
                                                            log::debug!("select {mod_meta:?}");
                                                        }
                                                        2 => {
                                                            // Activate/deactivate
                                                            log::debug!("toggle {mod_meta:?}");
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                MouseButton::Right => {
                                                    // Open context menu
                                                    log::debug!("context menu {mod_meta:?}");
                                                }
                                                _ => {}
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

    fn details(_cx: &mut gpui::Context<'_, Self>) -> impl gpui::IntoElement {
        div()
            .flex()
            .flex_col()
            .h_full()
            .w_2_4()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .px_2()
                    .py_1()
                    // .border_b_1()
                    // .border_color(rgb(0x30363d))
                    .child("Details".to_string()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .px_2()
                    .py_1()
                    .child("Details Content".to_string()),
            )
    }
}

impl Render for MainPane {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
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
            .child(Self::mod_list(cx, "Inactive".to_string(), inactive_mods))
            .child(Self::mod_list(cx, "Active".to_string(), active_mods))
            .child(Self::details(cx))
    }
}
