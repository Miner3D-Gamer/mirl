/// Other functions/structs like 3D points and Polygons
pub mod extra;

// #[cfg(feature = "imagery")]
// mod image_support;
// #[cfg(feature = "imagery")]
// pub use image_support::*;

// ---------------------------------------------------------------------
mod rendering;
pub use rendering::*;

#[cfg(feature = "std")]
mod buffer;
#[cfg(feature = "std")]
pub use buffer::*;

/// A const buffer making money on more compile time optimizations and `no_std` support
pub mod const_buffer;
pub use const_buffer::*;
