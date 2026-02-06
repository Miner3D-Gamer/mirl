//! A simple collection of potentially useful things

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
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "console")]
#[cfg(feature = "std")]
pub use crate::platform::windowing::console;
#[cfg(all(not(feature = "glfw"), not(feature = "minifb")))]
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "console")]
#[cfg(feature = "std")]
pub use crate::platform::windowing::console::Framework;
#[cfg(feature = "glfw")]
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "std")]
pub use crate::platform::windowing::glfw;
#[cfg(all(feature = "glfw", not(feature = "minifb")))]
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "std")]
pub use crate::platform::windowing::glfw::Framework;
#[cfg(feature = "minifb")]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
pub use crate::platform::windowing::minifb;
#[cfg(feature = "minifb")]
#[cfg(feature = "std")]
#[cfg(not(target_arch = "wasm32"))]
pub use crate::platform::windowing::minifb::Framework;
#[cfg(feature = "std")]
pub use crate::platform::windowing::traits::*;
#[cfg(feature = "std")]
pub use crate::render;
#[cfg(feature = "std")]
pub use crate::render::Buffer;
pub use crate::{
    extensions::*,
    math::{ConstNumbers128, ConstOne, ConstZero},
    platform::WindowSettings,
    render::ConstBuffer,
};
