#![allow(clippy::cast_possible_truncation)]
// pub fn wait_for_fps_ns(fps: u64, delta_time: u64) {
//     let target_frame_time = 1_000_000_000 / fps; // Time for one frame at the target FPS
//     if delta_time < target_frame_time {
//         let sleep_time = target_frame_time - delta_time;
//         sleep_ns(sleep_time);
//     }
// }
// pub fn wait_for_fps_us(fps: u64, delta_time: u64) {
//     let target_frame_time = 1_000_000 / fps; // Time for one frame at the target FPS
//     if delta_time < target_frame_time {
//         let sleep_time = target_frame_time - delta_time;
//         sleep_us(sleep_time);
//     }
// }
#[must_use]
/// Converts u128 nanoseconds into a `Duration`
pub const fn from_nanos_u128(nanos: u128) -> std::time::Duration {
    let secs = (nanos / 1_000_000_000) as u64;
    let subnanos = (nanos % 1_000_000_000) as u32;
    std::time::Duration::new(secs, subnanos)
}
#[must_use]
/// Converts u128 microseconds into a `Duration`
pub const fn from_micros_u128(micros: u128) -> std::time::Duration {
    let secs = (micros / 1_000_000) as u64;
    let nanos = ((micros % 1_000_000) * 1_000) as u32;
    std::time::Duration::new(secs, nanos)
}
#[must_use]
/// Converts u128 milliseconds into a `Duration`
pub const fn from_millis_u128(millis: u128) -> std::time::Duration {
    let secs = (millis / 1_000) as u64;
    let nanos = ((millis % 1_000) * 1_000_000) as u32;
    std::time::Duration::new(secs, nanos)
}

/// Nanoseconds per second
pub const NANOS_PER_SEC: usize = 1_000_000_000;
/// Nanoseconds per millisecond
pub const NANOS_PER_MILLI: usize = 1_000_000;
/// Nanoseconds per microseconds
pub const NANOS_PER_MICRO: usize = 1_000;
/// Milliseconds per second
pub const MILLIS_PER_SEC: usize = 1_000;
/// Microseconds per second
pub const MICROS_PER_SEC: usize = 1_000_000;
/// Seconds per minute
pub const SECS_PER_MINUTE: usize = 60;
/// Minutes per hour
pub const MINS_PER_HOUR: usize = 60;
/// Hours per day
pub const HOURS_PER_DAY: usize = 24;
/// Days per week
pub const DAYS_PER_WEEK: usize = 7;
