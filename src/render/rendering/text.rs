use super::{draw_pixel_safe, draw_pixel_unsafe, DrawPixelFunction};
use crate::platform::Buffer;
use std::collections::HashMap;
use std::sync::RwLock;

static GLYPH_CACHE: once_cell::sync::Lazy<
    RwLock<HashMap<(char, (i32, i32)), (fontdue::Metrics, Vec<u8>)>>,
> = once_cell::sync::Lazy::new(|| RwLock::new(HashMap::new()));

#[inline(always)]
pub fn _get_glyph_cache(
) -> &'static RwLock<HashMap<(char, (i32, i32)), (fontdue::Metrics, Vec<u8>)>> {
    &GLYPH_CACHE
}

#[inline(always)]
fn round_float_key(value: f32) -> (i32, i32) {
    let multiplier = 10.0_f32.powi(4);
    let rounded_int_x = (value * multiplier).round() as i32;
    let rounded_int_y = (value * multiplier).fract() as i32;
    (rounded_int_x, rounded_int_y)
}

pub fn draw_text_switch(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
    aliased: bool,
    fast: bool,
) {
    if aliased {
        if fast {
            draw_text_aliased_fast(buffer, text, x, y, color, size, font);
        } else {
            draw_text_aliased(buffer, text, x, y, color, size, font);
        }
    } else {
        if fast {
            draw_text_antialiased_fast(buffer, text, x, y, color, size, font);
        } else {
            draw_text_antialiased(buffer, text, x, y, color, size, font);
        }
    }
}

#[inline]
pub fn draw_text_aliased(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
) {
    draw_text_aliased_impl(
        buffer,
        text,
        x,
        y,
        color,
        size,
        font,
        draw_pixel_safe,
    );
}

#[inline]
pub fn draw_text_aliased_fast(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
) {
    draw_text_aliased_impl(
        buffer,
        text,
        x,
        y,
        color,
        size,
        font,
        draw_pixel_unsafe,
    );
}

#[inline]
pub fn draw_text_antialiased(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
) {
    draw_text_antialiased_impl(
        buffer,
        text,
        x,
        y,
        color,
        size,
        font,
        draw_pixel_safe,
    );
}

#[inline]
pub fn draw_text_antialiased_fast(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
) {
    draw_text_antialiased_impl(
        buffer,
        text,
        x,
        y,
        color,
        size,
        font,
        draw_pixel_unsafe,
    );
}

fn draw_text_aliased_impl(
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

    let rounded_size_key = round_float_key(size);

    for ch in text.chars() {
        // Try to get the glyph from cache first
        let cached_glyph = {
            let cache = _get_glyph_cache().read().unwrap();
            cache.get(&(ch, rounded_size_key)).cloned()
        };

        // If not in cache, rasterize and insert
        let (metrics, bitmap) = cached_glyph.unwrap_or_else(|| {
            let rasterized = font.rasterize(ch, size);

            // Insert into cache
            let mut cache_mut = _get_glyph_cache().write().unwrap();
            cache_mut.insert((ch, rounded_size_key), rasterized.clone());

            rasterized
        });

        let offset_y = ascent.saturating_sub(metrics.height);
        let w = metrics.width;
        let h = metrics.height;
        let advance_x = metrics.advance_width as usize;

        for gy in 0..h {
            let py = pen_y + gy + offset_y;
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

fn draw_text_antialiased_impl(
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

    let rounded_size_key = round_float_key(size);
    let font_metrics = font.horizontal_line_metrics(size).unwrap();

    for ch in text.chars() {
        // Try to get the glyph from cache first
        let (metrics, bitmap) = {
            let cache = _get_glyph_cache().read().unwrap();
            cache.get(&(ch, rounded_size_key)).cloned()
        }
        .unwrap_or_else(|| {
            let rasterized = font.rasterize(ch, size);

            // Insert into cache
            let mut cache_mut = _get_glyph_cache().write().unwrap();
            cache_mut.insert((ch, rounded_size_key), rasterized.clone());

            rasterized
        });

        // Draw each character into the buffer
        for gy in 0..metrics.height {
            for gx in 0..metrics.width {
                let px = pen_x + gx;
                // Correcting for letter height
                let py = pen_y
                    + gy
                    + (font_metrics.ascent - metrics.height as f32) as usize;

                if px < buffer.width && py < buffer.height {
                    let index = py * buffer.width + px;
                    let alpha = bitmap[gy * metrics.width + gx]; // Alpha (0-255)

                    if alpha > 0 {
                        unsafe {
                            let bg = *buffer.pointer.add(index);
                            // Extract RGBA
                            let (br, bg, bb, ba) = (
                                (bg >> 24) & 0xFF,
                                (bg >> 16) & 0xFF,
                                (bg >> 8) & 0xFF,
                                bg & 0xFF,
                            );
                            let (tr, tg, tb, ta) = (
                                (color >> 24) & 0xFF,
                                (color >> 16) & 0xFF,
                                (color >> 8) & 0xFF,
                                color & 0xFF,
                            );

                            // Alpha blending
                            let inv_alpha: u8 = 255 - alpha;
                            let nr = ((tr as u16 * alpha as u16
                                + br as u16 * inv_alpha as u16)
                                / 255)
                                as u8;
                            let ng = ((tg as u16 * alpha as u16
                                + bg as u16 * inv_alpha as u16)
                                / 255)
                                as u8;
                            let nb = ((tb as u16 * alpha as u16
                                + bb as u16 * inv_alpha as u16)
                                / 255)
                                as u8;
                            let na = ((ta as u16 * alpha as u16
                                + ba as u16 * inv_alpha as u16)
                                / 255)
                                as u8;

                            draw_pixel(
                                buffer,
                                px,
                                py,
                                (nr as u32) << 24
                                    | (ng as u32) << 16
                                    | (nb as u32) << 8
                                    | na as u32,
                            );
                        }
                    }
                }
            }
        }

        // Advance the cursor position
        pen_x += metrics.advance_width as usize;
    }
}
