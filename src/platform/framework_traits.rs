#[cfg(feature = "resvg")]
use super::mouse::Cursor;
use super::{Buffer, KeyCode, MouseButton, Time};
#[cfg(feature = "resvg")]
use crate::extensions::*;

/// Most basic of framework functionality
pub trait Framework: Window + Input + Output + Timing {}
impl<T: Window + Input + Output + Timing> Framework for T {}
/// Framework with all functionality it could support
pub trait ExtendedFramework<MouseManagerScrollAccuracy: num_traits::Float>:
    Framework
    + ExtendedInput<MouseManagerScrollAccuracy>
    + ExtendedWindow
    + ExtendedTiming
    + Control
    + ExtendedControl
{
}
impl<T, MouseManagerScrollAccuracy: num_traits::Float>
    ExtendedFramework<MouseManagerScrollAccuracy> for T
where
    T: Framework
        + ExtendedInput<MouseManagerScrollAccuracy>
        + ExtendedWindow
        + ExtendedTiming
        + Control
        + ExtendedControl,
{
}
#[derive(Debug, Clone, Copy, PartialEq)]
/// An enum for checking what kind of error was produced
pub enum Errors {
    /// When the buffer was too small for the window, contains the expected buffer size
    BufferTooSmall((usize, usize)),
}

/// A window instance with only the most basic of functionality
pub trait Window {
    /// ### Create a new window with the desired settings
    ///
    /// ### Inputs:
    /// `title`: How the window should be named regardless of if it's shown
    ///
    /// `settings`: See [WindowSettings](super::WindowSettings) for more info
    ///
    // /// `cursor`: If you wish to use cursors other than the default one, provide the cursor you want the window to show by default. If this is set to None, [`set_cursor_style()`](ExtendedWindow::set_cursor_style) may not work as intended
    fn new(
        title: &str,
        settings: super::WindowSettings,
        // #[cfg(feature = "resvg")] cursor: Option<Cursor>,
    ) -> Self
    where
        Self: Sized;
    /// Update what the current window displays using a Buffer or \[u32]
    fn update(&mut self, buffer: &[u32]);
    /// Wether the current window is still open
    fn is_open(&self) -> bool;
    /// Clean up any remaining data after closing -> Otherwise memory leaks might happen
    fn clean_up(&self) {}
}
/// Basic input detection
pub trait Input {
    /// Gets the current mouse position relative to the window
    fn get_mouse_position(&self) -> Option<(isize, isize)>;
    /// Checks if the requested key is down.
    /// Warning: Most backends to not support all keys (like 'f25', 'world2', or 'Þ') and will always return false in that case
    fn is_key_down(&self, key: KeyCode) -> bool;
    /// Checks if the requested mouse button is down
    fn is_mouse_down(&self, button: MouseButton) -> bool;
}
/// Basic logging
pub trait Output {
    /// Log the given object (to the terminal)
    fn log(&self, t: &str);
}

/// Basic timing control
pub trait Timing {
    /// Get the current time
    fn get_time(&self) -> Box<dyn Time>;
    /// Sleep for X milliseconds - You don't need more accuracy, do you?
    fn sleep(&self, time: std::time::Duration);
    /// Get
    fn get_delta_time(&mut self) -> f64;
}
/// Advanced timing control
pub trait ExtendedTiming {
    /// Automatically set the max fps, use manual fps management for more control
    fn set_target_fps(&mut self, fps: usize);
}
/// More advanced input methods
pub trait ExtendedInput<MouseManagerScrollAccuracy: num_traits::Float> {
    /// Get how much the mouse has been scrolled on its wheel (x, y)
    fn get_mouse_scroll(
        &self,
    ) -> Option<(MouseManagerScrollAccuracy, MouseManagerScrollAccuracy)>;
    /// Get all currently pressed keys
    fn get_all_keys_down(&self) -> Vec<KeyCode>;
}
/// More 'advanced' window control
pub trait ExtendedWindow {
    /// Set the title (Duh)
    fn set_title(&mut self, title: &str);
    /// Set the current icon (task bar)
    /// Width/Height should be something like 32x32 or 48x48 for maximal compatibility
    fn set_icon(&mut self, buffer: &[u32], width: u32, height: u32);
    /// Set what cursors the os should display on the current window
    #[cfg(feature = "resvg")]
    fn set_cursor_style(&mut self, style: &Cursor);
    #[cfg(feature = "resvg")]
    /// Load the custom cursors Mirl provides by default
    fn load_custom_cursor(
        &mut self,
        size: U2,
        main_color: u32,
        secondary_color: u32,
    ) -> super::mouse::Cursors;
    /// Get the current window handle
    fn get_window_handle(&self) -> raw_window_handle::RawWindowHandle;
}
/// Simple window management
pub trait Control {
    /// Set the window position relative to its current position
    fn move_window(&mut self, xy: (isize, isize)) {
        let current = self.get_position();
        self.set_position((current.0 + xy.0, current.1 + xy.1));
    }
    /// Get the position of the window
    fn get_position(&self) -> (isize, isize);
    /// Set the position of the window
    fn set_position(&mut self, xy: (isize, isize));
    /// Set the size of the window using the dimensions of a Buffer
    fn set_size(&mut self, buffer: &Buffer);
    /// Get the size of the window
    fn get_size(&self) -> (isize, isize);
}
/// More complex window controls
pub trait ExtendedControl {
    /// Set the window layer like topmost, bottommost, and default
    fn set_render_layer(&mut self, render_layer: super::WindowLevel);
    /// Minimize the window
    fn minimize(&mut self);
    /// Maximize the window (More or less fullscreen), to un-minimize use .restore()
    fn maximize(&mut self);
    /// Opposite of .minimize()
    fn restore(&mut self);
    /// Wether the current window is minimized
    fn is_minimized(&self) -> bool;
    /// Wether the current window is maximized
    fn is_maximized(&self) -> bool;
}
