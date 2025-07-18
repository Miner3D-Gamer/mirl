#[cfg(target_os = "windows")]
pub use cursors_windows::load_base_cursor_with_file;

use crate::{graphics::RawImage};
use crate::extensions::*;
/// Cursor stuff of glfw bc glfw is a bitch
pub mod cursor_glfw;
/// The Cursor Manager provides a way of easily loading cursors based on a RawImage or the default cursors that come with this lib
pub trait CursorManager {
    /// Create a Cursor instance using a RawImage
    fn load_cursor(
        &mut self,
        name: &str,
        size: U2,
        image_data: RawImage,
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
#[derive(Debug)]
/// Cursor Instance holding the required cursor data to be used somewhere else
pub enum Cursor {
    /// Windows
    #[cfg(target_os = "windows")]
    Win(Option<windows::Win32::UI::WindowsAndMessaging::HCURSOR>),
    /// Linux using X11, wayland support will be added later
    #[cfg(target_os = "linux")]
    X11(Option<u64>), // This could be a cursor ID from X11
    /// Mac
    #[cfg(target_os = "macos")]
    Mac(Option<*mut std::ffi::c_void>), // Placeholder for NSCursor or equivalent
    /// glfw lib, cross-platform
    Glfw(Option<(RawImage, u32, u32)>),
}
// pub struct CursorData {
//     raw_image_data: Vec<u32>,
// }
/// Implementation for cursors on windows
#[cfg(target_os = "windows")]
pub mod cursors_windows;

/// Default cursors this lib provides
pub struct Cursors {
    /// Default Pointer
    pub alias: Cursor,
    /// Resize vertically + Resize horizontally
    pub all_scroll: Cursor,
    /// Arrow pointing to the bottom left ⬋
    pub bottom_left_corner: Cursor,
    /// Arrow pointing to the bottom right ⬊
    pub bottom_right_corner: Cursor,
    /// Arrow down with a _ at the end
    pub bottom_side: Cursor,
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

impl Cursors {
    /// Load all defaultly supported cursors
    pub fn load<F>(
        size: U2,
        main_color: u32,
        secondary_color: u32,
        load_base_cursor_with_file: F,
    ) -> Self
    where
        F: Fn(BaseCursor, U2, u32, u32, String) -> Cursor,
    {
        Self {
            default: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 6,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/default.svg").to_string(),
            ),
            alias: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 6,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/alias.svg").to_string(),
            ),
            all_scroll: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/all-scroll.svg").to_string(),
            ),
            bottom_left_corner: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 5,
                    hot_spot_y: 27,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/bottom_left_corner.svg").to_string(),
            ),
            bottom_right_corner: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 29,
                    hot_spot_y: 27,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/bottom_right_corner.svg").to_string(),
            ),
            bottom_side: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 28,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/bottom_side.svg").to_string(),
            ),
            cell: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/cell.svg").to_string(),
            ),
            centered_pointer: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 15,
                    hot_spot_y: 7,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/center_ptr.svg").to_string(),
            ),
            col_resize: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/col-resize.svg").to_string(),
            ),
            color_picker: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 4,
                    hot_spot_y: 29,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/color-picker.svg").to_string(),
            ),
            context_menu: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 6,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/context-menu.svg").to_string(),
            ),
            copy: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 6,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/copy.svg").to_string(),
            ),
            crosshair: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/crosshair.svg").to_string(),
            ),
            closed_hand: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/closed-hand.svg").to_string(),
            ),
            closed_hand_no_drop: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/closed-hand-no-drop.svg").to_string(),
            ),
            arrow_down: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 28,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/down-arrow.svg").to_string(),
            ),
            draft: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 4,
                    hot_spot_y: 29,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/draft.svg").to_string(),
            ),
            fleur: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/fleur.svg").to_string(),
            ),
            help: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 4,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/help.svg").to_string(),
            ),
            arrow_left: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 4,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/left-arrow.svg").to_string(),
            ),
            side_left: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 4,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/left_side.svg").to_string(),
            ),
            no_drop: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 6,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/no-drop.svg").to_string(),
            ),
            not_allowed: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/not-allowed.svg").to_string(),
            ),
            open_hand: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/open-hand.svg").to_string(),
            ),
            pencil: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 4,
                    hot_spot_y: 29,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/pencil.svg").to_string(),
            ),
            pirate: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/pirate.svg").to_string(),
            ),
            pointer: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 13,
                    hot_spot_y: 7,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/pointer.svg").to_string(),
            ),
            arrow_right: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 28,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/right-arrow.svg").to_string(),
            ),
            mirrored_pointer: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 28,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/right_ptr.svg").to_string(),
            ),
            side_right: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 28,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/right_side.svg").to_string(),
            ),
            size_nesw: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/size_nesw.svg").to_string(),
            ),
            size_nwse: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/size_nwse.svg").to_string(),
            ),
            size_hor: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/size_hor.svg").to_string(),
            ),
            size_ver: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/size_ver.svg").to_string(),
            ),
            text: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/text.svg").to_string(),
            ),
            arrow_top_left: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 9,
                    hot_spot_y: 9,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/top_left_corner.svg").to_string(),
            ),
            arrow_top_right: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 21,
                    hot_spot_y: 9,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/top_right_corner.svg").to_string(),
            ),
            side_top: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/top_side.svg").to_string(),
            ),
            arrow_up: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/up-arrow.svg").to_string(),
            ),
            vertical_text: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/vertical-text.svg").to_string(),
            ),
            zoom_in: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/zoom-in.svg").to_string(),
            ),
            zoom_out: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/zoom-out.svg").to_string(),
            ),
        }
    }
}

/// Holds information any cursor would need
pub struct BaseCursor {
    //file_path: &'static str,
    //colors: u8,
    hot_spot_x: i32,
    hot_spot_y: i32,
}
/// Set the cursor of the current window
pub fn use_cursor(cursor: &Cursor, glfw_window: Option<&mut glfw::Window>) {
    if let Some(additional_info) = glfw_window {
        match cursor {
            Cursor::Glfw(new_cursor) => {
                let given = new_cursor.as_ref().unwrap().clone();
                let pixel = given.0.into();
                // Debug output for dimensions
                println!("Cursor dimensions: {} x {}", given.1, given.2);

                // Check if dimensions make sense
                if given.1 == 0
                    || given.2 == 0
                    || given.1 > 1024
                    || given.2 > 1024
                {
                    println!("Error: Invalid cursor dimensions");
                    return;
                }
                let new =
                    glfw::Cursor::create_from_pixels(pixel, given.1, given.2);

                additional_info.set_cursor(Some(new));
            }
            _ => {
                panic!("Cannot set GLFW cursor -> No cursors provided");
            }
        }
    }
    match cursor {
        #[cfg(target_os = "windows")]
        Cursor::Win(hcursor) => {
            let t = hcursor.as_ref().unwrap();
            cursors_windows::set_cursor(*t);
        }

        #[cfg(target_os = "linux")]
        Cursor::X11(xcursor_id) => {
            // Use the X11 cursor ID
            println!("X11 cursor id: {}", xcursor_id.unwrap());
        }

        #[cfg(target_os = "macos")]
        Cursor::Mac(ptr) => {
            // Use macOS cursor pointer (e.g., NSCursor*)
            println!("macOS cursor pointer: {:?}", ptr.unwrap());
        }
        Cursor::Glfw(_) => {
            panic!("Cannot set GLFW cursor -> Not a GLFW context");
        }
    }
}
/// Converts the U2 into the actual cursor size, up to 255
pub fn cursor_resolution(quality: U2) -> u8 {
    let t: u32 = quality.into();
    2u8.pow(t + 5) - 1
}
/// Converts a desired resolution into U2
pub fn resolution_to_quality(resolution: u8) -> Result<U2, &'static str> {
    let val = resolution as u32 + 1;

    if !val.is_power_of_two() {
        return Err("Resolution + 1 is not a power of two");
    }

    let t = val.trailing_zeros().checked_sub(5).ok_or("Resolution too small")?;

    if t > 3 {
        return Err("Resolution too large");
    }

    Ok(U2::new(t as u8))
}