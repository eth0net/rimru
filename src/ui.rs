use gpui::{AppContext, ParentElement, Pixels, Render, Styled, Window, div, px, rgb};

pub struct Rimru;

impl Render for Rimru {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x0d1117))
            .text_color(rgb(0xffffff))
            .child(cx.new(|_| TitleBar {}))
            .child(cx.new(|_| MainPane {}))
            .child(cx.new(|_| StatusBar {}))
    }
}

struct TitleBar;

impl TitleBar {
    #[cfg(not(target_os = "windows"))]
    pub fn height(window: &mut Window) -> Pixels {
        (1.5 * window.rem_size()).max(px(34.0))
    }

    #[cfg(target_os = "windows")]
    pub fn height(_window: &mut Window) -> Pixels {
        // todo(windows) instead of hard coded size report the actual size to the Windows platform API
        px(32.0)
    }
}

impl Render for TitleBar {
    fn render(
        &mut self,
        window: &mut Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        let height = Self::height(window);

        div()
            .w_full()
            .h(height)
            .flex()
            .flex_row()
            .items_center()
            .p_2()
            .pl_16()
            .border_b_1()
            .border_color(rgb(0x666666))
            .text_sm()
            .child("rimru".to_string())
    }
}

struct MainPane;

impl Render for MainPane {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        div()
            .flex_grow()
            .flex()
            .flex_col()
            .justify_center()
            .items_center()
            .text_xl()
            .child(format!("Hello, {}!", "world"))
    }
}

struct StatusBar;

impl StatusBar {
    #[cfg(not(target_os = "windows"))]
    pub fn height(window: &mut Window) -> Pixels {
        (1.5 * window.rem_size()).max(px(34.0))
    }

    #[cfg(target_os = "windows")]
    pub fn height(_window: &mut Window) -> Pixels {
        // todo(windows) instead of hard coded size report the actual size to the Windows platform API
        px(32.0)
    }
}

impl Render for StatusBar {
    fn render(
        &mut self,
        window: &mut Window,
        _cx: &mut gpui::Context<'_, Self>,
    ) -> impl gpui::IntoElement {
        let height = Self::height(window);

        div()
            .w_full()
            .h(height)
            .flex()
            .flex_row()
            .items_center()
            .p_2()
            .border_t_1()
            .border_color(rgb(0x666666))
            .text_sm()
            .child("status bar is wip".to_string())
    }
}
