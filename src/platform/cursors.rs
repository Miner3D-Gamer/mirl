use std::collections::HashMap;
use windows::{
    core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*,
};
#[derive(Default)]
pub struct CursorManager {
    pub cursors: HashMap<String, HCURSOR>,
}
impl CursorManager {
    pub fn new() -> Self {
        CursorManager {
            cursors: HashMap::new(),
        }
    }
    pub fn load_cursor(&mut self, name: &str, file_path: &str) {
        self.cursors.insert(name.to_string(), load_cursor(file_path));
    }
    pub fn get_cursor(&self, name: &str) -> HCURSOR {
        *self.cursors.get(name).unwrap()
    }
}
/// Expects .cur file
fn load_cursor(file_path: &str) -> HCURSOR {
    unsafe {
        let filename = std::ffi::CString::new(file_path).unwrap(); // null-terminated C-style byte string
        let cursor =
            LoadCursorFromFileA(PCSTR(filename.as_ptr() as *const u8)).unwrap();
        if cursor.0 == 0 {
            panic!("Failed to load cursor");
        }
        return cursor;
    }
}

// struct Cursors {
//     alias: HCURSOR,
//     all_scroll: HCURSOR,
//     bottom_left_corner: HCURSOR,
//     bottom_right_corner: HCURSOR,
//     bottom_side: HCURSOR,
//     cell: HCURSOR,
//     center_ptr: HCURSOR,
//     col_resize: HCURSOR,
//     color_picker: HCURSOR,
//     context_menu: HCURSOR,
//     copy: HCURSOR,
//     crosshair: HCURSOR,
//     default: HCURSOR,
//     closed_hand: HCURSOR,
//     closed_hand_no_drop: HCURSOR,
//     down_arrow: HCURSOR,
//     draft: HCURSOR,
//     fleur: HCURSOR,
//     help: HCURSOR,
//     left_side: HCURSOR,
//     left_arrow: HCURSOR,
//     no_drop: HCURSOR,
//     not_allowed: HCURSOR,
//     pointer: HCURSOR,
//     loading_progress_0: HCURSOR,
//     loading_progress_1: HCURSOR,
//     loading_progress_2: HCURSOR,
//     loading_progress_3: HCURSOR,
//     loading_progress_4: HCURSOR,
//     loading_progress_5: HCURSOR,
//     loading_progress_6: HCURSOR,
//     loading_progress_7: HCURSOR,
//     loading_progress_8: HCURSOR,
//     loading_progress_9: HCURSOR,
//     loading_progress_10: HCURSOR,
//     loading_progress_11: HCURSOR,
//     loading_progress_12: HCURSOR,
//     loading_progress_13: HCURSOR,
//     loading_progress_14: HCURSOR,
//     loading_progress_15: HCURSOR,
//     loading_progress_16: HCURSOR,
//     loading_progress_17: HCURSOR,
//     loading_progress_18: HCURSOR,
//     loading_progress_19: HCURSOR,
//     loading_progress_20: HCURSOR,
//     loading_progress_21: HCURSOR,
//     loading_progress_22: HCURSOR,
//     loading_progress_23: HCURSOR,
//     loading_progress_24: HCURSOR,
//     right_ptr: HCURSOR,
//     right_side: HCURSOR,
//     right_arrow: HCURSOR,
//     row_resize: HCURSOR,
//     size_bdiag: HCURSOR,
//     size_fdiag: HCURSOR,
//     size_hor: HCURSOR,
//     size_ver: HCURSOR,
//     text: HCURSOR,
//     top_left_corner: HCURSOR,
//     top_right_corner: HCURSOR,
//     top_side: HCURSOR,
//     vertical_text: HCURSOR,
//     wait_progress_0: HCURSOR,
//     wait_progress_1: HCURSOR,
//     wait_progress_2: HCURSOR,
//     wait_progress_3: HCURSOR,
//     wait_progress_4: HCURSOR,
//     wait_progress_5: HCURSOR,
//     wait_progress_6: HCURSOR,
//     wait_progress_7: HCURSOR,
//     wait_progress_8: HCURSOR,
//     wait_progress_9: HCURSOR,
//     wait_progress_10: HCURSOR,
//     wait_progress_11: HCURSOR,
//     wait_progress_12: HCURSOR,
//     wait_progress_13: HCURSOR,
//     wait_progress_14: HCURSOR,
//     wait_progress_15: HCURSOR,
//     wait_progress_16: HCURSOR,
//     wait_progress_17: HCURSOR,
//     wait_progress_18: HCURSOR,
//     wait_progress_19: HCURSOR,
//     wait_progress_20: HCURSOR,
//     wait_progress_21: HCURSOR,
//     wait_progress_22: HCURSOR,
//     wait_progress_23: HCURSOR,
//     wait_progress_24: HCURSOR,
//     zoom_in: HCURSOR,
//     zoom_out: HCURSOR,
// }
