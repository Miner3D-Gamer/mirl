use super::get_character;
use crate::{
    platform::Buffer,
    render::{
        draw_pixel_safe, draw_pixel_unsafe, rendering::DrawPixelFunction,
    },
};

pub fn draw_text_impl(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
    draw_pixel: DrawPixelFunction,
) {
    let mut pen_x = x;
    let pen_y = y;
    let font_metrics = font.horizontal_line_metrics(size).unwrap();
    let ascent = font_metrics.ascent as usize;

    for ch in text.chars() {
        // If not in cache, rasterize and insert
        let (metrics, bitmap) = get_character(ch, size, &font);

        let offset_y = ascent - metrics.height;
        let w = metrics.width;
        let h = metrics.height;
        let advance_x = metrics.advance_width as usize;

        for gy in 0..h {
            let py = ((pen_y + gy + offset_y) as i32 - metrics.ymin) as usize;
            if py >= buffer.height {
                continue;
            }

            let row_start = gy * w;
            for gx in 0..w {
                let px = pen_x + gx;
                if px >= buffer.width {
                    continue;
                }

                if bitmap[row_start + gx] > 0 {
                    draw_pixel(buffer, px, py, color);
                }
            }
        }
        pen_x += advance_x;
    }
}

#[inline]
pub fn draw_text(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
) {
    draw_text_impl(buffer, text, x, y, color, size, font, draw_pixel_safe);
}

#[inline]
pub fn draw_text_unsafe(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
) {
    draw_text_impl(buffer, text, x, y, color, size, font, draw_pixel_unsafe);
}

#[inline]
pub fn draw_text_stretched<F: num_traits::Float>(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
    stretch_x: F,
    stretch_y: F,
) {
    draw_text_stretch_impl(
        buffer,
        text,
        x,
        y,
        color,
        size,
        font,
        draw_pixel_safe,
        stretch_x,
        stretch_y,
    );
}

#[inline]
pub fn draw_text_stretched_unsafe<F: num_traits::Float>(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
    stretch_x: F,
    stretch_y: F,
) {
    draw_text_stretch_impl(
        buffer,
        text,
        x,
        y,
        color,
        size,
        font,
        draw_pixel_unsafe,
        stretch_x,
        stretch_y,
    );
}

pub fn draw_text_stretch_impl<F: num_traits::Float>(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
    draw_pixel: DrawPixelFunction,
    stretch_x: F,
    stretch_y: F,
) {
    let mut pen_x = x;
    let pen_y = y;
    let font_metrics = font.horizontal_line_metrics(size).unwrap();
    let ascent = font_metrics.ascent as usize;

    for ch in text.chars() {
        // If not in cache, rasterize and insert
        let (metrics, bitmap) = get_character(ch, size, &font);

        let offset_y = ascent.saturating_sub(metrics.height);
        let w = F::from(metrics.width).unwrap() * stretch_x;
        let h = F::from(metrics.height).unwrap() * stretch_y;
        let advance_x = metrics.advance_width as usize;

        for gy in 0..h.floor().to_usize().unwrap() {
            let py = ((pen_y + gy + offset_y) as i32 - metrics.ymin) as usize;
            if py >= buffer.height {
                continue;
            }

            let bitmap_row_start =
                ((F::from(gy).unwrap() / stretch_y).to_usize().unwrap())
                    * metrics.width;
            for gx in 0..w.floor().to_usize().unwrap() {
                let px = pen_x + gx;
                if px >= buffer.width {
                    continue;
                }

                if bitmap[bitmap_row_start
                    + (F::from(gx).unwrap() / stretch_x).to_usize().unwrap()]
                    > 0
                {
                    draw_pixel(buffer, px, py, color);
                }
            }
        }
        pen_x += advance_x;
    }
}
