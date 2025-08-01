/// The simplest actions you would expect from interacting with the os
pub trait Default {
    /// Set the position of a window, ¯\_(ツ)_/¯
    fn set_window_position(
        handle: &raw_window_handle::RawWindowHandle,
        x: i32,
        y: i32,
    ) -> bool;
    /// How the os should go about ordering this window, check the documentation of [WindowLevel] for more information
    fn set_window_level(
        handle: &raw_window_handle::RawWindowHandle,
        level: WindowLevel,
    ) -> bool;
    /// Get the current position of a window
    fn get_window_position(
        handle: &raw_window_handle::RawWindowHandle,
    ) -> (i32, i32);
    /// Get the current size of a window
    fn get_window_size(
        handle: &raw_window_handle::RawWindowHandle,
    ) -> (i32, i32);
}

/// Transparency information/Manipulation
pub trait Transparency {
    /// Culls the given color -> Essentially just a green screen
    fn make_color_transparent(
        handle: &raw_window_handle::RawWindowHandle,
        color: (u8, u8, u8),
    ) -> bool;
    /// Sets the opacity ¯\_(ツ)_/¯
    fn set_window_opacity(
        handle: &raw_window_handle::RawWindowHandle,
        opacity: u8,
    ) -> bool;
}
/// Decoration like os menu manipulation
pub trait Decoration {
    /// Remove/Give a window their border
    fn set_window_borderless(
        handle: &raw_window_handle::RawWindowHandle,
        boolean: bool,
    ) -> bool;
}
/// Stuff I couldn't categorize yet
pub trait Misc {
    /// The title says it all
    fn set_window_hidden_from_taskbar_and_alt_tab(
        handle: &raw_window_handle::RawWindowHandle,
        boolean: bool,
    ) -> bool;
    /// Get ALL windows the os reveals
    fn get_all_windows() -> Vec<raw_window_handle::RawWindowHandle>;
    /// Get the title of the application associated with the given id
    fn get_title_using_id(
        handle: &raw_window_handle::RawWindowHandle,
    ) -> String;
    /// Get the title of a window
    fn get_id_using_title(
        title: &str,
        exact_match: bool,
        case_sensitive: bool,
        include_hidden: bool,
        just_one: bool,
    ) -> Option<Vec<raw_window_handle::RawWindowHandle>>;
    /// Capture the screen with all application - What happens if you have multiple monitors? Idk
    fn capture_screen() -> Option<Buffer>;
    /// Capture the desktop background without any applications
    fn capture_desktop_background() -> Option<Buffer>;
    /// Sets if you can click through a window
    fn set_click_ability_of_window(
        handle: &raw_window_handle::RawWindowHandle,
        click_through: bool,
    );
    /// Get the current z ordering of a window
    fn get_window_z(handle: &raw_window_handle::RawWindowHandle) -> u32;

    /// Sets the z ordering of the current window - How does [WindowLevel] affect ordering? No clue.
    fn set_window_z(
        handle: &raw_window_handle::RawWindowHandle,
        z: u32,
    ) -> bool;
    /// Sets the z ordering of the current window - How does [WindowLevel] affect ordering? No clue.
    fn set_window_z_after(
        handle: &raw_window_handle::RawWindowHandle,
        after: &raw_window_handle::RawWindowHandle,
    ) -> bool;
}

#[cfg(target_os = "windows")]
mod windows_actions;
#[cfg(target_os = "windows")]
pub use windows_actions::WindowsActions as OsActions;

#[cfg(target_os = "linux")]
mod linux_actions;
#[cfg(target_os = "linux")]
use linux_actions::{
    capture_desktop_background_raw, capture_screen_raw, get_window_id_by_title,
};

#[cfg(target_arch = "wasm32")]
mod web_actions;
#[cfg(target_arch = "wasm32")]
use web_actions::{
    capture_desktop_background_raw, capture_screen_raw, get_window_id_by_title,
};

use crate::platform::{Buffer, WindowLevel};
