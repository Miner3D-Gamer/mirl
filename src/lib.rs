#![allow(dead_code)]

// enable feature "imagery"

// Miners Rust Lib
// Mirl (Mirl sounded better than Mrl)
pub mod graphics;
pub mod math;
pub mod misc;
pub mod time;
//pub mod dictionary;
pub mod extensions;
pub mod lists;
pub mod platform;
pub mod render;

#[cfg(not(target_arch = "wasm32"))]
pub mod console;
#[cfg(not(target_arch = "wasm32"))]
pub mod system;
