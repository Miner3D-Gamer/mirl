use std::collections::HashMap;
use std::sync::RwLock;

/// Key:
/// char: Requested letter
/// (i32, i32): Dimensions
/// usize: Hash (so multiple fonts can be used at the same time)
/// 
/// Data:
/// fontdue::Metrics: Positioning data
/// Vec<u8>: Rasterized font data (alpha)
static GLYPH_CACHE: once_cell::sync::Lazy<
    RwLock<HashMap<(char, (i32, i32), usize), (fontdue::Metrics, Vec<u8>)>>,
> = once_cell::sync::Lazy::new(|| RwLock::new(HashMap::new()));

/// Get a glyph from the cache if it exists
#[inline(always)]
pub fn _get_glyph_cache(
) -> &'static RwLock<HashMap<(char, (i32, i32), usize), (fontdue::Metrics, Vec<u8>)>> {
    &GLYPH_CACHE
}
/// Reset the glyph cache
pub fn _reset_glyph_cache() {
    GLYPH_CACHE.write().unwrap().clear();
}
/// Removes a selected glyph from the glyph cache
pub fn _remove_glyph_from_glyph_cache(glyph: &(char, (i32, i32), usize)) {
    GLYPH_CACHE.write().unwrap().remove(glyph);
}
/// Manually add a glyph to the glyph cache
pub fn _add_to_glyph_cache(
    key: (char, (i32, i32), usize),
    data: (fontdue::Metrics, Vec<u8>),
) {
    GLYPH_CACHE.write().unwrap().insert(key, data);
}

#[inline(always)]
fn round_float_key(value: f32) -> (i32, i32) {
    let multiplier = 10.0_f32.powi(4);
    let rounded_int_x = (value * multiplier).round() as i32;
    let rounded_int_y = (value * multiplier).fract() as i32;
    (rounded_int_x, rounded_int_y)
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
        cache.get(&(ch, rounded_size_key, font.file_hash())).cloned()
    }
    .unwrap_or_else(|| {
        let rasterized = font.rasterize(ch, size);

        _add_to_glyph_cache((ch, rounded_size_key, font.file_hash()), rasterized.clone());

        rasterized
    });
    return x;
}
