use super::{draw_pixel_safe, draw_pixel_unsafe, DrawPixelFunction};
use crate::platform::Buffer;

#[inline]
#[allow(clippy::cast_sign_loss)]
/// Draws a circle outline using the Midpoint Circle Algorithm
pub fn draw_circle_outline(
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

    let mut x = 0;
    let mut y = 0 - radius;
    let mut p = -radius;

    while (x) < (-y) {
        if p > 0 {
            y += 1;
            p += 2 * (x + y) + 1;
        } else {
            p += 2 * x + 1;
        }
        let temp_x = x as usize;
        let temp_y = y as usize;
        draw_pixel(buffer, pos_x + temp_x, pos_y + temp_y, color);
        draw_pixel(buffer, pos_x - temp_x, pos_y + temp_y, color);
        draw_pixel(buffer, pos_x + temp_x, pos_y - temp_y, color);
        draw_pixel(buffer, pos_x - temp_x, pos_y - temp_y, color);
        draw_pixel(buffer, pos_x + temp_y, pos_y + temp_x, color);
        draw_pixel(buffer, pos_x + temp_y, pos_y - temp_x, color);
        draw_pixel(buffer, pos_x - temp_y, pos_y + temp_x, color);
        draw_pixel(buffer, pos_x - temp_y, pos_y - temp_x, color);

        x += 1;
    }
}
