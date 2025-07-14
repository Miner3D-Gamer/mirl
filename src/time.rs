

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
/// Converts u128 micros into a [std::time::Duration]
pub const fn from_micros_u128(micros: u128) -> std::time::Duration {
    // We can safely break the u128 into two u64s.
    let secs = (micros / 1_000_000) as u64; // seconds
    let nanos = ((micros % 1_000_000) * 1000) as u32; // nanoseconds (from microseconds)

    std::time::Duration::new(secs, nanos)
}
