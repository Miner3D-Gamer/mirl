pub trait Info {
    fn get_os_name() -> String;
    fn get_total_memory(&self) -> u64;
    fn get_free_memory(&self) -> u64;
}
pub trait Screen {
    fn get_title_bar_height() -> i32;
    fn get_screen_resolution() -> (i32, i32);
}

pub trait Battery {
    fn get_battery_percentage(&self) -> Option<u8>;
    fn is_battery_charging() -> bool;
    fn is_in_low_power_mode() -> bool;
}
pub trait Clipboard {
    fn get_clipboard() -> ClipboardContents;
    fn set_clipboard();
}
pub trait Time {
    fn get_timezone_offset() -> i8;
}
pub trait Network {
    fn is_connected_to_wifi(website_connection: Option<String>) -> bool;
}

pub struct ClipboardContents {
    pub content_type: u8,
    pub text_content: Option<String>,
    pub image_content: Option<RawImage>,
    pub audio_content: Option<u8>,
    pub list_content: Option<Vec<String>>,
}

impl ClipboardContents {
    pub fn is_text(&self) -> bool {
        self.content_type == 1
    }
    pub fn is_image(&self) -> bool {
        self.content_type == 2
    }
    pub fn is_audio(&self) -> bool {
        self.content_type == 3
    }
    pub fn is_list(&self) -> bool {
        self.content_type == 4
    }
    pub fn get_as_text(&self) -> Option<String> {
        self.text_content.clone()
    }
    pub fn get_as_image(&self) -> Option<RawImage> {
        self.image_content.clone()
    }
    pub fn get_as_audio(&self) -> Option<u8> {
        self.audio_content.clone()
    }
    pub fn get_as_list(&self) -> Option<Vec<String>> {
        self.list_content.clone()
    }
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::WindowsInfo as OsInfo;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::windows::LinuxInfo as OsInfo;

use crate::graphics::RawImage;

pub fn get_center_of_screen_for_object(width: i32, height: i32) -> (i32, i32) {
    let title_bat_height = crate::system::info::OsInfo::get_title_bar_height();
    let (screen_width, screen_height) = OsInfo::get_screen_resolution();

    (
        screen_width / 2 - width / 2,
        screen_height / 2 - height / 2 - title_bat_height,
    )
}
