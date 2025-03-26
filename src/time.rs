use std::time::{Duration, SystemTime};

const NANOS_PER_SEC: u64 = 1_000_000_000;
const MICROS_PER_SEC: u64 = 1_000_000;
const MILLIS_PER_SEC: u64 = 1_000;

pub fn sleep_sec(ms: u64) {
    std::thread::sleep(Duration::from_secs(ms));
}
pub fn sleep_ms(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}
pub fn sleep_us(ms: u64) {
    std::thread::sleep(Duration::from_micros(ms));
}
pub fn sleep_ns(ns: u64) {
    std::thread::sleep(Duration::from_nanos(ns))
}
pub fn get_time() -> SystemTime {
    let time = std::time::SystemTime::now();
    return time;
}
pub fn get_elapsed_as_ns(time: SystemTime) -> u64 {
    let elapsed = time.elapsed();

    match elapsed {
        Ok(duration) => {
            return duration.as_nanos() as u64;
        }
        Err(_) => {
            print!("Error while getting time");
            panic!();
        }
    }
}
pub fn get_elapsed_as_us(time: SystemTime) -> u64 {
    let elapsed = time.elapsed();

    match elapsed {
        Ok(duration) => {
            return duration.as_micros() as u64;
        }
        Err(_) => {
            print!("Error while getting time");
            panic!();
        }
    }
}

pub fn wait_for_fps_ns(fps: u64, delta_time: u64) {
    let target_frame_time = 1_000_000_000 / fps; // Time for one frame at the target FPS
    if delta_time < target_frame_time {
        let sleep_time = target_frame_time - delta_time;
        sleep_ns(sleep_time as u64);
    }
}
pub fn wait_for_fps_us(fps: u64, delta_time: u64) {
    let target_frame_time = 1_000_000 / fps; // Time for one frame at the target FPS
    if delta_time < target_frame_time {
        let sleep_time = target_frame_time - delta_time;
        sleep_us(sleep_time as u64);
    }
}
