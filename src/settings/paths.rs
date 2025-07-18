use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("failed to get config directory")
        .join("rimru")
}

pub fn settings_file() -> PathBuf {
    config_dir().join("settings.toml")
}
