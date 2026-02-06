/// Draw a pixel color onto the buffer without checking if the pixel is on screen (which will crash the program if it isn't)
#[inline(always)]
#[allow(clippy::inline_always)]
#[track_caller]
pub const fn draw_pixel_unsafe(
    buffer: &mut (impl [const] BufferPointers + [const] BufferMetrics),
    xy: (usize, usize),
    color: u32,
) {
    unsafe {
        *buffer.mut_pointer().add(xy.1 * buffer.width() + xy.0) = color;
    }
}
/// Draw a pixel color onto the buffer by first checking if the pixel is on screen
#[inline(always)]
#[allow(clippy::inline_always)]
#[track_caller]
pub const fn draw_pixel_safe(
    buffer: &mut (impl [const] BufferPointers + [const] BufferMetrics),
    xy: (usize, usize),
    color: u32,
) {
    if xy.0 < buffer.width() && xy.1 < buffer.height() {
        unsafe {
            *buffer.mut_pointer().add(xy.1 * buffer.width() + xy.0) = color;
        }
    }
}
// type DrawPixelFunction =
//     fn(&mut (impl BufferPointers + BufferMetrics), (usize, usize), u32);

/// All buffer-type related traits
pub mod buffer_type;
pub use buffer_type::*;
#[cfg(feature = "std")]
/// Helpers that should make using stuff like Arc easier
pub mod buffer_compatibility;
// pub use buffer_compatibility::*;

// macro_rules! create_safe_and_unsafe {
//     (
//         fn $fn:ident($($arg:ident: $arg_ty:ty),*) $body:block
//     ) => {
//         // Safe version
//         pub fn $fn($($arg: $arg_ty),*) {
//             paste::paste! {
//                 [<$fn _impl>]($($arg),*, draw_pixel_safe);
//             }
//         }

//         // Unsafe (fast) version
//         paste::paste! {
//             pub fn [<$fn _unsafe>]($($arg: $arg_ty),*) {
//                 [<$fn _impl>]($($arg),*, draw_pixel_unsafe);
//             }
//         }

//         // Implementation function
//         paste::paste! {
//             fn [<$fn _impl>]($($arg: $arg_ty),*) {
//                 $body
//             }
//         }
//     };
// }

// #[derive(Copy, Clone)]
// pub enum Safety {
//     Safe,
//     Unsafe,
// }

// #[derive(Copy, Clone)]
// pub enum Quality {
//     Fast,
//     Pretty,
// }

mod circle_outline;
pub use circle_outline::*;
#[cfg(feature = "font_support")]
mod text;
#[cfg(feature = "font_support")]
pub use text::*;
mod line;
pub use line::*;
mod circle;
pub use circle::*;
mod rectangle;
pub use rectangle::*;
#[cfg(feature = "std")]
mod triangle;
#[cfg(feature = "std")]
pub use triangle::*;
mod texture;
pub use texture::*;
