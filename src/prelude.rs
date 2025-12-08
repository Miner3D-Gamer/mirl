//! A simple collection of potentially useful things
#[cfg(feature = "font_support")]
pub use fontdue;

pub use crate::extensions::*;
#[allow(unused_imports, unreachable_pub)]
#[cfg(feature = "std")]
pub use crate::platform::file_system::file_system_traits::*;
#[cfg(feature = "font_support")]
#[cfg(feature = "std")]
pub use crate::platform::file_system::get_default_font;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "std")]
pub use crate::platform::file_system::FileSystem;
#[cfg(feature = "std")]
#[cfg(feature = "system")]
#[cfg(feature = "keycodes")]
pub use crate::platform::framework_traits::*;
#[cfg(all(feature = "glfw", not(feature = "minifb")))]
pub use crate::platform::glfw::Framework;
#[cfg(feature = "minifb")]
#[cfg(feature = "std")]
pub use crate::platform::minifb::Framework;
#[cfg(feature = "std")]
pub use crate::platform::Buffer;
pub use crate::platform::WindowSettings;
#[cfg(feature = "std")]
pub use crate::render;
