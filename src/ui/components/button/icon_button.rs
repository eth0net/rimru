use gpui::ImageSource;

use crate::{
    theme::colors,
    ui::{IconSource, prelude::*},
};

use super::{ButtonLike, button_icon::ButtonIcon};

#[derive(IntoElement)]
pub struct IconButton {
    base: ButtonLike,
    icon_source: IconSource,
    icon_size: IconSize,
    icon_color: Hsla,
    selected_icon_source: Option<IconSource>,
    selected_icon_color: Option<Hsla>,
    alpha: Option<f32>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon_source: IconSource) -> Self {
        Self {
            base: ButtonLike::new(id),
            icon_source,
            icon_size: IconSize::default(),
            icon_color: rgba(colors::TEXT).into(),
            selected_icon_source: None,
            selected_icon_color: None,
            alpha: None,
        }
    }

    pub fn from_name(id: impl Into<ElementId>, icon_name: IconName) -> Self {
        Self::new(id, icon_name.into())
    }

    pub fn from_path(id: impl Into<ElementId>, icon_path: impl Into<ImageSource>) -> Self {
        Self::new(id, icon_path.into().into())
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = icon_size;
        self
    }

    pub fn icon_color(mut self, icon_color: Hsla) -> Self {
        self.icon_color = icon_color;
        self
    }

    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(alpha);
        self
    }

    pub fn selected_icon_source(mut self, icon_source: impl Into<Option<IconSource>>) -> Self {
        self.selected_icon_source = icon_source.into();
        self
    }

    /// Sets the icon color used when the button is in a selected state.
    pub fn selected_icon_color(mut self, color: impl Into<Option<Hsla>>) -> Self {
        self.selected_icon_color = color.into();
        self
    }
}

impl Disableable for IconButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Toggleable for IconButton {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.base = self.base.toggle_state(selected);
        self
    }
}

impl SelectableButton for IconButton {
    fn selected_style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.selected_style(style);
        self
    }
}

impl Clickable for IconButton {
    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.base = self.base.cursor_style(cursor_style);
        self
    }
}

impl ButtonCommon for IconButton {
    fn id(&self) -> &ElementId {
        self.base.id()
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.style(style);
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static) -> Self {
        self.base = self.base.tooltip(tooltip);
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _: &mut gpui::Window, _: &mut gpui::App) -> impl IntoElement {
        let is_disabled = self.base.disabled;
        let is_selected = self.base.selected;
        let selected_style = self.base.selected_style;

        let color = self.icon_color.opacity(self.alpha.unwrap_or(1.0));
        self.base.child(
            ButtonIcon::new(self.icon_source)
                .disabled(is_disabled)
                .toggle_state(is_selected)
                .selected_icon(self.selected_icon_source)
                .selected_icon_color(self.selected_icon_color)
                .when_some(selected_style, |this, style| this.selected_style(style))
                .size(self.icon_size)
                .color(color),
        )
    }
}
