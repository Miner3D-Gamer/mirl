use crate::{platform::frameworks::WindowError, prelude::WindowSettings, Buffer};

/// A window instance with only the most basic of functionality
pub const trait Window {
    /// ### Create a new window with the desired settings
    ///
    /// ### Inputs:
    /// `title`: How the window should be named regardless of if it's shown
    ///
    /// `settings`: See [`WindowSettings`](super::WindowSettings) for more info
    // /// `cursor`: If you wish to use cursors other than the default one, provide the cursor you want the window to show by default. If this is set to None, [`set_cursor_style()`](ExtendedWindow::set_cursor_style) may not work as intended
    /// # Errors
    /// See [Errors] for the error messages
    fn new(
        title: &str,
        settings: WindowSettings,
        // #[cfg(feature = "svg")] cursor: Option<Cursor>,
    ) -> Result<Self, WindowError>
    where
        Self: Sized;
    /// Update what the current window displays using a Buffer or \[u32]
    ///
    /// # Errors
    /// See [Errors]
    fn update_raw(
        &mut self,
        pixels: &[u32],
        width: usize,
        height: usize,
    ) -> Result<(), WindowError>;
    /// Wether the current window is still open
    fn is_open(&self) -> bool;
    /// Clean up any remaining data after closing -> Otherwise memory leaks might happen
    fn clean_up(&self) {}
}
/// More 'advanced' window control
pub const trait ExtendedWindow {
    /// Set the title (Duh)
    fn set_title(&mut self, title: &str);
    /// Get the current window handle
    fn get_window_handle(&self) -> raw_window_handle::RawWindowHandle;
}
/// Set the icon of the window
pub const trait IconControl {
    /// Set the current icon (task bar)
    /// Width/Height should be something like 32x32 or 48x48 for maximal compatibility
    ///
    /// # Errors
    /// See [Errors]
    fn set_icon(&mut self, buffer: &Buffer) -> Result<(), WindowError>;
}
// TODO: Add function to system that retrieves the current icon of a window
