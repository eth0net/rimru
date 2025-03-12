use std::{fs::read_dir, path::PathBuf};

use gpui::{Context, Entity};

use crate::{
    game::mods::{ModMeta, ModsConfig},
    settings::Settings,
};

pub struct Project {
    // rimru settings
    pub settings: Entity<Settings>,
    // mods configuration loaded from the game
    pub mods_config: Option<ModsConfig>,
    // list of all installed mods (local and steam)
    pub mods: Vec<ModMeta>,
    // list of active mod ids, sourced from the config or save file
    pub active_mods: Vec<String>,
    // current selected mod in rimru
    pub selected_mod: Option<ModMeta>,
}

impl Project {
    pub fn new(cx: &mut Context<Self>, settings: Entity<Settings>) -> Self {
        let mut project = Self {
            settings,
            mods_config: None,
            active_mods: Vec::new(),
            mods: Vec::new(),
            selected_mod: None,
        };

        match ModsConfig::load() {
            Some(config) => {
                project.active_mods = config.active_mods.clone();
                project.mods_config = Some(config);
            }
            None => {
                log::warn!("no mods config found");
                project.active_mods = Vec::new();
                project.mods_config = None;
            }
        }

        project.load_mods(cx);
        project
    }

    fn load_mods(&mut self, cx: &mut Context<Self>) {
        log::debug!("loading mods");

        let (local_mods_dir, steam_mods_dir) = self.settings.read_with(cx, |settings, _| {
            (
                settings.local_mods_dir().clone(),
                settings.steam_mods_dir().clone(),
            )
        });

        log::trace!("loading local mods from {:?}", local_mods_dir);
        self.load_mods_from_dir(&local_mods_dir, ModMeta::new_local);

        log::trace!("loading steam mods from {:?}", steam_mods_dir);
        self.load_mods_from_dir(&steam_mods_dir, ModMeta::new_steam);

        log::trace!("sorting loaded mods");
        self.mods.sort_by_key(|mod_meta| mod_meta.name.clone());

        self.selected_mod = self.mods.first().cloned();

        log::debug!("finished loading mods");
    }

    fn load_mods_from_dir<F>(&mut self, dir: &PathBuf, mod_fn: F)
    where
        F: Fn(PathBuf) -> Option<ModMeta>,
    {
        match read_dir(dir) {
            Ok(entries) => {
                entries.for_each(|entry| match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            if let Some(m) = mod_fn(path) {
                                self.mods.push(m)
                            }
                        }
                    }
                    Err(e) => log::warn!("error reading directory entry: {}", e),
                });
            }
            Err(_) => log::warn!("could not read directory"),
        }
    }

    pub fn toggle_mod(&mut self, mod_id: &str) {
        if let Some(index) = self.active_mods.iter().position(|id| id == mod_id) {
            self.active_mods.remove(index);
            log::info!("deactivated mod: {}", mod_id);
        } else {
            self.active_mods.push(mod_id.to_string());
            log::info!("activated mod: {}", mod_id);
        }
    }

    pub fn active_mods(&self) -> Vec<ModMeta> {
        let mut active_mods: Vec<ModMeta> = self
            .mods
            .iter()
            .filter(|m| self.active_mods.contains(&m.id.to_ascii_lowercase()))
            .cloned()
            .collect();

        active_mods.sort_by(|a, b| {
            let a_index = self
                .active_mods
                .iter()
                .position(|id| id == &a.id.to_ascii_lowercase())
                .unwrap_or(usize::MAX);
            let b_index = self
                .active_mods
                .iter()
                .position(|id| id == &b.id.to_ascii_lowercase())
                .unwrap_or(usize::MAX);

            a_index.cmp(&b_index)
        });

        active_mods
    }

    pub fn inactive_mods(&self) -> Vec<ModMeta> {
        self.mods
            .iter()
            .filter(|m| !self.active_mods.contains(&m.id.to_ascii_lowercase()))
            .cloned()
            .collect()
    }

    pub fn selected_mod(&self) -> Option<&ModMeta> {
        self.selected_mod.as_ref()
    }

    pub fn select_mod(&mut self, mod_meta: &ModMeta) {
        self.selected_mod = Some(mod_meta.clone());
    }
}
