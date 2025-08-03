use gpui::{
    Application, Bounds, KeyBinding, Size, TitlebarOptions, WindowBounds, WindowOptions, px, size,
};
use rimru::{
    actions::Quit,
    assets::Assets,
    menu,
    project::Project,
    settings::Settings,
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

        let settings = cx.new(|_| Settings::load_or_default());
        let project = cx.new(|cx| Project::new(cx, settings.clone()));

        let bounds = Bounds::centered(None, size(px(1280.), px(720.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Rimru".into()),
                    appears_transparent: true,
                    // traffic_light_position: Some(point(px(9.), px(9.))),
                    ..Default::default()
                }),
                window_min_size: Some(Size {
                    width: px(800.),
                    height: px(600.),
                }),
                // window_decorations: Some(WindowDecorations::Client),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| Workspace::new(settings, project, window, cx)),
        )
        .unwrap();
    });
}

fn quit(_: &Quit, cx: &mut App) {
    log::info!("======== quitting rimru ========");
    cx.quit();
}
