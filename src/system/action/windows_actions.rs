/// `OsImplementation` for Window
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct WindowsActions {}

impl Default for WindowsActions {
    fn set_window_position(
        handle: &raw_window_handle::RawWindowHandle,
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
    fn set_window_level(
        handle: &raw_window_handle::RawWindowHandle,
        level: WindowLevel,
    ) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                set_window_level_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                    level,
                );
                true
            }
            _ => false,
        }
    }
    fn get_window_position(
        handle: &raw_window_handle::RawWindowHandle,
    ) -> (i32, i32) {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_window_position_raw(windows::Win32::Foundation::HWND(
                    handle.hwnd.get(),
                ))
                .unwrap_or((i32::MIN, i32::MIN))
            }
            _ => (i32::MIN, i32::MIN),
        }
    }
    fn get_window_size(
        handle: &raw_window_handle::RawWindowHandle,
    ) -> (i32, i32) {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_window_size_raw(windows::Win32::Foundation::HWND(
                    handle.hwnd.get(),
                ))
            }
            _ => (i32::MIN, i32::MIN),
        }
    }
    fn get_window_hitbox_size(
        handle: &raw_window_handle::RawWindowHandle,
    ) -> (i32, i32) {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_window_hitbox_size_raw(windows::Win32::Foundation::HWND(
                    handle.hwnd.get(),
                ))
                .unwrap_or((i32::MIN, i32::MIN))
            }
            _ => (i32::MIN, i32::MIN),
        }
    }
    fn set_window_size(
        handle: &raw_window_handle::RawWindowHandle,
        size: (i32, i32),
    ) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                resize(handle.hwnd.get() as winapi::shared::windef::HWND, size);
                true
            }
            _ => false,
        }
    }
}

impl Transparency for WindowsActions {
    fn make_color_transparent(
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
    fn set_window_opacity(
        handle: &raw_window_handle::RawWindowHandle,
        opacity: u8,
    ) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                set_window_opacity_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                    opacity,
                );
                true
            }
            _ => false,
        }
    }
}
impl Decoration for WindowsActions {
    fn set_window_borderless(
        handle: &raw_window_handle::RawWindowHandle,
        boolean: bool,
    ) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                if boolean {
                    make_window_borderless_raw(
                        windows::Win32::Foundation::HWND(handle.hwnd.get()),
                    );
                } else {
                    give_window_a_border_raw(windows::Win32::Foundation::HWND(
                        handle.hwnd.get(),
                    ));
                }
                true
            }
            _ => false,
        }
    }
}
impl Misc for WindowsActions {
    fn set_window_hidden_from_taskbar_and_alt_tab(
        handle: &raw_window_handle::RawWindowHandle,
        boolean: bool,
    ) -> bool {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                if boolean {
                    hide_from_taskbar_and_alt_tab_raw(
                        windows::Win32::Foundation::HWND(handle.hwnd.get()),
                    );
                } else {
                    show_in_taskbar_and_alt_tab_raw(
                        windows::Win32::Foundation::HWND(handle.hwnd.get()),
                    );
                }
                true
            }
            _ => false,
        }
    }
    fn get_all_windows() -> Vec<raw_window_handle::RawWindowHandle> {
        let windows = get_all_windows_raw();
        let mut new = Vec::new();
        for i in windows {
            new.push(raw_window_handle::RawWindowHandle::Win32(
                raw_window_handle::Win32WindowHandle::new(unsafe {
                    core::num::NonZero::new(i.0).unwrap_unchecked()
                }),
            ));
        }
        new
    }
    fn get_title_using_id(
        handle: &raw_window_handle::RawWindowHandle,
    ) -> String {
        match handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_title_using_id_raw(
                    handle.hwnd.get() as winapi::shared::windef::HWND
                )
            }
            _ => String::new(),
        }
    }
    fn get_id_using_title(
        title: &str,
        exact_match: bool,
        case_sensitive: bool,
        include_hidden: bool,
        just_one: bool,
    ) -> Option<Vec<raw_window_handle::RawWindowHandle>> {
        get_window_id_by_title(
            title,
            exact_match,
            case_sensitive,
            include_hidden,
            just_one,
        )
    }
    fn capture_screen() -> Option<Buffer> {
        capture_screen_raw()
    }
    fn capture_desktop_background() -> Option<Buffer> {
        capture_desktop_background_raw()
    }
    fn set_click_ability_of_window(
        handle: &raw_window_handle::RawWindowHandle,
        click_through: bool,
    ) {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            if click_through {
                make_window_click_through_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                );
            } else {
                make_window_click_solid_raw(windows::Win32::Foundation::HWND(
                    handle.hwnd.get(),
                ));
            }
        }
    }
    fn get_window_z(handle: &raw_window_handle::RawWindowHandle) -> u32 {
        match handle {
            #[cfg(target_os = "windows")]
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                get_z_raw(windows::Win32::Foundation::HWND(handle.hwnd.get()))
                    .unwrap_or(u32::MIN)
            }
            _ => u32::MIN,
        }
    }
    fn set_window_z(
        handle: &raw_window_handle::RawWindowHandle,
        z: u32,
    ) -> bool {
        match handle {
            #[cfg(target_os = "windows")]
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                set_z_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                    z,
                );
                true
            }
            _ => false,
        }
    }
    fn set_window_z_after(
        handle: &raw_window_handle::RawWindowHandle,
        after: &raw_window_handle::RawWindowHandle,
    ) -> bool {
        // Matching 2 values at the same time feels cursed
        match (handle, after) {
            #[cfg(target_os = "windows")]
            (
                raw_window_handle::RawWindowHandle::Win32(handle),
                raw_window_handle::RawWindowHandle::Win32(after),
            ) => {
                set_z_to_after_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                    windows::Win32::Foundation::HWND(after.hwnd.get()),
                );
                true
            }
            _ => false,
        }
    }
    fn set_cpu_priority(
        handle: &raw_window_handle::RawWindowHandle,
        priority: CpuPriority,
    ) {
        if let raw_window_handle::RawWindowHandle::Win32(hwnd) = handle {
            set_cpu_priority(
                windows::Win32::Foundation::HWND(hwnd.hwnd.get()),
                priority,
            );
        }
    }
}

extern crate winapi;

use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        GetClientRect, GetWindowLongW, SetWindowLongW, SetWindowPos,
        GWL_EXSTYLE, SWP_NOSIZE, SWP_NOZORDER, WS_EX_LAYERED,
        WS_EX_TRANSPARENT,
    },
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

use crate::{
    platform::WindowLevel,
    prelude::Buffer,
    system::action::{
        CpuPriority, Decoration, Default, Host, Iconized, Misc,
        ProgressionState, Screen, TaskBar, Transparency,
    },
};

#[allow(trivial_casts)]
fn capture_screen_raw() -> Option<Buffer> {
    unsafe {
        // Get the desktop window handle
        let desktop_hwnd = GetDesktopWindow();
        if desktop_hwnd.0 == 0 {
            return None;
        }

        // Get desktop dimensions
        let mut rect = RECT::default();
        if !GetWindowRect(desktop_hwnd, &raw mut rect).as_bool() {
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
            Some(pixels.as_mut_ptr().cast::<std::ffi::c_void>()),
            (&raw const bmi).cast_mut(),
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
        Buffer::new((width as usize, height as usize), pixels).ok()
    }
}
#[allow(trivial_casts)]
fn capture_desktop_background_raw() -> Option<Buffer> {
    unsafe {
        // Get the shell window handle (desktop background + icons)
        let shell_hwnd = GetShellWindow();
        if shell_hwnd.0 == 0 {
            return None;
        }

        // Get desktop dimensions
        let mut rect = RECT::default();
        if !GetWindowRect(shell_hwnd, &raw mut rect).as_bool() {
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
            Some(pixels.as_mut_ptr().cast::<std::ffi::c_void>()),
            (&raw const bmi).cast_mut(),
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

        Buffer::new((width as usize, height as usize), pixels).ok()
    }
}

fn make_color_transparent_raw(
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

fn get_window_position_raw(
    hwnd: windows::Win32::Foundation::HWND,
) -> Option<(i32, i32)> {
    unsafe {
        let mut rect = RECT::default();
        if GetWindowRect(hwnd, &raw mut rect).as_bool() {
            Some((rect.left, rect.top))
        } else {
            None
        }
    }
}

fn set_window_position_raw(
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
fn make_window_click_through_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style | WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0) as i32,
        );
    }
}

fn make_window_click_solid_raw(hwnd: windows::Win32::Foundation::HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        SetWindowLongW(
            hwnd,
            GWL_EXSTYLE,
            (ex_style & !(WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0)) as i32,
        );
    }
}

fn make_window_borderless_raw(hwnd: windows::Win32::Foundation::HWND) {
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

fn give_window_a_border_raw(hwnd: windows::Win32::Foundation::HWND) {
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

fn hide_from_taskbar_and_alt_tab_raw(hwnd: windows::Win32::Foundation::HWND) {
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
fn show_in_taskbar_and_alt_tab_raw(hwnd: windows::Win32::Foundation::HWND) {
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

fn set_window_opacity_raw(hwnd: windows::Win32::Foundation::HWND, alpha: u8) {
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

fn get_z_raw(hwnd: windows::Win32::Foundation::HWND) -> Option<u32> {
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

fn set_z_to_after_raw(
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

fn set_z_raw(hwnd: windows::Win32::Foundation::HWND, index: u32) {
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

// fn get_desktop_pixel_color(x: i32, y: i32) -> u32 {
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
// PLEASE HELP WHAT DOES THIS FUNCTION DO??

// #[cfg(target_os = "windows")]
// fn optimize_large_window(window: &winit::window::Window) {
//     use winapi::um::winuser::{
//         SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SetWindowPos,
//     };

//     if let raw_window_handle::RawWindowHandle::Win32(handle) =
//         window.raw_window_handle()
//     {
//         let hwnd = handle.hwnd as winapi::shared::windef::HWND;

//         unsafe {
//             // Force window to refresh its layered window properties
//             SetWindowPos(
//                 hwnd,
//                 core::ptr::null_mut(),
//                 0,
//                 0,
//                 0,
//                 0,
//                 SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED,
//             );
//         }
//     }
// }
use windows::Win32::UI::WindowsAndMessaging::{
    HWND_BOTTOM, HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOMOVE,
};

fn set_window_level_raw(hwnd: HWND, level: WindowLevel) {
    let insert_after = match level {
        WindowLevel::AlwaysOnBottom => HWND_BOTTOM,
        WindowLevel::Normal => HWND_NOTOPMOST,
        WindowLevel::AlwaysOnTop => HWND_TOPMOST,
    };

    unsafe {
        SetWindowPos(hwnd, insert_after, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE);
    }
}

use raw_window_handle::{RawWindowHandle, Win32WindowHandle};
use winapi::um::winuser::{EnumWindows, GetWindowTextW, IsWindowVisible};
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
struct WindowSearchData {
    target_title: String,
    found_hwnds: Option<Vec<winapi::shared::windef::HWND>>,
    exact_match: bool,
    case_sensitive: bool,
    include_hidden: bool,
    just_one: bool,
}
#[allow(trivial_casts)]
fn get_window_id_by_title(
    title: &str,
    exact_match: bool,
    case_sensitive: bool,
    include_hidden: bool,
    just_one: bool,
) -> Option<Vec<RawWindowHandle>> {
    let mut search_data = WindowSearchData {
        target_title: title.to_string(),
        found_hwnds: None,
        exact_match,
        case_sensitive,
        include_hidden,
        just_one,
    };

    unsafe {
        EnumWindows(Some(enum_windows_proc), &raw mut search_data as isize);
    }

    search_data.found_hwnds.map(|hwnds| {
        let mut handles = Vec::new();
        for hwnd in hwnds {
            handles.push(RawWindowHandle::Win32(Win32WindowHandle::new(
                unsafe {
                    core::num::NonZero::new(hwnd as isize).unwrap_unchecked()
                },
            )));
        }

        handles
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_notepad() {
        if let Some(handle) =
            get_window_id_by_title("Notepad", false, true, false, false)
        {
            println!("Found Notepad window: {:?}", handle);
        } else {
            println!("Notepad window not found");
        }
    }
}

#[allow(unused_imports)]
use crate::extensions::*;
unsafe extern "system" fn enum_windows_proc(
    hwnd: winapi::shared::windef::HWND,
    long_param: isize,
) -> i32 {
    let data = &mut *(long_param as *mut WindowSearchData);

    // Check visibility based on criteria
    if !data.include_hidden && IsWindowVisible(hwnd) == 0 {
        return 1; // Continue enumeration
    }
    if !data.target_title.is_empty() {
        // Get window title
        let mut title_buf = [0u16; 512];
        let title_len = GetWindowTextW(
            hwnd,
            title_buf.as_mut_ptr(),
            title_buf.len() as i32,
        );

        let title = if title_len > 0 {
            String::from_utf16_lossy(&title_buf[..title_len as usize])
        } else {
            String::new()
        };

        // Check title match
        let title_matches = if data.exact_match {
            if data.case_sensitive {
                title == data.target_title
            } else {
                title.to_lowercase() == data.target_title.to_lowercase()
            }
        } else if data.case_sensitive {
            title.contains(&data.target_title)
        } else {
            title.to_lowercase().contains(&data.target_title.to_lowercase())
        };

        if !title_matches {
            return 1; // Continue enumeration
        }
    }

    data.found_hwnds =
        Some(data.found_hwnds.clone().unwrap_or_default().combined(hwnd));
    // 0 Stop enumeration
    // 1 Continue enumeration

    i32::from(!data.just_one)
}
#[allow(trivial_casts)]
fn get_all_windows_raw() -> Vec<HWND> {
    let mut search_data = WindowSearchData {
        target_title: String::new(),
        found_hwnds: None,
        exact_match: false,
        case_sensitive: false,
        include_hidden: false,
        just_one: false,
    };

    unsafe {
        EnumWindows(Some(enum_windows_proc), &raw mut search_data as isize);
    }
    let mut found_windows = Vec::new();
    for i in search_data.found_hwnds.unwrap_or_default() {
        found_windows.push(HWND(i as isize));
    }

    found_windows
}

fn get_window_hitbox_size_raw(hwnd: HWND) -> Option<(i32, i32)> {
    unsafe {
        let mut rect = RECT::default();
        if GetWindowRect(hwnd, &raw mut rect).as_bool() {
            Some((rect.right - rect.left, rect.bottom - rect.top))
        } else {
            None
        }
    }
}
fn get_window_size_raw(hwnd: HWND) -> (i32, i32) {
    unsafe {
        let mut rect = RECT::default();
        GetClientRect(hwnd, &raw mut rect);
        (rect.right - rect.left, rect.bottom - rect.top)
    }
}

fn get_title_using_id_raw(hwnd: winapi::shared::windef::HWND) -> String {
    let mut title_buf = [0u16; 512];
    let title_len;
    unsafe {
        title_len = GetWindowTextW(
            hwnd,
            title_buf.as_mut_ptr(),
            title_buf.len() as i32,
        );
    }

    if title_len > 0 {
        String::from_utf16_lossy(&title_buf[..title_len as usize])
    } else {
        String::new()
    }
}

use windows::Win32::{
    System::Threading::{
        OpenProcess, OpenThread, SetPriorityClass, SetThreadPriority,
        ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS,
        HIGH_PRIORITY_CLASS, IDLE_PRIORITY_CLASS, NORMAL_PRIORITY_CLASS,
        PROCESS_SET_INFORMATION, REALTIME_PRIORITY_CLASS,
        THREAD_PRIORITY_ABOVE_NORMAL, THREAD_PRIORITY_BELOW_NORMAL,
        THREAD_PRIORITY_HIGHEST, THREAD_PRIORITY_IDLE, THREAD_PRIORITY_NORMAL,
        THREAD_PRIORITY_TIME_CRITICAL, THREAD_SET_INFORMATION,
    },
    UI::WindowsAndMessaging::GetWindowThreadProcessId,
};

/// Set the priority of a running process
pub fn set_cpu_priority(hwnd: HWND, priority: CpuPriority) {
    unsafe {
        let mut process_id = 0u32;
        let thread_id =
            GetWindowThreadProcessId(hwnd, Some(&raw mut process_id));

        if thread_id == 0 {
            return; // Failed to get thread ID
        }

        // Open the process with permission to set priority
        let Ok(process) =
            OpenProcess(PROCESS_SET_INFORMATION, false, process_id)
        else {
            return;
        };

        // Open the thread with permission to set priority
        let Ok(thread) = OpenThread(THREAD_SET_INFORMATION, false, thread_id)
        else {
            return;
        };

        match priority {
            CpuPriority::Idle => {
                let _ = SetPriorityClass(process, IDLE_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_IDLE);
            }
            CpuPriority::BelowNormal => {
                let _ = SetPriorityClass(process, BELOW_NORMAL_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_BELOW_NORMAL);
            }
            CpuPriority::Normal => {
                let _ = SetPriorityClass(process, NORMAL_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_NORMAL);
            }
            CpuPriority::AboveNormal => {
                let _ = SetPriorityClass(process, ABOVE_NORMAL_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_ABOVE_NORMAL);
            }
            CpuPriority::High => {
                let _ = SetPriorityClass(process, HIGH_PRIORITY_CLASS);
                let _ = SetThreadPriority(thread, THREAD_PRIORITY_HIGHEST);
            }
            CpuPriority::Realtime => {
                let _ = SetPriorityClass(process, REALTIME_PRIORITY_CLASS);
                let _ =
                    SetThreadPriority(thread, THREAD_PRIORITY_TIME_CRITICAL);
            }
        }
    }
}

// use windows::Win32::UI::WindowsAndMessaging::{FLASHWINFO, FLASHW_ALL, FLASHW_TIMERNOFG, FlashWindowEx};

// pub fn flash_window(hwnd: HWND) {
//     unsafe {
//         let info = FLASHWINFO {
//             cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
//             hwnd,
//             dwFlags: FLASHW_ALL | FLASHW_TIMERNOFG,
//             uCount: 0,
//             dwTimeout: 0,
//         };
//         FlashWindowEx(&info);
//     }
// }

impl TaskBar for WindowsActions {
    fn set_icon_state(
        handle: &raw_window_handle::RawWindowHandle,
        state: &ProgressionState,
    ) {
        if let raw_window_handle::RawWindowHandle::Win32(raw) = handle {
            let _ = set_taskbar_progress_state(HWND(raw.hwnd.get()), state);
        }
    }
    fn set_icon_progress(
        handle: &raw_window_handle::RawWindowHandle,
        current: u64,
        total: u64,
    ) {
        if let raw_window_handle::RawWindowHandle::Win32(raw) = handle {
            let _ = set_taskbar_progress_value(
                HWND(raw.hwnd.get()),
                current,
                total,
            );
        }
    }
}

use windows::{
    core::Result,
    Win32::{
        System::Com::{CoCreateInstance, CLSCTX_ALL},
        UI::Shell::{
            ITaskbarList3, TaskbarList, TBPFLAG, TBPF_ERROR,
            TBPF_INDETERMINATE, TBPF_NOPROGRESS, TBPF_NORMAL, TBPF_PAUSED,
        },
    },
};
/// Convert between `ProgressionState` and TBPFLAG
#[must_use]
pub const fn map_state_to_windows_state(state: &ProgressionState) -> TBPFLAG {
    match state {
        ProgressionState::NoBar => TBPF_NOPROGRESS,
        ProgressionState::Error => TBPF_ERROR,
        ProgressionState::Loading => TBPF_INDETERMINATE,
        ProgressionState::Normal => TBPF_NORMAL,
        ProgressionState::Paused => TBPF_PAUSED,
    }
}
/// Set how the taskbar icon looks
///
/// # Errors
/// When unable to set the progress
pub fn set_taskbar_progress_state(
    hwnd: HWND,
    state: &ProgressionState,
) -> Result<()> {
    unsafe {
        let taskbar: ITaskbarList3 =
            CoCreateInstance(&TaskbarList, None, CLSCTX_ALL)?;
        taskbar.HrInit()?;
        taskbar.SetProgressState(hwnd, map_state_to_windows_state(state))?;
    }
    Ok(())
}
/// Set the progress of a few styles
///
/// # Errors
/// When the taskbar could not be referenced
/// When setting the value didn't work
pub fn set_taskbar_progress_value(
    hwnd: HWND,
    completed: u64,
    total: u64,
) -> Result<()> {
    unsafe {
        let taskbar: ITaskbarList3 =
            CoCreateInstance(&TaskbarList, None, CLSCTX_ALL)?;
        taskbar.HrInit()?;
        taskbar.SetProgressValue(hwnd, completed, total)?;
    }
    Ok(())
}
#[track_caller]
fn get_screen_resolution() -> (i32, i32) {
    use winapi::um::winuser::GetSystemMetrics;
    let width = unsafe { GetSystemMetrics(winapi::um::winuser::SM_CXSCREEN) };
    let height = unsafe { GetSystemMetrics(winapi::um::winuser::SM_CYSCREEN) };
    (width, height)
}
impl Host for WindowsActions {
    fn get_os_name() -> String {
        "Windows".to_string()
    }
}
impl Screen for WindowsActions {
    fn get_screen_resolution() -> (i32, i32) {
        get_screen_resolution()
    }
}

use winapi::um::winuser::{IsIconic, IsZoomed};
use winapi::um::winuser::{
    ShowWindow, //HWND_NOTOPMOST, HWND_TOPMOST,
    // SWP_NOSIZE,
    // SWP_SHOWWINDOW,
    SW_MAXIMIZE,
    SW_MINIMIZE,
    SW_RESTORE,
};
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Maximize the given window -> Expand its borders to fit the screen
pub fn maximize(hwnd: winapi::shared::windef::HWND) {
    unsafe {
        ShowWindow(hwnd, SW_MAXIMIZE);
    }
}
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Hide away the given window
pub fn minimize(hwnd: winapi::shared::windef::HWND) {
    unsafe {
        ShowWindow(hwnd, SW_MINIMIZE);
    }
}
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Unhide un-away the given window
pub fn restore(hwnd: winapi::shared::windef::HWND) {
    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
    }
}

// pub fn always_on_top(window: &*mut c_void, always_on_top_bool: bool) {
//     if always_on_top_bool {
//         topmost(window);
//     } else {
//         not_topmost(window);
//     }
// }

// pub fn topmost(window: &*mut c_void) {
//     let hwnd = *window as HWND;
//     unsafe {
//         SetWindowPos(
//             hwnd,
//             HWND_TOPMOST,
//             0,
//             0,
//             0,
//             0,
//             SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
//         );
//     }
// }
// pub fn not_topmost(window: &*mut c_void) {
//     let hwnd = *window as HWND;
//     unsafe {
//         SetWindowPos(
//             hwnd,
//             HWND_NOTOPMOST,
//             0,
//             0,
//             0,
//             0,
//             SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
//         );
//     }
// }
/// Resize the current window to the specified size
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn resize(hwnd: winapi::shared::windef::HWND, size: (i32, i32)) {
    unsafe {
        // Resize the window to the given width and height
        winapi::um::winuser::SetWindowPos(
            hwnd,
            core::ptr::null_mut(), // No changes to the window's position
            0,
            0,
            size.0,
            size.1,
            winapi::um::winuser::SWP_NOZORDER | winapi::um::winuser::SWP_NOMOVE, // Keep the current position, just resize
        );
    }
}
/// Wether the window is minimized on windows
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn is_window_minimized(hwnd: winapi::shared::windef::HWND) -> bool {
    unsafe { IsIconic(hwnd) != 0 }
}
// /// Get the numerical value of a window
// pub const fn get_window_handle(window: &*mut c_void) -> HWND {
//     *window as winapi::shared::windef::HWND
// }
#[allow(clippy::not_unsafe_ptr_arg_deref)]
/// Wether a window is maximized on windows
pub fn is_window_maximized(hwnd: winapi::shared::windef::HWND) -> bool {
    unsafe { IsZoomed(hwnd) != 0 }
}
impl Iconized for WindowsActions {
    fn is_minimized(handle: &raw_window_handle::RawWindowHandle) -> bool {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            is_window_minimized(
                handle.hwnd.get() as winapi::shared::windef::HWND
            );
            true
        } else {
            false
        }
    }

    fn is_maximized(handle: &raw_window_handle::RawWindowHandle) -> bool {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            is_window_maximized(
                handle.hwnd.get() as winapi::shared::windef::HWND
            );
            true
        } else {
            false
        }
    }

    fn restore(handle: &raw_window_handle::RawWindowHandle) -> bool {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            restore(handle.hwnd.get() as winapi::shared::windef::HWND);
            true
        } else {
            false
        }
    }

    fn minimize(handle: &raw_window_handle::RawWindowHandle) -> bool {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            minimize(handle.hwnd.get() as winapi::shared::windef::HWND);
            true
        } else {
            false
        }
    }

    fn maximize(handle: &raw_window_handle::RawWindowHandle) -> bool {
        if let raw_window_handle::RawWindowHandle::Win32(handle) = handle {
            maximize(handle.hwnd.get() as winapi::shared::windef::HWND);
            true
        } else {
            false
        }
    }
}
