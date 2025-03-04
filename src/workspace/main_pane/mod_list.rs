use gpui::{MouseButton, Window, div, prelude::*, rgb, uniform_list};

use crate::theme::colors;

pub struct ModList {
    pub(crate) name: String,
    pub(crate) mods: Vec<(String, String)>,
}

impl ModList {
    pub fn new(name: String, mods: Vec<(String, String)>) -> Self {
        Self { name, mods }
    }
}

impl Render for ModList {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .id("mod-list")
            .flex()
            .flex_col()
            .h_full()
            .w_1_4()
            .border_r_1()
            .border_color(rgb(colors::BORDER))
            .child(
                div()
                    .id("mod-list-header")
                    .flex()
                    .flex_row()
                    .justify_center()
                    .items_center()
                    .px_2()
                    .py_1()
                    .child(self.name.clone()),
            )
            .child(
                uniform_list(cx.entity().clone(), "mods", self.mods.len(), {
                    let mods = self.mods.clone();
                    move |_this, range, _window, _cx| {
                        let mut items = Vec::new();
                        for ix in range {
                            let mod_meta = mods[ix].clone();
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
}
