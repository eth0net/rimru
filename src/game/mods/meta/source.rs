use std::{convert::TryFrom, fmt::Display};

use anyhow::bail;
use rusqlite::{
    ToSql,
    types::{ToSqlOutput, Value},
};

use crate::ui::IconName;

pub const UNKNOWN_SOURCE: &str = "unknown";
pub const OFFICIAL_SOURCE: &str = "official";
pub const LOCAL_SOURCE: &str = "local";
pub const STEAM_SOURCE: &str = "steam";

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

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
        match self {
            Source::Unknown => UNKNOWN_SOURCE,
            Source::Official => OFFICIAL_SOURCE,
            Source::Local => LOCAL_SOURCE,
            Source::Steam => STEAM_SOURCE,
        }
    }
}

impl Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl ToSql for Source {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(self.to_string())))
    }
}

impl TryFrom<&str> for Source {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            UNKNOWN_SOURCE => Ok(Source::Unknown),
            OFFICIAL_SOURCE => Ok(Source::Official),
            LOCAL_SOURCE => Ok(Source::Local),
            STEAM_SOURCE => Ok(Source::Steam),
            _ => bail!("Unknown source: {}", s),
        }
    }
}
