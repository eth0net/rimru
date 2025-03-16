use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use xml::{
    reader::{EventReader, XmlEvent as XmlReaderEvent},
    writer::{EmitterConfig, EventWriter, XmlEvent as XmlWriterEvent},
};

use crate::game::paths;

#[derive(Debug, Clone, Default)]
pub struct ModsConfigData {
    pub version: String,
    pub active_mods: Vec<String>,
    pub known_expansions: Vec<String>,
}

impl ModsConfigData {
    pub fn load() -> Option<Self> {
        let mods_config_path = paths::mods_config_file();
        load_config_from_file(&mods_config_path)
    }

    pub fn save(&self) {
        let mods_config_path = paths::mods_config_file();
        backup_config(&mods_config_path);
        save_config_to_file(&mods_config_path, self);
    }
}

fn load_config_from_file(path: &Path) -> Option<ModsConfigData> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("error opening mods config file {:?}: {}", path, e);
            return None;
        }
    };

    let reader = BufReader::new(file);
    let parser_config = xml::ParserConfig::new()
        .whitespace_to_characters(true)
        .cdata_to_characters(true)
        .ignore_comments(true)
        .coalesce_characters(true);
    let event_reader = EventReader::new_with_config(reader, parser_config);

    match parse_mods_config(event_reader, path) {
        Ok(config) => Some(config),
        Err(e) => {
            log::error!("error parsing mods config file {:?}: {}", path, e);
            None
        }
    }
}

type ParseResult<T> = Result<T, String>;

fn parse_mods_config<R: Read>(
    mut event_reader: EventReader<R>,
    path: &Path,
) -> ParseResult<ModsConfigData> {
    let mut config = ModsConfigData::default();

    loop {
        let event = event_reader.next();
        match event {
            Ok(XmlReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("ModsConfigData") =>
            {
                parse_mods_config_data(&mut event_reader, path, &mut config)?;
            }
            Ok(XmlReaderEvent::EndDocument) => break,
            Ok(XmlReaderEvent::StartDocument { .. }) => {}
            Ok(event) => {
                log::trace!("unexpected root event {:?} from {:?}", event, path);
            }
            Err(e) => {
                return Err(format!(
                    "error parsing root event from {:?}: {}",
                    path.display(),
                    e
                ));
            }
        }
    }

    Ok(config)
}

fn parse_mods_config_data<R: Read>(
    event_reader: &mut EventReader<R>,
    path: &Path,
    config: &mut ModsConfigData,
) -> ParseResult<()> {
    loop {
        let event = event_reader.next();
        match event {
            Ok(XmlReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("activeMods") =>
            {
                config.active_mods = parse_list_of_strings(event_reader, path, "activeMods")?;
            }
            Ok(XmlReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("knownExpansions") =>
            {
                config.known_expansions =
                    parse_list_of_strings(event_reader, path, "knownExpansions")?;
            }
            Ok(XmlReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("version") =>
            {
                config.version = parse_text_element(event_reader, path, "version")?;
            }
            Ok(XmlReaderEvent::EndElement { name })
                if name.local_name.eq_ignore_ascii_case("ModsConfigData") =>
            {
                break;
            }
            Ok(event) => {
                log::warn!(
                    "unexpected event {:?} in modsConfigData from {:?}",
                    event,
                    path
                );
            }
            Err(e) => {
                return Err(format!(
                    "error parsing modsConfigData from {:?}: {}",
                    path.display(),
                    e
                ));
            }
        }
    }
    Ok(())
}

fn parse_list_of_strings<R: Read>(
    event_reader: &mut EventReader<R>,
    path: &Path,
    container_name: &str,
) -> ParseResult<Vec<String>> {
    let mut list = Vec::new();
    loop {
        let event = event_reader.next();
        match event {
            Ok(XmlReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("li") =>
            {
                let text_event = event_reader.next();
                match text_event {
                    Ok(XmlReaderEvent::Characters(chars)) => {
                        list.push(chars);
                        let end_li_event = event_reader.next();
                        match end_li_event {
                            Ok(XmlReaderEvent::EndElement { name })
                                if name.local_name.eq_ignore_ascii_case("li") => {}
                            Ok(event) => log::warn!(
                                "unexpected event {:?} in {} li from {:?}",
                                event,
                                container_name,
                                path
                            ),
                            Err(e) => {
                                return Err(format!(
                                    "error parsing {} li from {:?}: {}",
                                    container_name,
                                    path.display(),
                                    e
                                ));
                            }
                        }
                    }
                    Ok(XmlReaderEvent::EndElement { name })
                        if name.local_name.eq_ignore_ascii_case("li") => {} // Empty li element
                    Ok(event) => log::warn!(
                        "unexpected event {:?} in {} li from {:?}",
                        event,
                        container_name,
                        path
                    ),
                    Err(e) => {
                        return Err(format!(
                            "error parsing {} li from {:?}: {}",
                            container_name,
                            path.display(),
                            e
                        ));
                    }
                }
            }
            Ok(XmlReaderEvent::EndElement { name })
                if name.local_name.eq_ignore_ascii_case(container_name) =>
            {
                break;
            }
            Ok(XmlReaderEvent::Characters(chars)) => {
                if !chars.trim().is_empty() {
                    log::warn!(
                        "unexpected characters {} in {} from {:?}",
                        chars,
                        container_name,
                        path
                    );
                }
            }
            Ok(XmlReaderEvent::Whitespace(_)) => {} // ignore whitespace
            Ok(event) => log::warn!(
                "unexpected event {:?} in {} from {:?}",
                event,
                container_name,
                path
            ),
            Err(e) => {
                return Err(format!(
                    "error parsing {} from {:?}: {}",
                    container_name,
                    path.display(),
                    e
                ));
            }
        }
    }
    Ok(list)
}

fn parse_text_element<R: Read>(
    event_reader: &mut EventReader<R>,
    path: &Path,
    element_name: &str,
) -> ParseResult<String> {
    let mut text = String::new();
    loop {
        let event = event_reader.next();
        match event {
            Ok(XmlReaderEvent::Characters(chars)) => {
                text.push_str(&chars);
            }
            Ok(XmlReaderEvent::EndElement { name })
                if name.local_name.eq_ignore_ascii_case(element_name) =>
            {
                break;
            }
            Ok(XmlReaderEvent::StartElement { name, .. }) => {
                log::warn!(
                    "unexpected start element {} in {} from {:?}",
                    name,
                    element_name,
                    path
                );
                skip_element(event_reader)?;
            }
            Ok(event) => {
                log::warn!(
                    "unexpected event {:?} in {} from {:?}",
                    event,
                    element_name,
                    path
                );
            }
            Err(e) => {
                return Err(format!(
                    "error parsing {} from {:?}: {}",
                    element_name,
                    path.display(),
                    e
                ));
            }
        }
    }
    Ok(text)
}

/// Skips the current element and all its children.  This is crucial for robust error handling.
fn skip_element<R: Read>(event_reader: &mut EventReader<R>) -> ParseResult<()> {
    let mut depth = 1;
    loop {
        let event = event_reader.next();
        match event {
            Ok(XmlReaderEvent::StartElement { .. }) => depth += 1,
            Ok(XmlReaderEvent::EndElement { .. }) => depth -= 1,
            Ok(_) => {}
            Err(e) => return Err(format!("error skipping element: {}", e)),
        }
        if depth == 0 {
            break;
        }
    }
    Ok(())
}

fn backup_config(path: &Path) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should have passed since unix epoch")
        .as_micros();
    let file_stem = path
        .file_stem()
        .expect("path should be set in crate::game::paths");
    let file_extension = path.extension().expect("path should have an extension");
    let backup_path = path.with_file_name(format!(
        "{}.{}.{}",
        file_stem.to_str().unwrap(),
        timestamp,
        file_extension.to_str().unwrap()
    ));

    log::info!("backing up mods config to {:?}", backup_path);
    if let Err(err) = fs::copy(path, backup_path) {
        log::error!("error backing up mods config: {}", err);
    }
}

fn save_config_to_file(path: &Path, config: &ModsConfigData) {
    log::info!("saving mods config to {:?}", path);
    let file = match File::create(path) {
        Ok(f) => f,
        Err(err) => {
            log::error!("error creating mods config file {:?}: {}", path, err);
            return;
        }
    };

    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .write_document_declaration(true)
        .create_writer(file);

    if let Err(err) = write_mods_config(&mut writer, config) {
        log::error!("error writing mods config: {}", err);
    }
}

fn write_mods_config<W: std::io::Write>(
    writer: &mut EventWriter<W>,
    config: &ModsConfigData,
) -> Result<(), String> {
    writer
        .write(XmlWriterEvent::start_element("ModsConfigData"))
        .map_err(|e| e.to_string())?;

    write_element(writer, "version", &config.version)?;
    write_list_element(writer, "activeMods", &config.active_mods)?;
    write_list_element(writer, "knownExpansions", &config.known_expansions)?;

    writer
        .write(XmlWriterEvent::end_element())
        .map_err(|e| e.to_string())?; // ModsConfigData

    Ok(())
}

fn write_element<W: std::io::Write>(
    writer: &mut EventWriter<W>,
    element_name: &str,
    text: &str,
) -> Result<(), String> {
    writer
        .write(XmlWriterEvent::start_element(element_name))
        .map_err(|e| e.to_string())?;
    writer
        .write(XmlWriterEvent::characters(text))
        .map_err(|e| e.to_string())?;
    writer
        .write(XmlWriterEvent::end_element())
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn write_list_element<W: std::io::Write>(
    writer: &mut EventWriter<W>,
    element_name: &str,
    items: &[String],
) -> Result<(), String> {
    writer
        .write(XmlWriterEvent::start_element(element_name))
        .map_err(|e| e.to_string())?;

    for item in items {
        writer
            .write(XmlWriterEvent::start_element("li"))
            .map_err(|e| e.to_string())?;
        writer
            .write(XmlWriterEvent::characters(&item.to_ascii_lowercase()))
            .map_err(|e| e.to_string())?;
        writer
            .write(XmlWriterEvent::end_element())
            .map_err(|e| e.to_string())?; // li
    }

    writer
        .write(XmlWriterEvent::end_element())
        .map_err(|e| e.to_string())?; // element_name

    Ok(())
}
