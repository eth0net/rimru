use std::path::{Path, PathBuf};

#[cfg(target_os = "macos")]
const GAME_DIR: &str =
    "~/Library/Application Support/Steam/steamapps/common/Rimworld/RimworldMac.app";
#[cfg(target_os = "macos")]
const STEAM_MODS_DIR: &str =
    "~/Library/Application Support/Steam/steamapps/workshop/content/294100";
#[cfg(target_os = "macos")]
const CONFIG_DIR: &str = "~/Library/Application Support/Rimworld/Config";

#[cfg(target_os = "windows")]
const GAME_DIR: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\RimWorld";
#[cfg(target_os = "windows")]
const STEAM_MODS_DIR: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\workshop\\content\\294100";
#[cfg(target_os = "windows")]
const CONFIG_DIR: &str = "~\\AppData\\LocalLow\\Ludeon Studios\\RimWorld by Ludeon Studios\\Config";

const LOCAL_MODS_DIR: &str = "Mods";
const OFFICIAL_MODS_DIR: &str = "Data";
const MODS_CONFIG_FILE: &str = "ModsConfig.xml";
const MOD_ABOUT_DIR: &str = "About";
const MOD_ABOUT_FILE: &str = "About.xml";
const MOD_PREVIEW_FILE: &str = "Preview.png";
const MOD_ICON_FILE: &str = "ModIcon.png";

pub fn default_game_dir() -> PathBuf {
    PathBuf::from(shellexpand::tilde(GAME_DIR).as_ref())
}

pub fn default_steam_mods_dir() -> PathBuf {
    PathBuf::from(shellexpand::tilde(STEAM_MODS_DIR).as_ref())
}

pub fn default_config_dir() -> PathBuf {
    PathBuf::from(shellexpand::tilde(CONFIG_DIR).as_ref())
}

pub fn local_mods_dir(game_dir: &Path) -> PathBuf {
    game_dir.join(LOCAL_MODS_DIR)
}

pub fn official_mods_dir(game_dir: &Path) -> PathBuf {
    game_dir.join(OFFICIAL_MODS_DIR)
}

pub fn mods_config_file(config_dir: &Path) -> PathBuf {
    config_dir.join(MODS_CONFIG_FILE)
}

pub fn mod_about_file(mod_dir: &Path) -> PathBuf {
    mod_dir.join(MOD_ABOUT_DIR).join(MOD_ABOUT_FILE)
}

pub fn mod_preview_file(mod_dir: &Path) -> PathBuf {
    mod_dir.join(MOD_ABOUT_DIR).join(MOD_PREVIEW_FILE)
}

pub fn mod_icon_file(mod_dir: &Path) -> PathBuf {
    mod_dir.join(MOD_ABOUT_DIR).join(MOD_ICON_FILE)
}
