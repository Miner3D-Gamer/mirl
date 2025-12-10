// Windows
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "glfw")]
#[cfg(feature = "keycodes")]
/// The glfw version of the backend
pub mod glfw;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(any(not(target_os = "macos"), feature = "mac_cc_installed"))]
#[cfg(feature = "minifb")]
/// The minifb version of the backend
pub mod minifb;
#[cfg(feature = "system")]
/// Traits used by the backends
pub mod traits;

mod errors;
pub use errors::*;
