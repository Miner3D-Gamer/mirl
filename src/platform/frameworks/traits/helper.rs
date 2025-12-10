use super::{super::super::Buffer, *};
use crate::{extensions::*, platform::frameworks::WindowError};
/// Most basic of framework functionality
pub const trait Framework: Window + MouseInput + Output + Timing {}
impl<T: Window + MouseInput + Output + Timing> Framework for T {}

#[cfg(feature = "svg")]
/// Framework with all functionality it could support
pub const trait ExtendedFramework:
    Framework
    + ExtendedWindow
    + Control
    + Visibility
    + LoadCursorStyle
    + UseCursorStyle
    + IconControl
    + RenderLayer
    + MouseInput
    + ExtendedMouseInput
    + KeyboardInput
    + ExtendedKeyboardInput
{
}
#[cfg(feature = "svg")]
impl<T> ExtendedFramework for T where
    T: Framework
        + ExtendedWindow
        + Control
        + Visibility
        + LoadCursorStyle
        + UseCursorStyle
        + IconControl
        + RenderLayer
        + MouseInput
        + ExtendedMouseInput
        + KeyboardInput
        + ExtendedKeyboardInput
{
}
#[cfg(not(feature = "svg"))]
/// Framework with all functionality it could support
#[const_trait]
pub trait ExtendedFramework:
    Framework + ExtendedMouseInput + ExtendedWindow + Control + Visibility
{
}
#[cfg(not(feature = "svg"))]
impl<T> ExtendedFramework for T where
    T: Framework + ExtendedMouseInput + ExtendedWindow + Control + Visibility
{
}

/// A simple helper that implements functions that mustn't be written by hand
pub const trait WindowHelper {
    /// Update what the current window displays using a Buffer
    ///
    /// # Errors
    /// See [Errors]
    fn update_with_buffer(&mut self, buffer: &Buffer) -> Result<(), WindowError>;
}
impl<T: Window> WindowHelper for T {
    fn update_with_buffer(&mut self, buffer: &Buffer) -> Result<(), WindowError> {
        self.update_raw(buffer, buffer.width, buffer.height)
    }
}
/// A helper that is
pub const trait DeprecatedCompatibilityHelper {
    /// This function is deprecated as it can crash the program when the window has been resized.
    ///
    /// It is recommended to use [`Window::update_raw`] or [`WindowHelper::update_with_buffer`] instead
    /// # Errors
    /// See [Errors]
    #[deprecated]
    fn update(&mut self, buffer: &[u32]) -> Result<(), WindowError>;
}

impl<T: Window + Control> DeprecatedCompatibilityHelper for T {
    fn update(&mut self, buffer: &[u32]) -> Result<(), WindowError> {
        let Some(size) = self.get_size().try_tuple_into() else {
            return Err(WindowError::Misc("Window size negative".to_string()));
        };
        self.update_raw(buffer, size.0, size.1)
    }
}
/// Get the relative mouse position
pub const trait RelativeMousePos {
    /// Get the mouse position relative to the window
    fn get_mouse_position_relative(&self) -> Option<(i32, i32)>;
}

impl<T: MouseInput + Control> RelativeMousePos for T {
    fn get_mouse_position_relative(&self) -> Option<(i32, i32)> {
        let mouse_pos = self.get_mouse_position()?;
        let window_pos = self.get_position();
        Some(mouse_pos.add(window_pos))
    }
}
