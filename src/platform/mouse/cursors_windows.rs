use std::io::Write;

use windows::{
    core::*, //Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

use super::{cursor_resolution, BaseCursor, Cursor};
use crate::extensions::*;
use crate::graphics::{
    pixmap_to_buffer, rasterize_svg, u32_to_hex, u32_to_rgba,
};
use crate::platform::Buffer;

fn load_cursor(
    size: U2,
    image_data: Buffer,
    hotspot_x: u16,
    hotspot_y: u16,
) -> HCURSOR {
    let size = cursor_resolution(size);

    let (file_path, temp_file) = create_temp_file(create_cursor(
        size,
        size,
        hotspot_x,
        hotspot_y,
        &image_data,
    ))
    .unwrap();
    let _ = temp_file.keep();

    let temp = load_cursor_file(&file_path);
    delete_temp_file(&file_path).unwrap();
    return temp;
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
//     // if svg has one {}, insert main_color, if svg has two {}, insert main_color, secondary_color

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
//         pixmap_to_buffer(&image_data),
//         adjusted_hotspot_x,
//         adjusted_hotspot_y,
//     );
// }

/// Load a cursor SVG and replace it's placeholders with actual colors
pub fn load_base_cursor_with_file(
    cursor: BaseCursor,
    size: U2,
    main_color: u32,
    secondary_color: u32,
    svg_data: String,
) -> Cursor {
    let svg_size = 24; // WHO TF MAKES THE CURSOR NOT A MULTIPLE OF 16 ???

    let wanted_size = cursor_resolution(size);

    //let path = get_cursor_path(&cursor.file_path.to_string());
    //let svg_data = std::fs::read_to_string(path).unwrap();
    // if svg has one {}, insert main_color, if svg has two {}, insert main_color, secondary_color

    let result_svg = svg_data
        .replace_first_occurrence("{}", &u32_to_hex(main_color))
        .replace_first_occurrence("{}", &u32_to_hex(secondary_color));

    let image_data = rasterize_svg(
        &result_svg.as_bytes(),
        wanted_size as u32,
        wanted_size as u32,
    );

    // Adjust hotspot because of the psycho who made the cursor not a multiple of 16
    let adjusted_hotspot_x = ((cursor.hot_spot_x as f64 / svg_size as f64)
        * wanted_size as f64)
        .round() as u16;
    let adjusted_hotspot_y = ((cursor.hot_spot_y as f64 / svg_size as f64)
        * wanted_size as f64)
        .round() as u16;

    return Cursor::Win(load_cursor(
        //&extract_file_name_without_extension(&cursor.file_path),
        size,
        pixmap_to_buffer(&image_data),
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
    rgba: &Buffer,
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

    buffer.extend(&(size_in_bytes).to_le_bytes()); // Image size
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

    for pixel in rgba.flip_vertically().data.iter() {
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

// fn get_cursor_path(name: &str) -> String {
//     let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//     path.push("src");
//     path.push("platform");
//     path.push("cursors");
//     path.push("svg");
//     path.push(name);

//     return path.to_str().unwrap().to_string();
// }

fn create_temp_file(
    cursor_data: Vec<u8>,
) -> std::io::Result<(String, tempfile::NamedTempFile)> {
    let mut file = tempfile::NamedTempFile::new()?;
    file.write_all(&cursor_data)?;

    let path_str = file
        .path()
        .to_str()
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid UTF-8 in temp file path",
            )
        })?
        .to_string();

    //let (file, _path) = file.keep()?; // Keep returns (File, PathBuf)
    Ok((path_str, file))
}
/// Delete the file at the given path
fn delete_temp_file(path: &str) -> std::io::Result<()> {
    return std::fs::remove_file(path); // Removes the file at the given path
}

/// A windows only function to set the current cursor
pub fn set_cursor(cursor: HCURSOR) {
    unsafe {
        SetCursor(cursor);
    }
}
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

static mut ORIGINAL_WNDPROC: Option<WNDPROC> = None;
static mut CURRENT_CURSOR: HCURSOR = HCURSOR(0);

/// A hook to attach to a window -> Setting their cursor style
pub unsafe extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_SETCURSOR => {
            // Only handle if we're in the client area (low word of lparam == HTCLIENT)
            if (lparam.0 as u16) == HTCLIENT as u16 && CURRENT_CURSOR.0 != 0 {
                SetCursor(CURRENT_CURSOR);
                return LRESULT(1); // TRUE - we handled it
            }
            // Let default handler deal with non-client areas
        }
        _ => {}
    }

    if let Some(orig) = ORIGINAL_WNDPROC {
        CallWindowProcW(orig, hwnd, msg, wparam, lparam)
    } else {
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}
/// Define a window to attach to so custom cursors can be defined without flickering
pub unsafe fn subclass_window(
    handle: raw_window_handle::RawWindowHandle,
    cursor: Cursor,
) {
    if let raw_window_handle::RawWindowHandle::Win32(hwnd_handle) = handle {
        if let Cursor::Win(actual_cursor) = cursor {
            let hwnd = windows::Win32::Foundation::HWND(hwnd_handle.hwnd.get());
            CURRENT_CURSOR = actual_cursor;

            // Get the original window procedure
            let orig_proc = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
            ORIGINAL_WNDPROC = Some(std::mem::transmute(orig_proc));

            // Set our window procedure
            SetWindowLongPtrW(hwnd, GWLP_WNDPROC, wndproc as isize);
        }
    }
}

/// Helper function to update cursor without re-subclassing
pub unsafe fn update_cursor(cursor: HCURSOR) {
    CURRENT_CURSOR = cursor;
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
