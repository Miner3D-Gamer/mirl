use super::Time;

/// Native implementation of the Time trait
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NativeTime {
    /// The starting time
    pub time: std::time::Instant,
}
impl NativeTime {
    /// Create a new time instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            time: std::time::Instant::now(),
        }
    }
}
impl Default for NativeTime {
    fn default() -> Self {
        Self::new()
    }
}

impl Time for NativeTime {
    fn get_elapsed_time(&self) -> f64 {
        self.time.elapsed().as_secs_f64()
    }
}
