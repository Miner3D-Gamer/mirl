use super::{super::super::Buffer, *};
use crate::{extensions::*, platform::windowing::WindowError};
/// Most basic of framework functionality
#[cfg(feature = "keycodes")]
pub const trait WindowingFramework:
    Window
    + MouseInput
    + KeyboardInput
    + Output
    + Timing
    + WindowHelper
    + core::fmt::Debug
{
}
#[cfg(feature = "keycodes")]
impl<
        T: Window
            + MouseInput
            + KeyboardInput
            + Output
            + Timing
            + WindowHelper
            + core::fmt::Debug,
    > WindowingFramework for T
{
}
/// Most basic of framework functionality
#[cfg(not(feature = "keycodes"))]
pub const trait WindowingFramework:
    Window + MouseInput + Output + Timing + WindowHelper + core::fmt::Debug
{
}
#[cfg(not(feature = "keycodes"))]
impl<
        T: Window + MouseInput + Output + Timing + WindowHelper + core::fmt::Debug,
    > WindowingFramework for T
{
}
// All features enabled
#[cfg(all(feature = "svg", feature = "system", feature = "keycodes"))]
/// Framework with all functionality it could support
pub const trait ExtendedWindowingFramework:
    WindowingFramework
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
    + GetWindowHandle
{
}
#[cfg(all(feature = "svg", feature = "system", feature = "keycodes"))]
impl<T> ExtendedWindowingFramework for T where
    T: WindowingFramework
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
        + GetWindowHandle
{
}

// svg + system, no keycodes
#[cfg(all(feature = "svg", feature = "system", not(feature = "keycodes")))]
/// Framework with all functionality it could support
pub const trait ExtendedWindowingFramework:
    WindowingFramework
    + ExtendedWindow
    + Control
    + Visibility
    + LoadCursorStyle
    + UseCursorStyle
    + IconControl
    + RenderLayer
    + MouseInput
    + ExtendedMouseInput
    + GetWindowHandle
{
}
#[cfg(all(feature = "svg", feature = "system", not(feature = "keycodes")))]
impl<T> ExtendedWindowingFramework for T where
    T: WindowingFramework
        + ExtendedWindow
        + Control
        + Visibility
        + LoadCursorStyle
        + UseCursorStyle
        + IconControl
        + RenderLayer
        + MouseInput
        + ExtendedMouseInput
        + GetWindowHandle
{
}

// svg + keycodes, no system
#[cfg(all(feature = "svg", not(feature = "system"), feature = "keycodes"))]
/// Framework with all functionality it could support
pub const trait ExtendedWindowingFramework:
    WindowingFramework
    + ExtendedWindow
    + Control
    + Visibility
    + IconControl
    + RenderLayer
    + MouseInput
    + ExtendedMouseInput
    + KeyboardInput
    + ExtendedKeyboardInput
{
}
#[cfg(all(feature = "svg", not(feature = "system"), feature = "keycodes"))]
impl<T> ExtendedWindowingFramework for T where
    T: WindowingFramework
        + ExtendedWindow
        + Control
        + Visibility
        + IconControl
        + RenderLayer
        + MouseInput
        + ExtendedMouseInput
        + KeyboardInput
        + ExtendedKeyboardInput
{
}

// system + keycodes, no svg
#[cfg(all(not(feature = "svg"), feature = "system", feature = "keycodes"))]
/// Framework with all functionality it could support
pub const trait ExtendedWindowingFramework:
    WindowingFramework
    + ExtendedWindow
    + Control
    + Visibility
    + IconControl
    + RenderLayer
    + MouseInput
    + ExtendedMouseInput
    + KeyboardInput
    + ExtendedKeyboardInput
    + GetWindowHandle
{
}
#[cfg(all(not(feature = "svg"), feature = "system", feature = "keycodes"))]
impl<T> ExtendedWindowingFramework for T where
    T: WindowingFramework
        + ExtendedWindow
        + Control
        + Visibility
        + IconControl
        + RenderLayer
        + MouseInput
        + ExtendedMouseInput
        + KeyboardInput
        + ExtendedKeyboardInput
        + GetWindowHandle
{
}

// svg only
#[cfg(all(
    feature = "svg",
    not(feature = "system"),
    not(feature = "keycodes")
))]
/// Framework with all functionality it could support
pub const trait ExtendedWindowingFramework:
    WindowingFramework
    + ExtendedWindow
    + Control
    + Visibility
    + IconControl
    + RenderLayer
    + MouseInput
    + ExtendedMouseInput
{
}
#[cfg(all(
    feature = "svg",
    not(feature = "system"),
    not(feature = "keycodes")
))]
impl<T> ExtendedWindowingFramework for T where
    T: WindowingFramework
        + ExtendedWindow
        + Control
        + Visibility
        + IconControl
        + RenderLayer
        + MouseInput
        + ExtendedMouseInput
{
}

// system only
#[cfg(all(
    not(feature = "svg"),
    feature = "system",
    not(feature = "keycodes")
))]
/// Framework with all functionality it could support
pub const trait ExtendedWindowingFramework:
    WindowingFramework
    + ExtendedWindow
    + Control
    + Visibility
    + IconControl
    + RenderLayer
    + MouseInput
    + ExtendedMouseInput
    + GetWindowHandle
{
}
#[cfg(all(
    not(feature = "svg"),
    feature = "system",
    not(feature = "keycodes")
))]
impl<T> ExtendedWindowingFramework for T where
    T: WindowingFramework
        + ExtendedWindow
        + Control
        + Visibility
        + IconControl
        + RenderLayer
        + MouseInput
        + ExtendedMouseInput
        + GetWindowHandle
{
}

// keycodes only
#[cfg(all(
    not(feature = "svg"),
    not(feature = "system"),
    feature = "keycodes"
))]
/// Framework with all functionality it could support
pub const trait ExtendedWindowingFramework:
    WindowingFramework
    + ExtendedWindow
    + Control
    + Visibility
    + IconControl
    + RenderLayer
    + MouseInput
    + ExtendedMouseInput
    + KeyboardInput
    + ExtendedKeyboardInput
{
}
#[cfg(all(
    not(feature = "svg"),
    not(feature = "system"),
    feature = "keycodes"
))]
impl<T> ExtendedWindowingFramework for T where
    T: WindowingFramework
        + ExtendedWindow
        + Control
        + Visibility
        + IconControl
        + RenderLayer
        + MouseInput
        + ExtendedMouseInput
        + KeyboardInput
        + ExtendedKeyboardInput
{
}

// No features
#[cfg(all(
    not(feature = "svg"),
    not(feature = "system"),
    not(feature = "keycodes")
))]
/// Framework with all functionality it could support
pub const trait ExtendedWindowingFramework:
    WindowingFramework
    + ExtendedWindow
    + Control
    + Visibility
    + IconControl
    + RenderLayer
    + MouseInput
    + ExtendedMouseInput
{
}
#[cfg(all(
    not(feature = "svg"),
    not(feature = "system"),
    not(feature = "keycodes")
))]
impl<T> ExtendedWindowingFramework for T where
    T: WindowingFramework
        + ExtendedWindow
        + Control
        + Visibility
        + IconControl
        + RenderLayer
        + MouseInput
        + ExtendedMouseInput
{
}

/// A simple helper that implements functions that mustn't be written by hand
pub const trait WindowHelper {
    /// Update what the current window displays using a Buffer
    ///
    /// # Errors
    /// See [`WindowError`]
    fn update(&mut self, buffer: &Buffer) -> Result<(), WindowError>;
}
impl<T: Window> WindowHelper for T {
    fn update(&mut self, buffer: &Buffer) -> Result<(), WindowError> {
        self.update_raw(buffer, buffer.width, buffer.height)
    }
}

/// A simple helper that implements input related functions that mustn't be written by hand
#[cfg(feature = "std")]
pub const trait WindowInputHelper {
    /// Get a snapshot of the current state of the mouse
    fn get_mouse_snapshot(&self) -> crate::platform::mouse::MouseSnapShot;
}
#[cfg(feature = "std")]
impl<T: MouseInput + ExtendedMouseInput> WindowInputHelper for T {
    fn get_mouse_snapshot(&self) -> crate::platform::mouse::MouseSnapShot {
        crate::platform::mouse::MouseSnapShot {
            position: self.get_mouse_position(),
            scroll: self.get_mouse_scroll(),
            left_down: self.is_mouse_down(crate::platform::MouseButton::Left),
            middle_down: self
                .is_mouse_down(crate::platform::MouseButton::Middle),
            right_down: self.is_mouse_down(crate::platform::MouseButton::Right),
        }
    }
}

/// A helper that is
pub const trait DeprecatedCompatibilityHelper {
    /// This function is deprecated as it is annoying to type out
    ///
    /// It is recommended to use [`Window::update_raw`] or [`WindowHelper::update`] instead
    /// # Errors
    /// See [`WindowError`]
    #[deprecated]
    fn update_with_buffer(
        &mut self,
        buffer: &Buffer,
    ) -> Result<(), WindowError>;
}

impl<T: Window + Control> DeprecatedCompatibilityHelper for T {
    fn update_with_buffer(
        &mut self,
        buffer: &Buffer,
    ) -> Result<(), WindowError> {
        self.update(buffer)
    }
}
/// Get the relative mouse position
pub const trait RelativeMousePos {
    /// Get the mouse position relative to the window
    fn get_mouse_position_relative(&self) -> Option<(f32, f32)>;
}

impl<T: MouseInput + Control> RelativeMousePos for T {
    fn get_mouse_position_relative(&self) -> Option<(f32, f32)> {
        let mouse_pos = self.get_mouse_position()?;
        let window_pos = self.get_position().try_tuple_into()?;
        Some(mouse_pos.sub(window_pos))
    }
}
