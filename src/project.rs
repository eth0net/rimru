use std::{fs::read_dir, path::Path};

use anyhow::Context as _;
use gpui::{Context, Entity};

use crate::{
    game::{mods::*, paths},
    settings::Settings,
};

#[derive(Debug, Clone)]
pub struct Project {
    /// rimru settings
    settings: Entity<Settings>,
    /// mods configuration loaded from the game
    mods_config: Option<ModsConfigData>,
    /// list of all installed mods (local and steam)
    mods: Vec<ModMetaData>,
    /// list of active mod ids, sourced from the config or save file
    active_mod_ids: Vec<String>,
    /// cached list of active mods
    cached_active_mods: Vec<ModMetaData>,
    /// cached list of inactive mods
    cached_inactive_mods: Vec<ModMetaData>,
    /// current selected mod in rimru
    selected_mod: Option<ModMetaData>,
}

impl Project {
    pub fn new(cx: &mut Context<Self>, settings: Entity<Settings>) -> Self {
        let mut project = Self {
            settings,
            mods_config: None,
            mods: Vec::new(),
            active_mod_ids: Vec::new(),
            cached_active_mods: Vec::new(),
            cached_inactive_mods: Vec::new(),
            selected_mod: None,
        };

        project.load_mods_config();
        project.load_mods(cx);
        project.apply_mods_config();
        project
    }

    /// Load mods configuration from file.
    ///
    /// This function parses the mods configuration from game files and updates the project.
    pub fn load_mods_config(&mut self) {
        log::debug!("loading mods config");
        match ModsConfigData::load() {
            Some(config) => {
                self.mods_config = Some(config);
            }
            None => {
                log::warn!("no mods config found");
            }
        }
    }

    /// Apply the loaded mods configuration.
    ///
    /// This function updates the active mods list based on the loaded configuration.
    pub fn apply_mods_config(&mut self) {
        log::debug!("applying mods config");
        match self.mods_config {
            Some(ref config) => {
                self.active_mod_ids = config.active_mods.clone();
            }
            None => {
                log::warn!("no mods config loaded");
                self.active_mod_ids = Vec::new();
            }
        }
        self.cache_mods();
    }

    /// Save mods configuration to file.
    ///
    /// This function updates the mods configuration file with the current active mods list.
    pub fn save_mods_config(&mut self) {
        match &mut self.mods_config {
            Some(mods_config) => {
                log::info!("saving mods config");
                mods_config.active_mods = self.active_mod_ids.clone();
                mods_config.save()
            }
            None => {
                log::error!("no mods config to save");
            }
        }
    }

    /// Load installed mods from mods directories.
    ///
    /// This function loads mods from the official mods directory, local mods directory, and Steam mods directory.
    pub fn load_mods(&mut self, cx: &mut Context<Self>) {
        log::debug!("loading mods");

        self.mods.clear();
        self.load_official_mods();
        self.load_local_mods(cx);
        self.load_steam_mods(cx);

        log::trace!("sorting loaded mods");
        self.mods.sort_by(|a, b| match a.name.cmp(&b.name) {
            std::cmp::Ordering::Equal => a.id.cmp(&b.id),
            other => other,
        });

        self.selected_mod = self.mods.first().cloned();
    }

    fn load_official_mods(&mut self) {
        let official_mods_dir = &paths::official_mods_dir();
        log::trace!("loading official mods from {:?}", official_mods_dir);
        self.load_mods_from_dir(official_mods_dir, |path| {
            ModMetaData::new_official(path).map(|mut om| {
                om.name = match om.id.split('.').last() {
                    Some(name) if name.eq_ignore_ascii_case("rimworld") => "Core".to_string(),
                    Some(name) => name.to_string(),
                    None => unreachable!(),
                };
                om
            })
        });
    }

    fn load_local_mods(&mut self, cx: &mut Context<Self>) {
        let local_mods_dir = self.settings.read(cx).local_mods_dir();
        log::trace!("loading local mods from {:?}", local_mods_dir);
        self.load_mods_from_dir(local_mods_dir, ModMetaData::new_local);
    }

    fn load_steam_mods(&mut self, cx: &mut Context<Self>) {
        let steam_mods_dir = self.settings.read(cx).steam_mods_dir();
        log::trace!("loading steam mods from {:?}", steam_mods_dir);
        let mods = self.mods.clone();
        self.load_mods_from_dir(steam_mods_dir, move |path| {
            ModMetaData::new_steam(path).map(|mut sm| {
                match mods
                    .iter()
                    .any(|m| m.source == Source::Local && m.id == sm.id)
                {
                    true => {
                        sm.id += "_steam";
                        sm
                    }
                    false => sm,
                }
            })
        });
    }

    fn load_mods_from_dir<F>(&mut self, dir: &Path, mod_fn: F)
    where
        F: Fn(&Path) -> Result<ModMetaData, String>,
    {
        match read_dir(dir) {
            Ok(entries) => {
                entries.for_each(|entry| match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            match mod_fn(&path) {
                                Ok(m) => self.mods.push(m),
                                Err(e) => {
                                    // todo: show placeholder mod in list?
                                    // todo: populate status bar?
                                    log::error!(
                                        "error loading mod from path: {}: {e}",
                                        path.display(),
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => log::warn!("error reading directory entry: {}", e),
                });
            }
            Err(_) => log::warn!("could not read directory"),
        }
    }

    pub fn active_mods(&self) -> Vec<ModMetaData> {
        self.cached_active_mods.clone()
    }

    pub fn inactive_mods(&self) -> Vec<ModMetaData> {
        self.cached_inactive_mods.clone()
    }

    fn cache_mods(&mut self) {
        log::debug!("refreshing cached mods");
        let (mut active, inactive): (Vec<_>, Vec<_>) = self.mods.iter().cloned().partition(|m| {
            let mod_id = m.id.to_ascii_lowercase();
            self.active_mod_ids.contains(&mod_id)
                || (m.source.is_steam() && self.active_mod_ids.contains(&(mod_id + "_steam")))
        });

        active.sort_by(|a, b| {
            let a_index = self
                .active_mod_ids
                .iter()
                .position(|id| id.eq_ignore_ascii_case(&a.id))
                .unwrap_or(usize::MAX);
            let b_index = self
                .active_mod_ids
                .iter()
                .position(|id| id.eq_ignore_ascii_case(&b.id))
                .unwrap_or(usize::MAX);

            match a_index.cmp(&b_index) {
                std::cmp::Ordering::Equal => a.id.cmp(&b.id),
                other => other,
            }
        });

        self.cached_active_mods = active;
        self.cached_inactive_mods = inactive;
    }

    pub fn selected_mod(&self) -> Option<&ModMetaData> {
        self.selected_mod.as_ref()
    }

    pub fn select_mod(&mut self, mod_meta: &ModMetaData) {
        self.selected_mod = Some(mod_meta.clone());
    }

    pub fn toggle_mod(&mut self, mod_meta: &ModMetaData) {
        match self
            .active_mod_ids
            .iter()
            .position(|id| id.eq_ignore_ascii_case(&mod_meta.id))
        {
            Some(index) => {
                self.active_mod_ids.remove(index);
                log::info!("deactivated mod: {}", mod_meta.id);
            }
            None => {
                self.active_mod_ids.push(mod_meta.id.to_ascii_lowercase());
                log::info!("activated mod: {}", mod_meta.id);
            }
        }
        self.cache_mods();
    }

    pub fn move_active_mod(&mut self, source: String, target: String) -> anyhow::Result<()> {
        log::debug!("moving mod {source} to {target}");
        if source == target {
            return Ok(());
        }

        let mut source_index = None;
        let mut target_index = None;
        for (i, mod_id) in self.active_mod_ids.iter().enumerate() {
            if mod_id.eq_ignore_ascii_case(&source) {
                source_index = Some(i);
                if target_index.is_some() {
                    break;
                }
            }
            if mod_id.eq_ignore_ascii_case(&target) {
                target_index = Some(i);
                if source_index.is_some() {
                    break;
                }
            }
        }

        let source_index = source_index.with_context(|| "dragged mod is not active {source}")?;
        let target_index = target_index.with_context(|| "target mod is not active {target}")?;

        let moving = self.active_mod_ids.remove(source_index);
        self.active_mod_ids.insert(target_index, moving);
        self.cache_mods();
        Ok(())
    }

    pub fn clear_active_mods(&mut self) {
        log::info!("clearing active mods");
        self.active_mod_ids.clear();
        self.cache_mods();
    }

    // pub fn sort_active_mods(&mut self) {
    //     log::debug!("sorting active mods");
    //     self.active_mod_ids.sort_by(|a, b| {
    //         let mod_a
    //     });
    // }
}
