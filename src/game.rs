use std::path::PathBuf;

// linux
//   ~/.steam/steam/steamapps/common/RimWorld/Mods
//   ~/GOG/Games/RimWorld/game/Mods/
// macos
//   ~/Library/Application Support/Steam/steamapps/common/RimWorld/RimWorldMac.app/Mods
// windows
//   C:\Program Files (x86)\Steam\steamapps\common\RimWorld\Mods

#[cfg(target_os = "macos")]
const MODS_DIRS: [&str; 1] =
    ["~/Library/Application Support/Steam/steamapps/common/RimWorld/RimWorldMac.app/Mods"];

#[cfg(target_os = "windows")]
const MODS_DIRS: [&str; 1] = ["C:\\Program Files (x86)\\Steam\\steamapps\\common\\RimWorld\\Mods"];

#[cfg(target_os = "linux")]
const MODS_DIRS: [&str; 2] = [
    "~/.steam/steam/steamapps/common/RimWorld/Mods",
    "~/GOG/Games/RimWorld/game/Mods/",
];

pub fn detect_game_dir() -> Option<PathBuf> {
    MODS_DIRS.iter().find_map(|dir| {
        let path = PathBuf::from(shellexpand::tilde(dir).as_ref());
        match path.exists() && path.is_dir() {
            true => Some(path),
            false => None,
        }
    })
}
