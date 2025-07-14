/// Culls the given color -> Essentially just a green screen
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
/// Set the position of a window, ¯\_(ツ)_/¯
pub fn set_window_position(
    handle: &raw_window_handle::RawWindowHandle,
    x: i32,
    y: i32,
) -> bool {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows_actions::set_window_position_raw(
                windows::Win32::Foundation::HWND(handle.hwnd.get()),
                x,
                y,
            );
            true
        }
        _ => false,
    }
}
/// Remove/Give a window their border
pub fn set_window_borderless(
    handle: &raw_window_handle::RawWindowHandle,
    boolean: bool,
) -> bool {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            if boolean {
                windows_actions::make_window_borderless_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                );
            } else {
                windows_actions::give_window_a_border_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                );
            }
            true
        }
        _ => false,
    }
}
/// The title says it all
pub fn set_window_hidden_from_taskbar_and_alt_tab(
    handle: &raw_window_handle::RawWindowHandle,
    boolean: bool,
) -> bool {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            if boolean {
                windows_actions::hide_from_taskbar_and_alt_tab_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                );
            } else {
                windows_actions::show_in_taskbar_and_alt_tab_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                );
            }
            true
        }
        _ => false,
    }
}
/// How the os should go about ordering this window, check the documentation of [WindowLevel] for more information
pub fn set_window_level(
    handle: &raw_window_handle::RawWindowHandle,
    level: WindowLevel,
) -> bool {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows_actions::set_window_level_raw(
                windows::Win32::Foundation::HWND(handle.hwnd.get()),
                level,
            );
            true
        }
        _ => false,
    }
}
/// Get ALL windows the os allows for
pub fn get_all_windows() -> Vec<raw_window_handle::RawWindowHandle> {
    match crate::system::info::OsInfo::get_os_name().as_str() {
        "windows" => {
            let windows = windows_actions::get_all_windows_raw();
            let mut new = Vec::new();
            for i in windows {
                new.push(raw_window_handle::RawWindowHandle::Win32(
                    raw_window_handle::Win32WindowHandle::new(
                        std::num::NonZero::new(i.0 as isize).unwrap(),
                    ),
                ));
            }
            new
        }
        _ => Vec::new(),
    }
}
/// Get the current position of a window
pub fn get_window_position(
    handle: &raw_window_handle::RawWindowHandle,
) -> (i32, i32) {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows_actions::get_window_position_raw(
                windows::Win32::Foundation::HWND(handle.hwnd.get()),
            )
            .unwrap_or((i32::MIN, i32::MIN))
        }
        _ => (i32::MIN, i32::MIN),
    }
}
/// Get the current size of a window
pub fn get_window_size(
    handle: &raw_window_handle::RawWindowHandle,
) -> (i32, i32) {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows_actions::get_window_size_raw(
                windows::Win32::Foundation::HWND(handle.hwnd.get()),
            )
            .unwrap_or((i32::MIN, i32::MIN))
        }
        _ => (i32::MIN, i32::MIN),
    }
}
/// Get the title of a window
pub fn get_title_using_id(
    handle: &raw_window_handle::RawWindowHandle,
) -> String {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows_actions::get_title_using_id_raw(
                handle.hwnd.get() as winapi::shared::windef::HWND
            )
        }
        _ => "".into(),
    }
}
/// Get the title of a window
pub fn get_id_using_title(
    title: &str,
    exact_match: bool,
    case_sensitive: bool,
    include_hidden: bool,
    just_one: bool,
) -> Option<Vec<raw_window_handle::RawWindowHandle>> {
    return get_window_id_by_title(
        title,
        exact_match,
        case_sensitive,
        include_hidden,
        just_one,
    );
}
/// Get the screen resolution - What happens if you have multiple monitors? Idk
pub fn get_screen_resolution() -> (i32, i32) {
    return get_screen_resolution_raw();
}
/// Capture the screen with all application - What happens if you have multiple monitors? Idk
pub fn capture_screen() -> Option<RawImage> {
    return capture_screen_raw();
}
/// Capture the desktop background without any applications
pub fn capture_desktop_background() -> Option<RawImage> {
    return capture_desktop_background_raw();
}
/// Sets if you can click through a window
pub fn set_click_ability_of_window(
    handle: &raw_window_handle::RawWindowHandle,
    click_through: bool,
) {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            if click_through {
                windows_actions::make_window_click_through_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                );
            } else {
                windows_actions::make_window_click_solid_raw(
                    windows::Win32::Foundation::HWND(handle.hwnd.get()),
                )
            }
        }
        _ => {}
    }
}

/// Sets the opacity ¯\_(ツ)_/¯
pub fn set_window_opacity(
    handle: &raw_window_handle::RawWindowHandle,
    opacity: u8,
) -> bool {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows_actions::set_window_opacity_raw(
                windows::Win32::Foundation::HWND(handle.hwnd.get()),
                opacity,
            );
            true
        }
        _ => false,
    }
}

/// Get the current z ordering of a window
pub fn get_window_z(handle: &raw_window_handle::RawWindowHandle) -> u32 {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows_actions::get_z_raw(windows::Win32::Foundation::HWND(
                handle.hwnd.get(),
            ))
            .unwrap_or(u32::MIN)
        }
        _ => u32::MIN,
    }
}
/// Get the current z ordering of a window
pub fn get_task_bar_height() -> i32 {
    get_title_bar_height_raw()
}
/// Sets the z ordering of the current window - How does [WindowLevel] affect ordering? No clue.
pub fn set_window_z(
    handle: &raw_window_handle::RawWindowHandle,
    z: u32,
) -> bool {
    match handle {
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            windows_actions::set_z_raw(
                windows::Win32::Foundation::HWND(handle.hwnd.get()),
                z,
            );
            true
        }
        _ => false,
    }
}
/// Sets the z ordering of the current window - How does [WindowLevel] affect ordering? No clue.
pub fn set_window_z_after(
    handle: &raw_window_handle::RawWindowHandle,
    after: &raw_window_handle::RawWindowHandle,
) -> bool {
    // Matching 2 values at the same time feels cursed
    match (handle, after) {
        (
            raw_window_handle::RawWindowHandle::Win32(handle),
            raw_window_handle::RawWindowHandle::Win32(after),
        ) => {
            windows_actions::set_z_to_after_raw(
                windows::Win32::Foundation::HWND(handle.hwnd.get()),
                windows::Win32::Foundation::HWND(after.hwnd.get()),
            );
            true
        }
        _ => false,
    }
}

#[cfg(target_os = "windows")]
mod windows_actions;
#[cfg(target_os = "windows")]
use windows_actions::{
    capture_desktop_background_raw, capture_screen_raw,
    get_screen_resolution_raw, get_title_bar_height_raw,
    get_window_id_by_title,
};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::{
    capture_desktop_background_raw, capture_screen_raw,
    get_screen_resolution_raw, get_window_id_by_title,
};

use crate::{graphics::RawImage, platform::WindowLevel, system::info::Info};
