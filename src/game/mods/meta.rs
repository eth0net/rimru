use std::{fs::File, io::BufReader, path::PathBuf};

use xml::reader::XmlEvent;

use crate::game::paths;

#[derive(Debug, Clone, Default)]
pub struct ModMetaData {
    pub id: String,
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
    pub path: PathBuf,
    pub source: Source,
}

impl ModMetaData {
    pub fn new(path: PathBuf) -> Option<Self> {
        if !path.is_dir() {
            log::error!("Path is not a directory: {:?}", path);
            return None;
        }

        let mut mod_meta = ModMetaData {
            path: path.clone(),
            ..Default::default()
        };

        let about_path = paths::mod_about_file(&path);
        let about_file = File::open(&about_path).ok()?;
        let about_file = BufReader::new(about_file);
        let parser_config = xml::ParserConfig::new()
            .whitespace_to_characters(true)
            .cdata_to_characters(true)
            .ignore_comments(true)
            .coalesce_characters(true);
        let mut reader = parser_config.create_reader(about_file);

        // todo: remove loops over events now we coalesce characters
        loop {
            match reader.next() {
                Ok(XmlEvent::EndDocument) => {
                    break;
                }
                Ok(XmlEvent::StartDocument { .. }) => {}
                Ok(XmlEvent::StartElement { name, .. }) => match name
                    .local_name
                    .to_ascii_lowercase()
                    .as_str()
                {
                    "modmetadata" => loop {
                        match reader.next() {
                            Ok(XmlEvent::EndElement { name }) => {
                                if name.local_name.eq_ignore_ascii_case("modmetadata") {
                                    break;
                                }
                            }
                            Ok(XmlEvent::StartElement { name, .. }) => match name
                                .local_name
                                .to_ascii_lowercase()
                                .as_str()
                            {
                                "author" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name.eq_ignore_ascii_case("author") {
                                                break;
                                            }
                                        }
                                        Ok(XmlEvent::Characters(chars)) => {
                                            for author in chars.split(",") {
                                                mod_meta.authors.push(author.trim().to_string());
                                            }
                                        }
                                        Ok(event) => {
                                            log::warn!(
                                                "error parsing author from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing author from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "authors" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name.eq_ignore_ascii_case("authors") {
                                                break;
                                            }
                                        }
                                        Ok(XmlEvent::StartElement { name, .. }) => loop {
                                            if !name.local_name.eq_ignore_ascii_case("li") {
                                                log::error!(
                                                    "unexpected element in authors: {:?}",
                                                    name
                                                );
                                                break;
                                            }
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name.local_name.eq_ignore_ascii_case("li") {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlEvent::Characters(author)) => {
                                                    mod_meta.authors.push(author);
                                                }
                                                Ok(event) => {
                                                    log::warn!(
                                                        "error parsing author from {:?}: {}: {:?}",
                                                        about_path,
                                                        "unexpected element",
                                                        event,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing author from {:?}: {}",
                                                        about_path,
                                                        err
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        Ok(XmlEvent::Characters(_)) => {}
                                        Ok(event) => {
                                            log::warn!(
                                                "error parsing authors from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing authors from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "description" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name.eq_ignore_ascii_case("description") {
                                                break;
                                            }
                                        }
                                        Ok(XmlEvent::Characters(description)) => {
                                            mod_meta.description += description.as_str();
                                        }
                                        Ok(event) => {
                                            log::warn!(
                                                "error parsing description from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing description from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "descriptionsbyversion" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("descriptionsByVersion")
                                            {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing descriptionsByVersion from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "forceloadafter" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("forceloadafter")
                                            {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing forceLoadAfter from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "forceloadbefore" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("forceLoadBefore")
                                            {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing forceLoadBefore from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "incompatiblewith" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("incompatibleWith")
                                            {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing incompatibleWith from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "incompatiblewithbyversion" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("incompatibleWithByVersion")
                                            {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing incompatibleWithByVersion from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "loadafter" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name.eq_ignore_ascii_case("loadAfter") {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing loadAfter from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "loadafterbyversion" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("loadAfterByVersion")
                                            {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing loadAfterByVersion from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "loadbefore" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name.eq_ignore_ascii_case("loadBefore") {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing loadBefore from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "loadbeforebyversion" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("loadBeforeByVersion")
                                            {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing loadBeforeByVersion from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "moddependencies" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("modDependencies")
                                            {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing modDependencies from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "moddependenciesbyversion" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("modDependenciesByVersion")
                                            {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing modDependenciesByVersion from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "modiconpath" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name.eq_ignore_ascii_case("modIconPath") {
                                                break;
                                            }
                                        }
                                        Ok(XmlEvent::Characters(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Ok(event) => {
                                            log::error!(
                                                "error parsing modIconPath from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing modIconPath from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected error",
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "modversion" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name.eq_ignore_ascii_case("modVersion") {
                                                break;
                                            }
                                        }
                                        Ok(XmlEvent::Characters(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Ok(event) => {
                                            log::error!(
                                                "error parsing modVersion from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing modVersion from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected error",
                                                err,
                                            );
                                            break;
                                        }
                                    }
                                },
                                "name" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name.eq_ignore_ascii_case("name") {
                                                break;
                                            }
                                        }
                                        Ok(XmlEvent::Characters(chars)) => {
                                            mod_meta.name += chars.as_str();
                                        }
                                        Ok(event) => {
                                            log::warn!(
                                                "error parsing name from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing name from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "packageid" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name.eq_ignore_ascii_case("packageId") {
                                                break;
                                            }
                                        }
                                        Ok(XmlEvent::Characters(chars)) => {
                                            mod_meta.id = chars;
                                        }
                                        Ok(event) => {
                                            log::warn!(
                                                "error parsing packageId from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing packageId from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected error",
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "supportedversions" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("supportedVersions")
                                            {
                                                break;
                                            }
                                        }
                                        Ok(_) => {
                                            // todo: read and process the elements
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing supportedVersions from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                "url" => loop {
                                    match reader.next() {
                                        Ok(XmlEvent::EndElement { name }) => {
                                            if name.local_name.eq_ignore_ascii_case("url") {
                                                break;
                                            }
                                        }
                                        Ok(XmlEvent::Characters(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Ok(event) => {
                                            log::error!(
                                                "error parsing url from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Err(err) => {
                                            log::error!(
                                                "error parsing url from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected error",
                                                err
                                            );
                                            break;
                                        }
                                    }
                                },
                                unhandled => {
                                    loop {
                                        log::trace!(
                                            "unhandled token {} in modMetaData from {:?}",
                                            name,
                                            about_path
                                        );
                                        match reader.next() {
                                            Ok(XmlEvent::EndElement { name }) => {
                                                if name.local_name.eq_ignore_ascii_case(unhandled) {
                                                    break;
                                                }
                                            }
                                            Ok(_) => {
                                                // todo: read and process the elements
                                            }
                                            Err(err) => {
                                                log::error!(
                                                    "error parsing modMetaData from {:?}: {}",
                                                    about_path,
                                                    err
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }
                            },
                            Ok(XmlEvent::Characters(_)) => {}
                            Ok(event) => {
                                log::warn!(
                                    "parsing modMetaData from {:?}: {}: {:?}",
                                    about_path,
                                    "unexpected element",
                                    event,
                                );
                            }
                            Err(err) => {
                                log::error!("error parsing element from {:?}: {}", about_path, err);
                                break;
                            }
                        }
                    },
                    a => {
                        log::trace!("skipped parsing {} from {:?}", a, about_path);
                    }
                },
                Ok(next) => {
                    log::trace!("unexpected element {:?} from {:?}", next, about_path);
                }
                Err(err) => {
                    log::error!("error parsing element from {:?}: {}", about_path, err);
                    break;
                }
            }
        }

        Some(mod_meta)
    }

    pub fn new_official(path: PathBuf) -> Option<Self> {
        Self::new(path).map(|mut m| {
            m.source = Source::Official;
            m
        })
    }

    pub fn new_local(path: PathBuf) -> Option<Self> {
        Self::new(path).map(|mut m| {
            m.source = Source::Local;
            m
        })
    }

    pub fn new_steam(path: PathBuf) -> Option<Self> {
        Self::new(path).and_then(|mut m| {
            let dir_name = m.path.file_name().and_then(|name| name.to_str())?;

            // todo: use steamAppId from About.xml if
            m.source = Source::Steam {
                id: dir_name.to_string(),
            };

            Some(m)
        })
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Source {
    #[default]
    Unknown,
    Official,
    Local,
    Steam {
        id: String,
    },
}

impl Source {
    pub fn is_official(&self) -> bool {
        matches!(self, Source::Official)
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Source::Local)
    }

    pub fn is_steam(&self) -> bool {
        matches!(self, Source::Steam { .. })
    }
}
