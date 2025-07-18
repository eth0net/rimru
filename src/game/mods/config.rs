use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use xml::{
    reader::{EventReader, XmlEvent as ReaderEvent},
    writer::{EmitterConfig, EventWriter, XmlEvent as WriterEvent},
};

use crate::game::xml::*;

#[derive(Debug, Clone, Default)]
pub struct ModsConfigData {
    pub version: String,
    pub active_mods: Vec<String>,
    pub known_expansions: Vec<String>,
}

impl ModsConfigData {
    pub fn load(path: &Path) -> Option<Self> {
        load_config_from_file(path)
    }

    pub fn save(&self, path: &Path) {
        backup_config(path);
        save_config_to_file(path, self);
    }
}

fn load_config_from_file(path: &Path) -> Option<ModsConfigData> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("error opening mods config file {path:?}: {e}");
            return None;
        }
    };

    let reader = BufReader::new(file);
    let event_reader = create_reader(reader);

    match parse_mods_config(event_reader, path) {
        Ok(config) => Some(config),
        Err(e) => {
            log::error!("error parsing mods config file {path:?}: {e}");
            None
        }
    }
}

type ParseResult<T> = Result<T, String>;

fn parse_mods_config<R: Read>(
    mut events: EventReader<R>,
    path: &Path,
) -> ParseResult<ModsConfigData> {
    let mut config = ModsConfigData::default();

    loop {
        match events.next() {
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("ModsConfigData") =>
            {
                parse_mods_config_data(&mut events, path, &mut config)?;
            }
            Ok(ReaderEvent::EndDocument) => break,
            Ok(ReaderEvent::StartDocument { .. }) => {}
            Ok(event) => {
                log::trace!("unexpected root event {event:?} from {path:?}");
                if let ReaderEvent::StartElement { .. } = event {
                    skip_element(&mut events)?;
                }
            }
            Err(e) => {
                return Err(format!("error parsing root event from {path:?}: {e}"));
            }
        }
    }

    Ok(config)
}

fn parse_mods_config_data<R: Read>(
    events: &mut EventReader<R>,
    path: &Path,
    config: &mut ModsConfigData,
) -> ParseResult<()> {
    loop {
        match events.next() {
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("activeMods") =>
            {
                config.active_mods = parse_string_list(events, path, &name.local_name)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("knownExpansions") =>
            {
                config.known_expansions = parse_string_list(events, path, &name.local_name)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("version") =>
            {
                config.version = parse_text_element(events, path, &name.local_name)?;
            }
            Ok(ReaderEvent::EndElement { name })
                if name.local_name.eq_ignore_ascii_case("ModsConfigData") =>
            {
                break;
            }
            Ok(ReaderEvent::Characters(chars)) => {
                // ignore whitespace characters
                if !chars.trim().is_empty() {
                    log::warn!("unexpected characters {chars} in modsConfigData from {path:?}");
                }
            }
            Ok(event) => {
                log::warn!("unexpected event {event:?} in modsConfigData from {path:?}");
                if let ReaderEvent::StartElement { .. } = event {
                    skip_element(events)?;
                }
            }
            Err(e) => {
                return Err(format!("error parsing modsConfigData from {path:?}: {e}"));
            }
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

    log::info!("backing up mods config to {backup_path:?}");
    if let Err(err) = fs::copy(path, backup_path) {
        log::error!("error backing up mods config: {err}");
    }
}

fn save_config_to_file(path: &Path, config: &ModsConfigData) {
    log::info!("saving mods config to {path:?}");
    let file = match File::create(path) {
        Ok(f) => f,
        Err(err) => {
            log::error!("error creating mods config file {path:?}: {err}");
            return;
        }
    };

    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .write_document_declaration(true)
        .create_writer(file);

    if let Err(err) = write_mods_config(&mut writer, config) {
        log::error!("error writing mods config: {err}");
    }
}

fn write_mods_config<W: Write>(
    writer: &mut EventWriter<W>,
    config: &ModsConfigData,
) -> Result<(), String> {
    writer
        .write(WriterEvent::start_element("ModsConfigData"))
        .map_err(|e| e.to_string())?;

    write_element(writer, "version", &config.version)?;
    write_list_element(writer, "activeMods", &config.active_mods)?;
    write_list_element(writer, "knownExpansions", &config.known_expansions)?;

    writer
        .write(WriterEvent::end_element())
        .map_err(|e| e.to_string())?; // ModsConfigData

    Ok(())
}

fn write_element<W: Write>(
    writer: &mut EventWriter<W>,
    element_name: &str,
    text: &str,
) -> Result<(), String> {
    writer
        .write(WriterEvent::start_element(element_name))
        .map_err(|e| e.to_string())?;
    writer
        .write(WriterEvent::characters(text))
        .map_err(|e| e.to_string())?;
    writer
        .write(WriterEvent::end_element())
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn write_list_element<W: Write>(
    writer: &mut EventWriter<W>,
    element_name: &str,
    items: &[String],
) -> Result<(), String> {
    writer
        .write(WriterEvent::start_element(element_name))
        .map_err(|e| e.to_string())?;

    for item in items {
        writer
            .write(WriterEvent::start_element("li"))
            .map_err(|e| e.to_string())?;
        writer
            .write(WriterEvent::characters(&item.to_ascii_lowercase()))
            .map_err(|e| e.to_string())?;
        writer
            .write(WriterEvent::end_element())
            .map_err(|e| e.to_string())?; // li
    }

    writer
        .write(WriterEvent::end_element())
        .map_err(|e| e.to_string())?; // element_name

    Ok(())
}
