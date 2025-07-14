use super::get_character;
use crate::{
    platform::Buffer,
    render::{
        draw_pixel_safe, draw_pixel_unsafe,
        rendering::{get_pixel, DrawPixelFunction},
    },
};
/// Draw text in the specified font
pub fn draw_text_antialiased(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
    safe: bool,
) {
    let draw_pixel: DrawPixelFunction = {
        if safe {
            draw_pixel_safe
        } else {
            draw_pixel_unsafe
        }
    };
    let mut pen_x = x;
    let pen_y = y;

    let font_metrics = font.horizontal_line_metrics(size).unwrap();
    let ascent = font_metrics.ascent as usize;

    for ch in text.chars() {
        // Try to get the glyph from cache first
        let (metrics, bitmap) = get_character(ch, size, &font);

        let offset_y = ascent.saturating_sub(metrics.height);
        // Draw each character into the buffer
        for gy in 0..metrics.height {
            for gx in 0..metrics.width {
                let px = pen_x + gx;
                // Correcting for letter height
                let py =
                    ((pen_y + gy + offset_y) as i32 - metrics.ymin) as usize;

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


/// Draw text yet stretch the resulting characters
pub fn draw_text_antialiased_stretched<F: num_traits::Float>(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
    stretch_x: F,
    stretch_y: F,
    safe: bool,
) {
    let draw_pixel: DrawPixelFunction = {
        if safe {
            draw_pixel_safe
        } else {
            draw_pixel_unsafe
        }
    };
    let mut pen_x = x;
    let pen_y = y;
    let font_metrics = font.horizontal_line_metrics(size).unwrap();
    let ascent = font_metrics.ascent as usize;

    for ch in text.chars() {
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

                let bitmap_index = bitmap_row_start
                    + (F::from(gx).unwrap() / stretch_x).to_usize().unwrap();
                let alpha = bitmap[bitmap_index];

                if alpha > 0 {
                    let bg = get_pixel(buffer, px, py); // Fixed: use px instead of gx
                                                        // Extract RGBA
                    let (br, bg_g, bb, ba) = (
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
                        / 255) as u8;
                    let ng = ((tg as u16 * alpha as u16
                        + bg_g as u16 * inv_alpha as u16)
                        / 255) as u8;
                    let nb = ((tb as u16 * alpha as u16
                        + bb as u16 * inv_alpha as u16)
                        / 255) as u8;
                    let na = ((ta as u16 * alpha as u16
                        + ba as u16 * inv_alpha as u16)
                        / 255) as u8;

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

        pen_x += advance_x;
    }
}
