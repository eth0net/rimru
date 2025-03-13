use gpui::{Pixels, px};

/// Defines window border radius for platforms that use client side decorations.
pub const CLIENT_SIDE_DECORATION_ROUNDING: Pixels = px(10.0);
/// Defines window shadow size for platforms that use client side decorations.
pub const CLIENT_SIDE_DECORATION_SHADOW: Pixels = px(10.0);

pub mod colours {
    pub const BACKGROUND: u32 = 0x0d1117;
    pub const BORDER: u32 = 0x30363d;
    pub const TEXT: u32 = 0xffffff;
    pub const TEXT_SECONDARY: u32 = 0xcccccc;
    pub const TITLE_BAR_BACKGROUND: u32 = 0x0d1117;
    // pub const TITLE_BAR_INACTIVE_BACKGROUND: u32 = 0x1e2228;
}
