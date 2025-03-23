use crate::{theme::colors, ui::prelude::*};

/// An icon that appears within a button.
///
/// Can be used as either an icon alongside a label, like in [`Button`](crate::Button),
/// or as a standalone icon, like in [`IconButton`](crate::IconButton).
#[derive(IntoElement)]
pub(super) struct ButtonIcon {
    icon: IconSource,
    size: IconSize,
    color: Hsla,
    disabled: bool,
    selected: bool,
    selected_icon: Option<IconSource>,
    selected_icon_color: Option<Hsla>,
    selected_style: Option<ButtonStyle>,
}

impl ButtonIcon {
    pub fn new(icon: IconSource) -> Self {
        Self {
            icon,
            size: IconSize::default(),
            color: rgba(colors::TEXT).into(),
            disabled: false,
            selected: false,
            selected_icon: None,
            selected_icon_color: None,
            selected_style: None,
        }
    }

    pub fn size(mut self, size: impl Into<Option<IconSize>>) -> Self {
        if let Some(size) = size.into() {
            self.size = size;
        }

        self
    }

    pub fn color(mut self, color: impl Into<Option<Hsla>>) -> Self {
        if let Some(color) = color.into() {
            self.color = color;
        }

        self
    }

    pub fn selected_icon(mut self, icon: impl Into<Option<IconSource>>) -> Self {
        self.selected_icon = icon.into();
        self
    }

    pub fn selected_icon_color(mut self, color: impl Into<Option<Hsla>>) -> Self {
        self.selected_icon_color = color.into();
        self
    }
}

impl Disableable for ButtonIcon {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for ButtonIcon {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl SelectableButton for ButtonIcon {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.selected_style = Some(style);
        self
    }
}

impl RenderOnce for ButtonIcon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let icon = self
            .selected_icon
            .filter(|_| self.selected)
            .unwrap_or(self.icon);

        let icon_color = if self.disabled {
            rgba(colors::TEXT_DISABLED).into()
        } else if self.selected_style.is_some() && self.selected {
            self.selected_style.unwrap().into()
        } else if self.selected {
            self.selected_icon_color
                .unwrap_or(rgba(colors::TEXT_ACCENT).into())
        } else {
            self.color
        };

        let icon = Icon::new(icon).size(self.size).color(icon_color);

        icon.into_any_element()
    }
}
