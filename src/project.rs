use std::{fs::read_dir, path::PathBuf};

use gpui::{Context, Entity};

use crate::{
    game::{
        mods::{
            config::ModsConfigData,
            meta::{ModMetaData, Source},
        },
        paths,
    },
    settings::Settings,
};

#[derive(Debug, Clone)]
pub struct Project {
    // rimru settings
    pub settings: Entity<Settings>,
    // mods configuration loaded from the game
    pub mods_config: Option<ModsConfigData>,
    // list of all installed mods (local and steam)
    pub mods: Vec<ModMetaData>,
    // list of active mod ids, sourced from the config or save file
    pub active_mods: Vec<String>,
    // current selected mod in rimru
    pub selected_mod: Option<ModMetaData>,
}

impl Project {
    pub fn new(cx: &mut Context<Self>, settings: Entity<Settings>) -> Self {
        let mut project = Self {
            settings,
            mods_config: None,
            mods: Vec::new(),
            active_mods: Vec::new(),
            selected_mod: None,
        };

        project.load_mods_config();
        project.load_mods(cx);
        project
    }

    fn load_mods_config(&mut self) {
        log::debug!("loading mods config");
        match ModsConfigData::load() {
            Some(config) => {
                self.active_mods = config.active_mods.clone();
                self.mods_config = Some(config);
            }
            None => {
                log::warn!("no mods config found");
                self.active_mods = Vec::new();
            }
        }
    }

    fn load_mods(&mut self, cx: &mut Context<Self>) {
        log::debug!("loading mods");

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

    fn load_mods_from_dir<F>(&mut self, dir: &PathBuf, mod_fn: F)
    where
        F: Fn(PathBuf) -> Option<ModMetaData>,
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

    pub fn active_mods(&self) -> Vec<ModMetaData> {
        let mut active_mods: Vec<ModMetaData> = self
            .mods
            .iter()
            .filter(|m| {
                let mod_id = m.id.to_ascii_lowercase();
                self.active_mods.contains(&mod_id)
                    || (m.source.is_steam() && self.active_mods.contains(&(mod_id + "_steam")))
            })
            .cloned()
            .collect();

        active_mods.sort_by(|a, b| {
            let a_index = self
                .active_mods
                .iter()
                .position(|id| id.eq_ignore_ascii_case(&a.id))
                .unwrap_or(usize::MAX);
            let b_index = self
                .active_mods
                .iter()
                .position(|id| id.eq_ignore_ascii_case(&b.id))
                .unwrap_or(usize::MAX);

            match a_index.cmp(&b_index) {
                std::cmp::Ordering::Equal => a.id.cmp(&b.id),
                other => other,
            }
        });

        active_mods
    }

    pub fn inactive_mods(&self) -> Vec<ModMetaData> {
        self.mods
            .iter()
            .filter(|m| !self.active_mods.contains(&m.id.to_ascii_lowercase()))
            .cloned()
            .collect()
    }

    pub fn selected_mod(&self) -> Option<&ModMetaData> {
        self.selected_mod.as_ref()
    }

    pub fn select_mod(&mut self, mod_meta: &ModMetaData) {
        self.selected_mod = Some(mod_meta.clone());
    }

    pub fn toggle_mod(&mut self, mod_meta: &ModMetaData) {
        match self
            .active_mods
            .iter()
            .position(|id| id.eq_ignore_ascii_case(&mod_meta.id))
        {
            Some(index) => {
                self.active_mods.remove(index);
                log::info!("deactivated mod: {}", mod_meta.id);
            }
            None => {
                self.active_mods.push(mod_meta.id.to_ascii_lowercase());
                log::info!("activated mod: {}", mod_meta.id);
            }
        }
    }

    pub fn save_mod_config(&self) {
        match &self.mods_config {
            Some(mods_config) => {
                log::info!("saving mods config");
                mods_config.save()
            }
            None => {
                log::error!("no mods config to save");
            }
        }
    }
}
