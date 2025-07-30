use crate::platform::Buffer;
/// Draw a pixel color onto the buffer without checking if the pixel is on screen (which will crash the program if it isn't)
#[inline(always)]
#[allow(clippy::inline_always)]
pub fn draw_pixel_unsafe(buffer: &Buffer, x: usize, y: usize, color: u32) {
    unsafe {
        *buffer.pointer.add(y * buffer.width + x) = color;
    }
}
/// Draw a pixel color onto the buffer by first checking if the pixel is on screen
#[inline(always)]
#[allow(clippy::inline_always)]
pub fn draw_pixel_safe(buffer: &Buffer, x: usize, y: usize, color: u32) {
    if x < buffer.width && y < buffer.height {
        unsafe {
            *buffer.pointer.add(y * buffer.width + x) = color;
        }
    }
}

type DrawPixelFunction = fn(&Buffer, usize, usize, u32);

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
mod triangle;
pub use triangle::*;
mod texture;
pub use texture::*;
