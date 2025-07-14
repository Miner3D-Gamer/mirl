use super::{draw_pixel_safe, draw_pixel_unsafe, DrawPixelFunction};
use crate::extensions::*;
use crate::platform::Buffer;

#[inline(always)]
/// Draw a simple line using Bresenham's line algorithm
pub fn draw_line(
    buffer: &Buffer,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
    color: u32,
    thickness: isize,
    safe: bool,
) {
    let draw_pixel: DrawPixelFunction = {
        if safe {
            draw_pixel_safe
        } else {
            draw_pixel_unsafe
        }
    };
    let mut start_x = x1 as i16;
    let mut start_y = y1 as i16;
    let end_x = x2 as i16;
    let end_y = y2 as i16;

    let difference_x: i16 = end_x - start_x;
    let difference_y: i16 = end_y - start_y;
    let sign_x = difference_x.sign();
    let sign_y = difference_y.sign();
    let abs_difference_x: i16 = difference_x.abs();
    let abs_difference_y: i16 = difference_y.abs();

    // Calculate the perpendicular vector for the line
    let mut normal_x = -difference_y;
    let mut normal_y = difference_x;

    let length = (normal_x.pow(2) + normal_y.pow(2)) as f64;
    let normal_length = (length as f64).sqrt() as i16;
    normal_x =
        (normal_x as f64 * thickness as f64 / normal_length as f64) as i16;
    normal_y =
        (normal_y as f64 * thickness as f64 / normal_length as f64) as i16;

    if abs_difference_x > abs_difference_y {
        let mut error: i16 = abs_difference_x / 2;
        while start_x != end_x {
            start_x += sign_x;
            error -= abs_difference_y;
            if error < 0 {
                start_y += sign_y;
                error += abs_difference_x;
            }

            // Draw multiple pixels for thickness
            for offset_x in -thickness as i16 / 2..=thickness as i16 / 2 {
                for offset_y in -thickness as i16 / 2..=thickness as i16 / 2 {
                    let new_x = start_x + normal_x * offset_x;
                    let new_y = start_y + normal_y * offset_y;
                    draw_pixel(buffer, new_x as usize, new_y as usize, color);
                }
            }
        }
    } else {
        let mut error: i16 = abs_difference_y / 2;
        while start_y != end_y {
            start_y += sign_y;
            error -= abs_difference_x;
            if error < 0 {
                start_x += sign_x;
                error += abs_difference_y;
            }

            // Draw multiple pixels for thickness
            for offset_x in -thickness as i16 / 2..=thickness as i16 / 2 {
                for offset_y in -thickness as i16 / 2..=thickness as i16 / 2 {
                    let new_x = start_x + normal_x * offset_x;
                    let new_y = start_y + normal_y * offset_y;
                    draw_pixel(buffer, new_x as usize, new_y as usize, color);
                }
            }
        }
    }
}
