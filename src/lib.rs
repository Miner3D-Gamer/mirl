#![warn(missing_docs)]
//! Miners Rust Lib
//! Mirl (Mirl sounded better than Mrl)

/// Stuff related to graphics -> Color manipulation
/// For rendering use mirl::render
pub mod graphics;
/// Math and collision foused stuff
pub mod math;
/// Stuff I didn't know how to categorize -> Expect these functions to be moved in the future
pub mod misc;
/// Time related stuff
pub mod time;
//pub mod dictionary;
/// Directional stuff -> NESW, N NE E SE S SW W NW
pub mod directions;
/// Stuff that should exist by default yet doesn't; use mirl::extensions::*; to import all of 'em
pub mod extensions;
/// Stuff related to lists
pub mod lists;
/// Window creation/managing, file system creation/managing
/// For actually rendering stuff, use mirl::render
/// For basic collision, use mirl::math::collision
pub mod platform;
/// Rendering stuff (on mirl::platform::Buffer)
/// For color stuff, use mirl::graphics
pub mod render;


/// Terminal stuff
#[cfg(not(target_arch = "wasm32"))]
pub mod console;

/// Unified os specific features
#[cfg(feature = "system")]
pub mod system;