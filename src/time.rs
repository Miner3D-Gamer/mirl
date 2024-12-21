
use std::time::{Duration, SystemTime};



pub fn sleep_sec(ms: u64) {
    std::thread::sleep(Duration::from_secs(ms));
}
pub fn sleep_ms(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}
pub fn sleep_ns(ns: u64) {
    std::thread::sleep(Duration::from_nanos(ns))
}
pub fn get_time() -> SystemTime {
    let time = std::time::SystemTime::now();
    return time;
}
pub fn get_elapsed_as_ns(time: SystemTime) -> u128 {
    let elapsed = time.elapsed();

    match elapsed {
        Ok(duration) => {
            return duration.as_nanos();
        }
        Err(_) => {
            print!("Error while getting time");
            panic!();
        }
    }
}

pub fn wait_for_fps_ns(fps: u128, delta_time: u128) {
    let target_frame_time = 1_000_000_000 / fps; // Time for one frame at the target FPS
    if delta_time < target_frame_time {
        let sleep_time = target_frame_time - delta_time;
        sleep_ns(sleep_time as u64);
    }
}