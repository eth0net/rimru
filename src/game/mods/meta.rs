use std::{fs::File, io::BufReader, path::PathBuf};

use xml::reader::XmlEvent;

use crate::game::paths;

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
    pub fn new(path: PathBuf) -> Option<Self> {
        if !path.is_dir() {
            log::error!("Path is not a directory: {:?}", path);
            return None;
        }

        let mut mod_meta = ModMetaData {
            path: path.clone(),
            ..Default::default()
        };

        let about_path = mod_meta.about_file_path();
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
                Ok(XmlEvent::StartElement { name, .. }) => {
                    match name.local_name.to_ascii_lowercase().as_str() {
                        "modmetadata" => loop {
                            // todo: refactor and clean up
                            match reader.next() {
                                Ok(XmlEvent::EndElement { name }) => {
                                    if name.local_name.eq_ignore_ascii_case("modMetaData") {
                                        break;
                                    }
                                }
                                Ok(XmlEvent::StartElement { name, .. }) => {
                                    match name.local_name.to_ascii_lowercase().as_str() {
                                        "author" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("author")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlEvent::Characters(chars)) => {
                                                    for author in chars.split(",") {
                                                        mod_meta
                                                            .authors
                                                            .push(author.trim().to_string());
                                                    }
                                                }
                                                Ok(event) => {
                                                    log::warn!(
                                                        "unexpected event {:?} in author from {:?}",
                                                        event,
                                                        about_path,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing author from {:?}: {}",
                                                        about_path,
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "authors" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("authors")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlEvent::StartElement { name, .. }) => loop {
                                                    if !name.local_name.eq_ignore_ascii_case("li") {
                                                        log::error!(
                                                            "unexpected element {} in authors from {:?}",
                                                            name,
                                                            about_path,
                                                        );
                                                        break;
                                                    }
                                                    match reader.next() {
                                                        Ok(XmlEvent::EndElement { name }) => {
                                                            if name
                                                                .local_name
                                                                .eq_ignore_ascii_case("li")
                                                            {
                                                                break;
                                                            }
                                                        }
                                                        Ok(XmlEvent::Characters(chars)) => {
                                                            mod_meta.authors.push(chars);
                                                        }
                                                        Ok(event) => {
                                                            log::warn!(
                                                                "unexpected event {:?} in authors li from {:?}",
                                                                event,
                                                                about_path,
                                                            );
                                                        }
                                                        Err(err) => {
                                                            log::error!(
                                                                "error parsing authors li from {:?}: {}",
                                                                about_path,
                                                                err
                                                            );
                                                            break;
                                                        }
                                                    }
                                                },
                                                Ok(XmlEvent::Characters(chars)) => {
                                                    if chars.trim().is_empty() {
                                                        // ignore whitespace
                                                        continue;
                                                    }
                                                    log::warn!(
                                                        "unexpected characters {} in authors li from {:?}",
                                                        chars,
                                                        about_path,
                                                    );
                                                }
                                                Ok(XmlEvent::Whitespace(_)) => {} // ignore whitespace
                                                Ok(event) => {
                                                    log::warn!(
                                                        "unexpected event {:?} in authors from {:?}",
                                                        event,
                                                        about_path,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing authors li from {:?}: {}",
                                                        about_path,
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "description" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("description")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlEvent::Characters(chars)) => {
                                                    mod_meta.description += &chars;
                                                }
                                                Ok(event) => {
                                                    log::warn!(
                                                        "unexpected event {:?} in description from {:?}",
                                                        event,
                                                        about_path,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing description from {:?}: {}",
                                                        about_path,
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "descriptionsbyversion" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name.local_name.eq_ignore_ascii_case(
                                                        "descriptionsByVersion",
                                                    ) {
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
                                                        err,
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
                                                        .eq_ignore_ascii_case("forceLoadAfter")
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
                                                        err,
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
                                                        err,
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
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "incompatiblewithbyversion" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name.local_name.eq_ignore_ascii_case(
                                                        "incompatibleWithByVersion",
                                                    ) {
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
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "loadafter" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("loadAfter")
                                                    {
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
                                                        err,
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
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "loadbefore" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("loadBefore")
                                                    {
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
                                                        err,
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
                                                        err,
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
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "moddependenciesbyversion" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name.local_name.eq_ignore_ascii_case(
                                                        "modDependenciesByVersion",
                                                    ) {
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
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "modiconpath" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("modIconPath")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlEvent::Characters(_)) => {
                                                    // todo: read and process the elements
                                                }
                                                Ok(event) => {
                                                    log::warn!(
                                                        "unexpected event {:?} in modIconPath from {:?}",
                                                        event,
                                                        about_path,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing modIconPath from {:?}: {}",
                                                        about_path,
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "modversion" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("modVersion")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlEvent::Characters(_)) => {
                                                    // todo: read and process the elements
                                                }
                                                Ok(event) => {
                                                    log::warn!(
                                                        "unexpected event {:?} in modVersion from {:?}",
                                                        event,
                                                        about_path,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing modVersion from {:?}: {}",
                                                        about_path,
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "name" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name.local_name.eq_ignore_ascii_case("name")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlEvent::Characters(chars)) => {
                                                    mod_meta.name += &chars;
                                                }
                                                Ok(event) => {
                                                    log::warn!(
                                                        "unexpected event {:?} in name from {:?}",
                                                        event,
                                                        about_path,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing name from {:?}: {}",
                                                        about_path,
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "packageid" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("packageId")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlEvent::Characters(chars)) => {
                                                    mod_meta.id += &chars;
                                                }
                                                Ok(event) => {
                                                    log::warn!(
                                                        "unexpected event {:?} in packageId from {:?}",
                                                        event,
                                                        about_path,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing packageId from {:?}: {}",
                                                        about_path,
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "steamappid" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("steamAppId")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlEvent::Characters(chars)) => {
                                                    mod_meta.steam_app_id = match &mod_meta
                                                        .steam_app_id
                                                    {
                                                        Some(old_steam_app_id) => Some(
                                                            old_steam_app_id.to_string() + &chars,
                                                        ),
                                                        None => Some(chars),
                                                    };
                                                }
                                                Ok(event) => {
                                                    log::warn!(
                                                        "unexpected event {:?} in steamAppId from {:?}",
                                                        event,
                                                        about_path,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing steamAppId from {:?}: {}",
                                                        about_path,
                                                        err,
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
                                                        err,
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
                                                    log::warn!(
                                                        "unexpected event {:?} in url from {:?}",
                                                        event,
                                                        about_path,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing url from {:?}: {}",
                                                        about_path,
                                                        err,
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        unexpected => {
                                            loop {
                                                log::trace!(
                                                    "unexpected element {} in modMetaData from {:?}",
                                                    name,
                                                    about_path
                                                );
                                                match reader.next() {
                                                    Ok(XmlEvent::EndElement { name }) => {
                                                        if name
                                                            .local_name
                                                            .eq_ignore_ascii_case(unexpected)
                                                        {
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
                                    }
                                }
                                Ok(XmlEvent::Characters(_)) => {}
                                Ok(event) => {
                                    log::warn!(
                                        "unexpected event {:?} in modMetaData from {:?}",
                                        event,
                                        about_path,
                                    );
                                }
                                Err(err) => {
                                    log::error!(
                                        "error parsing element from {:?}: {}",
                                        about_path,
                                        err,
                                    );
                                    break;
                                }
                            }
                        },
                        unexpected => {
                            log::trace!(
                                "unexpected root element {} from {:?}",
                                unexpected,
                                about_path
                            );
                        }
                    }
                }
                Ok(event) => {
                    log::trace!("unexpected root event {:?} from {:?}", event, about_path);
                }
                Err(err) => {
                    log::error!("error parsing root event from {:?}: {}", about_path, err);
                    break;
                }
            }
        }

        Some(mod_meta)
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Source {
    #[default]
    Unknown,
    Official,
    Local,
    Steam,
}

impl Source {
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
