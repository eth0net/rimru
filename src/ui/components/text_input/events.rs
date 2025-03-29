use gpui::{EventEmitter, SharedString};

use super::TextInput;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextInputEvent {
    ContentChanged { content: SharedString },
}

impl EventEmitter<TextInputEvent> for TextInput {}
