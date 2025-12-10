// TODO: Add a middle step from data -> BaseInfoCursor -> Actual cursor used by os/framework

#[cfg(all(feature = "system", target_os = "windows"))]
#[cfg(feature = "svg")]
pub use cursors_windows::load_base_cursor_with_file;

#[cfg(feature = "keycodes")]
use crate::platform::frameworks::WindowError;
#[cfg(feature = "svg")]
use crate::platform::CursorStyle;
use crate::{extensions::*, platform::Buffer};

// Damn past me, why so sassy?
/// Cursor stuff of glfw bc glfw is a bitch
#[cfg(feature = "svg")]
pub mod cursor_glfw;
/// Implementation for cursors on windows
#[cfg(target_os = "windows")]
#[cfg(feature = "svg")]
pub mod cursors_windows;
/// Mouse position stuff like raw mouse input
pub mod position;
/// The Cursor Manager provides a way of easily loading cursors based on a Buffer or the default cursors that come with this lib
#[const_trait]
pub trait CursorManager {
    /// Create a Cursor instance using a Buffer
    fn load_cursor(
        &mut self,
        name: &str,
        size: U2,
        image_data: Buffer,
        hotspot_x: u16,
        hotspot_y: u16,
    );
    /// Get the cursor instance with the specified name
    fn get_cursor(&self, name: &str) -> Cursor;
    /// Load the builtin cursors this lib provides
    fn load_builtin_cursors(
        &mut self,
        size: U2,

        main_color: u32,
        secondary_color: u32,
    );
}

/// Cursor Instance holding the required cursor data to be used somewhere else
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cursor {
    /// Windows
    #[cfg(target_os = "windows")]
    #[cfg(feature = "svg")]
    Win(windows::Win32::UI::WindowsAndMessaging::HCURSOR),
    /// Linux using X11, wayland support will be added later
    #[cfg(target_os = "linux")]
    X11(u64), // This could be a cursor ID from X11
    /// Mac
    #[cfg(target_os = "macos")]
    Mac(*mut std::ffi::c_void), // Placeholder for NSCursor or equivalent
    /// glfw lib, cross-platform
    Glfw((Buffer, u32, u32)),
}
/// Set the current value to the new cursor
pub trait SetSelfToCursor {
    /// Set the current value to the new cursor
    fn set(&mut self, cursor: &Cursor);
}
impl SetSelfToCursor for Cursor {
    fn set(&mut self, cursor: &Cursor) {
        *self = cursor.clone();
    }
}
impl SetSelfToCursor for Option<Cursor> {
    fn set(&mut self, cursor: &Cursor) {
        *self = Some(cursor.clone());
    }
}
// /// Set the current value to the new cursor
// pub trait SetSelfToCursorStyle {
//     /// Set the current value to the new cursor
//     fn set(&mut self, cursor: &CursorStyle);
// }
// impl SetSelfToCursorStyle for CursorStyle {
//     fn set(&mut self, cursor: &CursorStyle) {
//         *self = cursor.clone()
//     }
// }
// impl SetSelfToCursorStyle for Option<CursorStyle> {
//     fn set(&mut self, cursor: &CursorStyle) {
//         *self = Some(cursor.clone())
//     }
// }

// impl Cursor {
//     pub fn get(&self) -> Self {
//         self.clone()
//     }
// }

// pub struct CursorData {
//     buffer_data: Vec<u32>,
// }

/// Default cursors this lib provides
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursors {
    /// Default Pointer
    pub alias: Cursor,
    /// Resize vertically + Resize horizontally
    pub all_scroll: Cursor,
    /// Arrow pointing to the bottom left ⬋
    pub bottom_left_corner: Cursor,
    /// Arrow pointing to the bottom right ⬊
    pub arrow_bottom_right: Cursor,
    /// Arrow down with a _ at the end
    pub side_bottom: Cursor,
    /// A plus shape
    pub cell: Cursor,
    /// Default cursor rotated to be vertical
    pub centered_pointer: Cursor,
    /// Horizontal resizing
    pub col_resize: Cursor,
    /// Eyedropper
    pub color_picker: Cursor,
    /// Default cursor with ≡ attached
    pub context_menu: Cursor,
    /// Default cursor with a plus
    pub copy: Cursor,
    /// Cross
    pub crosshair: Cursor,
    /// Default Pointer
    pub default: Cursor,
    /// Closed hand
    pub closed_hand: Cursor,
    /// Closed hand with an 🚫 attached
    pub closed_hand_no_drop: Cursor,
    /// Arrow pointing down
    pub arrow_down: Cursor,
    /// Tip of an ink pen
    pub draft: Cursor,
    /// Small pointers in all directions like this: ◄ ►
    pub fleur: Cursor,
    /// Question mark
    pub help: Cursor,
    /// Arrow pointing left
    pub arrow_left: Cursor,
    /// Arrow left with a stopper |←
    pub side_left: Cursor,
    /// Default cursor with a 🚫 attached
    pub no_drop: Cursor,
    /// "🚫"
    pub not_allowed: Cursor,
    /// Open hand
    pub open_hand: Cursor,
    /// A Pencil
    pub pencil: Cursor,
    /// Skull
    pub pirate: Cursor,
    /// Hand with pointing index finger
    pub pointer: Cursor,
    /// Arrow pointing right
    pub arrow_right: Cursor,
    /// Mirrored version of normal cursor
    pub mirrored_pointer: Cursor,
    /// Arrow pointing right with a stopper →|
    pub side_right: Cursor,
    /// Resize top right to bottom left
    pub size_nesw: Cursor,
    /// Resize top left to bottom right
    pub size_nwse: Cursor,
    /// Resize horizontally
    pub size_hor: Cursor,
    /// Resize vertically
    pub size_ver: Cursor,
    /// I Beam
    pub text: Cursor,
    /// Arrow pointing up top left
    pub arrow_top_left: Cursor,
    /// Arrow pointing up top right
    pub arrow_top_right: Cursor,
    /// Arrow pointing up with an _ on top
    pub side_top: Cursor,
    /// Arrow pointing up
    pub arrow_up: Cursor,
    /// I Beam rotated 90°
    pub vertical_text: Cursor,
    /// Magnifying glass with plus
    pub zoom_in: Cursor,
    /// Magnifying glass with minus
    pub zoom_out: Cursor,
}
#[cfg(feature = "svg")]
impl Cursors {
    #[allow(clippy::too_many_lines)]
    /// Load all defaultly supported cursors
    ///
    /// # Errors
    /// When a cursor cannot be loaded, it returns the name of the cursor that failed to load
    pub fn load<F>(
        size: CursorResolution,
        main_color: u32,
        secondary_color: u32,
        load_base_cursor_with_file: F,
    ) -> Result<Self, String>
    where
        F: Fn(
            BaseCursor,
            CursorResolution,
            String,
        ) -> Result<Cursor, LoadCursorError>,
    {
        let default = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 6,
                hot_spot_y: 4,
            },
            size,
            include_str!("./svg/default.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "default.svg".to_string())?;

        let alias = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 6,
                hot_spot_y: 4,
            },
            size,
            include_str!("./svg/alias.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "alias.svg".to_string())?;

        let all_scroll = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/all-scroll.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "all-scroll.svg".to_string())?;

        let bottom_left_corner = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 5,
                hot_spot_y: 27,
            },
            size,
            include_str!("./svg/bottom_left_corner.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "bottom_left_corner.svg".to_string())?;

        let arrow_bottom_right = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 29,
                hot_spot_y: 27,
            },
            size,
            include_str!("./svg/bottom_right_corner.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "bottom_right_corner.svg".to_string())?;

        let side_bottom = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 28,
            },
            size,
            include_str!("./svg/bottom_side.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "bottom_side.svg".to_string())?;

        let cell = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/cell.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "cell.svg".to_string())?;

        let centered_pointer = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 15,
                hot_spot_y: 7,
            },
            size,
            include_str!("./svg/center_ptr.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "center_ptr.svg".to_string())?;

        let col_resize = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/col-resize.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "col-resize.svg".to_string())?;

        let color_picker = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 29,
            },
            size,
            include_str!("./svg/color-picker.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "color-picker.svg".to_string())?;

        let context_menu = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 6,
                hot_spot_y: 4,
            },
            size,
            include_str!("./svg/context-menu.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "context-menu.svg".to_string())?;

        let copy = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 6,
                hot_spot_y: 4,
            },
            size,
            include_str!("./svg/copy.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "copy.svg".to_string())?;

        let crosshair = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/crosshair.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "crosshair.svg".to_string())?;

        let closed_hand = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/closed-hand.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "closed-hand.svg".to_string())?;

        let closed_hand_no_drop = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/closed-hand-no-drop.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "closed-hand-no-drop.svg".to_string())?;

        let arrow_down = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 28,
            },
            size,
            include_str!("./svg/down-arrow.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "down-arrow.svg".to_string())?;

        let draft = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 29,
            },
            size,
            include_str!("./svg/draft.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "draft.svg".to_string())?;

        let fleur = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/fleur.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "fleur.svg".to_string())?;

        let help = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 4,
            },
            size,
            include_str!("./svg/help.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "help.svg".to_string())?;

        let arrow_left = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/left-arrow.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "left-arrow.svg".to_string())?;

        let side_left = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/left_side.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "left_side.svg".to_string())?;

        let no_drop = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 6,
                hot_spot_y: 4,
            },
            size,
            include_str!("./svg/no-drop.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "no-drop.svg".to_string())?;

        let not_allowed = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/not-allowed.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "not-allowed.svg".to_string())?;

        let open_hand = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/open-hand.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "open-hand.svg".to_string())?;

        let pencil = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 29,
            },
            size,
            include_str!("./svg/pencil.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "pencil.svg".to_string())?;

        let pirate = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/pirate.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "pirate.svg".to_string())?;

        let pointer = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 13,
                hot_spot_y: 7,
            },
            size,
            include_str!("./svg/pointer.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "pointer.svg".to_string())?;

        let arrow_right = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 28,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/right-arrow.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "right-arrow.svg".to_string())?;

        let mirrored_pointer = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 28,
                hot_spot_y: 4,
            },
            size,
            include_str!("./svg/right_ptr.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "right_ptr.svg".to_string())?;

        let side_right = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 28,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/right_side.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "right_side.svg".to_string())?;

        let size_nesw = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/size_nesw.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "size_nesw.svg".to_string())?;

        let size_nwse = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/size_nwse.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "size_nwse.svg".to_string())?;

        let size_hor = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/size_hor.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "size_hor.svg".to_string())?;

        let size_ver = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/size_ver.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "size_ver.svg".to_string())?;

        let text = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 15,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/text.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "text.svg".to_string())?;

        let arrow_top_left = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 9,
                hot_spot_y: 9,
            },
            size,
            include_str!("./svg/top_left_corner.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "top_left_corner.svg".to_string())?;

        let arrow_top_right = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 21,
                hot_spot_y: 9,
            },
            size,
            include_str!("./svg/top_right_corner.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "top_right_corner.svg".to_string())?;

        let side_top = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 4,
            },
            size,
            include_str!("./svg/top_side.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "top_side.svg".to_string())?;

        let arrow_up = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 4,
            },
            size,
            include_str!("./svg/up-arrow.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "up-arrow.svg".to_string())?;

        let vertical_text = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/vertical-text.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "vertical-text.svg".to_string())?;

        let zoom_in = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/zoom-in.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "zoom-in.svg".to_string())?;

        let zoom_out = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            include_str!("./svg/zoom-out.svg")
                .to_string()
                .replace(
                    "{main}",
                    &crate::graphics::u32_to_hex_without_alpha(main_color),
                )
                .replace(
                    "{secondary}",
                    &crate::graphics::u32_to_hex_without_alpha(secondary_color),
                ),
        )
        .map_err(|_| "zoom-out.svg".to_string())?;

        Ok(Self {
            alias,
            all_scroll,
            bottom_left_corner,
            arrow_bottom_right,
            side_bottom,
            cell,
            centered_pointer,
            col_resize,
            color_picker,
            context_menu,
            copy,
            crosshair,
            default,
            closed_hand,
            closed_hand_no_drop,
            arrow_down,
            draft,
            fleur,
            help,
            arrow_left,
            side_left,
            no_drop,
            not_allowed,
            open_hand,
            pencil,
            pirate,
            pointer,
            arrow_right,
            mirrored_pointer,
            side_right,
            size_nesw,
            size_nwse,
            size_hor,
            size_ver,
            text,
            arrow_top_left,
            arrow_top_right,
            side_top,
            arrow_up,
            vertical_text,
            zoom_in,
            zoom_out,
        })
    }
    /// Get the cursor from the selected [`CursorStyle`]
    #[must_use]
    pub const fn from_cursor_style(&self, style: CursorStyle) -> &Cursor {
        match style {
            CursorStyle::Alias => &self.alias,
            CursorStyle::AllScroll => &self.all_scroll,
            CursorStyle::ArrowBottomLeft => &self.bottom_left_corner,
            CursorStyle::ArrowBottomRight => &self.arrow_bottom_right,
            CursorStyle::SideBottom => &self.side_bottom,
            CursorStyle::Cell => &self.cell,
            CursorStyle::CenteredPointer => &self.centered_pointer,
            CursorStyle::ResizeHorizontally => &self.col_resize,
            CursorStyle::ColorPicker => &self.color_picker,
            CursorStyle::ContextMenu => &self.context_menu,
            CursorStyle::Copy => &self.copy,
            CursorStyle::Crosshair => &self.crosshair,
            CursorStyle::Default => &self.default,
            CursorStyle::HandClosed => &self.closed_hand,
            CursorStyle::HandClosedNoDrop => &self.closed_hand_no_drop,
            CursorStyle::ArrowDown => &self.arrow_down,
            CursorStyle::Draft => &self.draft,
            CursorStyle::Fleur => &self.fleur,
            CursorStyle::Help => &self.help,
            CursorStyle::ArrowLeft => &self.arrow_left,
            CursorStyle::SideLeft => &self.side_left,
            CursorStyle::NoDrop => &self.no_drop,
            CursorStyle::NotAllowed => &self.not_allowed,
            CursorStyle::HandOpen => &self.open_hand,
            CursorStyle::Pencil => &self.pencil,
            CursorStyle::Pirate => &self.pirate,
            CursorStyle::Pointer => &self.pointer,
            CursorStyle::ArrowRight => &self.arrow_right,
            CursorStyle::MirroredPointer => &self.mirrored_pointer,
            CursorStyle::SideRight => &self.side_right,
            CursorStyle::ResizeNESW => &self.size_nesw,
            CursorStyle::ResizeNWSE => &self.size_nwse,
            CursorStyle::SizeHor => &self.size_hor,
            CursorStyle::ResizeVertically => &self.size_ver,
            CursorStyle::Text => &self.text,
            CursorStyle::ArrowTopLeft => &self.arrow_top_left,
            CursorStyle::ArrowTopRight => &self.arrow_top_right,
            CursorStyle::SideTop => &self.side_top,
            CursorStyle::ArrowUp => &self.arrow_up,
            CursorStyle::VerticalText => &self.vertical_text,
            CursorStyle::ZoomIn => &self.zoom_in,
            CursorStyle::ZoomOut => &self.zoom_out,
        }
    }
}
/// Holds information any cursor would need
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseCursor {
    //file_path: &'static str,
    //colors: u8,
    hot_spot_x: u16,
    hot_spot_y: u16,
}

#[cfg(feature = "keycodes")]
/// Set the cursor of the current window
///
/// # Errors
/// See [Errors]
pub fn use_cursor(
    cursor: &Cursor,
    #[cfg(all(feature = "glfw", not(target_arch = "wasm32")))]
    glfw_window: Option<&mut glfw::Window>,
    #[cfg(not(all(feature = "glfw", not(target_arch = "wasm32"))))]
    _glfw_window: std::option::Option<NoneOnly>,
) -> Result<(), WindowError> {
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "glfw")]
    if let Some(additional_info) = glfw_window {
        #[allow(irrefutable_let_patterns)]
        if let Cursor::Glfw(new_cursor) = cursor {
            use crate::graphics::buffer_to_pixel_image;

            let given = new_cursor;
            let buffer = &given.0;
            let pixel: glfw::PixelImage = buffer_to_pixel_image(buffer);
            //println!("Cursor dimensions: {} x {}", given.1, given.2);

            // Check if dimensions make sense
            if given.1 == 0 || given.2 == 0 || given.1 > 1024 || given.2 > 1024
            {
                //println!("Error: Invalid cursor dimensions");
                return Err(WindowError::IncorrectSize(
                    (given.1, given.2).try_tuple_into().unwrap_or_default(),
                ));
            }
            let new = glfw::Cursor::create_from_pixels(pixel, given.1, given.2);

            additional_info.set_cursor(Some(new));
        } else {
            use crate::platform::frameworks::WindowError;

            return Err(WindowError::Misc(format!(
                "Cursor of type Cursor::Glfw expected but got {cursor:?} instead"
            )));
            //panic!("Cannot set GLFW cursor -> No cursors provided");
        }
        #[allow(unreachable_code)]
        return Ok(());
    }
    match cursor {
        #[cfg(target_os = "windows")]
        #[cfg(feature = "svg")]
        Cursor::Win(cursor) => {
            unsafe {
                // Update the passive cursor provider windows uses
                cursors_windows::update_cursor(cursor);
                // Update currently used cursor (Passive provider only gets checked when mouse moves)
                cursors_windows::set_cursor(cursor);
            }
        }

        #[cfg(target_os = "linux")]
        Cursor::X11(xcursor_id) => {
            return Err(WindowError::NotImplemented);
            // Use the X11 cursor ID
            panic!("X11 cursor id: {}", xcursor_id.unwrap());
        }

        #[cfg(target_os = "macos")]
        Cursor::Mac(_ptr) => {
            return Err(WindowError::NotImplemented);
            // Use macOS cursor pointer (e.g., NSCursor*)
            //panic!("macOS cursor pointer: {:?}", ptr.unwrap());
        }
        Cursor::Glfw(_) => {
            return Err(WindowError::Misc("Impossible".to_string()));
            //panic!("Cannot set GLFW cursor -> Not a GLFW context");
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}

// /// Converts the U2 into the actual cursor size, up to 255
// #[must_use]
// #[cfg(feature = "num_traits")]
// pub fn cursor_resolution(quality: U2) -> u8 {
//     let t: u32 = quality.into();
//     2u8.pow(t + 5)
// }
// /// Converts a desired resolution into U2
// ///
// /// # Errors
// /// When quality +1 isn't a power of two
// /// When resolution is bigger than 255
// pub fn resolution_to_quality(resolution: u8) -> Result<U2, &'static str> {
//     let val = u32::from(resolution) + 1;

//     if !val.is_power_of_two() {
//         return Err("Resolution + 1 is not a power of two");
//     }

//     let t =
//         val.trailing_zeros().checked_sub(5).ok_or("Resolution too small")?;

//     if t > 3 {
//         return Err("Resolution too large");
//     }

//     Ok(U2::new(t as u8))
// }

impl ButtonState {
    #[must_use]
    /// Create a new button state -> Pressed, clicked, and released are calculated
    pub const fn new(current: bool, last: bool) -> Self {
        Self {
            down: current,
            clicked: current && !last,
            released: !current && last,
        }
    }
    /// Update the current state
    pub const fn update(&mut self, new: bool) {
        self.clicked = !self.down && new;
        self.released = self.down && !new;
        self.down = new;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
/// The current state of a mouse buttons and if they have just been pressed
#[allow(missing_docs, clippy::struct_excessive_bools)]
pub struct ButtonState {
    pub down: bool,
    pub clicked: bool,
    pub released: bool,
}
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
/// The current state of the mouse buttons and if they have just been pressed
pub struct MouseButtonState {
    pub left: ButtonState,
    pub middle: ButtonState,
    pub right: ButtonState,
}
impl MouseButtonState {
    /// Updates the mouse state
    pub const fn update(
        &mut self,
        left_down: bool,
        middle_down: bool,
        right_down: bool,
    ) {
        self.left.update(left_down);
        self.middle.update(middle_down);
        self.right.update(right_down);
    }
}

// pub struct MousePos<T> {
//     pos: (T, T),
//     delta_pos: (T, T),
// }

// pub struct MouseData<T> {
//     buttons: MouseState,
//     pos: MousePos<T>,
// }
#[derive(Debug, Clone, PartialEq, Eq)]
/// Things that could go wrong while loading a cursor
pub enum LoadCursorError {
    /// Image data corrupt
    InvalidImageData(String),
    /// Unknown
    Misc(String),
    /// A tempfile could not be created
    UnableToCreateTempfile,
    /// A tempfile could not be removed
    UnableToDeleteTempfile,
    /// Os could not load the cursor even though the cursor data has been constructed
    OsError,
    /// When the hotspot is in an invalid position
    InvalidHotspot,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The resolution/size of the given cursor
pub enum CursorResolution {
    /// 16x16
    X16,
    /// 32x32
    X32,
    /// 64x64
    X64,
    /// 128x128
    X128,
    /// 256x256
    X256,
}
impl CursorResolution {
    #[must_use]
    /// Get the size of the cursor in pixels
    pub fn get_size<T: TryFromPatch<u8>>(&self) -> Option<T> {
        match self {
            Self::X16 => 15.try_into_value(),
            Self::X32 => 31.try_into_value(),
            Self::X64 => 63.try_into_value(),
            Self::X128 => 127.try_into_value(),
            Self::X256 => 255.try_into_value(),
        }
    }
}
