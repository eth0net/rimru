use gpui::{App, Menu, MenuItem};

use crate::actions::Quit;

pub fn init(cx: &mut App) {
    cx.set_menus(menus());
}

fn menus() -> Vec<Menu> {
    vec![Menu {
        name: "Rimru".into(),
        items: vec![
            MenuItem::separator(),
            MenuItem::submenu(Menu {
                name: "Services".into(),
                items: vec![],
            }),
            MenuItem::separator(),
            MenuItem::action("Quit", Quit),
        ],
    }]
}
