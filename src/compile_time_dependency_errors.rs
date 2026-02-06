// imagery = ["dep:image"]
// no feature dependencies; no guard needed

// #[cfg(all(
//     feature = "minifb",
//     not(feature = "mac_cc_installed"),
//     target_os = "macos"
// ))]
// use crate::compile_warn;

// texture_manager_cleanup = ["std"]
#[cfg(all(feature = "texture_manager_cleanup", not(feature = "std")))]
compile_error!("feature `texture_manager_cleanup` requires `std`");

// ahash = ["dep:ahash", "std"]
#[cfg(all(feature = "ahash", not(feature = "std")))]
compile_error!("feature `ahash` requires `std`");

// svg = ["dep:resvg", "dep:tempfile"]
// no feature dependencies; no guard needed

// all_backends = ["minifb", "glfw", "font_support"]
#[cfg(all(feature = "all_backends", not(feature = "minifb")))]
compile_error!("feature `all_backends` requires `minifb`");
#[cfg(all(feature = "all_backends", not(feature = "glfw")))]
compile_error!("feature `all_backends` requires `glfw`");
#[cfg(all(feature = "all_backends", not(feature = "font_support")))]
compile_error!("feature `all_backends` requires `font_support`");

// minifb = ["dep:minifb", "system", "keycodes", "keyboard_query", "svg", "std"]
#[cfg(all(feature = "minifb", not(feature = "system")))]
compile_error!("feature `minifb` requires `system`");
#[cfg(all(feature = "minifb", not(feature = "keycodes")))]
compile_error!("feature `minifb` requires `keycodes`");
#[cfg(all(feature = "minifb", not(feature = "keyboard_query")))]
compile_error!("feature `minifb` requires `keyboard_query`");
#[cfg(all(feature = "minifb", not(feature = "svg")))]
compile_error!("feature `minifb` requires `svg`");
#[cfg(all(feature = "minifb", not(feature = "std")))]
compile_error!("feature `minifb` requires `std`");

// glfw = ["dep:glfw", "dep:gl", "system", "keycodes", "std"]
#[cfg(all(feature = "glfw", not(feature = "system")))]
compile_error!("feature `glfw` requires `system`");
#[cfg(all(feature = "glfw", not(feature = "keycodes")))]
compile_error!("feature `glfw` requires `keycodes`");
#[cfg(all(feature = "glfw", not(feature = "std")))]
compile_error!("feature `glfw` requires `std`");

// system = ["x11", "dep:windows", "dep:winapi", "dep:raw-window-handle", "std"]
#[cfg(all(feature = "system", not(feature = "x11")))]
compile_error!("feature `system` requires `x11`");
#[cfg(all(feature = "system", not(feature = "std")))]
compile_error!("feature `system` requires `std`");

// font_support = ["dep:fontdue", "dep:once_cell", "dep:font-kit", "std", "dep:parking_lot"]
#[cfg(all(feature = "font_support", not(feature = "std")))]
compile_error!("feature `font_support` requires `std`");

// keycodes = ["dep:strum", "dep:strum_macros", "std"]
#[cfg(all(feature = "keycodes", not(feature = "std")))]
compile_error!("feature `keycodes` requires `std`");

// keyboard_query = ["dep:device_query", "std"]
#[cfg(all(feature = "keyboard_query", not(feature = "std")))]
compile_error!("feature `keyboard_query` requires `std`");

// cursor_show_hotspot = []
// no guard needed

// discord = ["dep:serde_json", "dep:reqwest", "dep:serde", "std"]
#[cfg(all(feature = "discord", not(feature = "std")))]
compile_error!("feature `discord` requires `std`");

// random = ["dep:rand", "std"]
#[cfg(all(feature = "random", not(feature = "std")))]
compile_error!("feature `random` requires `std`");

// std = []
// no guard needed

// num_traits = ["dep:num-traits", "std"]
#[cfg(all(feature = "num_traits_with_std", not(feature = "std")))]
compile_error!("feature `num_traits_with_std` requires `std`");
#[cfg(all(feature = "num_traits_with_std", not(feature = "num_traits")))]
compile_error!("feature `num_traits_with_std` requires `num_traits`");

// #[cfg(all(
//     feature = "minifb",
//     not(feature = "mac_cc_installed"),
//     target_os = "macos"
// ))]
// compile_warn!("To compile minifb on mac, see https://docs.rs/cc/latest/cc/#compile-time-requirements");

#[cfg(test)]
#[cfg(not(feature = "test"))]
compile_error!("You're trying to test the lib without having the required dependencies enabled");
