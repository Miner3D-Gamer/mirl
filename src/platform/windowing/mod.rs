// Windows
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "glfw")]
#[cfg(feature = "keycodes")]
/// The glfw version of the backend
pub mod glfw;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "minifb")]
/// The minifb version of the backend
pub mod minifb;
/// Traits used by the backends
pub mod traits;

#[cfg(feature = "console")]
/// A console renderer
pub mod console;

mod errors;
pub use errors::*;
