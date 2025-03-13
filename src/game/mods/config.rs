use std::{fs::File, io::BufReader};

use xml::reader::XmlEvent;

use crate::game::paths;

#[derive(Debug, Clone, Default)]
pub struct ModsConfigData {
    pub version: String,
    pub active_mods: Vec<String>,
    pub known_expansions: Vec<String>,
}

impl ModsConfigData {
    // todo: use result instead
    pub fn load() -> Option<Self> {
        let mut mods_config = ModsConfigData::default();

        let mods_config_path = paths::mods_config_file();
        let mods_config_file = File::open(&mods_config_path);
        if let Err(e) = mods_config_file {
            log::error!("error opening mods config file: {}", e);
            return None;
        }
        let mods_config_file = BufReader::new(mods_config_file.unwrap());
        let parser_config = xml::ParserConfig::new()
            .whitespace_to_characters(true)
            .cdata_to_characters(true)
            .ignore_comments(true)
            .coalesce_characters(true);
        let mut reader = parser_config.create_reader(mods_config_file);

        loop {
            match reader.next() {
                Ok(XmlEvent::EndDocument) => {
                    break;
                }
                Ok(XmlEvent::StartDocument { .. }) => {}
                Ok(XmlEvent::StartElement { name, .. }) => {
                    match name.local_name.to_ascii_lowercase().as_str() {
                        "modsconfigdata" => loop {
                            match reader.next() {
                                Ok(XmlEvent::EndElement { name }) => {
                                    if name.local_name.eq_ignore_ascii_case("modsconfigdata") {
                                        break;
                                    }
                                }
                                Ok(XmlEvent::StartElement { name, .. }) => {
                                    match name.local_name.to_ascii_lowercase().as_str() {
                                        "activemods" => loop {
                                            match reader.next() {
                                                Ok(XmlEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("activeMods")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlEvent::StartElement { name, .. }) => loop {
                                                    if !name.local_name.eq_ignore_ascii_case("li") {
                                                        log::error!(
                                                            "unexpected element in activeMods: {:?}",
                                                            name
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
                                                        Ok(XmlEvent::Characters(author)) => {
                                                            mods_config.active_mods.push(author);
                                                        }
                                                        Ok(event) => {
                                                            log::warn!(
                                                                "error parsing activeMod from {:?}: {}: {:?}",
                                                                mods_config_path,
                                                                "unexpected element",
                                                                event,
                                                            );
                                                        }
                                                        Err(err) => {
                                                            log::error!(
                                                                "error parsing activeMod from {:?}: {}",
                                                                mods_config_path,
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
                                                        mods_config_path,
                                                        "unexpected element",
                                                        event,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing authors from {:?}: {}",
                                                        mods_config_path,
                                                        err
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        unhandled => {
                                            log::trace!(
                                                "unhandled token {} in modsConfigData from {:?}",
                                                name,
                                                mods_config_path
                                            );
                                            loop {
                                                match reader.next() {
                                                    Ok(XmlEvent::EndElement { name }) => {
                                                        if name
                                                            .local_name
                                                            .eq_ignore_ascii_case(unhandled)
                                                        {
                                                            break;
                                                        }
                                                    }
                                                    Ok(_) => {
                                                        // todo: read and process the elements
                                                    }
                                                    Err(err) => {
                                                        log::error!(
                                                            "error parsing modsConfigData from {:?}: {}",
                                                            mods_config_path,
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
                                        "parsing modMetaData from {:?}: {}: {:?}",
                                        mods_config_path,
                                        "unexpected element",
                                        event,
                                    );
                                }
                                Err(err) => {
                                    log::error!(
                                        "error parsing element from {:?}: {}",
                                        mods_config_path,
                                        err
                                    );
                                    break;
                                }
                            }
                        },
                        a => {
                            log::trace!(
                                "skipped parsing {} at root from {:?}",
                                a,
                                mods_config_path
                            );
                        }
                    }
                }
                Ok(next) => {
                    log::trace!("unexpected element {:?} from {:?}", next, mods_config_path);
                }
                Err(err) => {
                    log::error!("error parsing element from {:?}: {}", mods_config_path, err);
                    break;
                }
            }
        }

        Some(mods_config)
    }
}
