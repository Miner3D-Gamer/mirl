use super::Time;

pub struct NativeTime {
    pub time: std::time::Instant,
}
impl NativeTime {
    pub fn new() -> Self {
        Self {
            time: std::time::Instant::now(),
        }
    }
}
impl Time for NativeTime {
    fn get_elapsed_time(&self) -> u64 {
        self.time.elapsed().as_millis() as u64
    }
}
