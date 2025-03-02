use gpui::{
    AppContext, Application, Bounds, ParentElement, Render, Styled, WindowBounds, WindowOptions,
    div, px, rgb, size,
};

struct MyApp;

impl Render for MyApp {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .justify_center()
            .items_center()
            .shadow_lg()
            .border_1()
            .border_color(rgb(0x0000ff))
            .text_xl()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, {}!", "world"))
    }
}

fn main() {
    env_logger::init();
    log::info!("======== starting rimru ========");

    let app = Application::new();

    app.run(|cx| {
        let bounds = Bounds::centered(None, size(px(1280.), px(720.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.activate(false);
                cx.new(|_| MyApp {})
            },
        )
        .unwrap();
    });
}
