#[cfg(feature = "std")]
/// Enables the rust traceback by setting the environment variable `RUST_BACKTRACE` to `1`
pub fn enable_traceback() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
}
#[cfg(feature = "std")]
/// Enables the extended rust traceback by setting the environment variable `RUST_BACKTRACE` to `full`
pub fn enable_traceback_detailed() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "full");
    }
}
#[cfg(feature = "std")]
/// Disables the rust traceback by setting the environment variable `RUST_BACKTRACE` to `0`
pub fn disable_traceback() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "0");
    }
}
