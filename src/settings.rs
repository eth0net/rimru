use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::game;

mod paths;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    /// The game directory, where the game executable is located.
    game_dir: PathBuf,
    /// The directory where official mods are stored.
    official_mods_dir: PathBuf,
    /// The directory where local mods are stored.
    local_mods_dir: PathBuf,
    /// The directory where Steam mods are stored.
    steam_mods_dir: PathBuf,
    /// The directory where configuration files are stored.
    config_dir: PathBuf,

    /// Show advanced search controls for the mod search bar.
    advanced_search: bool,
    /// Separate the search bar from mod list controls.
    separate_search_bar: bool,
    /// Automatically activate case sensitivity if searches contain uppercase letters.
    smart_search: bool,
}

impl Settings {
    /// Create a new `Settings` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the game directory.
    pub fn game_dir(&self) -> &PathBuf {
        &self.game_dir
    }

    /// Set the game directory.
    pub fn set_game_dir(&mut self, game_dir: PathBuf) {
        self.game_dir = game_dir;
    }

    /// Get the official mods directory.
    pub fn official_mods_dir(&self) -> &PathBuf {
        &self.official_mods_dir
    }

    /// Set the official mods directory.
    pub fn set_official_mods_dir(&mut self, official_mods_dir: PathBuf) {
        self.official_mods_dir = official_mods_dir;
    }

    /// Get the local mods directory.
    pub fn local_mods_dir(&self) -> &PathBuf {
        &self.local_mods_dir
    }

    /// Set the local mods directory.
    pub fn set_local_mods_dir(&mut self, local_mods_dir: PathBuf) {
        self.local_mods_dir = local_mods_dir;
    }

    /// Get the Steam mods directory.
    pub fn steam_mods_dir(&self) -> &PathBuf {
        &self.steam_mods_dir
    }

    /// Set the Steam mods directory.
    pub fn set_steam_mods_dir(&mut self, steam_mods_dir: PathBuf) {
        self.steam_mods_dir = steam_mods_dir;
    }

    /// Get the configuration directory.
    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    /// Set the configuration directory.
    pub fn set_config_dir(&mut self, config_dir: PathBuf) {
        self.config_dir = config_dir;
    }

    /// Get the path to the mods config file.
    pub fn mods_config_file(&self) -> PathBuf {
        game::paths::mods_config_file(&self.config_dir)
    }

    /// Set whether to separate the search bar from mod list controls.
    pub fn set_separate_search_bar(&mut self, separate_search_bar: bool) {
        self.separate_search_bar = separate_search_bar;
    }

    /// Check if the search bar is separated from mod list controls.
    pub fn separate_search_bar(&self) -> bool {
        self.separate_search_bar
    }

    /// Set whether to use smart search.
    pub fn set_smart_search(&mut self, smart_search: bool) {
        self.smart_search = smart_search;
    }

    /// Check if smart search is enabled.
    pub fn smart_search(&self) -> bool {
        self.smart_search
    }

    /// Load settings from the default settings file, or return default settings if the file does not exist.
    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

    /// Load settings from the default settings file.
    pub fn load() -> Option<Self> {
        let settings_path = paths::settings_file();

        let settings = fs::read_to_string(&settings_path)
            .map_err(|e| {
                log::error!("error reading settings file {settings_path:?}: {e}");
            })
            .ok()?;

        toml::from_str(&settings)
            .map_err(|e| {
                log::error!("error parsing settings file {settings_path:?}: {e}");
            })
            .ok()
    }

    pub fn save(&self) {
        let settings_path = paths::settings_file();

        if let Err(e) = fs::create_dir_all(settings_path.parent().unwrap()) {
            log::error!("error creating settings directory: {e}");
            return;
        }

        if let Err(e) = fs::write(&settings_path, toml::to_string(self).unwrap()) {
            log::error!("error writing settings file {settings_path:?}: {e}");
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        let game_dir = game::paths::default_game_dir();
        let official_mods_dir = game::paths::official_mods_dir(&game_dir);
        let local_mods_dir = game::paths::local_mods_dir(&game_dir);
        let steam_mods_dir = game::paths::default_steam_mods_dir();
        let config_dir = game::paths::default_config_dir();
        Self {
            game_dir,
            official_mods_dir,
            local_mods_dir,
            steam_mods_dir,
            config_dir,

            advanced_search: true,
            separate_search_bar: true,
            smart_search: true,
        }
    }
}
