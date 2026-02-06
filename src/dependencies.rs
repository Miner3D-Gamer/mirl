#![allow(clippy::non_minimal_cfg)]
#![allow(unreachable_pub)]

#[cfg(any(feature = "console"))]
pub use crossterm;
#[cfg(any(feature = "font_support"))]
pub use font_kit;
#[cfg(any(feature = "font_support"))]
pub use fontdue;
#[cfg(any(feature = "imagery"))]
pub use image;
#[cfg(any(feature = "num_traits"))]
pub use num_traits;
#[cfg(any(feature = "font_support"))]
pub use once_cell;
#[cfg(any(feature = "font_support"))]
pub use parking_lot;
#[cfg(any(feature = "system"))]
pub use raw_window_handle;
#[cfg(any(feature = "svg"))]
pub use resvg;
#[cfg(any(feature = "discord"))]
pub use serde;
#[cfg(any(feature = "discord"))]
pub use serde_json;
#[cfg(any(feature = "keycodes"))]
pub use strum;
#[cfg(any(feature = "keycodes"))]
pub use strum_macros;

#[cfg(any(feature = "system"))]
#[cfg(target_os = "windows")]
mod w {
    pub use winapi;
    pub use windows;
}
#[cfg(any(feature = "system"))]
#[cfg(target_os = "windows")]
#[allow(unused_imports)]
pub use w::*;

#[cfg(any(feature = "system"))]
#[cfg(target_os = "linux")]
mod l {
    pub use x11;
}
#[cfg(any(feature = "system"))]
#[cfg(target_os = "linux")]
#[allow(unused_imports)]
pub use l::*;

#[cfg(not(target_arch = "wasm32"))]
mod n_w {
    #[cfg(any(feature = "ahash"))]
    pub use ahash;
    #[cfg(any(feature = "keyboard_query"))]
    pub use device_query;
    #[cfg(any(feature = "glfw"))]
    pub use gl;
    #[cfg(any(feature = "glfw"))]
    pub use glfw;
    #[cfg(any(feature = "random"))]
    pub use rand;
    #[cfg(any(feature = "discord"))]
    pub use reqwest;
    #[cfg(any(feature = "svg"))]
    pub use tempfile;
}

#[cfg(not(target_os = "linux"))]
#[allow(unused_imports)]
pub use n_w::*;

#[cfg(target_arch = "wasm32")]
mod w {
    pub use wasm_bindgen;
    pub use web_sys;
}

#[cfg(not(target_os = "macos"))]
#[cfg(any(feature = "minifb"))]
pub use minifb;
#[cfg(target_arch = "wasm32")]
#[allow(unused_imports)]
pub use w::*;
