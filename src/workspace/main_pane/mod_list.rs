use gpui::{
    Context, IntoElement, MouseButton, SharedString, Window, div, prelude::*, rgb, rgba,
    uniform_list,
};

use crate::{game::mods::Mod, theme::colours};

pub struct ModList {
    name: SharedString,
    mods: Vec<Mod>,
}

impl ModList {
    pub fn new(name: SharedString, mods: Vec<Mod>) -> Self {
        Self { name, mods }
    }
}

impl Render for ModList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            // .id("mod-list")
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
                    .child(self.name.clone()),
            )
            .child(
                uniform_list(cx.entity().clone(), self.name.clone(), self.mods.len(), {
                    let list_name = self.name.clone();
                    let mods = self.mods.clone();
                    move |_this, range, _window, _cx| {
                        let mut items = Vec::new();
                        for ix in range {
                            let mod_meta = mods[ix].clone();
                            let mod_name = mod_meta.name.clone();
                            items.push(
                                div()
                                    .id((list_name.clone(), ix))
                                    .cursor_pointer()
                                    .bg(rgba(0x77777777))
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
                                                _ => log::debug!("unhandled click {mod_meta:?}"),
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
