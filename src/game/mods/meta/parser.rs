use std::{collections::BTreeMap, io::Read, path::Path};

use xml::reader::{EventReader, XmlEvent as ReaderEvent};

use crate::game::xml::*;

use super::{ModDependency, ModMetaData};

pub(super) fn parse_mod_metadata<R: Read>(
    mut events: EventReader<R>,
    mod_meta: &mut ModMetaData,
) -> ParseResult<()> {
    loop {
        match events.next() {
            Ok(ReaderEvent::EndDocument) => break,
            Ok(ReaderEvent::StartDocument { .. }) => {}
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("modMetaData") =>
            {
                parse_mod_metadata_data(&mut events, mod_meta)?;
            }
            Ok(event) => {
                log::trace!("unexpected root event {event:?} from {:?}", mod_meta.path);
                if let ReaderEvent::StartElement { .. } = event {
                    skip_element(&mut events)?;
                }
            }
            Err(e) => {
                return Err(format!(
                    "error parsing root event from {:?}: {e}",
                    mod_meta.path
                ));
            }
        }
    }
    Ok(())
}

fn parse_mod_metadata_data<R: Read>(
    events: &mut EventReader<R>,
    mod_meta: &mut ModMetaData,
) -> ParseResult<()> {
    let path = mod_meta.path.as_ref();
    loop {
        match events.next() {
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("author") =>
            {
                // Handle single author tag
                let author_string = parse_text_element(events, path, &name.local_name)?;
                for author in author_string.split(',') {
                    mod_meta.authors.push(author.trim().to_string());
                }
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("authors") =>
            {
                mod_meta.authors = parse_string_list(events, path, &name.local_name)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("description") =>
            {
                mod_meta.description = parse_text_element(events, path, &name.local_name)?;
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
                mod_meta.force_load_after = parse_string_set(events, path, &name.local_name)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("forceLoadBefore") =>
            {
                mod_meta.force_load_before = parse_string_set(events, path, &name.local_name)?;
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
                mod_meta.load_after = parse_string_set(events, path, &name.local_name)?;
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
                mod_meta.load_before = parse_string_set(events, path, &name.local_name)?;
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
                mod_meta.dependencies = parse_mod_dependencies(events, path, &name.local_name)?;
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
                mod_meta.name = parse_text_element(events, path, &name.local_name)?;
            }
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("packageId") =>
            {
                mod_meta.id = parse_text_element(events, path, &name.local_name)?;
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
                mod_meta.steam_app_id = Some(parse_text_element(events, path, &name.local_name)?);
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
                if let ReaderEvent::StartElement { .. } = event {
                    skip_element(events)?;
                }
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

fn parse_mod_dependencies<R: Read>(
    events: &mut EventReader<R>,
    path: &Path,
    container_name: &str,
) -> ParseResult<BTreeMap<String, ModDependency>> {
    let mut dependencies = BTreeMap::new();
    loop {
        match events.next() {
            Ok(ReaderEvent::StartElement { name, .. })
                if name.local_name.eq_ignore_ascii_case("li") =>
            {
                let mut dependency = ModDependency::default();
                loop {
                    match events.next() {
                        Ok(ReaderEvent::StartElement { name, .. })
                            if name.local_name.eq_ignore_ascii_case("packageId") =>
                        {
                            dependency.id = parse_text_element(events, path, &name.local_name)?;
                        }
                        Ok(ReaderEvent::StartElement { name, .. })
                            if name.local_name.eq_ignore_ascii_case("displayName") =>
                        {
                            dependency.name = parse_text_element(events, path, &name.local_name)?;
                        }
                        Ok(ReaderEvent::StartElement { name, .. })
                            if name.local_name.eq_ignore_ascii_case("name") =>
                        {
                            dependency.name = parse_text_element(events, path, &name.local_name)?;
                        }
                        Ok(ReaderEvent::StartElement { name, .. })
                            if name.local_name.eq_ignore_ascii_case("downloadUrl") =>
                        {
                            skip_element(events)?;
                        }
                        Ok(ReaderEvent::StartElement { name, .. })
                            if name.local_name.eq_ignore_ascii_case("steamWorkshopUrl") =>
                        {
                            skip_element(events)?;
                        }
                        Ok(ReaderEvent::EndElement { name })
                            if name.local_name.eq_ignore_ascii_case("li") =>
                        {
                            break;
                        }
                        Ok(ReaderEvent::Characters(chars)) => {
                            // ignore whitespace characters
                            if !chars.trim().is_empty() {
                                log::warn!(
                                    "unexpected characters {chars} in {container_name} li from {path:?}"
                                );
                            }
                        }
                        Ok(event) => {
                            log::warn!(
                                "unexpected event {event:?} in {container_name} li from {path:?}"
                            );
                            if let ReaderEvent::StartElement { .. } = event {
                                skip_element(events)?;
                            }
                        }
                        Err(e) => {
                            return Err(format!(
                                "error parsing {container_name} li from {path:?}: {e}"
                            ));
                        }
                    }
                }
                dependencies.insert(dependency.id.clone(), dependency);
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
    Ok(dependencies)
}
