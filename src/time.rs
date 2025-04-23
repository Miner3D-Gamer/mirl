pub fn nanos_per_sec<T: num_traits::FromPrimitive>() -> T {
    T::from_u64(1_000_000_000).unwrap()
}

pub fn micros_per_sec<T: num_traits::FromPrimitive>() -> T {
    T::from_u64(1_000_000).unwrap()
}
pub fn millis_per_sec<T: num_traits::FromPrimitive>() -> T {
    T::from_u64(1_000).unwrap()
}

// pub fn sleep_sec(ms: u64) {
//     std::thread::sleep(std::time::Duration::from_secs(ms));
// }
// pub fn sleep_ms(ms: u64) {
//     std::thread::sleep(std::time::Duration::from_millis(ms));
// }
// pub fn sleep_us(ms: u64) {
//     std::thread::sleep(std::time::Duration::from_micros(ms));
// }
// pub fn sleep_ns(ns: u64) {
//     std::thread::sleep(std::time::Duration::from_nanos(ns))
// }
pub fn get_time() -> std::time::Instant {
    let time = std::time::Instant::now();
    return time;
}

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
