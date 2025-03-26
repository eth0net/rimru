use gpui::{Pixels, px};

/// Defines window border radius for platforms that use client side decorations.
pub const CLIENT_SIDE_DECORATION_ROUNDING: Pixels = px(10.0);
/// Defines window shadow size for platforms that use client side decorations.
pub const CLIENT_SIDE_DECORATION_SHADOW: Pixels = px(10.0);

pub mod colors {
    pub const BACKGROUND: u32 = 0x0d1117ff;
    pub const BORDER: u32 = 0x30363dff;
    pub const BORDER_DISABLED: u32 = 0x30363dff;
    pub const BORDER_FOCUSED: u32 = 0x1f6febff;
    pub const BORDER_SELECTED: u32 = 0x30363dff;
    pub const BORDER_VARIANT: u32 = 0x30363dff;
    pub const TEXT: u32 = 0xffffffff;
    pub const TEXT_ACCENT: u32 = 0xc2e6ffff;
    pub const TEXT_DISABLED: u32 = 0xfffdee73;
    pub const TEXT_SECONDARY: u32 = 0xccccccff;
    pub const TITLE_BAR_BACKGROUND: u32 = 0x0d1117ff;
    // pub const TITLE_BAR_INACTIVE_BACKGROUND: u32 = 0x1e2228ff;
    pub const ELEMENT_ACTIVE: u32 = 0x6e768166;
    pub const ELEMENT_BACKGROUND: u32 = 0x3e547166;
    pub const ELEMENT_DISABLED: u32 = 0xfefef31b;
    pub const ELEMENT_HOVER: u32 = 0x6e768166;
    pub const ELEMENT_SELECTED: u32 = 0x6e768166;
    pub const GHOST_ELEMENT_ACTIVE: u32 = 0x6e76811a;
    pub const GHOST_ELEMENT_BACKGROUND: u32 = 0x0000000000;
    pub const GHOST_ELEMENT_DISABLED: u32 = 0x2a2a28ff;
    pub const GHOST_ELEMENT_HOVER: u32 = 0x6e76811a;
    pub const DROP_TARGET_BACKGROUND: u32 = 0x008ff519;
    // pub const ELEVATED_SURFACE_BACKGROUND: u32 = 0x161b22ff;
    // pub const SURFACE_BACKGROUND: u32 = 0x010409ff;
    pub const INFO_BACKGROUND: u32 = 0x3b9effff;
    pub const INFO_BORDER: u32 = 0x3b9effff;
    pub const INFO_TEXT: u32 = 0x3b9effff;
    pub const ERROR_BACKGROUND: u32 = 0xec5d5eff;
    pub const ERROR_BORDER: u32 = 0xec5d5eff;
    pub const ERROR_TEXT: u32 = 0xec5d5eff;
    pub const WARNING_BACKGROUND: u32 = 0x161b22ff;
    pub const WARNING_BORDER: u32 = 0x30363dff;
    pub const WARNING_TEXT: u32 = 0xfbe557ff;
    pub const SUCCESS_BACKGROUND: u32 = 0x53b365ff;
    pub const SUCCESS_BORDER: u32 = 0x53b365ff;
    pub const SUCCESS_TEXT: u32 = 0x53b365ff;
    pub const PANEL_BACKGROUND: u32 = 0x010409ff;
    pub const ELEVATED_SURFACE_BACKGROUND: u32 = 0x161b22ff;
}
