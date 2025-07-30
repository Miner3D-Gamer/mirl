use super::{draw_pixel_safe, draw_pixel_unsafe, DrawPixelFunction};
use crate::extensions::*;
use crate::platform::Buffer;

#[inline]
/// Draw a simple line using Bresenham's line algorithm
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub fn draw_line(
    buffer: &Buffer,
    start: (usize, usize),
    end: (usize, usize),
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
    let mut start_x = start.0 as i16;
    let mut start_y = start.1 as i16;
    let end_x = end.0 as i16;
    let end_y = end.1 as i16;

    let difference_x: i16 = end_x - start_x;
    let difference_y: i16 = end_y - start_y;
    let sign_x = difference_x.sign();
    let sign_y = difference_y.sign();
    let abs_difference_x: i16 = difference_x.abs();
    let abs_difference_y: i16 = difference_y.abs();

    // Calculate the perpendicular vector for the line
    let mut normal_x = -difference_y;
    let mut normal_y = difference_x;

    let length = f64::from(normal_x.pow(2) + normal_y.pow(2));
    let normal_length = (length).sqrt() as i16;
    normal_x = (f64::from(normal_x) * thickness as f64
        / f64::from(normal_length)) as i16;
    normal_y = (f64::from(normal_y) * thickness as f64
        / f64::from(normal_length)) as i16;

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
