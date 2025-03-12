use std::{fs::read_dir, path::PathBuf};

use gpui::{Context, Entity};

use crate::{game::mods::Mod, settings::Settings};

pub struct Project {
    pub settings: Entity<Settings>,
    // this is the list of active mod ids, sourced from the config or save file
    pub active_mods: Vec<String>,
    // this is the list of all mods installed, sourced from the mods directory
    pub mods: Vec<Mod>,
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
        log::debug!("loading mods");

        let (local_mods_dir, steam_mods_dir) = self.settings.read_with(cx, |settings, _| {
            (
                settings.local_mods_dir().clone(),
                settings.steam_mods_dir().clone(),
            )
        });

        log::trace!("loading local mods from {:?}", local_mods_dir);
        self.load_mods_from_dir(&local_mods_dir, Mod::new_local);

        log::trace!("loading steam mods from {:?}", steam_mods_dir);
        self.load_mods_from_dir(&steam_mods_dir, Mod::new_steam);

        log::trace!("sorting loaded mods");
        self.mods.sort_by_key(|mod_meta| mod_meta.name.clone());

        log::debug!("finished loading mods");
    }

    fn load_mods_from_dir<F>(&mut self, dir: &PathBuf, mod_fn: F)
    where
        F: Fn(PathBuf) -> Option<Mod>,
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
}
