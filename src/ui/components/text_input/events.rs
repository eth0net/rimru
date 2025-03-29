use gpui::EventEmitter;

use super::TextInput;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextInputEvent {}

impl EventEmitter<TextInputEvent> for TextInput {}
