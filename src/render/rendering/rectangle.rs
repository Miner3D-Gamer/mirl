use super::{draw_pixel_safe, draw_pixel_unsafe, DrawPixelFunction};
use crate::platform::Buffer;

#[inline]
pub fn draw_rectangle_switch(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    width: isize,
    height: isize,
    color: u32,
    fast: bool,
) {
    if fast {
        draw_rectangle_unsafe(buffer, pos_x, pos_y, width, height, color);
    } else {
        draw_rectangle(buffer, pos_x, pos_y, width, height, color);
    }
}

#[inline]
pub fn draw_rectangle(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    width: isize,
    height: isize,
    color: u32,
) {
    draw_rectangle_impl(
        buffer,
        pos_x,
        pos_y,
        width,
        height,
        color,
        draw_pixel_safe,
    );
}

#[inline]
pub fn draw_rectangle_unsafe(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    width: isize,
    height: isize,
    color: u32,
) {
    draw_rectangle_impl(
        buffer,
        pos_x,
        pos_y,
        width,
        height,
        color,
        draw_pixel_unsafe,
    );
}

#[inline(always)]
fn draw_rectangle_impl(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    width: isize,
    height: isize,
    color: u32,
    draw_pixel: DrawPixelFunction,
) {
    for y in pos_y..pos_y + height as usize {
        for x in pos_x..pos_x + width as usize {
            draw_pixel(buffer, x as usize, y as usize, color)
        }
    }
}
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
) {
    draw_rectangle_angled_impl(
        buffer,
        pos_x,
        pos_y,
        width,
        height,
        color,
        anchor_x,
        anchor_y,
        rotation,
        draw_pixel_safe,
    );
}
#[inline]
fn draw_rectangle_angled_unsafe(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    width: isize,
    height: isize,
    color: u32,
    anchor_x: f32,
    anchor_y: f32,
    rotation: f32,
) {
    draw_rectangle_angled_impl(
        buffer,
        pos_x,
        pos_y,
        width,
        height,
        color,
        anchor_x,
        anchor_y,
        rotation,
        draw_pixel_unsafe,
    );
}

#[inline(always)]
fn draw_rectangle_angled_impl(
    buffer: &Buffer,
    pos_x: usize,
    pos_y: usize,
    width: isize,
    height: isize,
    color: u32,
    anchor_x: f32,
    anchor_y: f32,
    rotation: f32,
    draw_pixel: DrawPixelFunction,
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
                draw_pixel(buffer, final_x as usize, final_y as usize, color);
            }
        }
    }
}
