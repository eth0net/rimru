use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{File, metadata},
    io::BufReader,
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::game::{paths, xml::create_reader};

mod parser;
mod source;

use parser::*;
pub use source::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ModMetaData {
    pub id: String,
    pub name: String,
    pub short_name: Option<String>,
    pub icon_path: Option<PathBuf>,
    pub version: Option<String>,
    pub authors: Vec<String>,
    pub description: String,
    pub descriptions_by_version: BTreeMap<String, String>,
    pub dependencies: BTreeMap<String, ModDependency>,
    pub dependencies_by_version: BTreeMap<String, BTreeMap<String, ModDependency>>,
    pub load_after: BTreeSet<String>,
    pub load_after_by_version: BTreeMap<String, BTreeSet<String>>,
    pub load_before: BTreeSet<String>,
    pub load_before_by_version: BTreeMap<String, BTreeSet<String>>,
    pub force_load_after: BTreeSet<String>,
    pub force_load_before: BTreeSet<String>,
    pub incompatible_with: BTreeSet<String>,
    pub incompatible_with_by_version: BTreeMap<String, BTreeSet<String>>,
    pub steam_app_id: Option<String>,
    pub supported_versions: Vec<String>,
    pub url: Option<String>,
    pub path: PathBuf,
    pub source: Source,
    pub created: Option<SystemTime>,
    pub modified: Option<SystemTime>,
}

impl ModMetaData {
    pub fn new(path: &Path) -> Result<Self, String> {
        if !path.is_dir() {
            log::error!("path is not a directory: {path:?}");
            return Err("path is not a directory".into());
        }

        let mut mod_meta = ModMetaData {
            path: path.to_path_buf(),
            ..Default::default()
        };

        let dir_meta = metadata(path).map_err(|e| format!("getting directory metadata: {e}"))?;
        match dir_meta.created() {
            Ok(created) => mod_meta.created = Some(created),
            Err(e) => log::error!("getting date created: {e}"),
        }
        match dir_meta.modified() {
            Ok(modified) => mod_meta.modified = Some(modified),
            Err(e) => log::error!("getting date modified: {e}"),
        }

        let file = mod_meta.about_file_path();
        let file = File::open(&file).map_err(|e| format!("opening file {file:?}: {e}"))?;
        let reader = BufReader::new(file);
        let events = create_reader(reader);
        parse_mod_metadata(events, &mut mod_meta)?;

        Ok(mod_meta)
    }

    pub fn new_official(path: &Path) -> Result<Self, String> {
        Self::new(path).map(|mut mod_meta| {
            mod_meta.source = Source::Official;
            mod_meta
        })
    }

    pub fn new_local(path: &Path) -> Result<Self, String> {
        Self::new(path).map(|mut mod_meta| {
            mod_meta.source = Source::Local;
            mod_meta
        })
    }

    pub fn new_steam(path: &Path) -> Result<Self, String> {
        Self::new(path).map(|mut mod_meta| {
            mod_meta.source = Source::Steam;
            if mod_meta.steam_app_id.is_none() {
                if let Some(dir_name) = mod_meta.path.file_name().and_then(|name| name.to_str()) {
                    mod_meta.steam_app_id = Some(dir_name.to_string());
                }
            }
            mod_meta
        })
    }

    pub fn about_file_path(&self) -> PathBuf {
        paths::mod_about_file(&self.path)
    }

    pub fn preview_file_path(&self) -> PathBuf {
        paths::mod_preview_file(&self.path)
    }

    pub fn icon_file_path(&self) -> PathBuf {
        paths::mod_icon_file(&self.path)
    }

    pub fn is_official(&self) -> bool {
        self.source.is_official()
    }

    pub fn is_local(&self) -> bool {
        self.source.is_local()
    }

    pub fn is_steam(&self) -> bool {
        self.source.is_steam()
    }

    pub fn depends_on(&self, id: &str) -> bool {
        self.dependencies.contains_key(id)
    }

    pub fn load_after(&self, id: &str) -> bool {
        self.load_after.contains(id)
    }

    pub fn load_before(&self, id: &str) -> bool {
        self.load_before.contains(id)
    }

    pub fn force_load_after(&self, id: &str) -> bool {
        self.force_load_after.contains(id)
    }

    pub fn force_load_before(&self, id: &str) -> bool {
        self.force_load_before.contains(id)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ModDependency {
    pub id: String,
    pub name: String,
}

impl From<&ModMetaData> for ModDependency {
    fn from(mod_meta: &ModMetaData) -> Self {
        ModDependency {
            id: mod_meta.id.clone(),
            name: mod_meta.name.clone(),
        }
    }
}
