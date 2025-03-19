use gpui::{
    App, Hsla, ImageSource, IntoElement, Rems, RenderOnce, SharedString, Styled, Window, img, rems,
    rgb, svg,
};

use crate::theme::colours;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    // Actions
    Refresh,
    Reset,
    Save,
    Sort,
    // Mod sources
    Local,
    RimWorld,
    Steam,
    Unknown,
}

impl IconName {
    pub fn path(&self) -> &'static str {
        match self {
            IconName::Refresh => "icons/folder-sync.svg",
            IconName::Reset => "icons/list-restart.svg",
            IconName::Save => "icons/save.svg",
            IconName::Sort => "icons/arrow-up-down.svg",
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
    format: IconSource,
    colour: Hsla,
    size: Rems,
}

impl Icon {
    pub fn new(icon: IconName) -> Self {
        Icon {
            format: icon.into(),
            colour: rgb(colours::TEXT).into(),
            size: IconSize::default().rems(),
        }
    }

    pub fn from_path(path: impl Into<ImageSource>) -> Self {
        let path = path.into();
        Icon {
            format: path.into(),
            colour: rgb(colours::TEXT).into(),
            size: IconSize::default().rems(),
        }
    }

    pub fn colour(mut self, colour: impl Into<Hsla>) -> Self {
        self.colour = colour.into();
        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size.rems();
        self
    }
}

impl From<IconName> for Icon {
    fn from(icon: IconName) -> Self {
        Icon::new(icon)
    }
}

impl RenderOnce for Icon {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        match self.format {
            IconSource::Svg(path) => svg()
                .size(self.size)
                .flex_none()
                .path(path)
                .text_color(self.colour)
                .into_any_element(),
            IconSource::Img(path) => img(path).size(self.size).flex_none().into_any_element(),
        }
    }
}
