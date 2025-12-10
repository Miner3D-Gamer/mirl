/// Wrappers for active actions to take that would usually be OS specific
pub mod action;
pub use action::Os;
/// Information about the OS
pub mod deprecated;

/// Get the xy coordinates of where to put an object with the specified width and height for it to be centered
#[must_use]
#[cfg(any(target_arch = "wasm32", target_os = "linux", target_os = "windows"))]
pub fn get_center_of_screen_for_object(width: i32, height: i32) -> (i32, i32) {
    use crate::system::action::Screen;
    let (screen_width, screen_height): (i32, i32) = Os::get_screen_resolution();

    (screen_width / 2 - width / 2, screen_height / 2 - height / 2)
}

#[cfg(any(target_arch = "wasm32", target_os = "linux", target_os = "windows"))]
/// Get the xy coordinates of where to put the window associated with the [Buffer](crate::platform::Buffer) for it to be centered
#[must_use]
pub fn get_center_of_screen_of_buffer(
    buffer: &crate::platform::Buffer,
) -> (i32, i32) {
    use crate::system::action::Screen;
    let (screen_width, screen_height): (i32, i32) = Os::get_screen_resolution();

    (
        screen_width / 2 - buffer.width as i32 / 2,
        screen_height / 2 - buffer.height as i32 / 2,
    )
}
