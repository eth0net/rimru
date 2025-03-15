use std::{
    fs::{self, File},
    io::BufReader,
    time::{SystemTime, UNIX_EPOCH},
};

use xml::{reader::XmlEvent as XmlReaderEvent, writer::XmlEvent as XmlWriterEvent};

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
                Ok(XmlReaderEvent::EndDocument) => {
                    break;
                }
                Ok(XmlReaderEvent::StartDocument { .. }) => {}
                Ok(XmlReaderEvent::StartElement { name, .. }) => {
                    match name.local_name.to_ascii_lowercase().as_str() {
                        "modsconfigdata" => loop {
                            match reader.next() {
                                Ok(XmlReaderEvent::EndElement { name }) => {
                                    if name.local_name.eq_ignore_ascii_case("modsconfigdata") {
                                        break;
                                    }
                                }
                                Ok(XmlReaderEvent::StartElement { name, .. }) => {
                                    match name.local_name.to_ascii_lowercase().as_str() {
                                        "activemods" => loop {
                                            match reader.next() {
                                                Ok(XmlReaderEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("activeMods")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlReaderEvent::StartElement {
                                                    name, ..
                                                }) => loop {
                                                    if !name.local_name.eq_ignore_ascii_case("li") {
                                                        log::error!(
                                                            "unexpected element in activeMods: {:?}",
                                                            name
                                                        );
                                                        break;
                                                    }
                                                    match reader.next() {
                                                        Ok(XmlReaderEvent::EndElement { name }) => {
                                                            if name
                                                                .local_name
                                                                .eq_ignore_ascii_case("li")
                                                            {
                                                                break;
                                                            }
                                                        }
                                                        Ok(XmlReaderEvent::Characters(author)) => {
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
                                                Ok(XmlReaderEvent::Characters(_)) => {}
                                                Ok(event) => {
                                                    log::warn!(
                                                        "error parsing activeMods from {:?}: {}: {:?}",
                                                        mods_config_path,
                                                        "unexpected element",
                                                        event,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing activeMods from {:?}: {}",
                                                        mods_config_path,
                                                        err
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "knownexpansions" => loop {
                                            match reader.next() {
                                                Ok(XmlReaderEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("knownExpansions")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlReaderEvent::StartElement {
                                                    name, ..
                                                }) => loop {
                                                    if !name.local_name.eq_ignore_ascii_case("li") {
                                                        log::error!(
                                                            "unexpected element in knownExpansions: {:?}",
                                                            name
                                                        );
                                                        break;
                                                    }
                                                    match reader.next() {
                                                        Ok(XmlReaderEvent::EndElement { name }) => {
                                                            if name
                                                                .local_name
                                                                .eq_ignore_ascii_case("li")
                                                            {
                                                                break;
                                                            }
                                                        }
                                                        Ok(XmlReaderEvent::Characters(author)) => {
                                                            mods_config
                                                                .known_expansions
                                                                .push(author);
                                                        }
                                                        Ok(event) => {
                                                            log::warn!(
                                                                "error parsing knownExpansion from {:?}: {}: {:?}",
                                                                mods_config_path,
                                                                "unexpected element",
                                                                event,
                                                            );
                                                        }
                                                        Err(err) => {
                                                            log::error!(
                                                                "error parsing knownExpansion from {:?}: {}",
                                                                mods_config_path,
                                                                err
                                                            );
                                                            break;
                                                        }
                                                    }
                                                },
                                                Ok(XmlReaderEvent::Characters(_)) => {}
                                                Ok(event) => {
                                                    log::warn!(
                                                        "error parsing knownExpansions from {:?}: {}: {:?}",
                                                        mods_config_path,
                                                        "unexpected element",
                                                        event,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing knownExpansions from {:?}: {}",
                                                        mods_config_path,
                                                        err
                                                    );
                                                    break;
                                                }
                                            }
                                        },
                                        "version" => loop {
                                            match reader.next() {
                                                Ok(XmlReaderEvent::EndElement { name }) => {
                                                    if name
                                                        .local_name
                                                        .eq_ignore_ascii_case("version")
                                                    {
                                                        break;
                                                    }
                                                }
                                                Ok(XmlReaderEvent::Characters(chars)) => {
                                                    mods_config.version = chars;
                                                }
                                                Ok(event) => {
                                                    log::warn!(
                                                        "error parsing version from {:?}: {}: {:?}",
                                                        mods_config_path,
                                                        "unexpected element",
                                                        event,
                                                    );
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "error parsing version from {:?}: {}: {:?}",
                                                        mods_config_path,
                                                        "unexpected error",
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
                                                    Ok(XmlReaderEvent::EndElement { name }) => {
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
                                Ok(XmlReaderEvent::Characters(_)) => {}
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

    pub fn save(&self) {
        let mods_config_path = paths::mods_config_file();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should have passed since unix epoch")
            .as_micros();
        let file_stem = mods_config_path
            .file_stem()
            .expect("path should be set in crate::game::paths");
        let file_extension = mods_config_path
            .extension()
            .expect("path should have an extension");
        let backup_mods_config_path = mods_config_path.with_file_name(format!(
            "{}.{}.{}",
            file_stem.to_str().unwrap(),
            timestamp,
            file_extension.to_str().unwrap()
        ));

        log::info!("backing up mods config to {:?}", backup_mods_config_path);
        if let Err(err) = fs::copy(paths::mods_config_file(), backup_mods_config_path) {
            log::error!("error backing up mods config: {}", err);
            return;
        }

        log::info!("saving mods config to {:?}", mods_config_path);
        let mods_config_file = File::create(&mods_config_path);
        if let Err(err) = mods_config_file {
            log::error!(
                "error creating mods config file {:?}: {}",
                mods_config_path,
                err
            );
            return;
        }

        let mut writer = xml::EmitterConfig::new()
            .perform_indent(true)
            .write_document_declaration(true)
            .create_writer(mods_config_file.unwrap());

        if let Err(err) = writer.write(XmlWriterEvent::start_element("ModsConfigData")) {
            log::error!("error writing ModsConfigData start element: {}", err);
            return;
        }

        if let Err(err) = writer.write(XmlWriterEvent::start_element("version")) {
            log::error!("error writing version start element: {}", err);
            return;
        }
        if let Err(err) = writer.write(XmlWriterEvent::characters(&self.version)) {
            log::error!("error writing version characters: {}", err);
            return;
        }
        if let Err(err) = writer.write(XmlWriterEvent::end_element()) {
            log::error!("error writing version end element: {}", err);
            return;
        }

        if let Err(err) = writer.write(XmlWriterEvent::start_element("activeMods")) {
            log::error!("error writing activeMods start element: {}", err);
            return;
        }
        for mod_id in &self.active_mods {
            if let Err(err) = writer.write(XmlWriterEvent::start_element("li")) {
                log::error!("error writing mod start element: {}", err);
                return;
            }
            let mod_id = mod_id.to_ascii_lowercase();
            if let Err(err) = writer.write(XmlWriterEvent::characters(&mod_id)) {
                log::error!("error writing mod characters: {}", err);
                return;
            }
            if let Err(err) = writer.write(XmlWriterEvent::end_element()) {
                log::error!("error writing mod end element: {}", err);
                return;
            }
        }
        if let Err(err) = writer.write(XmlWriterEvent::end_element()) {
            log::error!("error writing activeMods end element: {}", err);
            return;
        }

        if let Err(err) = writer.write(XmlWriterEvent::start_element("knownExpansions")) {
            log::error!("error writing knownExpansions start element: {}", err);
            return;
        }
        for expansion in &self.known_expansions {
            if let Err(err) = writer.write(XmlWriterEvent::start_element("li")) {
                log::error!("error writing expansion start element: {}", err);
                return;
            }
            let expansion_id = expansion.to_ascii_lowercase();
            if let Err(err) = writer.write(XmlWriterEvent::characters(&expansion_id)) {
                log::error!("error writing expansion characters: {}", err);
                return;
            }
            if let Err(err) = writer.write(XmlWriterEvent::end_element()) {
                log::error!("error writing expansion end element: {}", err);
                return;
            }
        }
        if let Err(err) = writer.write(XmlWriterEvent::end_element()) {
            log::error!("error writing knownExpansions end element: {}", err);
            return;
        }

        if let Err(err) = writer.write(XmlWriterEvent::end_element()) {
            log::error!("error writing ModsConfigData end element: {}", err);
        }
    }
}
