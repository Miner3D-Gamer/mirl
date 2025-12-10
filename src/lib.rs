#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::expect_used)]
#![warn(clippy::todo)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::needless_doctest_main)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::inline_always)]
#![allow(clippy::overly_complex_bool_expr)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::fn_params_excessive_bools)]
#![allow(internal_features)]
#![feature(const_trait_impl)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_try)]
#![feature(core_intrinsics)]
#![feature(const_option_ops)]
#![feature(core_float_math)]
#![feature(f128)]
#![cfg_attr(not(feature = "std"), no_std)]
//! Miners
//! Rust
//! Lib
//! Mirl (Mirl sounded better than Mrl)
//!
//! How to get started:
//! ```
//! use mirl::platform::framework_traits::Window;
//! fn main() {
//!     let mut buffer = mirl::platform::Buffer::new_empty((800, 600));
//!     let mut window = mirl::platform::minifb::Framework::new(
//!         "Example window",
//!         mirl::platform::WindowSettings::default(&buffer),
//!     )
//!     .unwrap();
//!     while window.is_open() {
//!         buffer.clear();
//!
//!         // Draw here, use mirl::render for simple presets/helper functions
//!
//!         window.update(&buffer);
//!     }
//! }
//! ```
//! For a debugging window lib "similar" to `Dear ImGui` you can use the `dear_mirl_gui` crate (which is `RmMode`)
//!
//! This lib has got a ton to offer but the main attractions are in here:
//! ## Window/Rendering Bundle (flags: `minifb`/`glfw`/`all_backends`):
//!
//! - [Frameworks](crate::platform::framework_traits) - What are they capable of? (for [`crate::platform::minifb::Framework`] or [`crate::platform::glfw::Framework`])
//! - [Buffer] - The central struct many other functions rely on
//! - [Rendering](crate::render) - Render simple shapes
//! - [Platform](crate::platform) - Other neat stuff like [`crate::platform::keycodes::KeyCode`]/[`crate::platform::MouseButton`], or [`crate::platform::ScreenNormalizer`]
//! - [System interaction](crate::system::action) - Functions that are untypical for usual applications like moving the window, getting/setting the z position, or hiding a window from the taskbar
//! - [Color Stuff](crate::graphics) - What is rendering without color manipulation?
//! - [Modular File System](crate::platform::file_system::FileSystem) - A custom file system wrapper to support file accessing on web and natively
//! - [Rust functionality extension](crate::extensions) - Adding to rust what should've been there in the first place, especially tuple operations

/// Directional stuff -> NESW, N NE E SE S SW W NW
pub mod directions;

/// Stuff that should exist by default yet doesn't; use `mirl::extensions::*;` to import all of 'em
pub mod extensions;
/// Stuff related to graphics -> Color manipulation
///
/// For rendering use [`mirl::render`](crate::render)
pub mod graphics;

/// Math and collision focused stuff
pub mod math;

/// Stuff I didn't know how to categorize -> Expect these objects to be moved in the future
pub mod misc;
/// Window creation/managing, file system creation/managing
///
/// For actually rendering stuff, use [`mirl::render`](crate::render)
///
/// For basic collision, use [`mirl::math::collision`](crate::math::collision)
pub mod platform;
#[cfg(feature = "std")]
/// Rendering stuff, simple but powerful (on [`mirl::platform::Buffer`](crate::platform::Buffer))
///
/// For color stuff, use [`mirl::graphics`](crate::graphics)
pub mod render;
/// Time related stuff
pub mod time;

/// Terminal stuff
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
pub mod console;

/// Unified os specific features
#[cfg(feature = "std")]
#[cfg(feature = "system")]
pub mod system;

/// Functions for interacting with computers outside this one
pub mod network;

/// Useful constants -> std contains some of these internally yet doesn't expose them for anyone else to use
pub mod constants;
#[cfg(feature = "std")]
/// Enables the rust traceback by setting the environment variable `RUST_BACKTRACE` to `1`
pub fn enable_traceback() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
}
#[cfg(feature = "std")]
/// Enables the extended rust traceback by setting the environment variable `RUST_BACKTRACE` to `FULL`
pub fn enable_traceback_detailed() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "FULL");
    }
}
#[cfg(feature = "std")]
/// Disables the rust traceback by setting the environment variable `RUST_BACKTRACE` to `0`
pub fn disable_traceback() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "0");
    }
}
#[cfg(feature = "std")]
// This little fella is so often used that it only makes sense to reexport it
pub use platform::Buffer;

/// Defaults
pub mod prelude;

/// Settings to be applied throughout the lib
pub mod settings;

// TODO:
// - Clipboard support
// - Sound support
// - Networking support?
// - Linux Compatibility

// TODON'T:
// - Mac support

mod compile_time_dependency_errors;

// Web support
// Incompatible: ahash, device_query
