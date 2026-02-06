use std::io::Write;

use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

use super::{BaseCursor, Cursor};
use crate::{
    graphics::{pixmap_to_buffer, rasterize_svg, u32_to_rgba_u8},
    misc::EasyUnwrapUnchecked,
    platform::{
        mouse::{CursorResolution, LoadCursorError},
        Buffer,
    },
};

/// Load a custom cursor
// /// Size:
// /// 0.into(): 32x32
// /// 1.into(): 64x64
// /// 2.into(): 128x128
// /// 3.into(): 256x256
/// # Errors
/// When no tempfile could be created
/// When the tempfile could not be deleted
pub fn load_cursor(
    //size: U2,
    #[cfg(feature = "cursor_show_hotspot")] image_data: &mut Buffer,
    #[cfg(not(feature = "cursor_show_hotspot"))] image_data: &Buffer,
    hotspot_x: u16,
    hotspot_y: u16,
) -> core::result::Result<
    windows::Win32::UI::WindowsAndMessaging::HCURSOR,
    LoadCursorError,
> {
    //let size = cursor_resolution(size);

    let Ok((file_path, temp_file)) =
        create_temp_file(&create_cursor(hotspot_x, hotspot_y, image_data))
    else {
        return Err(LoadCursorError::UnableToCreateTempfile);
    };
    let _ = temp_file.keep();

    let temp = load_cursor_file(&file_path);
    if delete_temp_file(&file_path).is_err() {
        return Err(LoadCursorError::UnableToDeleteTempfile);
    }
    temp.map_or(Err(LoadCursorError::OsError), Ok)
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

/// Load a cursor SVG
///
/// # Errors
/// When the image could not be rasterized
#[allow(clippy::needless_pass_by_value)]
pub fn load_base_cursor_with_file(
    cursor: BaseCursor,
    size: CursorResolution,
    svg_data: String,
) -> core::result::Result<Cursor, LoadCursorError> {
    let wanted_size: u32 = size.get_size().easy_unwrap_unchecked(); // This will never error because u32 is bigger than u8

    let Ok(image_data) =
        rasterize_svg(svg_data.as_bytes(), wanted_size, wanted_size)
    else {
        return Err(LoadCursorError::InvalidImageData(
            "Unable to rasterize svg".to_string(),
        ));
    };

    match load_cursor(
        #[cfg(feature = "cursor_show_hotspot")]
        &mut pixmap_to_buffer(&image_data),
        #[cfg(not(feature = "cursor_show_hotspot"))]
        &pixmap_to_buffer(&image_data),
        cursor.hot_spot_x,
        cursor.hot_spot_y,
    ) {
        Ok(v) => Ok(Cursor::Win(v)),
        Err(e) => Err(e),
    }
}

/// Expects .cur file
fn load_cursor_file(
    file_path: &str,
) -> core::result::Result<
    windows::Win32::UI::WindowsAndMessaging::HCURSOR,
    &'static str,
> {
    unsafe {
        let Some(filename) = std::ffi::CString::new(file_path).ok() else {
            return Err("Unable to create null-terminated C-style byte string from file path");
        };
        let Ok(cursor) =
            LoadCursorFromFileA(PCSTR(filename.as_ptr().cast::<u8>()))
        else {
            return Err("why");
        };
        assert!(cursor.0 != 0, "Failed to load cursor");
        Ok(cursor)
    }
}
#[must_use]
/// Converts the image buffer into a windows compatible .cur
pub fn create_cursor(
    hotspot_x: u16,
    hotspot_y: u16,
    #[cfg(feature = "cursor_show_hotspot")] image: &mut Buffer,
    #[cfg(not(feature = "cursor_show_hotspot"))] image: &Buffer,
) -> Vec<u8> {
    let mut cursor_buffer: Vec<u8> = Vec::new();
    #[cfg(feature = "cursor_show_hotspot")]
    // Enabling this shows where the cursor will click
    image.set_pixel_safe(
        (hotspot_x as usize, hotspot_y as usize),
        crate::graphics::colors::RED,
    );
    let width = image.width as u8;
    let height = image.height as u8;

    // ICONDIR (6 bytes)
    cursor_buffer.extend(&[0x00, 0x00]); // Reserved
    cursor_buffer.extend(&[0x02, 0x00]); // Image type (2 = cursor)
    cursor_buffer.extend(&[0x01, 0x00]); // Number of images

    // ICONDIRENTRY (16 bytes)
    cursor_buffer.push(width); // Width
    cursor_buffer.push(height); // Height
    cursor_buffer.push(0); // Color count
    cursor_buffer.push(0); // Reserved
    cursor_buffer.extend(&hotspot_x.to_le_bytes()); // Hotspot X
    cursor_buffer.extend(&hotspot_y.to_le_bytes()); // Hotspot Y

    let image_data_offset = 6 + 16;
    let row_stride = (u32::from(width) * 32).div_ceil(32) * 4;
    let pixel_array_size = row_stride * u32::from(height);
    let bmp_header_size = 40;
    let and_mask_size = u32::from(height) * (u32::from(width).div_ceil(32) * 4);
    let size_in_bytes = bmp_header_size + pixel_array_size + and_mask_size;

    cursor_buffer.extend(&(size_in_bytes).to_le_bytes()); // Image size
    cursor_buffer.extend(&(image_data_offset as u32).to_le_bytes()); // Image offset

    // BITMAPINFOHEADER (40 bytes)
    let mut bmp_data: Vec<u8> = Vec::with_capacity(size_in_bytes as usize);
    bmp_data.extend(&(40u32.to_le_bytes())); // Header size
    bmp_data.extend(&i32::from(width).to_le_bytes()); // Width
    bmp_data.extend(&(2 * i32::from(height)).to_le_bytes()); // Height (x2 for AND mask)
    bmp_data.extend(&(1u16.to_le_bytes())); // Planes
    bmp_data.extend(&(32u16.to_le_bytes())); // Bit count
    bmp_data.extend(&(0u32.to_le_bytes())); // Compression
    bmp_data.extend(&(0u32.to_le_bytes())); // Image size
    bmp_data.extend(&(0u32.to_le_bytes())); // X pixels per meter
    bmp_data.extend(&(0u32.to_le_bytes())); // Y pixels per meter
    bmp_data.extend(&(0u32.to_le_bytes())); // Colors used
    bmp_data.extend(&(0u32.to_le_bytes())); // Important colors

    for pixel in &image.flip_vertically().data {
        let (r, g, b, a) = u32_to_rgba_u8(*pixel);
        #[allow(clippy::tuple_array_conversions)]
        bmp_data.extend(&[b, g, r, a]);
    }

    // AND mask (all zero = fully visible)
    bmp_data.extend(vec![0u8; and_mask_size as usize]);

    // Combine all into buffer
    cursor_buffer.extend(bmp_data);

    cursor_buffer
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
    cursor_data: &[u8],
) -> std::io::Result<(String, tempfile::NamedTempFile)> {
    let mut file = tempfile::NamedTempFile::new()?;
    file.write_all(cursor_data)?;

    let path_str = file
        .path()
        .to_str()
        .ok_or_else(|| {
            std::io::Error::other("Invalid UTF-8 in temp file path")
        })?
        .to_string();

    //let (file, _path) = file.keep()?; // Keep returns (File, PathBuf)
    Ok((path_str, file))
}
/// Delete the file at the given path
fn delete_temp_file(path: &str) -> std::io::Result<()> {
    std::fs::remove_file(path) // Removes the file at the given path
}

/// A windows only function to set the current cursor
pub fn set_cursor(cursor: &windows::Win32::UI::WindowsAndMessaging::HCURSOR) {
    unsafe {
        SetCursor(*cursor);
    }
}
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};

static mut ORIGINAL_WNDPROC: Option<WNDPROC> = None;
static mut CURRENT_CURSOR: windows::Win32::UI::WindowsAndMessaging::HCURSOR =
    windows::Win32::UI::WindowsAndMessaging::HCURSOR(0);

/// A hook to attach to a window -> Setting their cursor style
///
/// # Safety
/// Interacts with windows
#[must_use]
pub unsafe extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_SETCURSOR {
        // Only handle if we're in the client area (low word of lparam == HTCLIENT)
        if (lparam.0 as u16) == HTCLIENT as u16 && CURRENT_CURSOR.0 != 0 {
            SetCursor(CURRENT_CURSOR);
            return LRESULT(1); // TRUE - we handled it
        }
        // Let default handler deal with non-client areas
    }

    ORIGINAL_WNDPROC.map_or_else(
        || DefWindowProcW(hwnd, msg, wparam, lparam),
        |orig| CallWindowProcW(orig, hwnd, msg, wparam, lparam),
    )
}
#[allow(clippy::fn_to_numeric_cast)]
/// Define a window to attach to so custom cursors can be defined without flickering
///
/// # Safety
/// Interacts with windows
pub unsafe fn subclass_window(
    handle: raw_window_handle::RawWindowHandle,
    cursor: &Cursor,
) {
    if let raw_window_handle::RawWindowHandle::Win32(hwnd_handle) = handle {
        if let Cursor::Win(actual_cursor) = cursor {
            let hwnd = windows::Win32::Foundation::HWND(hwnd_handle.hwnd.get());
            CURRENT_CURSOR = *actual_cursor;

            // Get the original window procedure
            let orig_proc = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
            ORIGINAL_WNDPROC = Some(std::mem::transmute::<isize, core::option::Option<unsafe extern "system" fn(windows::Win32::Foundation::HWND, u32, windows::Win32::Foundation::WPARAM, windows::Win32::Foundation::LPARAM) -> windows::Win32::Foundation::LRESULT>>(orig_proc));

            // Set our window procedure
            #[allow(function_casts_as_integer)]
            SetWindowLongPtrW(hwnd, GWLP_WNDPROC, wndproc as isize);
        }
    }
}

/// Helper function to update cursor without re-sub classing
///
/// # Safety
/// Interacts with windows
pub unsafe fn update_cursor(
    cursor: &windows::Win32::UI::WindowsAndMessaging::HCURSOR,
) {
    CURRENT_CURSOR = *cursor;
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
//         CursorStyle::HandClosed => &cursors.dnd_move,
//         CursorStyle::HandClosedNoDrop => &cursors.dnd_no_drop,
//         CursorStyle::DownArrow => &cursors.down_arrow,
//         CursorStyle::Draft => &cursors.draft,
//         CursorStyle::Fleur => &cursors.fleur,
//         CursorStyle::Help => &cursors.help,
//         CursorStyle::LeftArrow => &cursors.left_arrow,
//         CursorStyle::LeftSide => &cursors.left_side,
//         CursorStyle::NoDrop => &cursors.no_drop,
//         CursorStyle::NotAllowed => &cursors.not_allowed,
//         CursorStyle::HandOpen => &cursors.open_hand,
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
