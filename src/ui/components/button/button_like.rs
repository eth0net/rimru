use gpui::{ClickEvent, CursorStyle, MouseButton, transparent_black};
use smallvec::SmallVec;

use crate::{theme::colors, ui::prelude::*};

/// A trait for buttons that can be Selected. Enables setting the [`ButtonStyle`] of a button when it is selected.
pub trait SelectableButton: Toggleable {
    fn selected_style(self, style: ButtonStyle) -> Self;
}

/// A common set of traits all buttons must implement.
pub trait ButtonCommon: Clickable + Disableable {
    /// A unique element ID to identify the button.
    fn id(&self) -> &ElementId;

    /// The visual style of the button.
    ///
    /// Most commonly will be [`ButtonStyle::Subtle`], or [`ButtonStyle::Filled`]
    /// for an emphasized button.
    fn style(self, style: ButtonStyle) -> Self;

    /// The size of the button.
    ///
    /// Most buttons will use the default size.
    ///
    /// [`ButtonSize`] can also be used to help build non-button elements
    /// that are consistently sized with buttons.
    fn size(self, size: ButtonSize) -> Self;

    /// The tooltip that shows when a user hovers over the button.
    ///
    /// Nearly all interactable elements should have a tooltip. Some example
    /// exceptions might a scroll bar, or a slider.
    fn tooltip(self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self;
}

type TooltipFunc = Box<dyn Fn(&mut Window, &mut App) -> AnyView>;

type OnClickFunc = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct ButtonLike {
    base: Div,
    id: ElementId,
    style: ButtonStyle,
    pub(super) disabled: bool,
    pub(super) selected: bool,
    pub(super) selected_style: Option<ButtonStyle>,
    width: Option<DefiniteLength>,
    height: Option<DefiniteLength>,
    size: ButtonSize,
    tooltip: Option<TooltipFunc>,
    cursor_style: CursorStyle,
    on_click: Option<OnClickFunc>,
    children: SmallVec<[AnyElement; 2]>,
}

impl ButtonLike {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            style: ButtonStyle::default(),
            disabled: false,
            selected: false,
            selected_style: None,
            width: None,
            height: None,
            size: ButtonSize::Default,
            tooltip: None,
            cursor_style: CursorStyle::PointingHand,
            on_click: None,
            children: SmallVec::new(),
        }
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.base = self.base.opacity(opacity);
        self
    }
}

impl Disableable for ButtonLike {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for ButtonLike {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl SelectableButton for ButtonLike {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.selected_style = Some(style);
        self
    }
}

impl Clickable for ButtonLike {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl ButtonCommon for ButtonLike {
    fn id(&self) -> &ElementId {
        &self.id
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.tooltip = Some(Box::new(tooltip));
        self
    }
}

impl ParentElement for ButtonLike {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for ButtonLike {
    fn render(self, _: &mut gpui::Window, _: &mut gpui::App) -> impl IntoElement {
        let style = self
            .selected_style
            .filter(|_| self.selected)
            .unwrap_or(self.style);

        self.base
            .id(self.id.clone())
            .flex()
            .flex_row()
            .items_center()
            .group("")
            .flex_none()
            .h(self.height.unwrap_or(self.size.rems().into()))
            .when_some(self.width, |this, width| {
                this.w(width).justify_center().text_center()
            })
            .rounded_sm()
            .gap_1()
            .map(|this| match self.size {
                ButtonSize::Large => this.px_1p5(),
                ButtonSize::Default | ButtonSize::Compact => this.px_1(),
                ButtonSize::None => this,
            })
            .bg(style.enabled().background)
            .when(self.disabled, |this| this.cursor_not_allowed())
            .when(!self.disabled, |this| {
                this.cursor_pointer()
                    .hover(|hover| hover.bg(style.hovered().background))
                    .active(|active| active.bg(style.active().background))
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
                        .on_click(move |event, window, cx| {
                            cx.stop_propagation();
                            (on_click)(event, window, cx)
                        })
                },
            )
            .when_some(self.tooltip, |this, tooltip| {
                this.tooltip(move |window, cx| tooltip(window, cx))
            })
            .children(self.children)
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonSize {
    Large,
    #[default]
    Default,
    Compact,
    None,
}

impl ButtonSize {
    pub fn rems(self) -> Rems {
        match self {
            ButtonSize::Large => rems(2.0),
            ButtonSize::Default => rems(1.375),
            ButtonSize::Compact => rems(1.125),
            ButtonSize::None => rems(1.0),
        }
    }
}

/// The visual appearance of a button.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ButtonStyle {
    /// A filled button with a solid background color. Provides emphasis versus
    /// the more common subtle button.
    Filled,

    /// Used to emphasize a button in some way, like a selected state, or a semantic
    /// coloring like an error or success button.
    Tinted(TintColor),

    /// The default button style, used for most buttons. Has a transparent background,
    /// but has a background color to indicate states like hover and active.
    #[default]
    Subtle,

    /// Used for buttons that only change foreground color on hover and active states.
    ///
    /// TODO: Better docs for this.
    Transparent,
}

impl ButtonStyle {
    fn enabled(self) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: rgba(colors::ELEMENT_BACKGROUND).into(),
                border_color: transparent_black(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: rgba(colors::GHOST_ELEMENT_BACKGROUND).into(),
                border_color: transparent_black(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
        }
    }

    fn hovered(self) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => {
                let mut filled_background: Hsla = rgba(colors::ELEMENT_BACKGROUND).into();
                filled_background.fade_out(0.92);

                ButtonLikeStyles {
                    background: filled_background,
                    border_color: transparent_black(),
                    label_color: rgba(colors::TEXT).into(),
                    icon_color: rgba(colors::TEXT).into(),
                }
            }
            ButtonStyle::Tinted(tint) => tint.button_like_style(),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: rgba(colors::GHOST_ELEMENT_HOVER).into(),
                border_color: transparent_black(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
        }
    }

    fn active(self) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: rgba(colors::ELEMENT_ACTIVE).into(),
                border_color: transparent_black(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: rgba(colors::GHOST_ELEMENT_ACTIVE).into(),
                border_color: transparent_black(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
        }
    }

    #[allow(unused)]
    // todo: look into why this is not used
    fn focused(self) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: rgba(colors::ELEMENT_BACKGROUND).into(),
                border_color: rgba(colors::BORDER_FOCUSED).into(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: rgba(colors::GHOST_ELEMENT_BACKGROUND).into(),
                border_color: rgba(colors::BORDER_FOCUSED).into(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: rgba(colors::BORDER_FOCUSED).into(),
                label_color: rgba(colors::TEXT_ACCENT).into(),
                icon_color: rgba(colors::TEXT_ACCENT).into(),
            },
        }
    }

    #[allow(unused)]
    // todo: look into why this is not used
    fn disabled(self) -> ButtonLikeStyles {
        match self {
            ButtonStyle::Filled => ButtonLikeStyles {
                background: rgba(colors::ELEMENT_DISABLED).into(),
                border_color: rgba(colors::BORDER_DISABLED).into(),
                label_color: rgba(colors::TEXT_DISABLED).into(),
                icon_color: rgba(colors::TEXT_DISABLED).into(),
            },
            ButtonStyle::Tinted(tint) => tint.button_like_style(),
            ButtonStyle::Subtle => ButtonLikeStyles {
                background: rgba(colors::GHOST_ELEMENT_DISABLED).into(),
                border_color: rgba(colors::BORDER_DISABLED).into(),
                label_color: rgba(colors::TEXT_DISABLED).into(),
                icon_color: rgba(colors::TEXT_DISABLED).into(),
            },
            ButtonStyle::Transparent => ButtonLikeStyles {
                background: transparent_black(),
                border_color: transparent_black(),
                label_color: rgba(colors::TEXT_DISABLED).into(),
                icon_color: rgba(colors::TEXT_DISABLED).into(),
            },
        }
    }
}

impl From<ButtonStyle> for Hsla {
    fn from(style: ButtonStyle) -> Self {
        match style {
            ButtonStyle::Tinted(tint) => tint.into(),
            _ => rgba(colors::TEXT).into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum TintColor {
    #[default]
    Accent,
    Error,
    Warning,
    Success,
}

impl TintColor {
    fn button_like_style(self) -> ButtonLikeStyles {
        match self {
            TintColor::Accent => ButtonLikeStyles {
                background: rgba(colors::INFO_BACKGROUND).into(),
                border_color: rgba(colors::INFO_BORDER).into(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
            TintColor::Error => ButtonLikeStyles {
                background: rgba(colors::ERROR_BACKGROUND).into(),
                border_color: rgba(colors::ERROR_BORDER).into(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
            TintColor::Warning => ButtonLikeStyles {
                background: rgba(colors::WARNING_BACKGROUND).into(),
                border_color: rgba(colors::WARNING_BORDER).into(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
            TintColor::Success => ButtonLikeStyles {
                background: rgba(colors::SUCCESS_BACKGROUND).into(),
                border_color: rgba(colors::SUCCESS_BORDER).into(),
                label_color: rgba(colors::TEXT).into(),
                icon_color: rgba(colors::TEXT).into(),
            },
        }
    }
}

impl From<TintColor> for Hsla {
    fn from(tint_color: TintColor) -> Self {
        match tint_color {
            TintColor::Accent => rgba(colors::TEXT_ACCENT).into(),
            TintColor::Error => rgba(colors::ERROR_TEXT).into(),
            TintColor::Warning => rgba(colors::WARNING_TEXT).into(),
            TintColor::Success => rgba(colors::SUCCESS_TEXT).into(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ButtonLikeStyles {
    pub background: Hsla,
    #[allow(unused)]
    pub border_color: Hsla,
    #[allow(unused)]
    pub label_color: Hsla,
    #[allow(unused)]
    pub icon_color: Hsla,
}
