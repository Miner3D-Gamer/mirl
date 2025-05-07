#[cfg(target_os = "windows")]
use cursors_windows::load_base_cursor_with_file;

use crate::{graphics::RawImage, render::U2};

pub trait CursorManager {
    fn load_cursor(
        &mut self,
        name: &str,
        size: U2,
        image_data: RawImage,
        hotspot_x: u16,
        hotspot_y: u16,
    );
    fn get_cursor(&self, name: &str) -> Cursor;

    fn load_builtin_cursors(
        &mut self,
        size: U2,
        main_color: u32,
        secondary_color: u32,
    );
}
#[derive(Debug)]
pub enum Cursor {
    #[cfg(target_os = "windows")]
    Win(windows::Win32::UI::WindowsAndMessaging::HCURSOR),

    #[cfg(target_os = "linux")]
    X11(u64), // This could be a cursor ID from X11

    #[cfg(target_os = "macos")]
    Mac(*mut std::ffi::c_void), // Placeholder for NSCursor or equivalent
}
pub struct CursorData {
    raw_image_data: Vec<u32>,
}

#[cfg(target_os = "windows")]
pub mod cursors_windows;

pub struct Cursors {
    pub alias: Cursor,
    pub all_scroll: Cursor,
    pub bottom_left_corner: Cursor,
    pub bottom_right_corner: Cursor,
    pub bottom_side: Cursor,
    pub cell: Cursor,
    pub center_ptr: Cursor,
    pub col_resize: Cursor,
    pub color_picker: Cursor,
    pub context_menu: Cursor,
    pub copy: Cursor,
    pub crosshair: Cursor,
    pub default: Cursor,
    pub closed_hand: Cursor,
    pub closed_hand_no_drop: Cursor,
    pub down_arrow: Cursor,
    pub draft: Cursor,
    pub fleur: Cursor,
    pub help: Cursor,
    pub left_arrow: Cursor,
    pub left_side: Cursor,
    pub no_drop: Cursor,
    pub not_allowed: Cursor,
    pub open_hand: Cursor,
    pub pencil: Cursor,
    pub pirate: Cursor,
    pub pointer: Cursor,
    pub right_arrow: Cursor,
    pub right_ptr: Cursor,
    pub right_side: Cursor,
    pub row_resize: Cursor,
    pub size_bdiag: Cursor,
    pub size_fdiag: Cursor,
    pub size_hor: Cursor,
    pub size_ver: Cursor,
    pub text: Cursor,
    pub top_left_corner: Cursor,
    pub top_right_corner: Cursor,
    pub top_side: Cursor,
    pub up_arrow: Cursor,
    pub vertical_text: Cursor,
    pub zoom_in: Cursor,
    pub zoom_out: Cursor,
}

impl Cursors {
    pub fn load(size: U2, main_color: u32, secondary_color: u32) -> Self {
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
            center_ptr: load_base_cursor_with_file(
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
            down_arrow: load_base_cursor_with_file(
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
            left_arrow: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 4,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/left-arrow.svg").to_string(),
            ),
            left_side: load_base_cursor_with_file(
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
            right_arrow: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 28,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/right-arrow.svg").to_string(),
            ),
            right_ptr: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 28,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/right_ptr.svg").to_string(),
            ),
            right_side: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 28,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/right_side.svg").to_string(),
            ),
            row_resize: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/row-resize.svg").to_string(),
            ),
            size_bdiag: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/size_bdiag.svg").to_string(),
            ),
            size_fdiag: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 16,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/size_fdiag.svg").to_string(),
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
            top_left_corner: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 9,
                    hot_spot_y: 9,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/top_left_corner.svg").to_string(),
            ),
            top_right_corner: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 21,
                    hot_spot_y: 9,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/top_right_corner.svg").to_string(),
            ),
            top_side: load_base_cursor_with_file(
                BaseCursor {
                    hot_spot_x: 16,
                    hot_spot_y: 4,
                },
                size,
                main_color,
                secondary_color,
                include_str!("./svg/top_side.svg").to_string(),
            ),
            up_arrow: load_base_cursor_with_file(
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

pub struct BaseCursor {
    //file_path: &'static str,
    //colors: u8,
    hot_spot_x: i32,
    hot_spot_y: i32,
}

pub fn use_cursor(cursor: &Cursor) {
    match cursor {
        #[cfg(target_os = "windows")]
        Cursor::Win(hcursor) => {
            cursors_windows::set_cursor(*hcursor);
        }

        #[cfg(target_os = "linux")]
        Cursor::X11(xcursor_id) => {
            // Use the X11 cursor ID
            println!("X11 cursor id: {}", xcursor_id);
        }

        #[cfg(target_os = "macos")]
        Cursor::Mac(ptr) => {
            // Use macOS cursor pointer (e.g., NSCursor*)
            println!("macOS cursor pointer: {:?}", ptr);
        }
    }
}
