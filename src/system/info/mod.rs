/// Basic OS info
pub trait Info {
    /// The name of the os
    fn get_os_name() -> String;
    /// The total amount of memory
    fn get_total_memory(&self) -> u64;
    /// The currently remaining amount of memory
    fn get_free_memory(&self) -> u64;
}
/// Basic screen information
pub trait Screen {
    /// The os menu on top of windows
    fn get_os_menu_height() -> i32;
    /// Screen resolution
    fn get_screen_resolution() -> (i32, i32);
    /// Height of task bar
    fn get_taskbar_height() -> i32;
}
/// Basic Battery information
pub trait Battery {
    /// From 0 to 100
    fn get_battery_percentage(&self) -> Option<u8>;
    /// If the battery is currently charging
    fn is_battery_charging() -> bool;
    /// If the os is in low power mode
    fn is_in_low_power_mode() -> bool;
}
/// Basic Clipboard operations
// pub trait Clipboard {
//     fn get_clipboard() -> ClipboardContents;
//     fn set_clipboard();
// }
/// Basic time information (To get the current time, use std)
pub trait Time {
    /// Get the timezone offset -> How many hours over/under the standart time
    fn get_timezone_offset() -> i8;
}
/// Basic network information
pub trait Network {
    /// Tries to open a connection to the website, Default by None: http://example.com
    fn is_connected_to_internet(website_connection: Option<String>) -> bool;
}

// enum ClipboardContentType{
//     Text,
//     Image,
//     Audio,
//     ListOfStrings
// }
// struct ClipboardContents {
//     pub content_type: u8,
//     pub text_content: Option<String>,
//     pub image_content: Option<RawImage>,
//     pub audio_content: Option<u8>,
//     pub list_content: Option<Vec<String>>,
// }

// impl ClipboardContents {
//     pub fn is_text(&self) -> bool {
//         self.content_type == 1
//     }
//     pub fn is_image(&self) -> bool {
//         self.content_type == 2
//     }
//     pub fn is_audio(&self) -> bool {
//         self.content_type == 3
//     }
//     pub fn is_list(&self) -> bool {
//         self.content_type == 4
//     }
//     pub fn get_as_text(&self) -> Option<String> {
//         self.text_content.clone()
//     }
//     pub fn get_as_image(&self) -> Option<RawImage> {
//         self.image_content.clone()
//     }
//     pub fn get_as_audio(&self) -> Option<u8> {
//         self.audio_content.clone()
//     }
//     pub fn get_as_list(&self) -> Option<Vec<String>> {
//         self.list_content.clone()
//     }
// }

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::WindowsInfo as OsInfo;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::windows::LinuxInfo as OsInfo;

// use crate::graphics::RawImage;
/// Get the xy coordinates of where to put an object with the specified width and height for it to be centered
pub fn get_center_of_screen_for_object(width: i32, height: i32) -> (i32, i32) {
    let title_bat_height = crate::system::info::OsInfo::get_os_menu_height();
    let (screen_width, screen_height) = OsInfo::get_screen_resolution();

    (
        screen_width / 2 - width / 2,
        screen_height / 2 - height / 2 - title_bat_height,
    )
}
use crate::platform::Buffer;

/// Get the xy coordinates of where to put the window associated with the [Buffer] for it to be centered
pub fn get_center_of_screen_of_buffer(
    buffer: &Buffer,
) -> (i32, i32) {
    let title_bat_height = crate::system::info::OsInfo::get_os_menu_height();
    let (screen_width, screen_height) = OsInfo::get_screen_resolution();

    (
        screen_width / 2 - buffer.width as i32 / 2,
        screen_height / 2 - buffer.height as i32 / 2 - title_bat_height,
    )
}
