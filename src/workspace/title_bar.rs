use gpui::{Decorations, MouseButton, Pixels, Window, div, prelude::*, px, rgb};
use platforms::{PlatformStyle, macos};

use crate::theme::{self, colors};

mod platforms;

pub struct TitleBar {
    platform_style: PlatformStyle,
}

impl TitleBar {
    pub fn new() -> Self {
        let platform_style = PlatformStyle::platform();

        Self { platform_style }
    }

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
        let decorations = window.window_decorations();
        let titlebar_color = rgb(colors::TITLE_BAR_BACKGROUND);
        // let titlebar_color = cx.theme().colors().title_bar_background;

        div()
            .id("title-bar")
            .w_full()
            .h(height)
            .map(|this| {
                if window.is_fullscreen() {
                    this.pl_2()
                } else if self.platform_style == PlatformStyle::Mac {
                    this.pl(px(macos::TRAFFIC_LIGHT_PADDING))
                } else {
                    this.pl_2()
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
                    .w_full()
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
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .text_sm() // todo: remove?
                            .child("rimru".to_string())
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation()),
                    ),
            )
    }
}
