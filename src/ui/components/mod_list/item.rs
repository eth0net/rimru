use std::fs;

use gpui::ClickEvent;

use crate::{
    game::mods::{ModIssues, ModMetaData},
    theme::colors,
    ui::prelude::*,
};

type OnClickFunc = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct ModListItem {
    id: ElementId,
    mod_meta: Entity<ModMetaData>,
    mod_issues: Option<ModIssues>,
    selected: bool,
    on_click: Option<OnClickFunc>,
}

impl ModListItem {
    pub fn new(
        id: impl Into<ElementId>,
        mod_meta: Entity<ModMetaData>,
        mod_issues: Option<ModIssues>,
    ) -> Self {
        Self {
            id: id.into(),
            mod_meta,
            mod_issues,
            selected: false,
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Toggleable for ModListItem {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl RenderOnce for ModListItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let mod_meta = self.mod_meta.read(cx);
        let mod_name = mod_meta.name.clone();
        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .px_2()
            .when_some(self.on_click, |this, on_click| {
                this.cursor_pointer().on_click(on_click)
            })
            .child(
                IconButton::from_name(
                    SharedString::from(format!("{mod_name}-source")),
                    mod_meta.source.icon_name(),
                )
                .style(ButtonStyle::Transparent),
            )
            .child({
                let id = format!("{mod_name}-icon");
                let icon_path = mod_meta.icon_file_path();
                let icon_source = match fs::metadata(&icon_path) {
                    Ok(_) => icon_path.into(),
                    Err(_) => IconName::Unknown.into(),
                };
                IconButton::new(SharedString::from(id), icon_source).style(ButtonStyle::Transparent)
            })
            .child(
                div()
                    .flex_grow()
                    .overflow_hidden()
                    .text_ellipsis()
                    .child(mod_name.clone()),
            )
            // todo: indicate if mod is incompatible with game version
            // todo: indicate if the mod has any load order conflicts
            .when_some(self.mod_issues, |this, issues: ModIssues| {
                this.child(div().flex().flex_row().items_center().px_2().child({
                    let id = format!("{mod_name}-issues");

                    let icon = if issues.has_errors() {
                        IconName::Error
                    } else if issues.has_warnings() {
                        IconName::Warning
                    } else {
                        // todo: info?
                        IconName::Warning
                    };

                    IconButton::from_name(SharedString::from(id), icon)
                        .style(ButtonStyle::Transparent)
                        .tooltip(Tooltip::text(issues.to_string()))
                        .when(issues.has_errors(), |el| {
                            el.icon_color(Hsla::from(rgba(colors::ERROR_TEXT)))
                        })
                        .when(issues.has_warnings(), |el| {
                            el.icon_color(Hsla::from(rgba(colors::WARNING_TEXT)))
                        })
                }))
            })
    }
}
