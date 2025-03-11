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
