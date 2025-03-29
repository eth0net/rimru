use gpui::{
    Application, Bounds, KeyBinding, TitlebarOptions, WindowBounds, WindowOptions, px, size,
};
use rimru::{
    actions::Quit,
    assets::Assets,
    menu,
    ui::{prelude::*, text_input::*},
    workspace::Workspace,
};

fn main() {
    env_logger::init();
    log::info!("======== starting rimru ========");

    let app = Application::new().with_assets(Assets);

    app.run(|cx| {
        menu::init(cx);
        cx.on_action(quit);
        cx.activate(true);

        // todo: extract to settings
        cx.bind_keys(vec![
            KeyBinding::new("backspace", Backspace, Some("TextInput")),
            KeyBinding::new("backspace", Backspace, Some("TextInput")),
            KeyBinding::new("delete", Delete, Some("TextInput")),
            KeyBinding::new("left", Left, Some("TextInput")),
            KeyBinding::new("right", Right, Some("TextInput")),
            KeyBinding::new("shift-left", SelectLeft, Some("TextInput")),
            KeyBinding::new("shift-right", SelectRight, Some("TextInput")),
            KeyBinding::new("cmd-a", SelectAll, Some("TextInput")),
            KeyBinding::new("cmd-v", Paste, Some("TextInput")),
            KeyBinding::new("cmd-c", Copy, Some("TextInput")),
            KeyBinding::new("cmd-x", Cut, Some("TextInput")),
            KeyBinding::new("home", Home, Some("TextInput")),
            KeyBinding::new("end", End, Some("TextInput")),
        ]);

        let bounds = Bounds::centered(None, size(px(1280.), px(720.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Rimru".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| Workspace::new(window, cx)),
        )
        .unwrap();
    });
}

fn quit(_: &Quit, cx: &mut App) {
    log::info!("======== quitting rimru ========");
    cx.quit();
}
