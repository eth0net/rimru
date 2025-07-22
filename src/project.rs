use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_dir,
    path::Path,
};

use anyhow::Context as _;
use gpui::{Context, Entity};

use crate::{game::mods::*, settings::Settings};

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
    /// flag to indicate if settings pane is open
    settings_open: bool,
    /// map of mod id (lowercase) to mod issues
    mod_issues: HashMap<String, ModIssue>,
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
            settings_open: false,
            mod_issues: HashMap::new(),
        };

        project.load_mods_config(cx);
        project.load_mods(cx);
        project.apply_mods_config();
        project.update_mod_issues();
        project
    }

    /// Load mods configuration from file.
    ///
    /// This function parses the mods configuration from game files and updates the project.
    pub fn load_mods_config(&mut self, cx: &mut Context<Self>) {
        let path = &self.settings.read(cx).mods_config_file();
        log::debug!("loading mods config from {path:?}");
        match ModsConfigData::load(path) {
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
        self.update_mod_issues();
    }

    /// Save mods configuration to file.
    ///
    /// This function updates the mods configuration file with the current active mods list.
    pub fn save_mods_config(&mut self, cx: &mut Context<Self>) {
        match &mut self.mods_config {
            Some(mods_config) => {
                let path = &self.settings.read(cx).mods_config_file();
                log::info!("saving mods config to {path:?}");
                mods_config.active_mods = self.active_mod_ids.clone();
                mods_config.save(path)
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
        self.load_official_mods(cx);
        self.load_local_mods(cx);
        self.load_steam_mods(cx);

        log::trace!("sorting loaded mods");
        self.mods.sort_by(|a, b| match a.name.cmp(&b.name) {
            std::cmp::Ordering::Equal => a.id.cmp(&b.id),
            other => other,
        });

        self.selected_mod = self.mods.first().cloned();
        self.cache_mods();
        self.update_mod_issues();
    }

    fn load_official_mods(&mut self, cx: &mut Context<Self>) {
        let official_mods_dir = self.settings.read(cx).official_mods_dir();
        log::trace!("loading official mods from {official_mods_dir:?}");
        self.load_mods_from_dir(official_mods_dir, |path| {
            ModMetaData::new_official(path).map(|mut om| {
                om.name = match om.id.split('.').next_back() {
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
        log::trace!("loading local mods from {local_mods_dir:?}");
        self.load_mods_from_dir(local_mods_dir, ModMetaData::new_local);
    }

    fn load_steam_mods(&mut self, cx: &mut Context<Self>) {
        let steam_mods_dir = self.settings.read(cx).steam_mods_dir();
        log::trace!("loading steam mods from {steam_mods_dir:?}");
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
                    Err(e) => log::warn!("error reading directory entry: {e}"),
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

    pub fn cache_mods(&mut self) {
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
        self.update_mod_issues();
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
        self.update_mod_issues();
    }

    pub fn sort_active_mods(&mut self) {
        log::debug!("sorting active mods");
        let mut active_mods = self.active_mods();
        active_mods.sort_by(Order::Topological.sort_fn());
        self.active_mod_ids = active_mods
            .iter()
            .map(|m| m.id.to_ascii_lowercase())
            .collect();
        self.cached_active_mods = active_mods;
        self.update_mod_issues();
    }

    pub fn toggle_settings(&mut self, cx: &mut Context<Self>) {
        self.settings.update(cx, |settings, _cx| {
            if self.settings_open {
                settings.save();
            } else if let Some(loaded_settings) = Settings::load() {
                *settings = loaded_settings;
            } else {
                log::warn!("no settings found, using defaults");
            }
        });
        self.settings_open = !self.settings_open;
        self.load_mods(cx);
        self.cache_mods();
        self.update_mod_issues();
    }

    pub fn is_settings_open(&self) -> bool {
        self.settings_open
    }

    // /// Update the map of mod issues and log them for debugging/UI.
    // pub fn update_mod_issues(&mut self) {
    //     use std::collections::HashMap;

    //     let mod_map: HashMap<String, &ModMetaData> = self
    //         .mods
    //         .iter()
    //         .map(|m| (m.id.to_ascii_lowercase(), m))
    //         .collect();
    //     let active_ids: Vec<String> = self
    //         .active_mod_ids
    //         .iter()
    //         .map(|s| s.to_ascii_lowercase())
    //         .collect();

    //     let mut issues = HashMap::new();

    //     for (idx, mod_id) in active_ids.iter().enumerate() {
    //         let mut missing_deps = Vec::new();
    //         let mut deps_after = Vec::new();
    //         let mut load_order_violations = Vec::new();

    //         if let Some(mod_meta) = mod_map.get(mod_id) {
    //             // 1. Check for missing dependencies
    //             for dep_id in mod_meta.dependencies.keys() {
    //                 let dep_id_lc = dep_id.to_ascii_lowercase();
    //                 if !active_ids.contains(&dep_id_lc) {
    //                     log::warn!("Mod '{mod_id}' is missing dependency '{dep_id}'");
    //                     missing_deps.push(dep_id.clone());
    //                 } else {
    //                     // Dependency is present, but warn if it loads after this mod
    //                     let dep_idx = active_ids.iter().position(|id| id == &dep_id_lc).unwrap();
    //                     if dep_idx > idx {
    //                         log::info!(
    //                             "Mod '{mod_id}' dependency '{dep_id}' loads after it (index {dep_idx} > {idx})"
    //                         );
    //                         deps_after.push(dep_id.clone());
    //                     }
    //                 }
    //             }

    //             // 2. Check load_after/force_load_after: these mods must come before this mod
    //             for after_id in mod_meta
    //                 .load_after
    //                 .iter()
    //                 .chain(mod_meta.force_load_after.iter())
    //             {
    //                 let after_id_lc = after_id.to_ascii_lowercase();
    //                 if let (Some(this_idx), Some(after_idx)) = (
    //                     active_ids.iter().position(|id| id == mod_id),
    //                     active_ids.iter().position(|id| id == &after_id_lc),
    //                 ) {
    //                     if after_idx > this_idx {
    //                         let msg = format!(
    //                             "Should load after '{after_id}', but '{mod_id}' comes after '{after_id}'"
    //                         );
    //                         log::warn!("Load order violation: {msg}");
    //                         load_order_violations.push(msg);
    //                     }
    //                 }
    //             }

    //             // 3. Check load_before/force_load_before: these mods must come after this mod
    //             for before_id in mod_meta
    //                 .load_before
    //                 .iter()
    //                 .chain(mod_meta.force_load_before.iter())
    //             {
    //                 let before_id_lc = before_id.to_ascii_lowercase();
    //                 if let (Some(this_idx), Some(before_idx)) = (
    //                     active_ids.iter().position(|id| id == mod_id),
    //                     active_ids.iter().position(|id| id == &before_id_lc),
    //                 ) {
    //                     if before_idx < this_idx {
    //                         let msg = format!(
    //                             "Should load before '{before_id}', but '{mod_id}' comes before '{before_id}'"
    //                         );
    //                         log::warn!("Load order violation: {msg}");
    //                         load_order_violations.push(msg);
    //                     }
    //                 }
    //             }
    //         }

    //         if !missing_deps.is_empty()
    //             || !deps_after.is_empty()
    //             || !load_order_violations.is_empty()
    //         {
    //             issues.insert(
    //                 mod_id.clone(),
    //                 ModIssue {
    //                     mod_id: mod_id.clone(),
    //                     missing_dependencies: missing_deps,
    //                     dependencies_after: deps_after,
    //                     load_order_violations,
    //                 },
    //             );
    //         }
    //     }

    //     if !issues.is_empty() {
    //         log::info!("Collected {} mod issues for UI presentation", issues.len());
    //     }
    //     self.mod_issues = issues;
    // }

    /// Get the current map of mod issues for UI presentation.
    pub fn mod_issues(&self) -> &HashMap<String, ModIssue> {
        &self.mod_issues
    }

    /// Get the issues for a specific mod id (case-insensitive).
    pub fn issues_for_mod(&self, mod_id: &str) -> Option<&ModIssue> {
        self.mod_issues.get(&mod_id.to_ascii_lowercase())
    }

    /// Get a vector of issues for a list of mod ids, in the same order.
    pub fn issues_for_mods<I>(&self, mod_ids: I) -> Vec<Option<&ModIssue>>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        mod_ids
            .into_iter()
            .map(|id| self.issues_for_mod(id.as_ref()))
            .collect()
    }

    pub fn update_mod_issues(&mut self) {
        let active_ids: Vec<String> = self
            .active_mod_ids
            .iter()
            .map(|s| s.to_ascii_lowercase())
            .collect();

        if let Some(cycle_mods) = self.detect_cycle(&active_ids) {
            self.report_cycle_issues(cycle_mods);
            return;
        }

        self.collect_regular_issues(&active_ids);
    }

    fn detect_cycle(&self, active_ids: &[String]) -> Option<Vec<String>> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        for mod_id in active_ids {
            graph.entry(mod_id.clone()).or_default();
        }
        for (idx, mod_id) in active_ids.iter().enumerate() {
            if let Some(mod_meta) = self.cached_active_mods.get(idx) {
                let mut edges = Vec::new();
                for after_id in mod_meta
                    .load_after
                    .iter()
                    .chain(mod_meta.force_load_after.iter())
                {
                    let after_id_lc = after_id.to_ascii_lowercase();
                    if active_ids.contains(&after_id_lc) {
                        edges.push(after_id_lc);
                    }
                }
                graph.insert(mod_id.clone(), edges);
            }
        }

        // Kahn's algorithm for topological sort (cycle check only)
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for mod_id in active_ids {
            in_degree.insert(mod_id.clone(), 0);
        }
        for edges in graph.values() {
            for dep in edges {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }

        let mut queue: VecDeque<String> = in_degree
            .iter()
            .filter_map(|(k, &v)| if v == 0 { Some(k.clone()) } else { None })
            .collect();

        let mut visited = 0;
        while let Some(mod_id) = queue.pop_front() {
            visited += 1;
            if let Some(edges) = graph.get(&mod_id) {
                for dep in edges {
                    if let Some(e) = in_degree.get_mut(dep) {
                        *e -= 1;
                        if *e == 0 {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }

        if visited != active_ids.len() {
            let cycle_mods: Vec<String> = in_degree
                .iter()
                .filter(|(_, deg)| **deg > 0)
                .map(|(id, _)| id.clone())
                .collect();
            Some(cycle_mods)
        } else {
            None
        }
    }

    fn report_cycle_issues(&mut self, cycle_mods: Vec<String>) {
        log::error!("Cycle detected in mod load order; cannot sort active mods.");
        log::error!("Mods in cycle: {:?}", cycle_mods);
        let mut issues = HashMap::new();
        for mod_id in cycle_mods {
            issues.insert(
                mod_id.clone(),
                ModIssue {
                    mod_id: mod_id.clone(),
                    missing_dependencies: Vec::new(),
                    dependencies_after: Vec::new(),
                    load_order_violations: vec![
                        "Cycle detected in mod load order involving this mod".to_string(),
                    ],
                },
            );
        }
        self.mod_issues = issues;
    }

    fn collect_regular_issues(&mut self, active_ids: &[String]) {
        let active_id_set: HashSet<String> = active_ids.iter().cloned().collect();
        let id_to_index: HashMap<String, usize> = active_ids
            .iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect();

        let mod_map: HashMap<String, &ModMetaData> = self
            .mods
            .iter()
            .map(|m| (m.id.to_ascii_lowercase(), m))
            .collect();

        let mut issues = HashMap::new();

        for (idx, mod_id) in active_ids.iter().enumerate() {
            let mut missing_deps = Vec::new();
            let mut deps_after = Vec::new();
            let mut load_order_violations = Vec::new();

            if let Some(mod_meta) = mod_map.get(mod_id) {
                // 1. Check for missing dependencies
                for dep_id in mod_meta.dependencies.keys() {
                    let dep_id_lc = dep_id.to_ascii_lowercase();
                    if !active_id_set.contains(&dep_id_lc) {
                        log::warn!("Mod '{mod_id}' is missing dependency '{dep_id}'");
                        missing_deps.push(dep_id.clone());
                    } else if let Some(&dep_idx) = id_to_index.get(&dep_id_lc) {
                        if dep_idx > idx {
                            log::info!(
                                "Mod '{mod_id}' dependency '{dep_id}' loads after it (index {dep_idx} > {idx})"
                            );
                            deps_after.push(dep_id.clone());
                        }
                    }
                }

                // 2. Check load_after/force_load_after: these mods must come before this mod
                for after_id in mod_meta
                    .load_after
                    .iter()
                    .chain(mod_meta.force_load_after.iter())
                {
                    let after_id_lc = after_id.to_ascii_lowercase();
                    if let (Some(&this_idx), Some(&after_idx)) =
                        (id_to_index.get(mod_id), id_to_index.get(&after_id_lc))
                    {
                        if after_idx > this_idx {
                            let msg = format!(
                                "Should load after '{after_id}', but '{mod_id}' comes after '{after_id}'"
                            );
                            log::warn!("Load order violation: {msg}");
                            load_order_violations.push(msg);
                        }
                    }
                }

                // 3. Check load_before/force_load_before: these mods must come after this mod
                for before_id in mod_meta
                    .load_before
                    .iter()
                    .chain(mod_meta.force_load_before.iter())
                {
                    let before_id_lc = before_id.to_ascii_lowercase();
                    if let (Some(&this_idx), Some(&before_idx)) =
                        (id_to_index.get(mod_id), id_to_index.get(&before_id_lc))
                    {
                        if before_idx < this_idx {
                            let msg = format!(
                                "Should load before '{before_id}', but '{mod_id}' comes before '{before_id}'"
                            );
                            log::warn!("Load order violation: {msg}");
                            load_order_violations.push(msg);
                        }
                    }
                }
            }

            if !missing_deps.is_empty()
                || !deps_after.is_empty()
                || !load_order_violations.is_empty()
            {
                issues.insert(
                    mod_id.clone(),
                    ModIssue {
                        mod_id: mod_id.clone(),
                        missing_dependencies: missing_deps,
                        dependencies_after: deps_after,
                        load_order_violations,
                    },
                );
            }
        }

        log::info!("Found {} mod issues", issues.len());
        self.mod_issues = issues;
    }
}
