use std::path::PathBuf;

use gpui::{EntityInputHandler, relative};

use crate::{
    settings::Settings,
    ui::{TextInput, TextInputEvent, prelude::*},
};

pub struct SettingsPane {
    settings: Entity<Settings>,
    game: Entity<TextInput>,
    official_mods: Entity<TextInput>,
    local_mods: Entity<TextInput>,
    steam_mods: Entity<TextInput>,
    config: Entity<TextInput>,
    // todo: add toggle for separate search bar
}

impl SettingsPane {
    pub fn new(settings: Entity<Settings>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let game = TextInput::new(cx);
        let official_mods = TextInput::new(cx);
        let local_mods = TextInput::new(cx);
        let steam_mods = TextInput::new(cx);
        let config = TextInput::new(cx);

        settings.update(cx, |settings, cx| {
            if let Some(path) = settings.game_dir().to_str() {
                game.update(cx, |input, cx| {
                    input.replace_text_in_range(None, path, window, cx);
                });
            }

            if let Some(path) = settings.official_mods_dir().to_str() {
                official_mods.update(cx, |input, cx| {
                    input.replace_text_in_range(None, path, window, cx);
                });
            }

            if let Some(path) = settings.local_mods_dir().to_str() {
                local_mods.update(cx, |input, cx| {
                    input.replace_text_in_range(None, path, window, cx);
                });
            }

            if let Some(path) = settings.steam_mods_dir().to_str() {
                steam_mods.update(cx, |input, cx| {
                    input.replace_text_in_range(None, path, window, cx);
                });
            }

            if let Some(path) = settings.config_dir().to_str() {
                config.update(cx, |input, cx| {
                    input.replace_text_in_range(None, path, window, cx);
                });
            }
        });

        cx.subscribe(&game, |this, _, event, cx| match event {
            TextInputEvent::ContentChanged { content } => {
                this.settings.update(cx, |settings, _| {
                    settings.set_game_dir(PathBuf::from(content.to_string()));
                });
            }
        })
        .detach();

        cx.subscribe(&official_mods, |this, _, event, cx| match event {
            TextInputEvent::ContentChanged { content } => {
                this.settings.update(cx, |settings, _| {
                    settings.set_official_mods_dir(PathBuf::from(content.to_string()));
                });
            }
        })
        .detach();

        cx.subscribe(&local_mods, |this, _, event, cx| match event {
            TextInputEvent::ContentChanged { content } => {
                this.settings.update(cx, |settings, _| {
                    settings.set_local_mods_dir(PathBuf::from(content.to_string()));
                });
            }
        })
        .detach();

        cx.subscribe(&steam_mods, |this, _, event, cx| match event {
            TextInputEvent::ContentChanged { content } => {
                this.settings.update(cx, |settings, _| {
                    settings.set_steam_mods_dir(PathBuf::from(content.to_string()));
                });
            }
        })
        .detach();

        cx.subscribe(&config, |this, _, event, cx| match event {
            TextInputEvent::ContentChanged { content } => {
                this.settings.update(cx, |settings, _| {
                    settings.set_config_dir(PathBuf::from(content.to_string()));
                });
            }
        })
        .detach();

        Self {
            settings,
            game,
            official_mods,
            local_mods,
            steam_mods,
            config,
        }
    }
}

impl Render for SettingsPane {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex_grow()
            .flex()
            .flex_col()
            .overflow_hidden()
            .p_2()
            .gap_2()
            .child("Settings")
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .gap_1()
                    .child(
                        div()
                            .flex_none()
                            .flex_basis(relative(0.1))
                            .min_w_24()
                            .child("Game:"),
                    )
                    .child(div().flex_auto().child(self.game.clone())),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .gap_1()
                    .child(
                        div()
                            .flex_none()
                            .flex_basis(relative(0.1))
                            .min_w_24()
                            .child("Official Mods:"),
                    )
                    .child(div().flex_auto().child(self.official_mods.clone())),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .gap_1()
                    .child(
                        div()
                            .flex_none()
                            .flex_basis(relative(0.1))
                            .min_w_24()
                            .child("Local Mods:"),
                    )
                    .child(div().flex_auto().child(self.local_mods.clone())),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .gap_1()
                    .child(
                        div()
                            .flex_none()
                            .flex_basis(relative(0.1))
                            .min_w_24()
                            .child("Steam Mods:"),
                    )
                    .child(div().flex_auto().child(self.steam_mods.clone())),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .w_full()
                    .gap_1()
                    .child(
                        div()
                            .flex_none()
                            .flex_basis(relative(0.1))
                            .min_w_24()
                            .child("Config:"),
                    )
                    .child(div().flex_auto().child(self.config.clone())),
            )
    }
}
