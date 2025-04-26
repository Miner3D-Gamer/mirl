use crate::platform::Buffer;

#[inline(always)]
pub fn draw_pixel_unsafe(buffer: &Buffer, x: usize, y: usize, color: u32) {
    unsafe {
        *buffer.pointer.add(y * buffer.width + x) = color;
    }
}
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
//             pub fn [<$fn _fast>]($($arg: $arg_ty),*) {
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
    return buffer.buffer[index];
}
#[inline(always)]
fn get_pixel_isize(buffer: &Buffer, x: isize, y: isize) -> u32 {
    if x < 0 || y < 0 {
        return 0;
    }
    let _y = y as usize;
    let _x = x as usize;
    if _x >= buffer.width || _y >= buffer.height {
        return 0;
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
