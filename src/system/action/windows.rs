extern crate winapi;
use super::ScreenImage;

use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowPos, SWP_NOSIZE, SWP_NOZORDER,
};

use windows::{
    Win32::Foundation::RECT,
    Win32::Graphics::Gdi::{
        BitBlt,
        CreateCompatibleBitmap,
        CreateCompatibleDC,
        DeleteDC,
        DeleteObject,
        GetDC,
        ReleaseDC,
        SelectObject,
        SRCCOPY, //GetPixel
    },
    Win32::UI::WindowsAndMessaging::{
        GetDesktopWindow, GetShellWindow, GetWindowRect,
    },
};

use windows::{
    Win32::Foundation::HWND,
    Win32::UI::WindowsAndMessaging::{
        GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED,
        WS_EX_TRANSPARENT,
    },
};

pub fn get_screen_resolution() -> (i32, i32) {
    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
    (width, height)
}
pub fn capture_screen() -> Option<ScreenImage> {
    unsafe {
        // Get the desktop window handle
        let desktop_hwnd = GetDesktopWindow();
        if desktop_hwnd.0 == 0 {
            return None;
        }

        // Get desktop dimensions
        let mut rect = RECT::default();
        if !GetWindowRect(desktop_hwnd, &mut rect).as_bool() {
            return None;
        }
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        // Get the device context for the desktop
        let hdc = GetDC(desktop_hwnd);
        if hdc.is_invalid() {
            return None;
        }

        // Create a compatible DC and bitmap
        let hdc_mem = CreateCompatibleDC(hdc);
        if hdc_mem.is_invalid() {
            ReleaseDC(desktop_hwnd, hdc);
            return None;
        }

        let hbitmap = CreateCompatibleBitmap(hdc, width, height);
        if hbitmap.is_invalid() {
            DeleteDC(hdc_mem);
            ReleaseDC(desktop_hwnd, hdc);
            return None;
        }

        // Select the bitmap into the compatible DC
        let old_obj = SelectObject(hdc_mem, hbitmap);

        // Copy screen contents to our bitmap
        let result = BitBlt(
            hdc_mem, 0, 0, width, height, hdc, rect.left, rect.top, SRCCOPY,
        );

        // Create a vector to store the pixel data
        // Calculate the correct size: width * height (as usize to prevent overflow)
        let size = (width as usize) * (height as usize);
        let mut pixels = vec![0u32; size];

        // Get the bitmap data
        let bmi = windows::Win32::Graphics::Gdi::BITMAPINFO {
            bmiHeader: windows::Win32::Graphics::Gdi::BITMAPINFOHEADER {
                biSize: std::mem::size_of::<
                    windows::Win32::Graphics::Gdi::BITMAPINFOHEADER,
                >() as u32,
                biWidth: width,
                biHeight: -height, // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0, // BI_RGB is 0
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD::default()],
        };

        let _scan_lines = windows::Win32::Graphics::Gdi::GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut std::ffi::c_void),
            &bmi as *const _ as *mut _,
            windows::Win32::Graphics::Gdi::DIB_RGB_COLORS,
        );

        // Clean up
        SelectObject(hdc_mem, old_obj);
        DeleteObject(hbitmap);
        DeleteDC(hdc_mem);
        ReleaseDC(desktop_hwnd, hdc);

        if !result.as_bool() {
            return None;
        }

        Some(ScreenImage {
            width,
            height,
            pixels,
        })
    }
}
pub fn capture_desktop_background() -> Option<ScreenImage> {
    unsafe {
        // Get the shell window handle (desktop background + icons)
        let shell_hwnd = GetShellWindow();
        if shell_hwnd.0 == 0 {
            return None;
        }

        // Get desktop dimensions
        let mut rect = RECT::default();
        if !GetWindowRect(shell_hwnd, &mut rect).as_bool() {
            return None;
        }
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        // Get the device context for the shell window
        let hdc = GetDC(shell_hwnd);
        if hdc.is_invalid() {
            return None;
        }

        // Create a compatible DC and bitmap
        let hdc_mem = CreateCompatibleDC(hdc);
        if hdc_mem.is_invalid() {
            ReleaseDC(shell_hwnd, hdc);
            return None;
        }

        let hbitmap = CreateCompatibleBitmap(hdc, width, height);
        if hbitmap.is_invalid() {
            DeleteDC(hdc_mem);
            ReleaseDC(shell_hwnd, hdc);
            return None;
        }

        // Select the bitmap into the compatible DC
        let old_obj = SelectObject(hdc_mem, hbitmap);

        // Copy screen contents to our bitmap
        let result = BitBlt(
            hdc_mem, 0, 0, width, height, hdc, rect.left, rect.top, SRCCOPY,
        );

        // Create a vector to store the pixel data
        let size = (width as usize) * (height as usize);
        let mut pixels = vec![0u32; size];

        // Get the bitmap data
        let bmi = windows::Win32::Graphics::Gdi::BITMAPINFO {
            bmiHeader: windows::Win32::Graphics::Gdi::BITMAPINFOHEADER {
                biSize: std::mem::size_of::<
                    windows::Win32::Graphics::Gdi::BITMAPINFOHEADER,
                >() as u32,
                biWidth: width,
                biHeight: -height, // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0, // BI_RGB is 0
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD::default()],
        };

        let _scan_lines = windows::Win32::Graphics::Gdi::GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut std::ffi::c_void),
            &bmi as *const _ as *mut _,
            windows::Win32::Graphics::Gdi::DIB_RGB_COLORS,
        );

        // Clean up
        SelectObject(hdc_mem, old_obj);
        DeleteObject(hbitmap);
        DeleteDC(hdc_mem);
        ReleaseDC(shell_hwnd, hdc);

        if !result.as_bool() {
            return None;
        }

        Some(ScreenImage {
            width,
            height,
            pixels,
        })
    }
}

pub fn make_color_transparent(
    handle: &raw_window_handle::RawWindowHandle,
    color: (u8, u8, u8),
) -> bool {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            make_color_transparent_raw(
                handle.hwnd.get() as winapi::shared::windef::HWND,
                color,
            );
            true
        }
        _ => false,
    }
}

pub fn make_color_transparent_raw(
    hwnd: *mut winapi::shared::windef::HWND__,
    color: (u8, u8, u8),
) {
    unsafe {
        let ex_style = winapi::um::winuser::GetWindowLongW(
            hwnd,
            winapi::um::winuser::GWL_EXSTYLE,
        );
        winapi::um::winuser::SetWindowLongW(
            hwnd,
            winapi::um::winuser::GWL_EXSTYLE,
            ex_style | winapi::um::winuser::WS_EX_LAYERED as i32,
        );

        winapi::um::winuser::SetLayeredWindowAttributes(
            hwnd,
            winapi::um::wingdi::RGB(color.0, color.1, color.2), // RGB color to make transparent
            0, // Alpha (not used with color key)
            winapi::um::winuser::LWA_COLORKEY, // Use color key transparency
        );
    }
}

pub fn get_window_position_raw(
    hwnd: windows::Win32::Foundation::HWND,
) -> Option<(i32, i32)> {
    unsafe {
        let mut rect = RECT::default();
        if GetWindowRect(hwnd, &mut rect).as_bool() {
            Some((rect.left, rect.top))
        } else {
            None
        }
    }
}
pub fn set_window_position(
    handle: raw_window_handle::RawWindowHandle,
    x: i32,
    y: i32,
) -> bool {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            set_window_position_raw(
                windows::Win32::Foundation::HWND(handle.hwnd.get()),
                x,
                y,
            );
            true
        }
        _ => false,
    }
}

pub fn set_window_position_raw(
    hwnd: windows::Win32::Foundation::HWND,
    x: i32,
    y: i32,
) {
    unsafe {
        SetWindowPos(
            hwnd,
            windows::Win32::Foundation::HWND(0),
            x,
            y,
            0,
            0,
            SWP_NOSIZE | SWP_NOZORDER,
        );
    }
}
pub fn make_window_click_through_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style | WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0) as i32,
        );
    }
}

pub fn make_window_click_solid_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style & !(WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0)) as i32,
        );
    }
}

pub fn make_window_borderless_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let style = GetWindowLongW(
            hwnd,
            windows::Win32::UI::WindowsAndMessaging::GWL_STYLE,
        ) as u32;
        SetWindowLongW(
            hwnd,
            windows::Win32::UI::WindowsAndMessaging::GWL_STYLE,
            (style
                & !(windows::Win32::UI::WindowsAndMessaging::WS_CAPTION.0
                    | windows::Win32::UI::WindowsAndMessaging::WS_THICKFRAME.0))
                as i32,
        );
        SetWindowPos(
            hwnd,
            HWND(0),
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOZORDER,
        );
    }
}

pub fn give_window_a_border_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let style = GetWindowLongW(
            hwnd,
            windows::Win32::UI::WindowsAndMessaging::GWL_STYLE,
        ) as u32;
        SetWindowLongW(
            hwnd,
            windows::Win32::UI::WindowsAndMessaging::GWL_STYLE,
            (style
                | windows::Win32::UI::WindowsAndMessaging::WS_CAPTION.0
                | windows::Win32::UI::WindowsAndMessaging::WS_THICKFRAME.0)
                as i32,
        );
        SetWindowPos(
            hwnd,
            HWND(0),
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOZORDER,
        );
    }
}

pub fn hide_from_taskbar_and_alt_tab_raw(
    hwnd: windows::Win32::Foundation::HWND,
) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            ((ex_style
                | windows::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW.0)
                & !windows::Win32::UI::WindowsAndMessaging::WS_EX_APPWINDOW.0)
                as i32,
        );
        SetWindowPos(
            hwnd,
            HWND(0),
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOZORDER,
        );
    }
}
pub fn show_in_taskbar_and_alt_tab_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            ((ex_style
                | windows::Win32::UI::WindowsAndMessaging::WS_EX_APPWINDOW.0)
                & !windows::Win32::UI::WindowsAndMessaging::WS_EX_TOOLWINDOW.0)
                as i32,
        );
        SetWindowPos(
            hwnd,
            HWND(0),
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | SWP_NOSIZE
                | SWP_NOZORDER,
        );
    }
}

pub fn set_window_opacity(hwnd: windows::Win32::Foundation::HWND, alpha: u8) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(hwnd, GWL_EXSTYLE, (ex_style | WS_EX_LAYERED.0) as i32);
        windows::Win32::UI::WindowsAndMessaging::SetLayeredWindowAttributes(
            hwnd,
            windows::Win32::Foundation::COLORREF(0),
            alpha,
            windows::Win32::UI::WindowsAndMessaging::LWA_ALPHA,
        );
    }
}

pub fn get_z_raw(hwnd: windows::Win32::Foundation::HWND) -> Option<u32> {
    unsafe {
        let mut order = 0;
        let mut current =
            windows::Win32::UI::WindowsAndMessaging::GetTopWindow(HWND(0));
        while current.0 != 0 {
            if current.0 == hwnd.0 {
                return Some(order);
            }
            current = windows::Win32::UI::WindowsAndMessaging::GetWindow(
                current,
                windows::Win32::UI::WindowsAndMessaging::GW_HWNDNEXT,
            );
            order += 1;
        }
    }
    None
}

pub fn set_z_to_after_raw(
    hwnd: windows::Win32::Foundation::HWND,
    insert_after: windows::Win32::Foundation::HWND,
) {
    unsafe {
        SetWindowPos(
            hwnd,
            insert_after,
            0,
            0,
            0,
            0,
            windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE
                | SWP_NOSIZE
                | windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE,
        );
    }
}

pub fn set_z_raw(hwnd: windows::Win32::Foundation::HWND, index: u32) {
    unsafe {
        let mut order = 0;
        let mut current =
            windows::Win32::UI::WindowsAndMessaging::GetTopWindow(HWND(0));
        while order == index {
            if current.0 == hwnd.0 {
                SetWindowPos(
                    hwnd,
                    current,
                    0,
                    0,
                    0,
                    0,
                    windows::Win32::UI::WindowsAndMessaging::SWP_NOMOVE | SWP_NOSIZE | windows::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE,
                );
            }
            current = windows::Win32::UI::WindowsAndMessaging::GetWindow(
                current,
                windows::Win32::UI::WindowsAndMessaging::GW_HWNDNEXT,
            );
            order += 1;
        }
    }
}

// pub fn get_desktop_pixel_color(x: i32, y: i32) -> u32 {
//     unsafe {
//         // Get the desktop window handle directly
//         let desktop_hwnd = GetDesktopWindow();
//         if desktop_hwnd.0 == 0 {
//             return 0;
//         }

//         // Get the device context for the desktop window
//         let hdc = GetDC(desktop_hwnd);
//         if hdc.is_invalid() {
//             return 0;
//         }

//         // Make sure we release the DC even if getting the pixel fails
//         let result = std::panic::catch_unwind(|| {
//             let pixel_color = GetPixel(hdc, x, y).0;
//             if pixel_color == u32::MAX {
//                 return Err(windows::core::Error::new(
//                     windows::core::HRESULT(0),
//                     "Failed to get pixel color".into(),
//                 ));
//             }
//             return Ok(pixel_color);
//             // // Convert to u32 first, then perform the bitwise operations
//             // let color_u32: u32 = pixel_color;

//             // Ok(PixelColor {
//             //     r: (color_u32 & 0xFF) as u8,
//             //     g: ((color_u32 >> 8) & 0xFF) as u8,
//             //     b: ((color_u32 >> 16) & 0xFF) as u8,
//             // })
//         });

//         // Always release the DC
//         ReleaseDC(desktop_hwnd, hdc);

//         match result {
//             Ok(Ok(color)) => color,
//             Ok(Err(_e)) => 0,
//             Err(_) => 0,
//         }
//     }
// }
pub fn get_title_bar_height() -> i32 {
    unsafe { GetSystemMetrics(4) }
}
