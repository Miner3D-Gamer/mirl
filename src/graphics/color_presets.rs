use crate::graphics::rgba_to_u32;

/// The "color" white -> 255, 255, 255
pub const WHITE: u32 = rgba_to_u32(255, 255, 255, 255);

/// The "color" black -> 0, 0, 0
pub const BLACK: u32 = rgba_to_u32(0, 0, 0, 255);

/// The color pure red -> 255, 0, 0
pub const PURE_RED: u32 = rgba_to_u32(255, 0, 0, 255);

/// The color green -> 0, 255, 0
pub const PURE_GREEN: u32 = rgba_to_u32(0, 255, 0, 255);

/// The color blue -> 0, 0, 255
pub const PURE_BLUE: u32 = rgba_to_u32(0, 0, 255, 255);

/// The color magenta -> 255, 0, 255
pub const PURE_MAGENTA: u32 = rgba_to_u32(255, 0, 255, 255);

/// The color light blue -> 0, 255, 255
pub const PURE_LIGHT_BLUE: u32 = rgba_to_u32(0, 255, 255, 255);

/// The color pure yellow -> 255, 0, 0
pub const PURE_YELLOW: u32 = rgba_to_u32(255, 255, 0, 255);
