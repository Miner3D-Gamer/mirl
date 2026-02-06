#[cfg(feature = "std")]
use super::Time;

#[cfg(feature = "std")]
/// Native implementation of the Time trait
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NativeTime {
    /// The starting time
    pub time: std::time::Instant,
}
#[cfg(feature = "std")]
impl NativeTime {
    /// Create a new time instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            time: std::time::Instant::now(),
        }
    }
}
#[cfg(feature = "std")]
impl Default for NativeTime {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(feature = "std")]
impl Time for NativeTime {
    fn get_elapsed_time(&self) -> f64 {
        self.time.elapsed().as_secs_f64()
    }
}
