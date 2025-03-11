use std::path::PathBuf;

use crate::game;

pub struct Settings {
    mods_dir: Option<PathBuf>,
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mods_dir(&self) -> &Option<PathBuf> {
        &self.mods_dir
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            mods_dir: game::detect_game_dir(),
        }
    }
}
