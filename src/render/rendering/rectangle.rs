use super::{draw_pixel_safe, draw_pixel_unsafe};
use crate::{platform::Buffer, render::rendering::DrawPixelFunction};

/// Draw a simple rectangle
#[inline]
pub fn draw_rectangle(
    buffer: &Buffer,
    pos_x: isize,
    pos_y: isize,
    width: isize,
    height: isize,
    color: u32,
    safe: bool,
) {
    for y in pos_y..pos_y + height {
        for x in pos_x..pos_x + width {
            if safe {
                if x > 0 && y > 0 {
                    draw_pixel_safe(buffer, x as usize, y as usize, color);
                }
            } else {
                draw_pixel_unsafe(buffer, x as usize, y as usize, color);
            }
        }
    }
}
/// Draw a simple rectangle
#[inline]
pub fn execute_at_rectangle(
    buffer: &Buffer,
    pos_x: isize,
    pos_y: isize,
    width: isize,
    height: isize,
    color: u32,
    safe: bool,
    function: DrawPixelFunction
) {
    for y in pos_y..pos_y + height {
        for x in pos_x..pos_x + width {
            if safe {
                if x > 0 && y > 0 {
                    function(buffer, x as usize, y as usize, color);
                }
            } else {
                function(buffer, x as usize, y as usize, color);
            }
        }
    }
}

/// Draw a rotated rectangle
#[inline]
pub fn draw_rectangle_angled(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    width: isize,
    height: isize,
    color: u32,
    anchor_x: f32,
    anchor_y: f32,
    rotation: f32,
    safe: bool,
) {
    let cos_theta = rotation.cos();
    let sin_theta = rotation.sin();

    let anchor_x_offset = anchor_x * width as f32;
    let anchor_y_offset = anchor_y * height as f32;

    for y in 0..height as usize {
        for x in 0..width as usize {
            let rel_x = x as f32 - anchor_x_offset;
            let rel_y = y as f32 - anchor_y_offset;

            let rotated_x = rel_x * cos_theta - rel_y * sin_theta;
            let rotated_y = rel_x * sin_theta + rel_y * cos_theta;

            let final_x =
                (pos_x as f32 + rotated_x + anchor_x_offset).round() as isize;
            let final_y =
                (pos_y as f32 + rotated_y + anchor_y_offset).round() as isize;

            if final_x >= 0 && final_y >= 0 || true {
                if safe {
                    draw_pixel_safe(
                        buffer,
                        final_x as usize,
                        final_y as usize,
                        color,
                    );
                } else {
                    draw_pixel_unsafe(
                        buffer,
                        final_x as usize,
                        final_y as usize,
                        color,
                    );
                }
            }
        }
    }
}
/// Tried to draw a rectangle 4 pixels at the time in the hopes of improving performance
#[inline]
pub fn draw_rectangle_impl_simd(
    buffer: &Buffer,
    pos_x: isize,
    pos_y: isize,
    width: isize,
    height: isize,
    color: u32,
    safe: bool,
) {
    if safe {
        if pos_x < 0
            || pos_y < 0
            || pos_x + width > buffer.width as isize
            || pos_y + height > buffer.height as isize
        {
            return;
        }
    }

    let start_x = pos_x as usize;
    let start_y = pos_y as usize;
    let rect_width = width as usize;
    let rect_height = height as usize;

    for y in 0..rect_height {
        let row_start = (start_y + y) * buffer.width + start_x;
        unsafe {
            let mut ptr = buffer.pointer.add(row_start);
            let mut remaining = rect_width;

            // Process 4 pixels at a time
            while remaining >= 4 {
                *ptr = color;
                *ptr.add(1) = color;
                *ptr.add(2) = color;
                *ptr.add(3) = color;
                ptr = ptr.add(4);
                remaining -= 4;
            }

            // Handle remaining pixels
            while remaining > 0 {
                *ptr = color;
                ptr = ptr.add(1);
                remaining -= 1;
            }
        }
    }
}
