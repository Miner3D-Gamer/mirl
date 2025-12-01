//! A simple collection of potentially useful things
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
#[cfg(feature = "minifb")]
#[cfg(feature = "std")]
pub use crate::platform::minifb::Framework;
#[cfg(all(feature = "glfw", not(feature = "minifb")))]
pub use crate::platform::glfw::Framework;
#[cfg(feature = "std")]
pub use crate::platform::Buffer;
#[cfg(feature = "std")]
pub use crate::render;
