//! A simple collection of potentially useful things
#[cfg(feature = "font_support")]
pub use fontdue;

#[allow(unused_imports, unreachable_pub)]
#[cfg(feature = "std")]
pub use crate::platform::file_system::file_system_traits::*;
#[cfg(feature = "font_support")]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
pub use crate::platform::file_system::get_default_font;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "std")]
pub use crate::platform::file_system::FileSystem;
#[cfg(all(feature = "glfw", not(feature = "minifb")))]
#[cfg(not(target_arch = "wasm32"))]
pub use crate::platform::frameworks::glfw::Framework;
#[cfg(feature = "minifb")]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
#[cfg(any(not(target_os = "macos"), feature = "mac_cc_installed"))]
pub use crate::platform::frameworks::minifb::Framework;
#[cfg(feature = "std")]
#[cfg(feature = "system")]
#[cfg(feature = "keycodes")]
pub use crate::platform::frameworks::traits::*;
#[cfg(feature = "std")]
pub use crate::platform::Buffer;
#[cfg(feature = "std")]
pub use crate::render;
pub use crate::{extensions::*, platform::WindowSettings};
