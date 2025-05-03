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
        draw_circle_fast(buffer, pos_x, pos_y, radius, color);
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
pub fn draw_circle_fast(
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
    for y in -radius..=radius {
        let dx = (radius * radius - y * y).abs().sqrt() as isize;
        for x in -dx..=dx {
            draw_pixel(buffer, pos_x + x as usize, pos_y + y as usize, color);
        }
    }
}
