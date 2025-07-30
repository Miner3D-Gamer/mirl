use super::{draw_pixel_safe, draw_pixel_unsafe, DrawPixelFunction};
use crate::extensions::*;
use crate::platform::Buffer;

/// Draw a filled circle by checking if every pixel is inside the radius
#[inline]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_wrap)]
pub fn draw_circle(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    radius: isize,
    color: u32,
    safe: bool,
) {
    let draw_pixel: DrawPixelFunction = {
        if safe {
            draw_pixel_safe
        } else {
            draw_pixel_unsafe
        }
    };
    let pos_x = pos_x as isize;
    let pos_y = pos_y as isize;

    for y in -radius..=radius {
        let dy = y * y;
        let dx = (radius * radius - dy).abs().sqrt();

        for x in -dx..=dx {
            let x_pos = pos_x + x;
            let y_pos = pos_y + y;

            if x_pos >= 0 && y_pos >= 0 {
                draw_pixel(buffer, x_pos as usize, y_pos as usize, color);
            }
        }
    }
}
