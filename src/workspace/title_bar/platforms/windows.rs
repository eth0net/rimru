use gpui::{Pixels, Rgba, WindowControlArea, px};

use crate::{theme::colors, ui::prelude::*};

#[derive(IntoElement)]
pub struct WindowControls {
    button_height: Pixels,
}

impl WindowControls {
    pub fn new(button_height: Pixels) -> Self {
        Self { button_height }
    }

    #[cfg(not(target_os = "windows"))]
    fn get_font() -> &'static str {
        "Segoe Fluent Icons"
    }

    #[cfg(target_os = "windows")]
    fn get_font() -> &'static str {
        use windows::Wdk::System::SystemServices::RtlGetVersion;

        let mut version = unsafe { std::mem::zeroed() };
        let status = unsafe { RtlGetVersion(&mut version) };

        if status.is_ok() && version.dwBuildNumber >= 22000 {
            "Segoe Fluent Icons"
        } else {
            "Segoe MDL2 Assets"
        }
    }
}

impl RenderOnce for WindowControls {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let close_button_hover_color = Rgba {
            r: 232.0 / 255.0,
            g: 17.0 / 255.0,
            b: 32.0 / 255.0,
            a: 1.0,
        };

        let button_hover_color = rgba(colors::GHOST_ELEMENT_HOVER);
        let button_active_color = rgba(colors::GHOST_ELEMENT_ACTIVE);

        div()
            .id("windows-window-controls")
            .font_family(Self::get_font())
            .flex()
            .flex_row()
            .justify_center()
            .content_stretch()
            .max_h(self.button_height)
            .min_h(self.button_height)
            .child(CaptionButton::new(
                "minimize",
                CaptionButtonIcon::Minimize,
                button_hover_color,
                button_active_color,
            ))
            .child(CaptionButton::new(
                "maximize-or-restore",
                if window.is_maximized() {
                    CaptionButtonIcon::Restore
                } else {
                    CaptionButtonIcon::Maximize
                },
                button_hover_color,
                button_active_color,
            ))
            .child(CaptionButton::new(
                "close",
                CaptionButtonIcon::Close,
                close_button_hover_color,
                button_active_color,
            ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum CaptionButtonIcon {
    Minimize,
    Restore,
    Maximize,
    Close,
}

#[derive(IntoElement)]
struct CaptionButton {
    id: ElementId,
    icon: CaptionButtonIcon,
    hover_background_color: Hsla,
    active_background_color: Hsla,
}

impl CaptionButton {
    pub fn new(
        id: impl Into<ElementId>,
        icon: CaptionButtonIcon,
        hover_background_color: impl Into<Hsla>,
        active_background_color: impl Into<Hsla>,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            hover_background_color: hover_background_color.into(),
            active_background_color: active_background_color.into(),
        }
    }
}

impl RenderOnce for CaptionButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .id(self.id)
            .justify_center()
            .content_center()
            .occlude()
            .w(px(36.))
            .h_full()
            .text_size(px(10.0))
            .hover(|style| style.bg(self.hover_background_color))
            .active(|style| style.bg(self.active_background_color))
            .map(|this| match self.icon {
                CaptionButtonIcon::Close => this.window_control_area(WindowControlArea::Close),
                CaptionButtonIcon::Maximize | CaptionButtonIcon::Restore => {
                    this.window_control_area(WindowControlArea::Max)
                }
                CaptionButtonIcon::Minimize => this.window_control_area(WindowControlArea::Min),
            })
            .child(match self.icon {
                CaptionButtonIcon::Minimize => "\u{e921}",
                CaptionButtonIcon::Restore => "\u{e923}",
                CaptionButtonIcon::Maximize => "\u{e922}",
                CaptionButtonIcon::Close => "\u{e8bb}",
            })
    }
}
