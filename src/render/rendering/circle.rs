use super::{draw_pixel_safe, draw_pixel_unsafe, DrawPixelFunction};
use crate::extensions::*;
use crate::platform::Buffer;

/// Draw a filled circle by checking if every pixel is inside the radius
#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap)]
pub fn draw_circle<const SAFE: bool, const FIX_STRAY_PIXEL: bool>(
    buffer: &mut Buffer,
    pos: (isize, isize),
    radius: isize,
    color: u32,
) {
    let draw_pixel: DrawPixelFunction = {
        if SAFE {
            draw_pixel_safe
        } else {
            draw_pixel_unsafe
        }
    };
    // let pos_x = pos_x as isize;
    // let pos_y = pos_y as isize;

    for y in -radius..=radius {
        let dy = y * y;
        let mut dx = (radius * radius - dy).abs().sqrt();
        if FIX_STRAY_PIXEL && (y == 0 || y == radius || y == -radius) {
            dx -= 1;
        }

        for x in -dx..=dx {
            let x_pos = pos.0 + x;
            let y_pos = pos.1 + y;

            if x_pos >= 0 && y_pos >= 0 {
                draw_pixel(buffer, (x_pos as usize, y_pos as usize), color);
            }
        }
    }
}
