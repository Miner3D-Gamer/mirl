use crate::{
    graphics::{
        pixmap_to_raw_image, rasterize_svg, u32_to_hex, u32_to_rgba, RawImage,
    },
    render::{StringExtensions, U2},
};
use std::collections::HashMap;
use std::io::Write;
use windows::{
    core::*, //Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use super::{BaseCursor, Cursor};

#[derive(Default)]
pub struct CursorManagerWindows {
    pub cursors: HashMap<String, HCURSOR>,
}
pub struct CursorWindows {
    pub cursor: HCURSOR,
}
fn cursor_resolution(quality: U2) -> u8 {
    let t: u32 = quality.into();
    2u8.pow(t + 5)
}

impl CursorManagerWindows {
    fn new() -> Self {
        CursorManagerWindows {
            cursors: HashMap::new(),
        }
    }
}

// impl CursorManager for CursorManagerWindows {
//     fn get_cursor(&self, name: &str) -> Box<dyn Cursor> {
//         Box::new(CursorWindows {
//             cursor: *self.cursors.get(name).unwrap(),
//         })
//     }
//     fn load_builtin_cursors(
//         &mut self,
//         size: U2,
//         main_color: u32,
//         secondary_color: u32,
//     ) {
//         let cursors = [
//             BaseCursor {
//                 file_path: "alias.svg",
//                 colors: 2,
//                 hot_spot_x: 6,
//                 hot_spot_y: 4,
//             },
//             BaseCursor {
//                 file_path: "all-scroll.svg",
//                 colors: 1,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "bottom_left_corner.svg",
//                 colors: 0,
//                 hot_spot_x: 5,
//                 hot_spot_y: 27,
//             },
//             BaseCursor {
//                 file_path: "bottom_right_corner.svg",
//                 colors: 0,
//                 hot_spot_x: 29,
//                 hot_spot_y: 27,
//             },
//             BaseCursor {
//                 file_path: "bottom_side.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 28,
//             },
//             BaseCursor {
//                 file_path: "cell.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "center_ptr.svg",
//                 colors: 1,
//                 hot_spot_x: 15,
//                 hot_spot_y: 7,
//             },
//             BaseCursor {
//                 file_path: "col-resize.svg",
//                 colors: 1,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "color-picker.svg",
//                 colors: 0,
//                 hot_spot_x: 4,
//                 hot_spot_y: 29,
//             },
//             BaseCursor {
//                 file_path: "context-menu.svg",
//                 colors: 2,
//                 hot_spot_x: 6,
//                 hot_spot_y: 4,
//             },
//             BaseCursor {
//                 file_path: "copy.svg",
//                 colors: 2,
//                 hot_spot_x: 6,
//                 hot_spot_y: 4,
//             },
//             BaseCursor {
//                 file_path: "crosshair.svg",
//                 colors: 1,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "default.svg",
//                 colors: 1,
//                 hot_spot_x: 6,
//                 hot_spot_y: 4,
//             },
//             BaseCursor {
//                 file_path: "closed-hand-move.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "closed-hand-no-drop.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "down-arrow.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 28,
//             },
//             BaseCursor {
//                 file_path: "draft.svg",
//                 colors: 0,
//                 hot_spot_x: 4,
//                 hot_spot_y: 29,
//             },
//             BaseCursor {
//                 file_path: "fleur.svg",
//                 colors: 1,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "help.svg",
//                 colors: 2,
//                 hot_spot_x: 4,
//                 hot_spot_y: 4,
//             },
//             BaseCursor {
//                 file_path: "left-arrow.svg",
//                 colors: 0,
//                 hot_spot_x: 4,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "left_side.svg",
//                 colors: 0,
//                 hot_spot_x: 4,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "no-drop.svg",
//                 colors: 1,
//                 hot_spot_x: 6,
//                 hot_spot_y: 4,
//             },
//             BaseCursor {
//                 file_path: "not-allowed.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "open-hand.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "pencil.svg",
//                 colors: 1,
//                 hot_spot_x: 4,
//                 hot_spot_y: 29,
//             },
//             BaseCursor {
//                 file_path: "pirate.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "pointer.svg",
//                 colors: 0,
//                 hot_spot_x: 13,
//                 hot_spot_y: 7,
//             },
//             // BaseCursor {
//             //     file_path: "progress-01.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-02.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-03.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-04.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-05.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-06.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-07.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-08.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-09.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-10.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-11.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-12.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-13.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-14.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-15.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-16.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-17.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-18.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-19.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-20.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-21.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-22.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-23.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             // BaseCursor {
//             //     file_path: "progress-00.svg",
//             //     colors: 0,
//             //     hot_spot_x: 6,
//             //     hot_spot_y: 4,
//             // },
//             BaseCursor {
//                 file_path: "right-arrow.svg",
//                 colors: 0,
//                 hot_spot_x: 28,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "right_ptr.svg",
//                 colors: 1,
//                 hot_spot_x: 28,
//                 hot_spot_y: 4,
//             },
//             BaseCursor {
//                 file_path: "right_side.svg",
//                 colors: 0,
//                 hot_spot_x: 28,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "row-resize.svg",
//                 colors: 1,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "size_bdiag.svg",
//                 colors: 1,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "size_fdiag.svg",
//                 colors: 1,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "size_hor.svg",
//                 colors: 1,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "size_ver.svg",
//                 colors: 1,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "text.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "top_left_corner.svg",
//                 colors: 0,
//                 hot_spot_x: 9,
//                 hot_spot_y: 9,
//             },
//             BaseCursor {
//                 file_path: "top_right_corner.svg",
//                 colors: 0,
//                 hot_spot_x: 21,
//                 hot_spot_y: 9,
//             },
//             BaseCursor {
//                 file_path: "top_side.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 4,
//             },
//             BaseCursor {
//                 file_path: "up-arrow.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 4,
//             },
//             BaseCursor {
//                 file_path: "vertical-text.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             // BaseCursor {
//             //     file_path: "wait-01.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-02.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-03.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-04.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-05.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-06.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-07.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-08.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-09.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-10.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-11.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-12.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-13.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-14.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-15.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-16.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-17.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-18.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-19.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-20.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-21.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-22.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-23.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             // BaseCursor {
//             //     file_path: "wait-00.svg",
//             //     colors: 0,
//             //     hot_spot_x: 16,
//             //     hot_spot_y: 16,
//             // },
//             BaseCursor {
//                 file_path: "zoom-in.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//             BaseCursor {
//                 file_path: "zoom-out.svg",
//                 colors: 0,
//                 hot_spot_x: 16,
//                 hot_spot_y: 16,
//             },
//         ];

//         let expected_size = 24; // WHO TF MAKES THE CURSOR NOT A MULTIPLE OF 16 ???

//         let wanted_size = cursor_resolution(size);

//         for cursor in cursors {
//             let path = get_cursor_path(&cursor.file_path.to_string());
//             let svg_data = std::fs::read_to_string(path).unwrap();
//             // if svg has one {}, inser main_color, if svg has two {}, insert main_color, secondary_color

//             let result_svg = svg_data
//                 .replace_first_occurrence("{}", &u32_to_hex(main_color))
//                 .replace_first_occurrence("{}", &u32_to_hex(secondary_color));

//             let image_data = rasterize_svg(
//                 &result_svg.as_bytes(),
//                 wanted_size as u32,
//                 wanted_size as u32,
//             );

//             // Adjust hotspot because of the psycho who made the cursor not a multiple of 16
//             let adjusted_hotspot_x = ((cursor.hot_spot_x as f64
//                 / expected_size as f64)
//                 * wanted_size as f64)
//                 .round() as u16;
//             let adjusted_hotspot_y = ((cursor.hot_spot_y as f64
//                 / expected_size as f64)
//                 * wanted_size as f64)
//                 .round() as u16;

//             self.load_cursor(
//                 &extract_file_name_without_extension(&cursor.file_path),
//                 size,
//                 pixmap_to_raw_image(&image_data),
//                 adjusted_hotspot_x,
//                 adjusted_hotspot_y,
//             );
//         }
//     }
//     fn load_cursor(
//         &mut self,
//         name: &str,
//         size: U2,
//         image_data: RawImage,
//         hotspot_x: u16,
//         hotspot_y: u16,
//     ) {
//         self.cursors.insert(
//             name.to_string(),
//             load_cursor(size, image_data, hotspot_x, hotspot_y),
//         );
//     }
// }

fn load_cursor(
    size: U2,
    image_data: RawImage,
    hotspot_x: u16,
    hotspot_y: u16,
) -> HCURSOR {
    let size = cursor_resolution(size);
    let (file_path, _file) = create_temp_file(create_cursor(
        size,
        size,
        hotspot_x,
        hotspot_y,
        &image_data,
    ));
    delete_temp_file(&file_path).unwrap();
    return load_cursor_file(&file_path);
}

// fn load_base_cursor(
//     cursor: BaseCursor,
//     size: U2,
//     main_color: u32,
//     secondary_color: u32,
// ) -> windows::Win32::UI::WindowsAndMessaging::HCURSOR {
//     let expected_size = 24; // WHO TF MAKES THE CURSOR NOT A MULTIPLE OF 16 ???

//     let wanted_size = cursor_resolution(size);

//     let path = get_cursor_path(&cursor.file_path.to_string());
//     let svg_data = std::fs::read_to_string(path).unwrap();
//     // if svg has one {}, inser main_color, if svg has two {}, insert main_color, secondary_color

//     let result_svg = svg_data
//         .replace_first_occurrence("{}", &u32_to_hex(main_color))
//         .replace_first_occurrence("{}", &u32_to_hex(secondary_color));

//     let image_data = rasterize_svg(
//         &result_svg.as_bytes(),
//         wanted_size as u32,
//         wanted_size as u32,
//     );

//     // Adjust hotspot because of the psycho who made the cursor not a multiple of 16
//     let adjusted_hotspot_x = ((cursor.hot_spot_x as f64 / expected_size as f64)
//         * wanted_size as f64)
//         .round() as u16;
//     let adjusted_hotspot_y = ((cursor.hot_spot_y as f64 / expected_size as f64)
//         * wanted_size as f64)
//         .round() as u16;

//     return load_cursor(
//         //&extract_file_name_without_extension(&cursor.file_path),
//         size,
//         pixmap_to_raw_image(&image_data),
//         adjusted_hotspot_x,
//         adjusted_hotspot_y,
//     );
// }

pub fn load_base_cursor_with_file(
    cursor: BaseCursor,
    size: U2,
    main_color: u32,
    secondary_color: u32,
    svg_data: String,
) -> Cursor {
    let expected_size = 24; // WHO TF MAKES THE CURSOR NOT A MULTIPLE OF 16 ???

    let wanted_size = cursor_resolution(size);

    //let path = get_cursor_path(&cursor.file_path.to_string());
    //let svg_data = std::fs::read_to_string(path).unwrap();
    // if svg has one {}, inser main_color, if svg has two {}, insert main_color, secondary_color

    let result_svg = svg_data
        .replace_first_occurrence("{}", &u32_to_hex(main_color))
        .replace_first_occurrence("{}", &u32_to_hex(secondary_color));

    let image_data = rasterize_svg(
        &result_svg.as_bytes(),
        wanted_size as u32,
        wanted_size as u32,
    );

    // Adjust hotspot because of the psycho who made the cursor not a multiple of 16
    let adjusted_hotspot_x = ((cursor.hot_spot_x as f64 / expected_size as f64)
        * wanted_size as f64)
        .round() as u16;
    let adjusted_hotspot_y = ((cursor.hot_spot_y as f64 / expected_size as f64)
        * wanted_size as f64)
        .round() as u16;

    return Cursor::Win(load_cursor(
        //&extract_file_name_without_extension(&cursor.file_path),
        size,
        pixmap_to_raw_image(&image_data),
        adjusted_hotspot_x,
        adjusted_hotspot_y,
    ));
}

/// Expects .cur file
fn load_cursor_file(file_path: &str) -> HCURSOR {
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
fn create_cursor(
    width: u8,
    height: u8,
    hotspot_x: u16,
    hotspot_y: u16,
    rgba: &RawImage,
) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    // ICONDIR (6 bytes)
    buffer.extend(&[0x00, 0x00]); // Reserved
    buffer.extend(&[0x02, 0x00]); // Image type (2 = cursor)
    buffer.extend(&[0x01, 0x00]); // Number of images

    // ICONDIRENTRY (16 bytes)
    buffer.push(width); // Width
    buffer.push(height); // Height
    buffer.push(0); // Color count
    buffer.push(0); // Reserved
    buffer.extend(&hotspot_x.to_le_bytes()); // Hotspot X
    buffer.extend(&hotspot_y.to_le_bytes()); // Hotspot Y

    let image_data_offset = 6 + 16;
    let row_stride = ((width as u32 * 32 + 31) / 32) * 4;
    let pixel_array_size = row_stride * height as u32;
    let bmp_header_size = 40;
    let and_mask_size = height as u32 * ((width as u32 + 31) / 32 * 4);
    let size_in_bytes = bmp_header_size + pixel_array_size + and_mask_size;

    buffer.extend(&(size_in_bytes as u32).to_le_bytes()); // Image size
    buffer.extend(&(image_data_offset as u32).to_le_bytes()); // Image offset

    // BITMAPINFOHEADER (40 bytes)
    let mut bmp_data: Vec<u8> = Vec::with_capacity(size_in_bytes as usize);
    bmp_data.extend(&(40u32.to_le_bytes())); // Header size
    bmp_data.extend(&(width as i32).to_le_bytes()); // Width
    bmp_data.extend(&(2 * height as i32).to_le_bytes()); // Height (x2 for AND mask)
    bmp_data.extend(&(1u16.to_le_bytes())); // Planes
    bmp_data.extend(&(32u16.to_le_bytes())); // Bit count
    bmp_data.extend(&(0u32.to_le_bytes())); // Compression
    bmp_data.extend(&(0u32.to_le_bytes())); // Image size
    bmp_data.extend(&(0u32.to_le_bytes())); // X pixels per meter
    bmp_data.extend(&(0u32.to_le_bytes())); // Y pixels per meter
    bmp_data.extend(&(0u32.to_le_bytes())); // Colors used
    bmp_data.extend(&(0u32.to_le_bytes())); // Important colors

    for pixel in rgba.data.iter() {
        let (r, g, b, a) = u32_to_rgba(*pixel);
        bmp_data.extend(&[b, g, r, a]);
    }

    // // Pixel data (BGRA format, bottom-up)
    // for y in (0..height).rev() {
    //     for x in 0..width {
    //         let pixel = rgba.get_pixel(x as u32, y as u32).0;
    //         bmp_data.extend(&[pixel[2], pixel[1], pixel[0], pixel[3]]); // BGRA
    //     }
    // }

    // AND mask (all zero = fully visible)
    bmp_data.extend(vec![0u8; and_mask_size as usize]);

    // Combine all into buffer
    buffer.extend(bmp_data);

    return buffer;
}

// const BUILTIN_CURSORS: [BaseCursor; 1] = [BaseCursor {
//     file_path: "arrow.cur",
//     colors: 2,
//     hot_spot_x: 0,
//     hot_spot_y: 0,
// }];

fn get_cursor_path(name: &str) -> String {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src");
    path.push("platform");
    path.push("cursors");
    path.push("svg");
    path.push(name);

    return path.to_str().unwrap().to_string();
}

fn create_temp_file(cursor_data: Vec<u8>) -> (String, std::fs::File) {
    let mut file = tempfile::NamedTempFile::new().unwrap();

    let _ = file.write_all(&cursor_data);
    let path = file.path().to_str().unwrap().to_string();
    //let path_bytes = Vec::from(path.as_bytes());
    let file = file.keep().unwrap(); // This makes other parts have access to the file when using the path

    return (path, file.0);
}

fn delete_temp_file(path: &str) -> std::io::Result<()> {
    return std::fs::remove_file(path); // Removes the file at the given path
}

pub fn set_cursor(cursor: HCURSOR) {
    unsafe {
        SetCursor(cursor);
    }
}
// pub fn set_cursor_style(cursor: CursorStyle, cursors: &Cursors) {
//     unsafe {
//         SetCursor(*custom_map_cursor_style(cursor, cursors));
//     }
// }

// const fn custom_map_cursor_style(
//     style: CursorStyle,
//     cursors: &Cursors,
// ) -> &HCURSOR {
//     match style {
//         CursorStyle::Default => &cursors.default,
//         CursorStyle::Alias => &cursors.alias,
//         CursorStyle::AllScroll => &cursors.all_scroll,
//         CursorStyle::BottomLeftCorner => &cursors.bottom_left_corner,
//         CursorStyle::BottomRightCorner => &cursors.bottom_right_corner,
//         CursorStyle::BottomSide => &cursors.bottom_side,
//         CursorStyle::Cell => &cursors.cell,
//         CursorStyle::CenterPtr => &cursors.center_ptr,
//         CursorStyle::ColResize => &cursors.col_resize,
//         CursorStyle::ColorPicker => &cursors.color_picker,
//         CursorStyle::ContextMenu => &cursors.context_menu,
//         CursorStyle::Copy => &cursors.copy,
//         CursorStyle::Crosshair => &cursors.crosshair,
//         CursorStyle::ClosedHand => &cursors.dnd_move,
//         CursorStyle::ClosedHandNoDrop => &cursors.dnd_no_drop,
//         CursorStyle::DownArrow => &cursors.down_arrow,
//         CursorStyle::Draft => &cursors.draft,
//         CursorStyle::Fleur => &cursors.fleur,
//         CursorStyle::Help => &cursors.help,
//         CursorStyle::LeftArrow => &cursors.left_arrow,
//         CursorStyle::LeftSide => &cursors.left_side,
//         CursorStyle::NoDrop => &cursors.no_drop,
//         CursorStyle::NotAllowed => &cursors.not_allowed,
//         CursorStyle::OpenHand => &cursors.open_hand,
//         CursorStyle::Pencil => &cursors.pencil,
//         CursorStyle::Pirate => &cursors.pirate,
//         CursorStyle::Pointer => &cursors.pointer,
//         CursorStyle::RightArrow => &cursors.right_arrow,
//         CursorStyle::RightPtr => &cursors.right_ptr,
//         CursorStyle::RightSide => &cursors.right_side,
//         CursorStyle::RowResize => &cursors.row_resize,
//         CursorStyle::SizeBDiag => &cursors.size_bdiag,
//         CursorStyle::SizeFDiag => &cursors.size_fdiag,
//         CursorStyle::SizeHor => &cursors.size_hor,
//         CursorStyle::SizeVer => &cursors.size_ver,
//         CursorStyle::Text => &cursors.text,
//         CursorStyle::TopLeftCorner => &cursors.top_left_corner,
//         CursorStyle::TopRightCorner => &cursors.top_right_corner,
//         CursorStyle::TopSide => &cursors.top_side,
//         CursorStyle::UpArrow => &cursors.up_arrow,
//         CursorStyle::VerticalText => &cursors.vertical_text,
//         CursorStyle::ZoomIn => &cursors.zoom_in,
//         CursorStyle::ZoomOut => &cursors.zoom_out,
//     }
// }
