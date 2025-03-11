use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Mod {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub source: Source,
}

impl Mod {
    pub fn is_local(&self) -> bool {
        self.source.is_local()
    }

    pub fn is_steam(&self) -> bool {
        self.source.is_steam()
    }

    pub fn new_local(path: PathBuf) -> Option<Self> {
        if !path.is_dir() {
            log::error!("Path is not a directory: {:?}", path);
            return None;
        }

        // todo: parse about.xml

        let dir_name = path.file_name().and_then(|name| name.to_str())?;
        let id = dir_name.to_string(); // todo: get id from about.xml
        let name = dir_name.to_string(); // todo: get name from about.xml
        let source = Source::Local;

        Some(Mod {
            id,
            name,
            path,
            source,
        })
    }

    pub fn new_steam(path: PathBuf) -> Option<Self> {
        Self::new_local(path).and_then(|mut m| {
            let dir_name = m.path.file_name().and_then(|name| name.to_str())?;

            m.source = Source::Steam {
                id: dir_name.to_string(),
            };

            Some(m)
        })
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
