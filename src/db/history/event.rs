use crate::game::mods::Source;

use super::EventType;

/// Represents a mod event (install, uninstall, update, etc.) for historical tracking.
#[derive(Debug, Clone)]
pub struct Event {
    pub event_id: i64,
    pub event_type: EventType,
    // todo: use an actual timestamp
    pub timestamp: String, // ISO8601 or Unix time
    pub mod_id: String,
    pub name: String,
    pub version: Option<String>,
    pub authors: Option<String>, // comma-separated
    pub steam_app_id: Option<String>,
    pub path: String,
    pub source: Source,
    pub created: Option<String>,  // ISO8601 or Unix time
    pub modified: Option<String>, // ISO8601 or Unix time
}
