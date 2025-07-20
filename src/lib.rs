#![warn(missing_docs)]
//! Miners
//! Rust
//! Lib
//! Mirl (Mirl sounded better than Mrl)
//!
//!
//! This lib has got a ton to offer but the main attractions are in here:
//! ## Window/Rendering Bundle (flags: `minifb_backend`/`glfw_backend`/`full_backend_support`):
//!
//! - [Frameworks](crate::platform::framework_traits) - What are they capable of? (for [crate::platform::minifb::Framework](crate::platform::minifb::Framework) or [crate::platform::glfw::Framework](crate::platform::glfw::Framework))
//! - [Buffer](crate::platform::Buffer) - The central struct many other functions rely on
//! - [Rendering](crate::render) - Render simple shapes
//! - [Platform](crate::platform) - Other neat stuff like [crate::platform::KeyCode]/[crate::platform::MouseButton], or [crate::platform::ScreenNormalizer]
//! - [System interaction](crate::system::action) - Functions that are untypical for usual applications like moving the window, getting/setting the z position, or hiding a window from the taskbar
//! - [Color Stuff](crate::graphics) - What is rendering without manipulating color?
//! - [Modular File System](crate::platform::FileSystem) - A custom file system wrapper to support file accessing on both natively and web
//! - [Rust functionality extension](crate::extensions) with a big focus yet not limited to new tuple functionality
//!
//! If anyone knows why the result of `cargo docs` is so inconsistent for crate links, please let me know

/// Directional stuff -> NESW, N NE E SE S SW W NW
pub mod directions;
/// Stuff that should exist by default yet doesn't; use mirl::extensions::*; to import all of 'em
pub mod extensions;
/// Stuff related to graphics -> Color manipulation
///
/// For rendering use [crate::render]
pub mod graphics;
/// Stuff related to lists
pub mod lists;
/// Math and collision focused stuff
pub mod math;
/// Stuff I didn't know how to categorize -> Expect these objects to be moved in the future
pub mod misc;
/// Window creation/managing, file system creation/managing
///
/// For actually rendering stuff, use [crate::render]
///
/// For basic collision, use [crate::math::collision]
pub mod platform;
/// Rendering stuff, simple but powerful (on [crate::platform::Buffer])
///
/// For color stuff, use [crate::graphics]
pub mod render;
/// Time related stuff
pub mod time;

/// Terminal stuff
#[cfg(not(target_arch = "wasm32"))]
pub mod console;

/// Unified os specific features
#[cfg(feature = "system")]
pub mod system;

/// Enables the rust traceback by setting the environment variable `RUST_BACKTRACE` to `1`
pub fn enable_traceback() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
}

/// Disables the rust traceback by setting the environment variable `RUST_BACKTRACE` to `0`
pub fn disable_traceback() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "0");
    }
}
