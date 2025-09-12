#[cfg(all(feature = "system", target_os = "windows"))]
pub use cursors_windows::load_base_cursor_with_file;

use crate::{
    extensions::*,
    //misc::copyable_list::VariableSizeList,
    platform::{framework_traits::Errors, Buffer, CursorStyle},
};

// Damn past me, why so sassy?
/// Cursor stuff of glfw bc glfw is a bitch
pub mod cursor_glfw;
/// The Cursor Manager provides a way of easily loading cursors based on a Buffer or the default cursors that come with this lib
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

// pub struct CursorData {
//     buffer_data: Vec<u32>,
// }
/// Implementation for cursors on windows
#[cfg(target_os = "windows")]
pub mod cursors_windows;

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

impl Cursors {
    #[allow(clippy::too_many_lines)]
    /// Load all defaultly supported cursors
    ///
    /// # Errors
    /// When a cursor cannot be loaded, it returns the name of the cursor that failed to load
    pub fn load<F>(
        size: U2,
        main_color: u32,
        secondary_color: u32,
        load_base_cursor_with_file: F,
    ) -> Result<Self, String>
    where
        F: Fn(BaseCursor, U2, u32, u32, String) -> Option<Cursor>,
    {
        let default = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 5,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/default.svg").to_string(),
        )
        .ok_or_else(|| "default.svg".to_string())?;

        let alias = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 6,
                hot_spot_y: 4,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/alias.svg").to_string(),
        )
        .ok_or_else(|| "alias.svg".to_string())?;

        let all_scroll = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/all-scroll.svg").to_string(),
        )
        .ok_or_else(|| "all-scroll.svg".to_string())?;

        let bottom_left_corner = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 5,
                hot_spot_y: 27,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/bottom_left_corner.svg").to_string(),
        )
        .ok_or_else(|| "bottom_left_corner.svg".to_string())?;

        let arrow_bottom_right = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 29,
                hot_spot_y: 27,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/bottom_right_corner.svg").to_string(),
        )
        .ok_or_else(|| "bottom_right_corner.svg".to_string())?;

        let side_bottom = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 28,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/bottom_side.svg").to_string(),
        )
        .ok_or_else(|| "bottom_side.svg".to_string())?;

        let cell = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/cell.svg").to_string(),
        )
        .ok_or_else(|| "cell.svg".to_string())?;

        let centered_pointer = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 15,
                hot_spot_y: 7,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/center_ptr.svg").to_string(),
        )
        .ok_or_else(|| "center_ptr.svg".to_string())?;

        let col_resize = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/col-resize.svg").to_string(),
        )
        .ok_or_else(|| "col-resize.svg".to_string())?;

        let color_picker = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 29,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/color-picker.svg").to_string(),
        )
        .ok_or_else(|| "color-picker.svg".to_string())?;

        let context_menu = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 6,
                hot_spot_y: 4,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/context-menu.svg").to_string(),
        )
        .ok_or_else(|| "context-menu.svg".to_string())?;

        let copy = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 6,
                hot_spot_y: 4,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/copy.svg").to_string(),
        )
        .ok_or_else(|| "copy.svg".to_string())?;

        let crosshair = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/crosshair.svg").to_string(),
        )
        .ok_or_else(|| "crosshair.svg".to_string())?;

        let closed_hand = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/closed-hand.svg").to_string(),
        )
        .ok_or_else(|| "closed-hand.svg".to_string())?;

        let closed_hand_no_drop = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/closed-hand-no-drop.svg").to_string(),
        )
        .ok_or_else(|| "closed-hand-no-drop.svg".to_string())?;

        let arrow_down = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 28,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/down-arrow.svg").to_string(),
        )
        .ok_or_else(|| "down-arrow.svg".to_string())?;

        let draft = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 29,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/draft.svg").to_string(),
        )
        .ok_or_else(|| "draft.svg".to_string())?;

        let fleur = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/fleur.svg").to_string(),
        )
        .ok_or_else(|| "fleur.svg".to_string())?;

        let help = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 4,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/help.svg").to_string(),
        )
        .ok_or_else(|| "help.svg".to_string())?;

        let arrow_left = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/left-arrow.svg").to_string(),
        )
        .ok_or_else(|| "left-arrow.svg".to_string())?;

        let side_left = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/left_side.svg").to_string(),
        )
        .ok_or_else(|| "left_side.svg".to_string())?;

        let no_drop = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 6,
                hot_spot_y: 4,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/no-drop.svg").to_string(),
        )
        .ok_or_else(|| "no-drop.svg".to_string())?;

        let not_allowed = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/not-allowed.svg").to_string(),
        )
        .ok_or_else(|| "not-allowed.svg".to_string())?;

        let open_hand = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/open-hand.svg").to_string(),
        )
        .ok_or_else(|| "open-hand.svg".to_string())?;

        let pencil = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 4,
                hot_spot_y: 29,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/pencil.svg").to_string(),
        )
        .ok_or_else(|| "pencil.svg".to_string())?;

        let pirate = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/pirate.svg").to_string(),
        )
        .ok_or_else(|| "pirate.svg".to_string())?;

        let pointer = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 13,
                hot_spot_y: 7,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/pointer.svg").to_string(),
        )
        .ok_or_else(|| "pointer.svg".to_string())?;

        let arrow_right = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 28,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/right-arrow.svg").to_string(),
        )
        .ok_or_else(|| "right-arrow.svg".to_string())?;

        let mirrored_pointer = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 28,
                hot_spot_y: 4,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/right_ptr.svg").to_string(),
        )
        .ok_or_else(|| "right_ptr.svg".to_string())?;

        let side_right = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 28,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/right_side.svg").to_string(),
        )
        .ok_or_else(|| "right_side.svg".to_string())?;

        let size_nesw = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/size_nesw.svg").to_string(),
        )
        .ok_or_else(|| "size_nesw.svg".to_string())?;

        let size_nwse = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/size_nwse.svg").to_string(),
        )
        .ok_or_else(|| "size_nwse.svg".to_string())?;

        let size_hor = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/size_hor.svg").to_string(),
        )
        .ok_or_else(|| "size_hor.svg".to_string())?;

        let size_ver = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/size_ver.svg").to_string(),
        )
        .ok_or_else(|| "size_ver.svg".to_string())?;

        let text = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/text.svg").to_string(),
        )
        .ok_or_else(|| "text.svg".to_string())?;

        let arrow_top_left = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 9,
                hot_spot_y: 9,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/top_left_corner.svg").to_string(),
        )
        .ok_or_else(|| "top_left_corner.svg".to_string())?;

        let arrow_top_right = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 21,
                hot_spot_y: 9,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/top_right_corner.svg").to_string(),
        )
        .ok_or_else(|| "top_right_corner.svg".to_string())?;

        let side_top = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 4,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/top_side.svg").to_string(),
        )
        .ok_or_else(|| "top_side.svg".to_string())?;

        let arrow_up = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 4,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/up-arrow.svg").to_string(),
        )
        .ok_or_else(|| "up-arrow.svg".to_string())?;

        let vertical_text = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/vertical-text.svg").to_string(),
        )
        .ok_or_else(|| "vertical-text.svg".to_string())?;

        let zoom_in = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/zoom-in.svg").to_string(),
        )
        .ok_or_else(|| "zoom-in.svg".to_string())?;

        let zoom_out = load_base_cursor_with_file(
            BaseCursor {
                hot_spot_x: 16,
                hot_spot_y: 16,
            },
            size,
            main_color,
            secondary_color,
            include_str!("./svg/zoom-out.svg").to_string(),
        )
        .ok_or_else(|| "zoom-out.svg".to_string())?;

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
            CursorStyle::ClosedHand => &self.closed_hand,
            CursorStyle::ClosedHandNoDrop => &self.closed_hand_no_drop,
            CursorStyle::ArrowDown => &self.arrow_down,
            CursorStyle::Draft => &self.draft,
            CursorStyle::Fleur => &self.fleur,
            CursorStyle::Help => &self.help,
            CursorStyle::ArrowLeft => &self.arrow_left,
            CursorStyle::SideLeft => &self.side_left,
            CursorStyle::NoDrop => &self.no_drop,
            CursorStyle::NotAllowed => &self.not_allowed,
            CursorStyle::OpenHand => &self.open_hand,
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
#[must_use]
/// Set the cursor of the current window
pub fn use_cursor(
    cursor: &Cursor,
    #[cfg(all(feature = "glfw_backend", not(target_arch = "wasm32")))]
    glfw_window: Option<&mut glfw::Window>,
    #[cfg(not(all(feature = "glfw_backend", not(target_arch = "wasm32"))))]
    _glfw_window: std::option::Option<NoneOnly>,
) -> Errors {
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "glfw_backend")]
    if let Some(additional_info) = glfw_window {
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
                return Errors::IncorrectSize((1, 1));
            }
            let new = glfw::Cursor::create_from_pixels(pixel, given.1, given.2);

            additional_info.set_cursor(Some(new));
        } else {
            use crate::platform::framework_traits::Errors;

            return Errors::Unknown;
            //panic!("Cannot set GLFW cursor -> No cursors provided");
        }
        return Errors::AllGood;
    }
    match cursor {
        #[cfg(target_os = "windows")]
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
            return Errors::NotImplemented;
            // Use the X11 cursor ID
            panic!("X11 cursor id: {}", xcursor_id.unwrap());
        }

        #[cfg(target_os = "macos")]
        Cursor::Mac(ptr) => {
            return Errors::NotImplemented;
            // Use macOS cursor pointer (e.g., NSCursor*)
            //panic!("macOS cursor pointer: {:?}", ptr.unwrap());
        }
        Cursor::Glfw(_) => {
            return Errors::Unknown;
            //panic!("Cannot set GLFW cursor -> Not a GLFW context");
        }
    }
    Errors::AllGood
}

/// Converts the U2 into the actual cursor size, up to 255
#[must_use]
pub fn cursor_resolution(quality: U2) -> u8 {
    let t: u32 = quality.into();
    2u8.pow(t + 5)
}
/// Converts a desired resolution into U2
///
/// # Errors
/// When quality +1 isn't a power of two
/// When resolution is bigger than 255
pub fn resolution_to_quality(resolution: u8) -> Result<U2, &'static str> {
    let val = u32::from(resolution) + 1;

    if !val.is_power_of_two() {
        return Err("Resolution + 1 is not a power of two");
    }

    let t =
        val.trailing_zeros().checked_sub(5).ok_or("Resolution too small")?;

    if t > 3 {
        return Err("Resolution too large");
    }

    Ok(U2::new(t as u8))
}
/// Mouse position stuff like raw mouse input
pub mod position;
