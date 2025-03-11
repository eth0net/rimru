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
        // todo: detect active mods
        project
    }

    fn load_mods(&mut self, cx: &mut Context<Self>) {
        log::trace!("loading mods");

        let (local_mods_dir, steam_mods_dir) = self.settings.read_with(cx, |settings, _| {
            (
                settings.local_mods_dir().clone(),
                settings.steam_mods_dir().clone(),
            )
        });

        log::trace!("loading local mods from {:?}", local_mods_dir);
        self.load_mods_from_dir(&local_mods_dir, |_| Source::Local);

        log::trace!("loading steam mods from {:?}", steam_mods_dir);
        self.load_mods_from_dir(&steam_mods_dir, |id| Source::Steam { id });

        log::trace!("sorting loaded mods");
        self.mods.sort_by_key(|mod_meta| mod_meta.id.clone());

        log::trace!("finished loading mods");
    }

    fn load_mods_from_dir<F>(&mut self, dir: &PathBuf, source_fn: F)
    where
        F: Fn(String) -> Source,
    {
        if let Ok(entries) = read_dir(dir) {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if !path.is_dir() {
                            continue;
                        }

                        // todo: parse about.xml

                        if let Some(dir) = path.file_name().and_then(|name| name.to_str()) {
                            self.mods.push(ModMeta {
                                id: dir.to_string(),   // todo: get id from about.xml
                                name: dir.to_string(), // todo: get name from about.xml
                                path: path.clone(),
                                source: source_fn(dir.to_string()),
                            });
                        }
                    }
                    Err(e) => {
                        log::warn!("Error reading directory entry: {}", e);
                    }
                }
            }
        } else {
            log::warn!("Could not read directory");
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModMeta {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub source: Source,
}

impl ModMeta {
    pub fn is_local(&self) -> bool {
        self.source.is_local()
    }

    pub fn is_steam(&self) -> bool {
        self.source.is_steam()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    Local,
    Steam { id: String },
}

impl Source {
    pub fn is_local(&self) -> bool {
        matches!(self, Source::Local)
    }

    pub fn is_steam(&self) -> bool {
        matches!(self, Source::Steam { .. })
    }
}
