use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use xml::reader::{EventReader, XmlEvent as ReaderEvent};

use crate::{
    game::{paths, xml::*},
    ui::IconName,
};

#[derive(Debug, Clone, Default)]
pub struct ModMetaData {
    pub id: String,
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub steam_app_id: Option<String>,
    pub path: PathBuf,
    pub source: Source,
}

impl ModMetaData {
    // todo: use result instead
    pub fn new(path: PathBuf) -> Option<Self> {
        if !path.is_dir() {
            log::error!("Path is not a directory: {path:?}");
            return None;
        }

        let mut mod_meta = ModMetaData {
            path: path.clone(),
            ..Default::default()
        };

        let about_path = mod_meta.about_file_path();
        match load_mod_metadata_from_file(&about_path, &mut mod_meta) {
            Ok(_) => Some(mod_meta),
            Err(e) => {
                log::error!("Error loading mod metadata from {about_path:?}: {e}");
                None
            }
        }
    }

    pub fn new_official(path: PathBuf) -> Option<Self> {
        Self::new(path).map(|mut mod_meta| {
            mod_meta.source = Source::Official;
            mod_meta
        })
    }

    pub fn new_local(path: PathBuf) -> Option<Self> {
        Self::new(path).map(|mut mod_meta| {
            mod_meta.source = Source::Local;
            mod_meta
        })
    }

    pub fn new_steam(path: PathBuf) -> Option<Self> {
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
}

fn load_mod_metadata_from_file(path: &Path, mod_meta: &mut ModMetaData) -> ParseResult<()> {
    let file = File::open(path).map_err(|e| format!("Error opening file {path:?}: {e}"))?;
    let reader = BufReader::new(file);
    let events = create_reader(reader);

    parse_mod_metadata(events, path, mod_meta)
}

fn parse_mod_metadata<R: Read>(
    mut events: EventReader<R>,
    path: &Path,
    mod_meta: &mut ModMetaData,
) -> ParseResult<()> {
    loop {
        match events.next() {
            Ok(ReaderEvent::EndDocument) => break,
            Ok(ReaderEvent::StartDocument { .. }) => {}
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("modMetaData") =>
            {
                parse_mod_metadata_data(&mut events, path, mod_meta)?;
            }
            Ok(event) => log::trace!("unexpected root event {event:?} from {path:?}"),
            Err(e) => {
                return Err(format!("error parsing root event from {path:?}: {e}"));
            }
        }
    }
    Ok(())
}

fn parse_mod_metadata_data<R: Read>(
    events: &mut EventReader<R>,
    path: &Path,
    mod_meta: &mut ModMetaData,
) -> ParseResult<()> {
    loop {
        match events.next() {
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("author") =>
            {
                // Handle single author tag
                let author_string = parse_text_element(events, path, "author")?;
                for author in author_string.split(',') {
                    mod_meta.authors.push(author.trim().to_string());
                }
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("authors") =>
            {
                mod_meta.authors = parse_string_list(events, path, "authors")?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("description") =>
            {
                mod_meta.description = parse_text_element(events, path, "description")?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name
                    .local_name
                    .eq_ignore_ascii_case("descriptionsByVersion") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("forceLoadAfter") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("forceLoadBefore") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("incompatibleWith") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name
                    .local_name
                    .eq_ignore_ascii_case("incompatibleWithByVersion") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("loadAfter") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("loadAfterByVersion") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("loadBefore") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("loadBeforeByVersion") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("modDependencies") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name
                    .local_name
                    .eq_ignore_ascii_case("modDependenciesByVersion") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("modIconPath") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("modVersion") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("name") =>
            {
                mod_meta.name = parse_text_element(events, path, "name")?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("packageId") =>
            {
                mod_meta.id = parse_text_element(events, path, "packageId")?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("shortName") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("steamAppId") =>
            {
                mod_meta.steam_app_id = Some(parse_text_element(events, path, "steamAppId")?);
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("supportedVersions") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("url") =>
            {
                // todo: read and process the elements
                skip_element(events)?;
            }
            Ok(ReaderEvent::EndElement { name })
                if name.local_name.eq_ignore_ascii_case("modMetaData") =>
            {
                break;
            }
            Ok(ReaderEvent::Characters(chars)) => {
                // ignore whitespace characters
                if !chars.trim().is_empty() {
                    log::warn!("unexpected characters {chars} in modMetaData from {path:?}");
                }
            }
            Ok(event) => {
                log::warn!("unexpected event {event:?} in modMetaData from {path:?}");
            }
            Err(e) => {
                return Err(format!(
                    "error parsing event in modMetaData from {path:?}: {e}",
                ));
            }
        }
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Source {
    #[default]
    Unknown,
    Official,
    Local,
    Steam,
}

impl Source {
    pub fn icon_name(&self) -> IconName {
        match self {
            Source::Unknown => IconName::Unknown,
            Source::Official => IconName::RimWorld,
            Source::Local => IconName::Local,
            Source::Steam => IconName::Steam,
        }
    }

    pub fn is_official(&self) -> bool {
        matches!(self, Source::Official)
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Source::Local)
    }

    pub fn is_steam(&self) -> bool {
        matches!(self, Source::Steam)
    }
}
