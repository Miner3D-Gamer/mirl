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
pub fn _reset_glyph_cache() {
    GLYPH_CACHE.write().unwrap().clear();
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
    unsafe_fast: bool,
    stretch_x: f32,
    stretch_y: f32,
) {
    if stretch_x == 1.0 && stretch_y == 1.0 {
        if aliased {
            if unsafe_fast {
                draw_text_unsafe(buffer, text, x, y, color, size, font);
            } else {
                draw_text(buffer, text, x, y, color, size, font);
            }
        } else {
            if unsafe_fast {
                draw_text_antialiased_unsafe(
                    buffer, text, x, y, color, size, font,
                );
            } else {
                draw_text_antialiased(buffer, text, x, y, color, size, font);
            }
        }
    } else {
        if aliased {
            if unsafe_fast {
                draw_text_stretched_unsafe(
                    buffer, text, x, y, color, size, font, stretch_x, stretch_y,
                );
            } else {
                draw_text_stretched(
                    buffer, text, x, y, color, size, font, stretch_x, stretch_y,
                );
            }
        } else {
            if unsafe_fast {
                draw_text_antialiased_stretched_unsafe(
                    buffer, text, x, y, color, size, font, stretch_x, stretch_y,
                );
            } else {
                draw_text_antialiased_stretched(
                    buffer, text, x, y, color, size, font, stretch_x, stretch_y,
                );
            }
        }
    }
}

mod aliased;
mod antialiased;
pub use aliased::*;
pub use antialiased::*;

// #[inline]
// pub fn draw_text_angled_aliased(
//     buffer: &Buffer,
//     text: &str,
//     x: usize,
//     y: usize,
//     color: u32,
//     size: f32,
//     font: &fontdue::Font,
//     angle: f32,
// ) {
//     draw_text_angled_aliased_impl(
//         buffer,
//         text,
//         x,
//         y,
//         color,
//         size,
//         angle,
//         font,
//         draw_pixel_safe,
//     );
// }

// #[inline]
// pub fn draw_text_angled_aliased_unsafe(
//     buffer: &Buffer,
//     text: &str,
//     x: usize,
//     y: usize,
//     color: u32,
//     size: f32,
//     font: &fontdue::Font,
//     angle: f32,
// ) {
//     draw_text_angled_aliased_impl(
//         buffer,
//         text,
//         x,
//         y,
//         color,
//         size,
//         angle,
//         font,
//         draw_pixel_unsafe,
//     );
// }

/// Returns the metrics and bitmap of the character from cache (creating and rasterizing it if needed)
pub fn get_character(
    ch: char,
    size: f32,
    font: &fontdue::Font,
) -> (fontdue::Metrics, Vec<u8>) {
    let rounded_size_key = round_float_key(size);

    let x = {
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
    return x;
}
