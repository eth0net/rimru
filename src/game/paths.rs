use std::path::{Path, PathBuf};

#[cfg(target_os = "macos")]
pub(crate) const GAME_DIR: &str =
    "~/Library/Application Support/Steam/steamapps/common/Rimworld/RimworldMac.app";

#[cfg(target_os = "macos")]
pub(crate) const CONFIG_DIR: &str = "~/Library/Application Support/Rimworld/Config";

#[cfg(target_os = "macos")]
pub(crate) const STEAM_MODS_DIR: &str =
    "~/Library/Application Support/Steam/steamapps/workshop/content/294100";

#[cfg(target_os = "windows")]
pub(crate) const GAME_DIR: &str = "C:/Program Files (x86)/Steam/steamapps/common/RimWorld";

#[cfg(target_os = "windows")]
pub(crate) const CONFIG_DIR: &str =
    "~/AppData/LocalLow/Ludeon Studios/RimWorld by Ludeon Studios/Config";

#[cfg(target_os = "windows")]
pub(crate) const STEAM_MODS_DIR: &str =
    "C:/Program Files (x86)/Steam/steamapps/workshop/content/294100";

pub fn game_dir() -> PathBuf {
    PathBuf::from(shellexpand::tilde(GAME_DIR).as_ref())
}

pub fn config_dir() -> PathBuf {
    PathBuf::from(shellexpand::tilde(CONFIG_DIR).as_ref())
}

pub fn steam_mods_dir() -> PathBuf {
    PathBuf::from(shellexpand::tilde(STEAM_MODS_DIR).as_ref())
}

pub fn local_mods_dir() -> PathBuf {
    game_dir().join("Mods")
}

pub fn mod_about_file(mod_dir: &Path) -> PathBuf {
    mod_dir.join("About/About.xml")
}
