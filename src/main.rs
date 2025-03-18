use gpui::{
    App, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px, size,
};
use rimru::{actions::Quit, assets::Assets, menu, workspace::Workspace};

fn main() {
    env_logger::init();
    log::info!("======== starting rimru ========");

    let app = Application::new().with_assets(Assets);

    app.run(|cx| {
        menu::init(cx);
        cx.on_action(quit);
        cx.activate(true);

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
            |_, cx| cx.new(Workspace::new),
        )
        .unwrap();
    });
}

fn quit(_: &Quit, cx: &mut App) {
    log::info!("======== quitting rimru ========");
    cx.quit();
}
