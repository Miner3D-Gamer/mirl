use crate::graphics::{
    adjust_brightness, get_hue_of_rgb, hsl_to_rgb_f32, BrightnessModel,
};
use crate::render::RenderSettings;
use crate::render::{get_glyph_cache, round_float_key};

pub struct RenderSettingsPretty {}
impl RenderSettings for RenderSettingsPretty {
    #[inline]
    fn draw_pixel(
        &self,
        buffer: *mut u32,
        width: usize,
        height: usize,
        x: usize,
        y: usize,
        color: u32,
    ) {
        if x >= width || y >= height {
            return;
        }

        unsafe {
            *buffer.add(y * width + x) = color;
        }
    }
    #[inline]
    fn draw_text(
        &self,
        buffer: *mut u32,
        width: usize,
        height: usize,
        text: &str,
        x: usize,
        y: usize,
        color: u32,
        size: f32,
        font: &fontdue::Font,
    ) {
        let mut pen_x = x;
        let pen_y = y;

        let rounded_size_key = round_float_key(size);
        let font_metrics = font.horizontal_line_metrics(size).unwrap();

        for ch in text.chars() {
            // Try to get the glyph from cache first
            let (metrics, bitmap) = {
                let cache = get_glyph_cache().read().unwrap();
                cache.get(&(ch, rounded_size_key)).cloned()
            }
            .unwrap_or_else(|| {
                let rasterized = font.rasterize(ch, size);

                // Insert into cache
                let mut cache_mut = get_glyph_cache().write().unwrap();
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
                        + (font_metrics.ascent - metrics.height as f32)
                            as usize;

                    if px < width && py < height {
                        let index = py * width + px;
                        let alpha = bitmap[gy * metrics.width + gx]; // Alpha (0-255)

                        if alpha > 0 {
                            unsafe {
                                let bg = *buffer.add(index);
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

                                self.draw_pixel(
                                    buffer,
                                    width,
                                    height,
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
    #[inline]
    fn adjust_brightness(&self, color: u32, x: i32) -> u32 {
        adjust_brightness(color, x, BrightnessModel::LinearWeighted)
    }
    #[inline]
    fn desaturate(&self, color: u32, amount: f32) -> u32 {
        let r = ((color >> 16) & 0xFF) as f32 / 255.0;
        let g = ((color >> 8) & 0xFF) as f32 / 255.0;
        let b = (color & 0xFF) as f32 / 255.0;

        // Convert RGB to HSL
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        let s = if max == min {
            0.0
        } else if l < 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        };

        // Reduce saturation toward grayscale
        let new_s = s * (1.0 - amount);

        // Reconstruct color from HSL
        let (r2, g2, b2) = hsl_to_rgb_f32(get_hue_of_rgb(r, g, b), new_s, l);
        let r_new = (r2 * 255.0).round().clamp(0.0, 255.0) as u32;
        let g_new = (g2 * 255.0).round().clamp(0.0, 255.0) as u32;
        let b_new = (b2 * 255.0).round().clamp(0.0, 255.0) as u32;

        (r_new << 16) | (g_new << 8) | b_new
    }
}
