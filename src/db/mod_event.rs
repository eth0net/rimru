use crate::game::mods::Source;

/// Represents a mod event (install, uninstall, update, etc.) for historical tracking.
#[derive(Debug, Clone)]
pub struct ModEvent {
    pub event_id: i64,
    pub mod_id: String,
    pub source: Source,
    pub steam_app_id: Option<String>,
    pub event_type: String, // e.g., "install", "uninstall", "update"
    // todo: use an actual timestamp
    pub timestamp: String, // ISO8601 or Unix time
    pub version: Option<String>,
    pub path: String,
    pub name: String,
    pub authors: Option<String>,  // JSON or comma-separated
    pub created: Option<String>,  // ISO8601 or Unix time
    pub modified: Option<String>, // ISO8601 or Unix time
}

impl ModEvent {
    /// Serialize the source enum as a string for DB storage.
    pub fn serialize_source(&self) -> &'static str {
        match self.source {
            Source::Steam => "steam",
            Source::Local => "local",
            Source::Official => "official",
            Source::Unknown => "unknown",
        }
    }

    /// Serialize the steam_app_id as an Option<&str> for DB storage.
    pub fn serialize_steam_app_id(&self) -> Option<&str> {
        self.steam_app_id.as_deref()
    }

    /// Parse the source string from DB into the Source enum.
    pub fn parse_source(source_str: &str) -> Source {
        match source_str {
            "steam" => Source::Steam,
            "local" => Source::Local,
            "official" => Source::Official,
            _ => Source::Unknown,
        }
    }
}
