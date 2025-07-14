use crate::platform::Buffer;
/// Draw a pixel color onto the buffer without checking if the pixel is on screen (which will crash the program if it isn't)
#[inline(always)]
pub fn draw_pixel_unsafe(buffer: &Buffer, x: usize, y: usize, color: u32) {
    unsafe {
        *buffer.pointer.add(y * buffer.width + x) = color;
    }
}
/// Draw a pixel color onto the buffer by first checking if the pixel is on screen
#[inline(always)]
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

#[inline(always)]
fn get_pixel(buffer: &Buffer, x: usize, y: usize) -> u32 {
    if x >= buffer.width || y >= buffer.height {
        return 0;
    }
    let index = y * buffer.width + x;
    unsafe {
        return *buffer.pointer.add(index);
    }
}
/// Get the pixel color at a position in a buffer without checking if the pixel is on screen (which will crash the program if it isn't)
/// The function for getting a pixel safely is [get_pixel_isize]
#[inline(always)]
pub fn get_pixel_unsafe(buffer: &Buffer, x: usize, y: usize) -> u32 {
    let index = y * buffer.width + x;
    unsafe {
        return *buffer.pointer.add(index);
    }
}
/// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
#[inline(always)]
pub fn get_pixel_isize(buffer: &Buffer, x: isize, y: isize) -> Option<u32> {
    if x < 0 || y < 0 {
        return None;
    }
    let _y = y as usize;
    let _x = x as usize;
    if _x >= buffer.width || _y >= buffer.height {
        return None;
    }
    let index = _y as usize * buffer.width + _x;
    return Some(buffer.buffer[index]);
}
/// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
/// Instead of returning None if the result isn't in the buffer, it will return the specified fallback value
#[inline(always)]
pub fn get_pixel_isize_fallback(
    buffer: &Buffer,
    x: isize,
    y: isize,
    fallback: u32,
) -> u32 {
    if x < 0 || y < 0 {
        return fallback;
    }
    let _y = y as usize;
    let _x = x as usize;
    if _x >= buffer.width || _y >= buffer.height {
        return fallback;
    }
    let index = _y as usize * buffer.width + _x;
    return buffer.buffer[index];
}

mod circle_outline;
pub use circle_outline::*;
mod text;
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
