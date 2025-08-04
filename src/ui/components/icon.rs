use gpui::{ImageSource, img, svg};

use crate::{theme::colors, ui::prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    // Controls
    CaseSensitive,
    Clear,
    Reload,
    Reset,
    Save,
    Sort,
    Supported,
    // Panes?
    Settings,
    // Indicators
    Warning,
    Error,
    // Mod sources
    Local,
    RimWorld,
    Steam,
    Unknown,
}

impl IconName {
    pub fn path(&self) -> &'static str {
        match self {
            IconName::Clear => "icons/list-x.svg",
            IconName::CaseSensitive => "icons/a-large-small.svg",
            IconName::Reload => "icons/folder-sync.svg",
            IconName::Reset => "icons/list-restart.svg",
            IconName::Save => "icons/save.svg",
            IconName::Sort => "icons/arrow-up-down.svg",
            IconName::Supported => "icons/cable.svg",
            IconName::Settings => "icons/settings.svg",
            IconName::Warning => "icons/triangle-alert.svg",
            IconName::Error => "icons/octagon-x.svg",
            IconName::Local => "icons/hard-drive.svg",
            IconName::RimWorld => "icons/rimworld.png",
            IconName::Steam => "icons/steam.png",
            IconName::Unknown => "icons/square-x.svg",
        }
    }
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    Indicator,
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
    Custom(Rems),
}

impl IconSize {
    pub fn rems(self) -> Rems {
        match self {
            IconSize::Indicator => rems(0.625),
            IconSize::XSmall => rems(0.75),
            IconSize::Small => rems(0.875),
            IconSize::Medium => rems(1.0),
            IconSize::Large => rems(2.0),
            IconSize::XLarge => rems(3.0),
            IconSize::Custom(size) => size,
        }
    }
}

pub enum IconSource {
    Svg(SharedString),
    Img(ImageSource),
}

impl From<IconName> for IconSource {
    fn from(icon: IconName) -> Self {
        if icon.path().ends_with(".svg") {
            IconSource::Svg(icon.path().into())
        } else {
            IconSource::Img(icon.path().into())
        }
    }
}

impl<I> From<I> for IconSource
where
    I: Into<ImageSource>,
{
    fn from(path: I) -> Self {
        IconSource::Img(path.into())
    }
}

#[derive(IntoElement)]
pub struct Icon {
    source: IconSource,
    color: Hsla,
    size: Rems,
}

impl Icon {
    pub fn new(icon: IconSource) -> Self {
        Icon {
            source: icon,
            color: rgba(colors::TEXT).into(),
            size: IconSize::default().rems(),
        }
    }

    pub fn from_name(name: IconName) -> Self {
        Icon::new(name.into())
    }

    pub fn from_path(path: impl Into<ImageSource>) -> Self {
        Icon::new(path.into().into())
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = color.into();
        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size.rems();
        self
    }
}

impl From<IconName> for Icon {
    fn from(icon: IconName) -> Self {
        Icon::from_name(icon)
    }
}

impl RenderOnce for Icon {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        match self.source {
            IconSource::Svg(path) => svg()
                .size(self.size)
                .flex_none()
                .path(path)
                .text_color(self.color)
                .into_any_element(),
            IconSource::Img(path) => img(path).size(self.size).flex_none().into_any_element(),
        }
    }
}
