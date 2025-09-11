use super::get_character;
use crate::{
    platform::Buffer,
    render::{
        draw_pixel_safe, draw_pixel_unsafe, rendering::DrawPixelFunction,
    },
};
/// Draw text in the specified font
pub fn draw_text<const SAFE: bool>(
    buffer: &Buffer,
    text: &str,
    x: usize,
    y: usize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
) {
    let mut pen_x = x as f32;
    let pen_y = y;
    let font_metrics = font.horizontal_line_metrics(size).unwrap();
    let ascent = font_metrics.ascent as usize;

    for ch in text.chars() {
        // If not in cache, rasterize and insert
        let char = get_character(ch, size, &font);
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height);
        let w = metrics.width;
        let h = metrics.height;
        let advance_x = metrics.advance_width;

        for gy in 0..h {
            let py = ((pen_y + gy + offset_y) as i32 - metrics.ymin) as usize;
            if py >= buffer.height {
                continue;
            }

            let row_start = gy * w;
            for gx in 0..w {
                let px = pen_x as usize + gx;
                if px >= buffer.width {
                    continue;
                }

                if bitmap[row_start + gx] > 0 {
                    if SAFE {
                        draw_pixel_safe(buffer, (px, py), color)
                    } else {
                        draw_pixel_unsafe(buffer, (px, py), color)
                    }
                }
            }
        }
        pen_x += advance_x;
    }
}

/// Draw text yet stretch the resulting characters
pub fn draw_text_stretched<F: num_traits::Float>(
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
    let mut pen_x = x as f32;
    let pen_y = y;
    let font_metrics = font.horizontal_line_metrics(size).unwrap();
    let ascent = font_metrics.ascent as usize;

    for ch in text.chars() {
        // If not in cache, rasterize and insert
        let char = get_character(ch, size, &font);
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height);
        let w = F::from(metrics.width).unwrap() * stretch_x;
        let h = F::from(metrics.height).unwrap() * stretch_y;
        let advance_x = metrics.advance_width;

        for gy in 0..h.floor().to_usize().unwrap() {
            let py = ((pen_y + gy + offset_y) as i32 - metrics.ymin) as usize;
            if py >= buffer.height {
                continue;
            }

            let bitmap_row_start =
                ((F::from(gy).unwrap() / stretch_y).to_usize().unwrap())
                    * metrics.width;
            for gx in 0..w.floor().to_usize().unwrap() {
                let px = pen_x as usize + gx;
                if px >= buffer.width {
                    continue;
                }

                if bitmap[bitmap_row_start
                    + (F::from(gx).unwrap() / stretch_x).to_usize().unwrap()]
                    > 0
                {
                    draw_pixel(buffer, (px, py), color);
                }
            }
        }
        pen_x += advance_x;
    }
}

/// Same as [draw_text] but uses isize for positioning allowing for partially out of bounce text (left and top)
pub fn draw_text_isize<const SAFE: bool>(
    buffer: &Buffer,
    text: &str,
    x: isize,
    y: isize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
) {
    let mut pen_x = x as f32;
    let pen_y = y;
    let font_metrics = font.horizontal_line_metrics(size).unwrap();
    let ascent = font_metrics.ascent as isize;

    for ch in text.chars() {
        // If not in cache, rasterize and insert
        let char = get_character(ch, size, &font);
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height as isize);
        let w = metrics.width;
        let h = metrics.height;
        let advance_x = metrics.advance_width;

        for gy in 0..h {
            let py = pen_y + gy as isize + offset_y - metrics.ymin as isize;
            if py < 0 || py >= buffer.height as isize {
                continue;
            }

            let row_start = gy * w;
            for gx in 0..w {
                let px = pen_x as isize + gx as isize;
                if px < 0 || px >= buffer.width as isize {
                    continue;
                }

                if bitmap[row_start + gx] > 0 {
                    let px_u = px as usize;
                    let py_u = py as usize;
                    let index = py_u * buffer.width + px_u;

                    if SAFE && index >= buffer.width * buffer.height {
                        continue;
                    }

                    if SAFE {
                        draw_pixel_safe(buffer, (px_u, py_u), color)
                    } else {
                        draw_pixel_unsafe(buffer, (px_u, py_u), color)
                    }
                }
            }
        }
        pen_x += advance_x;
    }
}

/// Same as [draw_text_stretched] but uses isize for positioning allowing for partially out of bounce text (left and top)
pub fn draw_text_stretch_isize<F: num_traits::Float>(
    buffer: &Buffer,
    text: &str,
    x: isize,
    y: isize,
    color: u32,
    size: f32,
    font: &fontdue::Font,
    draw_pixel: DrawPixelFunction,
    stretch_x: F,
    stretch_y: F,
) {
    let mut pen_x = x as f32;
    let pen_y = y;
    let font_metrics = font.horizontal_line_metrics(size).unwrap();
    let ascent = font_metrics.ascent as isize;

    for ch in text.chars() {
        // If not in cache, rasterize and insert
        let char = get_character(ch, size, &font);
        let metrics = char.0;
        let bitmap = &char.1;

        let offset_y = ascent.saturating_sub(metrics.height as isize);
        let w = F::from(metrics.width).unwrap() * stretch_x;
        let h = F::from(metrics.height).unwrap() * stretch_y;
        let advance_x = metrics.advance_width;

        for gy in 0..h.floor().to_usize().unwrap() {
            let py = pen_y + gy as isize + offset_y - metrics.ymin as isize;
            if py < 0 || py >= buffer.height as isize {
                continue;
            }

            let bitmap_row_start =
                ((F::from(gy).unwrap() / stretch_y).to_usize().unwrap())
                    * metrics.width;
            for gx in 0..w.floor().to_usize().unwrap() {
                let px = pen_x as isize + gx as isize;
                if px < 0 || px >= buffer.width as isize {
                    continue;
                }

                if bitmap[bitmap_row_start
                    + (F::from(gx).unwrap() / stretch_x).to_usize().unwrap()]
                    > 0
                {
                    draw_pixel(buffer, (px as usize, py as usize), color);
                }
            }
        }
        pen_x += advance_x;
    }
}
