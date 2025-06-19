//use std;
pub struct ScreenImage {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<u32>,
}

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
