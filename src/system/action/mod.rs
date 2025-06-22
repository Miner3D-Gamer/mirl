pub fn make_color_transparent(
    handle: &raw_window_handle::RawWindowHandle,
    color: (u8, u8, u8),
) -> bool {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows_actions::make_color_transparent_raw(
                handle.hwnd.get() as winapi::shared::windef::HWND,
                color,
            );
            true
        }
        _ => false,
    }
}

pub fn set_window_position(
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

pub fn set_window_borderless(
    handle: &raw_window_handle::RawWindowHandle,
    boolean: bool,
) -> bool {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            if boolean {
                make_window_borderless_raw(windows::Win32::Foundation::HWND(
                    handle.hwnd.get(),
                ));
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
pub fn set_window_hidden_from_taskbar(
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

pub fn set_window_level(
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

#[cfg(target_os = "windows")]
pub mod windows_actions;
#[cfg(target_os = "windows")]
pub use windows_actions::*;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

use crate::platform::WindowLevel;
