use super::{draw_pixel_safe, draw_pixel_unsafe, DrawPixelFunction};
use crate::extensions::*;
use crate::platform::Buffer;

#[inline]
pub fn draw_circle_switch(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    radius: isize,
    color: u32,
    fast: bool,
) {
    if fast {
        draw_circle_unsafe(buffer, pos_x, pos_y, radius, color);
    } else {
        draw_circle(buffer, pos_x, pos_y, radius, color);
    }
}

#[inline]
/// Draws a filled circle
pub fn draw_circle(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    radius: isize,
    color: u32,
) {
    draw_circle_impl(buffer, pos_x, pos_y, radius, color, draw_pixel_safe);
}

#[inline]
/// Draws a filled circle without bounds checking
pub fn draw_circle_unsafe(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    radius: isize,
    color: u32,
) {
    draw_circle_impl(buffer, pos_x, pos_y, radius, color, draw_pixel_unsafe);
}

#[inline(always)]
fn draw_circle_impl(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    radius: isize,
    color: u32,
    draw_pixel: DrawPixelFunction,
) {
    let pos_x = pos_x as isize;
    let pos_y = pos_y as isize;

    for y in -radius..=radius {
        let dy = y * y;
        let dx = (radius * radius - dy).abs().sqrt() as isize;

        for x in -dx..=dx {
            let x_pos = pos_x + x;
            let y_pos = pos_y + y;

            if x_pos >= 0 && y_pos >= 0 {
                draw_pixel(buffer, x_pos as usize, y_pos as usize, color);
            }
        }
    }
}
