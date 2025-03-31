use crate::ui::IconName;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Source {
    #[default]
    Unknown,
    Official,
    Local,
    Steam,
}

impl Source {
    pub fn icon_name(&self) -> IconName {
        match self {
            Source::Unknown => IconName::Unknown,
            Source::Official => IconName::RimWorld,
            Source::Local => IconName::Local,
            Source::Steam => IconName::Steam,
        }
    }

    pub fn is_official(&self) -> bool {
        matches!(self, Source::Official)
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Source::Local)
    }

    pub fn is_steam(&self) -> bool {
        matches!(self, Source::Steam)
    }
}
