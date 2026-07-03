//! A bunch of constants, including values `std` uses yet doesn't publically expose
//!
//! This lib might get absorbed by `mirl_extensions` in the future
//!
//! Currently covers:
//! - Colors
//! - Time formats (Incomplete, different time systems are missing)
//! - Byte formats (Incomplete, conversion is missing)
#![feature(core_intrinsics)]
// TODO: Turn these into const values into traits so any number can natively use them

/// Computer file sizes
pub mod bytes;

/// Time related constants
pub mod time;

// /// Presets for common colors
// pub mod colors;
