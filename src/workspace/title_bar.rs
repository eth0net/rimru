use gpui::{Decorations, Pixels, px};
use platforms::{PlatformStyle, macos, windows};

use crate::{
    theme::{self, colors},
    ui::prelude::*,
};

mod platforms;

pub struct TitleBar {
    platform_style: PlatformStyle,
    // should_move: bool, // todo(linux)
}

impl TitleBar {
    pub fn new() -> Self {
        let platform_style = PlatformStyle::platform();

        Self {
            platform_style,
            // should_move: false, // todo(linux)
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn height(window: &mut Window) -> Pixels {
        (1.75 * window.rem_size()).max(px(34.0))
    }

    #[cfg(target_os = "windows")]
    pub fn height(_window: &mut Window) -> Pixels {
        // todo(windows) instead of hard coded size report the actual size to the Windows platform API
        px(32.0)
    }
}

impl Render for TitleBar {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let height = Self::height(window);
        // todo(linux): implement window controls
        // let supported_controls = window.window_controls();
        let decorations = window.window_decorations();
        let titlebar_color = rgba(colors::TITLE_BAR_BACKGROUND);
        // todo(linux): implement titlebar color
        // let titlebar_color = if cfg!(any(target_os = "linux", target_os = "freebsd")) {
        //     if window.is_window_active() && !self.should_move {
        //         rgba(colors::TITLE_BAR_BACKGROUND)
        //     } else {
        //         rgba(colors::TITLE_BAR_INACTIVE_BACKGROUND)
        //     }
        // } else {
        //     rgba(colors::TITLE_BAR_BACKGROUND)
        // };

        div()
            .id("title-bar")
            .w_full()
            .h(height)
            .map(|this| {
                match !window.is_fullscreen() && self.platform_style == PlatformStyle::Mac {
                    true => this.pl(px(macos::TRAFFIC_LIGHT_PADDING)),
                    false => this.pl_2(),
                }
            })
            .map(|el| match decorations {
                Decorations::Server => el,
                Decorations::Client { tiling, .. } => el
                    .when(!(tiling.top || tiling.right), |el| {
                        el.rounded_tr(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                    })
                    .when(!(tiling.top || tiling.left), |el| {
                        el.rounded_tl(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                    })
                    // this border is to avoid a transparent gap in the rounded corners
                    .mt(px(-1.))
                    .border(px(1.))
                    .border_color(titlebar_color),
            })
            .bg(titlebar_color)
            .content_stretch()
            .child(
                div()
                    .id("titlebar-content")
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .size_full()
                    // Note: On Windows the title bar behavior is handled by the platform implementation.
                    .when(self.platform_style != PlatformStyle::Windows, |this| {
                        this.on_click(|event, window, _| {
                            if event.up.click_count == 2 {
                                window.zoom_window();
                            }
                        })
                    })
                    .child(
                        div()
                            .id("title")
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .text_sm()
                            .child("rimru".to_string()),
                    ),
            )
            .when(!window.is_fullscreen(), |title_bar| {
                match self.platform_style {
                    PlatformStyle::Mac => title_bar,
                    // todo(linux): implement titlebar for linux
                    // PlatformStyle::Linux => {
                    //     if matches!(decorations, Decorations::Client { .. }) {
                    //         title_bar
                    //             .child(LinuxWindowControls::new(close_action))
                    //             .when(supported_controls.window_menu, |titlebar| {
                    //                 titlebar
                    //                     .on_mouse_down(MouseButton::Right, move |ev, window, _| {
                    //                         window.show_window_menu(ev.position)
                    //                     })
                    //             })
                    //             .on_mouse_move(cx.listener(move |this, _ev, window, _| {
                    //                 if this.should_move {
                    //                     this.should_move = false;
                    //                     window.start_window_move();
                    //                 }
                    //             }))
                    //             .on_mouse_down_out(cx.listener(move |this, _ev, _window, _cx| {
                    //                 this.should_move = false;
                    //             }))
                    //             .on_mouse_up(
                    //                 MouseButton::Left,
                    //                 cx.listener(move |this, _ev, _window, _cx| {
                    //                     this.should_move = false;
                    //                 }),
                    //             )
                    //             .on_mouse_down(
                    //                 MouseButton::Left,
                    //                 cx.listener(move |this, _ev, _window, _cx| {
                    //                     this.should_move = true;
                    //                 }),
                    //             )
                    //     } else {
                    //         title_bar
                    //     }
                    // }
                    // todo(windows): implement titlebar for windows
                    PlatformStyle::Windows => title_bar.child(windows::WindowControls::new(height)),
                    _ => title_bar,
                }
            })
    }
}
