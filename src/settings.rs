use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::game;

mod paths;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    game_dir: PathBuf,
    official_mods_dir: PathBuf,
    local_mods_dir: PathBuf,
    steam_mods_dir: PathBuf,
    config_dir: PathBuf,
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn game_dir(&self) -> &PathBuf {
        &self.game_dir
    }

    pub fn set_game_dir(&mut self, game_dir: PathBuf) {
        self.game_dir = game_dir;
    }

    pub fn official_mods_dir(&self) -> &PathBuf {
        &self.official_mods_dir
    }

    pub fn set_official_mods_dir(&mut self, official_mods_dir: PathBuf) {
        self.official_mods_dir = official_mods_dir;
    }

    pub fn local_mods_dir(&self) -> &PathBuf {
        &self.local_mods_dir
    }

    pub fn set_local_mods_dir(&mut self, local_mods_dir: PathBuf) {
        self.local_mods_dir = local_mods_dir;
    }

    pub fn steam_mods_dir(&self) -> &PathBuf {
        &self.steam_mods_dir
    }

    pub fn set_steam_mods_dir(&mut self, steam_mods_dir: PathBuf) {
        self.steam_mods_dir = steam_mods_dir;
    }

    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn set_config_dir(&mut self, config_dir: PathBuf) {
        self.config_dir = config_dir;
    }

    pub fn mods_config_file(&self) -> PathBuf {
        game::paths::mods_config_file(&self.config_dir)
    }

    pub fn load_or_default() -> Self {
        Self::load().unwrap_or_default()
    }

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
        }
    }
}
