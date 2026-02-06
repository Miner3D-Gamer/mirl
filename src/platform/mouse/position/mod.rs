/// A way of getting the raw mouse input since using the mouse input of the os would always result in a frame of latency
pub const trait RawMouseInputTrait {
    /// Create a new raw mouse input getter under the window from which should poll the mouse move event
    ///
    /// # Errors
    fn new(handle: RawWindowHandle) -> Result<Self, &'static str>
    where
        Self: Sized;
    /// Get current delta, add this to the position the os gives you to get the true mouse pos (assuming acceleration, smoothing and other mouse position manipulation methods aren't applied by the os)
    fn get_delta(&self) -> (i32, i32);
}

/// Represents mouse movement delta values
#[derive(Debug, Clone, Copy, Default)]
pub struct MouseDelta {
    /// Delta X
    pub dx: i32,
    /// Delta Y
    pub dy: i32,
}
#[cfg(target_os = "windows")]
mod windows;
use raw_window_handle::RawWindowHandle;
#[cfg(target_os = "windows")]
pub use windows::RawMouseInputWindows as RawMouseInput;
