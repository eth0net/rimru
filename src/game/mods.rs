use std::{fs::File, io::BufReader, path::PathBuf};

use xml::reader::XmlEvent;

#[derive(Debug, Clone, Default)]
pub struct ModMeta {
    pub id: String,
    pub name: String,
    pub authors: Vec<String>,
    pub description: String,
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

    fn new(path: PathBuf) -> Option<Self> {
        if !path.is_dir() {
            log::error!("Path is not a directory: {:?}", path);
            return None;
        }

        let mut mod_meta = ModMeta {
            path: path.clone(),
            ..Default::default()
        };

        let about_path = path.join("About/About.xml");
        let about_file = File::open(&about_path).ok()?;
        let about_file = BufReader::new(about_file);
        let config = xml::ParserConfig::new()
            .whitespace_to_characters(true)
            .cdata_to_characters(true)
            .ignore_comments(true)
            .coalesce_characters(true);
        let reader = config.create_reader(about_file);
        let mut reader = reader.into_iter();

        loop {
            match reader.next() {
                Some(Ok(XmlEvent::EndDocument)) => {
                    break;
                }
                Some(Ok(XmlEvent::StartDocument { .. })) => {}
                Some(Ok(XmlEvent::StartElement { name, .. })) => match name
                    .local_name
                    .to_ascii_lowercase()
                    .as_str()
                {
                    "modmetadata" => loop {
                        match reader.next() {
                            Some(Ok(XmlEvent::EndElement { name })) => {
                                if name.local_name.eq_ignore_ascii_case("modmetadata") {
                                    break;
                                }
                            }
                            Some(Ok(XmlEvent::StartElement { name, .. })) => match name
                                .local_name
                                .to_ascii_lowercase()
                                .as_str()
                            {
                                "packageid" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name.local_name.eq_ignore_ascii_case("packageId") {
                                                break;
                                            }
                                        }
                                        Some(Ok(XmlEvent::Characters(chars))) => {
                                            mod_meta.id = chars;
                                        }
                                        Some(Ok(event)) => {
                                            log::warn!(
                                                "error parsing packageId from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing packageId from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected error",
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing packageId from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "name" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name.local_name.eq_ignore_ascii_case("name") {
                                                break;
                                            }
                                        }
                                        Some(Ok(XmlEvent::Characters(chars))) => {
                                            mod_meta.name += chars.as_str();
                                        }
                                        Some(Ok(event)) => {
                                            log::warn!(
                                                "error parsing name from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing name from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing name from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "author" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name.local_name.eq_ignore_ascii_case("author") {
                                                break;
                                            }
                                        }
                                        Some(Ok(XmlEvent::Characters(chars))) => {
                                            for author in chars.split(",") {
                                                mod_meta.authors.push(author.trim().to_string());
                                            }
                                        }
                                        Some(Ok(event)) => {
                                            log::warn!(
                                                "error parsing author from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing author from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing author from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "authors" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name.local_name.eq_ignore_ascii_case("authors") {
                                                break;
                                            }
                                        }
                                        Some(Ok(XmlEvent::StartElement { name, .. })) => loop {
                                            if !name.local_name.eq_ignore_ascii_case("li") {
                                                log::error!(
                                                    "unexpected element in authors: {:?}",
                                                    name
                                                );
                                                break;
                                            }
                                            match reader.next() {
                                                Some(Ok(XmlEvent::EndElement { name })) => {
                                                    if name.local_name.eq_ignore_ascii_case("li") {
                                                        break;
                                                    }
                                                }
                                                Some(Ok(XmlEvent::Characters(author))) => {
                                                    mod_meta.authors.push(author);
                                                }
                                                Some(Ok(event)) => {
                                                    log::warn!(
                                                        "error parsing author from {:?}: {}: {:?}",
                                                        about_path,
                                                        "unexpected element",
                                                        event,
                                                    );
                                                }
                                                Some(Err(err)) => {
                                                    log::error!(
                                                        "error parsing author from {:?}: {}",
                                                        about_path,
                                                        err
                                                    );
                                                    break;
                                                }
                                                None => {
                                                    log::error!(
                                                        "error parsing author from {:?}: {}",
                                                        about_path,
                                                        "unexpected end of file"
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        Some(Ok(XmlEvent::Characters(_))) => {}
                                        Some(Ok(event)) => {
                                            log::warn!(
                                                "error parsing authors from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing authors from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing authors from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "description" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name.local_name.eq_ignore_ascii_case("description") {
                                                break;
                                            }
                                        }
                                        Some(Ok(XmlEvent::Characters(description))) => {
                                            mod_meta.description += description.as_str();
                                        }
                                        Some(Ok(event)) => {
                                            log::warn!(
                                                "error parsing description from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing description from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing description from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "supportedversions" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("supportedVersions")
                                            {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing supportedVersions from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing supportedVersions from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "modversion" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name.local_name.eq_ignore_ascii_case("modVersion") {
                                                break;
                                            }
                                        }
                                        Some(Ok(XmlEvent::Characters(_))) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Ok(event)) => {
                                            log::error!(
                                                "error parsing modVersion from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing modVersion from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected error",
                                                err,
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing modVersion from {:?}: {}",
                                                about_path,
                                                "unexpected end of file",
                                            );
                                            break;
                                        }
                                    }
                                },
                                "modiconpath" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name.local_name.eq_ignore_ascii_case("modIconPath") {
                                                break;
                                            }
                                        }
                                        Some(Ok(XmlEvent::Characters(_))) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Ok(event)) => {
                                            log::error!(
                                                "error parsing modIconPath from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing modIconPath from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected error",
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing modIconPath from {:?}: {}",
                                                about_path,
                                                "unexpected end of file",
                                            );
                                            break;
                                        }
                                    }
                                },
                                "url" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name.local_name.eq_ignore_ascii_case("url") {
                                                break;
                                            }
                                        }
                                        Some(Ok(XmlEvent::Characters(_))) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Ok(event)) => {
                                            log::error!(
                                                "error parsing url from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected element",
                                                event,
                                            );
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing url from {:?}: {}: {:?}",
                                                about_path,
                                                "unexpected error",
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing url from {:?}: {}",
                                                about_path,
                                                "unexpected end of file",
                                            );
                                            break;
                                        }
                                    }
                                },
                                "descriptionsbyversion" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("descriptionsByVersion")
                                            {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing descriptionsByVersion from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing descriptionsByVersion from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "moddependencies" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("modDependencies")
                                            {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing modDependencies from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing modDependencies from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "moddependenciesbyversion" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("modDependenciesByVersion")
                                            {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing modDependenciesByVersion from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing modDependenciesByVersion from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "loadbefore" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name.local_name.eq_ignore_ascii_case("loadBefore") {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing loadBefore from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing loadBefore from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "loadbeforebyversion" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("loadBeforeByVersion")
                                            {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing loadBeforeByVersion from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing loadBeforeByVersion from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "forceloadbefore" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("forceLoadBefore")
                                            {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing forceLoadBefore from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing forceLoadBefore from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "loadafter" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name.local_name.eq_ignore_ascii_case("loadAfter") {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing loadAfter from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing loadAfter from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "loadafterbyversion" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("loadAfterByVersion")
                                            {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing loadAfterByVersion from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing loadAfterByVersion from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "forceloadafter" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("forceloadafter")
                                            {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing forceLoadAfter from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing forceLoadAfter from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "incompatiblewith" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("incompatibleWith")
                                            {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing incompatibleWith from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing incompatibleWith from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                "incompatiblewithbyversion" => loop {
                                    match reader.next() {
                                        Some(Ok(XmlEvent::EndElement { name })) => {
                                            if name
                                                .local_name
                                                .eq_ignore_ascii_case("incompatibleWithByVersion")
                                            {
                                                break;
                                            }
                                        }
                                        Some(Ok(_)) => {
                                            // todo: read and process the elements
                                        }
                                        Some(Err(err)) => {
                                            log::error!(
                                                "error parsing incompatibleWithByVersion from {:?}: {}",
                                                about_path,
                                                err
                                            );
                                            break;
                                        }
                                        None => {
                                            log::error!(
                                                "error parsing incompatibleWithByVersion from {:?}: {}",
                                                about_path,
                                                "unexpected end of file"
                                            );
                                            break;
                                        }
                                    }
                                },
                                unhandled => {
                                    loop {
                                        match reader.next() {
                                            Some(Ok(XmlEvent::EndElement { name })) => {
                                                log::trace!(
                                                    "skipped parsing {} from {:?}",
                                                    name,
                                                    about_path
                                                );
                                                if name.local_name.eq_ignore_ascii_case(unhandled) {
                                                    break;
                                                }
                                            }
                                            Some(Ok(_)) => {
                                                // todo: read and process the elements
                                            }
                                            Some(Err(err)) => {
                                                log::error!(
                                                    "error parsing incompatibleWithByVersion from {:?}: {}",
                                                    about_path,
                                                    err
                                                );
                                                break;
                                            }
                                            None => {
                                                log::error!(
                                                    "error parsing incompatibleWithByVersion from {:?}: {}",
                                                    about_path,
                                                    "unexpected end of file"
                                                );
                                                break;
                                            }
                                        }
                                    }
                                }
                            },
                            Some(Ok(XmlEvent::Characters(_))) => {}
                            Some(Ok(event)) => {
                                log::warn!(
                                    "parsing modMetaData from {:?}: {}: {:?}",
                                    about_path,
                                    "unexpected element",
                                    event,
                                );
                            }
                            Some(Err(err)) => {
                                log::error!("error parsing element from {:?}: {}", about_path, err);
                                break;
                            }
                            None => {
                                log::error!(
                                    "error parsing element from {:?}: unexpected end of file",
                                    about_path
                                );
                                break;
                            }
                        }
                    },
                    a => {
                        log::trace!("skipped parsing {} from {:?}", a, about_path);
                    }
                },
                Some(Ok(next)) => {
                    log::trace!("unexpected element {:?} from {:?}", next, about_path);
                }
                Some(Err(err)) => {
                    log::error!("error parsing element from {:?}: {}", about_path, err);
                    break;
                }
                None => {
                    log::error!(
                        "error parsing element from {:?}: unexpected end of file",
                        about_path
                    );
                    break;
                }
            }
        }

        Some(mod_meta)
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

            m.source = Source::Steam {
                id: dir_name.to_string(),
            };

            Some(m)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Source {
    #[default]
    Unknown,
    Local,
    Steam {
        id: String,
    },
}

impl Source {
    pub fn is_local(&self) -> bool {
        matches!(self, Source::Local)
    }

    pub fn is_steam(&self) -> bool {
        matches!(self, Source::Steam { .. })
    }
}
