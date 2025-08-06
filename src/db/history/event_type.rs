use std::{convert::TryFrom, fmt::Display};

use anyhow::bail;
use rusqlite::{
    ToSql,
    types::{ToSqlOutput, Value},
};

pub const INSTALL_EVENT: &str = "install";
pub const UNINSTALL_EVENT: &str = "uninstall";
pub const UPDATE_EVENT: &str = "update";

/// Represents the type of event for a mod.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    Install,
    Uninstall,
    Update,
}

impl AsRef<str> for EventType {
    fn as_ref(&self) -> &str {
        match self {
            EventType::Install => INSTALL_EVENT,
            EventType::Uninstall => UNINSTALL_EVENT,
            EventType::Update => UPDATE_EVENT,
        }
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl ToSql for EventType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(self.as_ref().to_owned())))
    }
}

impl TryFrom<&str> for EventType {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            INSTALL_EVENT => Ok(EventType::Install),
            UNINSTALL_EVENT => Ok(EventType::Uninstall),
            UPDATE_EVENT => Ok(EventType::Update),
            _ => bail!("Unknown event type: {}", s),
        }
    }
}
