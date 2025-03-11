use std::path::PathBuf;

use crate::game;

pub struct Settings {
    local_mods_dir: PathBuf,
    steam_mods_dir: PathBuf,
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn local_mods_dir(&self) -> &PathBuf {
        &self.local_mods_dir
    }

    pub fn steam_mods_dir(&self) -> &PathBuf {
        &self.steam_mods_dir
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            local_mods_dir: game::local_mods_dir(),
            steam_mods_dir: game::steam_mods_dir(),
        }
    }
}
