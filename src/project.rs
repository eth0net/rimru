use std::{fs::read_dir, path::PathBuf};

use gpui::{Context, Entity};

use crate::settings::Settings;

pub struct Project {
    pub settings: Entity<Settings>,
    // this is the list of active mod ids, sourced from the config or save file
    pub active_mods: Vec<String>,
    // this is the list of all mods installed, sourced from the mods directory
    pub mods: Vec<ModMeta>,
}

impl Project {
    pub fn new(cx: &mut Context<Self>, settings: Entity<Settings>) -> Self {
        let mut project = Self {
            settings,
            active_mods: Vec::new(),
            mods: Vec::new(),
        };
        project.load_mods(cx);
        project
    }

    fn load_mods(&mut self, cx: &mut Context<Self>) {
        log::trace!("loading mods");

        self.settings.read_with(cx, |settings, _| {
            log::trace!("loading local mods from {:?}", settings.local_mods_dir());
            if let Ok(entries) = read_dir(settings.local_mods_dir()) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(id) = path.file_name().and_then(|name| name.to_str()) {
                            self.mods.push(ModMeta {
                                id: id.to_string(),
                                name: id.to_string(),
                                path: path.clone(),
                                mod_type: ModType::Local,
                            });
                        }
                    }
                }
            }

            log::trace!("loading steam mods from {:?}", settings.steam_mods_dir());
            if let Ok(entries) = read_dir(settings.steam_mods_dir()) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(id) = path.file_name().and_then(|name| name.to_str()) {
                            self.mods.push(ModMeta {
                                id: id.to_string(),
                                name: id.to_string(),
                                path: path.clone(),
                                mod_type: ModType::Steam,
                            });
                        }
                    }
                }
            }

            self.mods.sort_by_key(|mod_meta| mod_meta.id.clone());

            log::trace!("finished loading mods");
        });
    }
}

#[derive(Debug, Clone)]
pub struct ModMeta {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub mod_type: ModType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModType {
    Local,
    Steam,
}
