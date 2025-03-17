use std::io::Read;
use std::path::Path;

use xml::reader::{EventReader, ParserConfig, XmlEvent as ReaderEvent};

// todo: probably refactor this and wrap events etc

pub type ParseResult<T> = Result<T, String>;

pub fn create_reader<R: Read>(reader: R) -> EventReader<R> {
    let parser_config = ParserConfig::new()
        .whitespace_to_characters(true)
        .cdata_to_characters(true)
        .ignore_comments(true)
        .coalesce_characters(true);
    EventReader::new_with_config(reader, parser_config)
}

// todo: review this is correctly handling characters inside li
pub fn parse_string_list<R: Read>(
    reader: &mut EventReader<R>,
    path: &Path,
    container_name: &str,
) -> ParseResult<Vec<String>> {
    let mut list = Vec::new();
    loop {
        match reader.next() {
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("li") =>
            {
                match reader.next() {
                    Ok(ReaderEvent::Characters(chars)) => {
                        list.push(chars);
                        match reader.next() {
                            Ok(ReaderEvent::EndElement { name })
                                if name.local_name.eq_ignore_ascii_case("li") => {}
                            Ok(event) => log::warn!(
                                "unexpected event {event:?} in {container_name} li from {path:?}",
                            ),
                            Err(e) => {
                                return Err(format!(
                                    "error parsing {container_name} li from {path:?}: {e}",
                                ));
                            }
                        }
                    }
                    Ok(ReaderEvent::EndElement { name })
                        if name.local_name.eq_ignore_ascii_case("li") => {} // Empty li element
                    Ok(event) => {
                        log::warn!(
                            "unexpected event {event:?} in {container_name} li from {path:?}"
                        );
                    }
                    Err(e) => {
                        return Err(format!(
                            "error parsing {container_name} li from {path:?}: {e}"
                        ));
                    }
                }
            }
            Ok(ReaderEvent::EndElement { name })
                if name.local_name.eq_ignore_ascii_case(container_name) =>
            {
                break;
            }
            Ok(ReaderEvent::Characters(chars)) => {
                if !chars.trim().is_empty() {
                    log::warn!("unexpected characters {chars} in {container_name} from {path:?}");
                }
            }
            Ok(ReaderEvent::Whitespace(_)) => {} // ignore whitespace
            Ok(event) => log::warn!("unexpected event {event:?} in {container_name} from {path:?}"),
            Err(e) => {
                return Err(format!("error parsing {container_name} from {path:?}: {e}"));
            }
        }
    }
    Ok(list)
}

pub fn parse_text_element<R: Read>(
    reader: &mut EventReader<R>,
    path: &Path,
    element_name: &str,
) -> ParseResult<String> {
    let mut text = String::new();
    loop {
        match reader.next() {
            Ok(ReaderEvent::Characters(chars)) => {
                text.push_str(&chars);
            }
            Ok(ReaderEvent::EndElement { name })
                if name.local_name.eq_ignore_ascii_case(element_name) =>
            {
                break;
            }
            Ok(ReaderEvent::StartElement { name, .. }) => {
                log::warn!("unexpected start element {name} in {element_name} from {path:?}");
                skip_element(reader)?;
            }
            Ok(event) => {
                log::warn!("unexpected event {event:?} in {element_name} from {path:?}");
            }
            Err(e) => {
                return Err(format!("error parsing {element_name} from {path:?}: {e}"));
            }
        }
    }
    Ok(text)
}

/// Skips the current element and all its children.  This is crucial for robust error handling.
pub fn skip_element<R: Read>(reader: &mut EventReader<R>) -> ParseResult<()> {
    let mut depth = 1;
    loop {
        match reader.next() {
            Ok(ReaderEvent::StartElement { .. }) => depth += 1,
            Ok(ReaderEvent::EndElement { .. }) => depth -= 1,
            Ok(_) => {}
            Err(e) => return Err(format!("error skipping element: {e}")),
        }
        if depth == 0 {
            break;
        }
    }
    Ok(())
}
