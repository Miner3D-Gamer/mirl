use super::super::super::mouse::Cursor;
use crate::platform::{
    mouse::{CursorResolution, LoadCursorError},
    windowing::WindowError,
};

/// Load a cursor style into the native format
pub const trait LoadCursorStyle {
    /// Load the custom cursors Mirl provides by default
    ///
    /// # Errors
    /// If it was unable to load the custom cursors, it returns the file name of the cursor that failed - This should only ever happen when a file corrupted
    fn load_custom_cursors(
        &mut self,
        size: CursorResolution,
        main_color: u32,
        secondary_color: u32,
    ) -> Result<super::super::super::mouse::Cursors, LoadCursorError>;
    /// Load your own custom cursor
    /// Just make sure the size of the buffer is 32x32, 64x64, 128x128 or 256x256
    ///
    /// # Errors
    /// String explaining what went wrong
    fn load_custom_cursor(
        &mut self,
        image: super::super::super::Buffer,
        hotspot: (u8, u8),
    ) -> Result<super::super::super::mouse::Cursor, LoadCursorError>;
}

/// Control over the cursor style while the mouse of hovering over it
pub const trait UseCursorStyle {
    /// Set what cursors the os should display on the current window
    ///
    /// # Errors
    /// See [`WindowError`]
    fn set_cursor_style(&mut self, style: &Cursor) -> Result<(), WindowError>;
}
