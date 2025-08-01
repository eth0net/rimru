use std::io::Read;
use std::{collections::BTreeMap, path::Path};

use xml::reader::{EventReader, ParserConfig, XmlEvent as ReaderEvent};

/// Result type used for XML parsing functions in this module.
pub type ParseResult<T> = Result<T, String>;

/// Creates an XML event reader with custom configuration for whitespace, CDATA, and comments.
/// This is the standard entry point for XML parsing in this codebase.
pub fn create_reader<R: Read>(reader: R) -> EventReader<R> {
    let parser_config = ParserConfig::new()
        .whitespace_to_characters(true)
        .cdata_to_characters(true)
        .ignore_comments(true)
        .coalesce_characters(true);
    EventReader::new_with_config(reader, parser_config)
}

/// Generic parser for mapping keys to maps of values (e.g., BTreeMap<String, BTreeMap<String, ModDependency>>).
/// Each child element's tag is used as the key, and its value is parsed using the provided value_parser function.
pub fn parse_map_of_maps<R: Read, V>(
    events: &mut EventReader<R>,
    path: &Path,
    container_name: &str,
    value_parser: fn(&mut EventReader<R>, &Path, &str) -> ParseResult<V>,
) -> ParseResult<BTreeMap<String, V>> {
    let mut map = BTreeMap::new();
    loop {
        match events.next() {
            Ok(ReaderEvent::StartElement { name, .. }) => {
                let key = name.local_name.clone();
                let value = value_parser(events, path, &key)?;
                map.insert(key, value);
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
            Ok(ReaderEvent::Whitespace(_)) => {}
            Ok(event) => {
                log::warn!("unexpected event {event:?} in {container_name} from {path:?}");
                if let ReaderEvent::StartElement { .. } = event {
                    skip_element(events)?;
                }
            }
            Err(e) => {
                return Err(format!("error parsing {container_name} from {path:?}: {e}"));
            }
        }
    }
    Ok(map)
}

/// Parses a collection of strings from a container of <li> elements.
/// The collection type is generic and inferred from the return type (e.g., Vec<String>, BTreeSet<String>).
///
/// # Example XML
/// <container>
///   <li>foo</li>
///   <li>bar</li>
/// </container>
pub fn parse_string_collection<R: Read, C>(
    events: &mut EventReader<R>,
    path: &Path,
    container_name: &str,
) -> ParseResult<C>
where
    C: FromIterator<String>,
{
    let mut items = Vec::new();
    loop {
        match events.next() {
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("li") =>
            {
                match events.next() {
                    Ok(ReaderEvent::Characters(chars)) => {
                        items.push(chars);
                        match events.next() {
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
    Ok(items.into_iter().collect())
}

/// Parses a map from keys to collections of strings, where each key is a child element and its value is a collection of <li> elements.
/// The collection type is generic and inferred from the return type (e.g., BTreeMap<String, Vec<String>>).
///
/// # Example XML
/// <container>
///   <Version1>
///     <li>foo</li>
///     <li>bar</li>
///   </Version1>
///   <Version2>
///     <li>baz</li>
///   </Version2>
/// </container>
pub fn parse_string_collection_map<R: Read, C>(
    events: &mut EventReader<R>,
    path: &Path,
    container_name: &str,
) -> ParseResult<BTreeMap<String, C>>
where
    C: FromIterator<String>,
{
    let mut map = BTreeMap::new();
    loop {
        match events.next() {
            Ok(ReaderEvent::StartElement { name, .. }) => {
                let key = name.local_name.clone();
                let collection = parse_string_collection(events, path, &key)?;
                map.insert(key, collection);
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
            Ok(ReaderEvent::Whitespace(_)) => {}
            Ok(event) => {
                log::warn!("unexpected event {event:?} in {container_name} from {path:?}");
                if let ReaderEvent::StartElement { .. } = event {
                    skip_element(events)?;
                }
            }
            Err(e) => {
                return Err(format!("error parsing {container_name} from {path:?}: {e}"));
            }
        }
    }
    Ok(map)
}

/// Parses the text content of an XML element, consuming all character data until the end tag.
/// Ignores and skips any unexpected child elements.
pub fn parse_text_element<R: Read>(
    events: &mut EventReader<R>,
    path: &Path,
    element_name: &str,
) -> ParseResult<String> {
    let mut text = String::new();
    loop {
        match events.next() {
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
                skip_element(events)?;
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

/// Parses a map of strings from XML, where each child element's tag is the key and its text content is the value.
/// For example:
/// <container>
///   <Key1>Value1</Key1>
///   <Key2>Value2</Key2>
/// </container>
pub fn parse_string_map<R: Read>(
    events: &mut EventReader<R>,
    path: &Path,
    container_name: &str,
) -> ParseResult<BTreeMap<String, String>> {
    let mut map = BTreeMap::new();
    loop {
        match events.next() {
            Ok(ReaderEvent::StartElement { name, .. }) => {
                let key = name.local_name.clone();
                let value = parse_text_element(events, path, &key)?;
                map.insert(key, value);
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
            Ok(ReaderEvent::Whitespace(_)) => {}
            Ok(event) => {
                log::warn!("unexpected event {event:?} in {container_name} from {path:?}");
                if let ReaderEvent::StartElement { .. } = event {
                    skip_element(events)?;
                }
            }
            Err(e) => {
                return Err(format!("error parsing {container_name} from {path:?}: {e}"));
            }
        }
    }
    Ok(map)
}

/// Skips the current element and all its children, consuming events until the matching end tag.
/// This is crucial for robust error handling and ignoring unknown or unsupported elements.
pub fn skip_element<R: Read>(events: &mut EventReader<R>) -> ParseResult<()> {
    let mut depth = 1;
    loop {
        match events.next() {
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
