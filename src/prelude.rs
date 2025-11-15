//! A simple collection of potentially useful things
pub use crate::extensions::*;
#[allow(unused_imports, unreachable_pub)]
pub use crate::platform::file_system::file_system_traits::*;
#[cfg(feature = "font_support")]
pub use crate::platform::file_system::get_default_font;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::platform::file_system::FileSystem;
pub use crate::platform::framework_traits::*;
#[cfg(feature = "minifb_backend")]
pub use crate::platform::minifb::Framework;
#[cfg(all(feature = "glfw_backend", not(feature = "minifb_backend")))]
pub use crate::platform::minifb::Framework;
pub use crate::platform::Buffer;
pub use crate::render;
