use crate::extensions::*;

/// Convert an r b g format into u32 argb format
#[inline(always)]
pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

#[inline(always)]
/// Convert r g b a in argb format
pub fn rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}
#[inline(always)]
/// Convert r g b a into u32 argb
pub fn rgba_u32_to_u32(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (a) << 24 | (r) << 16 | (g) << 8 | b
}
#[inline(always)]
/// Convert u32 argb to r g b
pub fn u32_to_rgb(color: u32) -> (u8, u8, u8) {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    (r, g, b)
}
#[inline(always)]
/// Convert u32 argb to r g b
pub fn u32_to_rgb_u32(color: u32) -> (u32, u32, u32) {
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;
    (r, g, b)
}
#[inline(always)]
/// Convert u32 argb to r g b a or u32 rgba to a g b r
pub fn u32_to_rgba(color: u32) -> (u8, u8, u8, u8) {
    let a = ((color >> 24) & 0xFF) as u8;
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    (r, g, b, a)
}
#[inline(always)]
/// Convert u32 argb to r g b a or u32 rgba to a g b r
pub fn u32_to_rgba_u32(color: u32) -> (u32, u32, u32, u32) {
    let a = (color >> 24) & 0xFF;
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;
    (r, g, b, a)
}
#[inline(always)]
/// Convert u32 argb to a g b r or u32 rgba to r g b a
pub fn u32_to_argb(color: u32) -> (u8, u8, u8, u8) {
    let a = ((color >> 24) & 0xFF) as u8;
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    (a, r, g, b)
}
#[inline(always)]
/// Convert u32 argb to a g b r or u32 rgba to r g b a
pub fn u32_to_argb_u32(color: u32) -> (u32, u32, u32, u32) {
    let a = (color >> 24) & 0xFF;
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;
    (a, r, g, b)
}

#[inline(always)]
/// Get the alpha of a u32 in argb style, get red rgba style
pub fn get_alpha_of_u32(color: u32) -> u8 {
    ((color >> 24) & 0xFF) as u8
}

#[inline(always)]
/// Get the red of a u32 in argb style, get alpha rgba style
pub fn get_red_of_u32(color: u32) -> u8 {
    ((color >> 16) & 0xFF) as u8
}
#[inline(always)]
/// Get the green of a u32
pub fn get_green_of_u32(color: u32) -> u8 {
    ((color >> 8) & 0xFF) as u8
}
#[inline(always)]
/// Get the blue of a u32
pub fn get_blue_of_u32(color: u32) -> u8 {
    (color & 0xFF) as u8
}
//

#[inline(always)]
/// Get the alpha of a u32 in argb style, get red rgba style
pub fn get_u32_alpha_of_u32(color: u32) -> u32 {
    (color >> 24) & 0xFF
}

#[inline(always)]
/// Get the red of a u32 in argb style, get alpha rgba style
pub fn get_u32_red_of_u32(color: u32) -> u32 {
    (color >> 16) & 0xFF
}
#[inline(always)]
/// Get the green of a u32
pub fn get_u32_green_of_u32(color: u32) -> u32 {
    (color >> 8) & 0xFF
}
#[inline(always)]
/// Get the blue of a u32
pub fn get_u32_blue_of_u32(color: u32) -> u32 {
    color & 0xFF
}
/// Image support for mirl
#[cfg(feature = "imagery")]
pub mod imagery;
use std::collections::HashSet;

#[cfg(feature = "imagery")]
pub use imagery::*;

#[inline]
/// Get hue of rgb (hsl space)
pub fn get_hue_of_rgb(r: f32, g: f32, b: f32) -> f32 {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);

    if max == min {
        0.0
    } else if max == r {
        ((g - b) / (max - min)).rem_euclid(6.0) * 60.0
    } else if max == g {
        ((b - r) / (max - min) + 2.0) * 60.0
    } else {
        ((r - g) / (max - min) + 4.0) * 60.0
    }
}

#[inline]
/// Change the brightness of a hsl space
pub fn adjust_brightness_hsl_of_rgb(color: u32, change: i32) -> u32 {
    let a = (color >> 24) & 0xFF;
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;

    let (h, s, l) = rgb_to_hsl(r as u8, g as u8, b as u8);

    // Adjust lightness in HSL space (most perceptually accurate)
    let l_new = (l + change as f32).clamp(0.0, 100.0);

    let (r_new, g_new, b_new) = hsl_to_rgb_u32(h, s, l_new);

    (a << 24) | ((r_new as u32) << 16) | ((g_new as u32) << 8) | b_new as u32
}
#[inline]
/// Convert hsl space to rgb space
pub fn hsl_to_rgb_f32(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r1, g1, b1) = match h as i32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=359 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    (r1 + m, g1 + m, b1 + m)
}

#[inline]
/// Convert rgb space to hsl space
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r_norm = r as f32 / 255.0;
    let g_norm = g as f32 / 255.0;
    let b_norm = b as f32 / 255.0;

    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);
    let delta = max - min;

    let lightness = (max + min) / 2.0;

    let saturation = if delta < 0.0001 {
        0.0 // achromatic (gray)
    } else {
        if lightness < 0.5 {
            delta / (max + min)
        } else {
            delta / (2.0 - max - min)
        }
    };

    let hue = if delta < 0.0001 {
        0.0 // achromatic (gray)
    } else if max == r_norm {
        60.0 * (((g_norm - b_norm) / delta) % 6.0)
    } else if max == g_norm {
        60.0 * ((b_norm - r_norm) / delta + 2.0)
    } else {
        60.0 * ((r_norm - g_norm) / delta + 4.0)
    };

    (hue, saturation * 100.0, lightness * 100.0)
}

#[inline]
/// Convert hsl space to rgb space

pub fn hsl_to_rgb_u32(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let h_norm = h / 360.0;
    let s_norm = s / 100.0;
    let l_norm = l / 100.0;

    if s_norm < 0.0001 {
        let gray = (l_norm * 255.0).round() as u8;
        return (gray, gray, gray);
    }

    let hue_to_rgb = |p: f32, q: f32, t: f32| -> f32 {
        let t_adj = if t < 0.0 {
            t + 1.0
        } else if t > 1.0 {
            t - 1.0
        } else {
            t
        };

        if t_adj < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t_adj
        } else if t_adj < 1.0 / 2.0 {
            q
        } else if t_adj < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t_adj) * 6.0
        } else {
            p
        }
    };

    let q = if l_norm < 0.5 {
        l_norm * (1.0 + s_norm)
    } else {
        l_norm + s_norm - l_norm * s_norm
    };
    let p = 2.0 * l_norm - q;

    let r = hue_to_rgb(p, q, h_norm + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h_norm);
    let b = hue_to_rgb(p, q, h_norm - 1.0 / 3.0);

    let r_8bit = (r * 255.0).round() as u8;
    let g_8bit = (g * 255.0).round() as u8;
    let b_8bit = (b * 255.0).round() as u8;

    (r_8bit, g_8bit, b_8bit)
}

/// Higher-level function that provides both perceptual models
pub enum BrightnessModel {
    /// Uses RGB with perceptual weights
    LinearWeighted,
    /// Uses HSL color space
    HSL,
}
#[inline]
/// Change the brightness of an rgba color
pub fn adjust_brightness_based_on_human_eye(
    color: u32,
    x: i32,
    model: BrightnessModel,
) -> u32 {
    match model {
        BrightnessModel::LinearWeighted => {
            // Extract color components
            let a = (color >> 24) & 0xFF;
            let r = ((color >> 16) & 0xFF) as f32;
            let g = ((color >> 8) & 0xFF) as f32;
            let b = (color & 0xFF) as f32;

            // Apply perceptual weights to adjustment
            let r_adj = x as f32 * 0.2126;
            let g_adj = x as f32 * 0.7152;
            let b_adj = x as f32 * 0.0722;

            // Apply adjustments with clamping
            let r_new = (r + r_adj).clamp(0.0, 255.0) as u32;
            let g_new = (g + g_adj).clamp(0.0, 255.0) as u32;
            let b_new = (b + b_adj).clamp(0.0, 255.0) as u32;

            // Recombine with alpha
            (a << 24) | (r_new << 16) | (g_new << 8) | b_new
        }
        BrightnessModel::HSL => adjust_brightness_hsl_of_rgb(color, x),
    }
}

#[inline]
/// Shift the color (hue) of rgb
pub fn shift_color_rgb(r: u8, g: u8, b: u8, hue_shift: f32) -> (u8, u8, u8) {
    let (h, s, l) = rgb_to_hsl(r, g, b);

    let new_h = (h + hue_shift) % 360.0;

    let l_norm = l / 100.0;
    let s_norm = s / 100.0;

    let new_s = if l_norm > 0.5 {
        // Brighter colors - reduce saturation as lightness increases
        s_norm * (1.0 - (l_norm - 0.5) * 2.0) * 100.0
    } else {
        // Darker colors - reduce saturation as lightness decreases
        s_norm * (1.0 - (0.5 - l_norm) * 2.0) * 100.0
    };

    let new_l = if l_norm > 0.5 {
        // Brighter colors - darken slightly to maintain balance
        l_norm * 0.95 * 100.0
    } else {
        // Darker colors - lighten slightly, but don't go over 100
        (l_norm * 1.1).min(1.0) * 100.0
    };

    let new_s = new_s.max(0.0).min(100.0);
    let new_l = new_l.max(0.0).min(100.0);

    hsl_to_rgb_u32(new_h, new_s, new_l)
}

/// Shift the hue of rgb, isn't there another function that does the exact same?
pub fn shift_hue_rgb(
    r: u8,
    g: u8,
    b: u8,
    hue_shift_degrees: f32,
) -> (u8, u8, u8) {
    // Convert to floating point RGB
    let mut hsv = rgb_to_hsl(r, g, b);

    // Shift hue
    hsv.0 = (hsv.0 + hue_shift_degrees) % 360.0;

    // Convert back to integer RGB
    let (r, g, b) = hsl_to_rgb_u32(hsv.0, hsv.1, hsv.2);
    (r, g, b)
}
/// Shift the hue of a rgba u32
pub fn shift_hue_u32(color: u32, hue_shift: f32) -> u32 {
    let (r, g, b) = u32_to_rgb(color);
    let (r, g, b) = shift_hue_rgb(r, g, b, hue_shift);
    return rgb_to_u32(r, g, b);
}
#[inline]
/// Shift the color of a rgba u32
pub fn shift_color_u32(color: u32, hue_shift: f32) -> u32 {
    let a = (color >> 24) & 0xFF;
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;

    let (r_new, g_new, b_new) =
        shift_color_rgb(r as u8, g as u8, b as u8, hue_shift);

    (a << 24) | ((r_new as u32) << 16) | ((g_new as u32) << 8) | b_new as u32
}

#[inline]
/// Adjust the brightness of a rgba u32 color faster than with the function that does it with with human perception in mind
pub fn adjust_brightness_fast(color: u32, x: i32) -> u32 {
    // Extract color components
    let r = ((color >> 16) & 0xFF) as i32;
    let g = ((color >> 8) & 0xFF) as i32;
    let b = (color & 0xFF) as i32;

    // Calculate new values with clamping
    let r_new = (r + x).clamp(0, 255) as u32;
    let g_new = (g + x).clamp(0, 255) as u32;
    let b_new = (b + x).clamp(0, 255) as u32;

    // Recombine into a single color value
    (r_new << 16) | (g_new << 8) | b_new
}
/// Desaturate the current color without caring that much about human vision
#[inline]
pub fn desaturate_fast(color: u32, amount: f32) -> u32 {
    // Extract color components
    let r = ((color >> 16) & 0xFF) as f32;
    let g = ((color >> 8) & 0xFF) as f32;
    let b = (color & 0xFF) as f32;

    // Compute grayscale (luminance approximation)
    let gray = 0.299 * r + 0.587 * g + 0.114 * b;

    // Interpolate between color and gray based on amount (0.0 to 1.0)
    let r_new =
        ((r * (1.0 - amount)) + (gray * amount)).clamp(0.0, 255.0) as u32;
    let g_new =
        ((g * (1.0 - amount)) + (gray * amount)).clamp(0.0, 255.0) as u32;
    let b_new =
        ((b * (1.0 - amount)) + (gray * amount)).clamp(0.0, 255.0) as u32;

    // Recombine
    (r_new << 16) | (g_new << 8) | b_new
}
/// Rasterize an svg in the desired dimensions
#[cfg(feature = "resvg")]
pub fn rasterize_svg(
    svg_data: &[u8],
    width: u32,
    height: u32,
) -> resvg::tiny_skia::Pixmap {
    let opt = resvg::usvg::Options::default();
    //let fontdb = usvg::fontdb::Database::new();

    let tree = resvg::usvg::Tree::from_data(&svg_data, &opt).unwrap();

    // Create a pixmap with desired size (from SVG's size)
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)
        .ok_or("Failed to create pixmap")
        .unwrap();

    // Render the SVG
    resvg::render(
        &tree,
        resvg::usvg::Transform::default(),
        &mut pixmap.as_mut(),
    );

    return pixmap;
}
#[cfg(feature = "resvg")]
/// To use this function, enable the "svg_support" feature
pub fn pixmap_to_raw_image(pixmap: &resvg::tiny_skia::Pixmap) -> RawImage {
    let mut data = Vec::new();
    for y in 0..pixmap.height() {
        for x in 0..pixmap.width() {
            let color = pixmap.pixel(x, y).unwrap();
            data.push(rgba_to_u32(
                color.red(),
                color.green(),
                color.blue(),
                color.alpha(),
            ));
        }
    }
    RawImage::new(data, pixmap.width() as usize, pixmap.height() as usize)
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "glfw_backend")]
use glfw::PixelImage;
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "glfw_backend")]

/// Convert a RawImage into a glfw::PixelImage
#[inline(always)]
pub fn raw_image_to_pixel_image(raw_image: &RawImage) -> glfw::PixelImage {
    return glfw::PixelImage {
        width: raw_image.width as u32,
        height: raw_image.height as u32,
        pixels: argb_list_to_rgba_list(&raw_image.data),
    };
}
/// Convert a glfw::PixelImage into a RawImage
#[cfg(feature = "glfw_backend")]
#[cfg(not(target_arch = "wasm32"))]
#[inline(always)]
pub fn pixel_image_to_raw_image(pixel_image: &glfw::PixelImage) -> RawImage {
    return RawImage::new(
        rgba_list_to_argb_list(&pixel_image.pixels),
        pixel_image.width as usize,
        pixel_image.height as usize,
    );
}
/// A RawImage to be accessed without compression
/// What is the difference between RawImage and Buffer? Buffer has more attributes ig :|
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawImage {
    /// The Raw Data
    pub data: Box<[u32]>,
    /// The width of the image
    pub width: usize,
    /// The height of the image
    pub height: usize,
}

impl RawImage {
    /// Create a new RawImage with some data, if you want to create an empty image use RawImage::new_empty()
    pub fn new(data: Vec<u32>, width: usize, height: usize) -> Self {
        Self {
            data: data.into_boxed_slice(),
            width,
            height,
        }
    }
    /// Create a new, empty, RawImage instance. If you already have data to fill it with you can use RawImage::new()
    pub fn new_empty(width: usize, height: usize) -> Self {
        Self {
            data: repeat_data(0, width * height).into(),
            width: width,
            height: height,
        }
    }
    /// Gets the pixel color at the requested 3d coordinates
    pub fn get_pixel(&self, pos: (usize, usize)) -> u32 {
        return self.data[pos.1 * self.width + pos.0];
    }
    /// Generate a error texture with the desired size
    pub fn generate_fallback(
        width: usize,
        height: usize,
        square_size: usize,
    ) -> Self {
        let mut data = Vec::with_capacity(width * height);

        let purple = rgb_to_u32(128, 0, 128);
        let black = rgb_to_u32(0, 0, 0);

        for y in 0..height {
            for x in 0..width {
                let square_x = x / square_size;
                let square_y = y / square_size;

                let color = if (square_x + square_y) % 2 == 0 {
                    purple
                } else {
                    black
                };

                data.push(color);
            }
        }

        Self::new(data, width, height)
    }
    /// Optimizes the image by removing empty space around the image
    pub fn remove_margins(&mut self) {
        // Remove all margins in one pass to avoid multiple data copies
        let (top_trim, bottom_trim, left_trim, right_trim) =
            self.calculate_trims();

        if top_trim > 0 || bottom_trim > 0 || left_trim > 0 || right_trim > 0 {
            self.apply_trim(top_trim, bottom_trim, left_trim, right_trim);
        }
    }
    /// Calculates the empty space around the image
    pub fn calculate_trims(&self) -> (usize, usize, usize, usize) {
        let mut top_trim = 0;
        let mut bottom_trim = 0;
        let mut left_trim = 0;
        let mut right_trim = 0;

        // Calculate top trim
        for row in 0..self.height {
            if self.is_row_transparent(row) {
                top_trim += 1;
            } else {
                break;
            }
        }

        // Calculate bottom trim
        for row in (0..self.height).rev() {
            if self.is_row_transparent(row) {
                bottom_trim += 1;
            } else {
                break;
            }
        }

        // Calculate left trim
        for col in 0..self.width {
            if self.is_col_transparent(col) {
                left_trim += 1;
            } else {
                break;
            }
        }

        // Calculate right trim
        for col in (0..self.width).rev() {
            if self.is_col_transparent(col) {
                right_trim += 1;
            } else {
                break;
            }
        }

        (top_trim, bottom_trim, left_trim, right_trim)
    }
    /// Checks if the requested row only has fully transparent pixels
    pub fn is_row_transparent(&self, row: usize) -> bool {
        let start = row * self.width;
        let end = start + self.width;
        self.data[start..end]
            .iter()
            .all(|&pixel| get_u32_alpha_of_u32(pixel) == 0)
    }
    /// Checks if the requested column only has fully transparent pixels
    pub fn is_col_transparent(&self, col: usize) -> bool {
        (0..self.height).all(|row| {
            get_u32_alpha_of_u32(self.data[row * self.width + col]) == 0
        })
    }
    /// Trims the image by the given restrictions
    pub fn apply_trim(
        &mut self,
        top: usize,
        bottom: usize,
        left: usize,
        right: usize,
    ) {
        let new_width = self.width - left - right;
        let new_height = self.height - top - bottom;
        let mut new_data = Vec::with_capacity(new_width * new_height);

        for row in top..(self.height - bottom) {
            let row_start = row * self.width + left;
            let row_end = row_start + new_width;
            new_data.extend_from_slice(&self.data[row_start..row_end]);
        }

        self.data = new_data.into();
        self.width = new_width;
        self.height = new_height;
    }
}

impl From<Buffer> for RawImage {
    fn from(p: Buffer) -> Self {
        RawImage {
            data: p.buffer,
            width: p.width,
            height: p.height,
        }
    }
}
impl From<RawImage> for Buffer {
    fn from(p: RawImage) -> Self {
        let mut buffer = Buffer::new(p.width, p.height);
        buffer.buffer = p.data;
        return buffer;
    }
}

mod pixel;
pub use pixel::*;

#[cfg(feature = "imagery")]
use crate::platform::FileSystem;
use crate::{math::interpolate, misc::repeat_data, platform::Buffer};
/// Convert u32 argb to hex
#[inline(always)]
pub fn u32_to_hex(color: u32) -> String {
    format!(
        "{:02x}{:02x}{:02x}{:02x}",
        color >> 24,
        (color >> 16) & 0xFF,
        (color >> 8) & 0xFF,
        color & 0xFF
    )
}

/// Convert hex into u32 argb
#[inline(always)]
pub fn hex_to_u32(hex: &str) -> u32 {
    let a = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let r = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let g = u8::from_str_radix(&hex[4..6], 16).unwrap();
    let b = u8::from_str_radix(&hex[6..8], 16).unwrap();
    (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}
/// Convert hex into u32 rgba
#[inline(always)]
pub fn hex_to_u32_rgba(hex: &str) -> u32 {
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    let a = u8::from_str_radix(&hex[6..8], 16).unwrap();
    (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}
/// Convert hex into u32 rgb
#[inline(always)]
pub fn hex_to_u32_rgb(hex: &str) -> u32 {
    let r = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let g = u8::from_str_radix(&hex[4..6], 16).unwrap();
    let b = u8::from_str_radix(&hex[6..8], 16).unwrap();
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

/// Converts rgb into hex
#[inline(always)]
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("{:02x}{:02x}{:02x}", r, g, b)
}

/// Converts a list of argb to rgba and vice versa
#[inline(always)]
pub fn argb_list_to_rgba_list(input: &[u32]) -> Vec<u32> {
    input
        .iter()
        .map(|&argb| {
            let a = (argb >> 24) & 0xFF;
            let r = (argb >> 16) & 0xFF;
            let g = (argb >> 8) & 0xFF;
            let b = argb & 0xFF;
            (r as u32)
                | ((g as u32) << 8)
                | ((b as u32) << 16)
                | ((a as u32) << 24)
            // RGBA layout: 0xRRGGBBAA
        })
        .collect()
}
/// Converts a list of rgba to argb and vice versa
pub fn rgba_list_to_argb_list(input: &[u32]) -> Vec<u32> {
    input
        .iter()
        .map(|&rgba| {
            let r = (rgba >> 16) & 0xFF;
            let g = (rgba >> 8) & 0xFF;
            let b = rgba & 0xFF;
            let a = (rgba >> 24) & 0xFF;
            (r as u32)
                | ((g as u32) << 8)
                | ((b as u32) << 16)
                | ((a as u32) << 24)
            // ARGB layout: 0xAARRGGBB
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "glfw_backend")]
impl From<RawImage> for glfw::PixelImage {
    fn from(raw_image: RawImage) -> Self {
        raw_image_to_pixel_image(&raw_image)
    }
}

#[cfg(feature = "glfw_backend")]
#[cfg(not(target_arch = "wasm32"))]
impl From<glfw::PixelImage> for RawImage {
    fn from(pixel_image: PixelImage) -> Self {
        pixel_image_to_raw_image(&pixel_image)
    }
}

#[inline]
fn advance_color(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    if b == 255 {
        if g == 255 {
            if r == 255 {
                return (0, 0, 0);
            } else {
                return (r + 1, g, b);
            }
        } else {
            return (r, g + 1, b);
        }
    } else {
        return (r, g, b + 1);
    }
}

/// This is quite expensive
pub fn get_unused_color(
    buffer: &[u8],
    current_color: (u8, u8, u8),
) -> (u8, u8, u8) {
    let mut current_color = current_color;
    let mut unique_colors = HashSet::new();
    for i in buffer.chunks_exact(4) {
        if i[0] != 0 {
            unique_colors.insert((i[1], i[2], i[3]));
        }
    }
    while unique_colors.contains(&current_color) {
        current_color =
            advance_color(current_color.0, current_color.1, current_color.2)
    }
    current_color
}
/// A function specifically designed and optimized to work with the buffer of this lib
pub fn get_unused_color_of_buffer(
    buffer: &Buffer,
    current_color: (u8, u8, u8),
) -> (u8, u8, u8) {
    let mut current_color = current_color;
    let mut unique_colors = HashSet::new();
    for index in 0..buffer.total_size {
        unsafe {
            let i = u32_to_argb(*buffer.pointer.add(index));
            if i.0 != 0 {
                unique_colors.insert((i.1, i.2, i.3));
            }
        }
    }
    while unique_colors.contains(&current_color) {
        current_color =
            advance_color(current_color.0, current_color.1, current_color.2)
    }
    current_color
}
/// The interpolation mode for tge resizing of a buffer like object
pub enum InterpolationMode {
    /// Nearest neighbor - Best for pixel art
    Nearest,
    /// Linear interpolation - Idk this one always sucks, non pixel art ig
    Linear,
}
/// Resize a list of u32 to a list of u32s with a different visual size
pub fn resize_buffer(
    buffer: &[u32],
    src_width: usize,
    src_height: usize,
    dst_width: usize,
    dst_height: usize,
    resize_mode: InterpolationMode,
) -> Vec<u32> {
    match resize_mode {
        InterpolationMode::Nearest => resize_buffer_nearest(
            buffer, src_width, src_height, dst_width, dst_height,
        ),
        InterpolationMode::Linear => resize_buffer_linear(
            buffer, src_width, src_height, dst_width, dst_height,
        ),
    }
}
/// Resize u32 list using linear interpolation
pub fn resize_buffer_linear(
    buffer: &[u32],
    src_width: usize,
    src_height: usize,
    dst_width: usize,
    dst_height: usize,
) -> Vec<u32> {
    let mut result = Vec::with_capacity(dst_width * dst_height);

    let x_ratio = src_width as f32 / dst_width as f32;
    let y_ratio = src_height as f32 / dst_height as f32;

    for y in 0..dst_height {
        for x in 0..dst_width {
            let src_x = x as f32 * x_ratio;
            let src_y = y as f32 * y_ratio;

            let x1 = src_x.floor() as usize;
            let y1 = src_y.floor() as usize;
            let x2 = (x1 + 1).min(src_width - 1);
            let y2 = (y1 + 1).min(src_height - 1);

            let dx = src_x - x1 as f32;
            let dy = src_y - y1 as f32;

            let p1 = buffer[y1 * src_width + x1]; // top-left
            let p2 = buffer[y1 * src_width + x2]; // top-right
            let p3 = buffer[y2 * src_width + x1]; // bottom-left
            let p4 = buffer[y2 * src_width + x2]; // bottom-right

            let interpolated = bilinear_interpolate_u32(p1, p2, p3, p4, dx, dy);
            result.push(interpolated);
        }
    }

    result
}

/// Interpolate a u32 rgba based on 4 other u32 rgba
pub fn bilinear_interpolate_u32(
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    dx: f32,
    dy: f32,
) -> u32 {
    let (r1, g1, b1, a1) = u32_to_rgba(p1).tuple_4_into();
    let (r2, g2, b2, a2) = u32_to_rgba(p2).tuple_4_into();
    let (r3, g3, b3, a3) = u32_to_rgba(p3).tuple_4_into();
    let (r4, g4, b4, a4) = u32_to_rgba(p4).tuple_4_into();

    let interpolate_channel = |c1: f32, c2: f32, c3: f32, c4: f32| -> u8 {
        let top = c1 * (1.0 - dx) + c2 * dx;
        let bottom = c3 * (1.0 - dx) + c4 * dx;
        let result = top * (1.0 - dy) + bottom * dy;
        result.round().clamp(0.0, 255.0) as u8
    };

    let r = interpolate_channel(r1, r2, r3, r4);
    let g = interpolate_channel(g1, g2, g3, g4);
    let b = interpolate_channel(b1, b2, b3, b4);
    let a = interpolate_channel(a1, a2, a3, a4);

    ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
}

/// Resize u32 list using the nearest neighbor
pub fn resize_buffer_nearest(
    buffer: &[u32],
    src_width: usize,
    src_height: usize,
    dst_width: usize,
    dst_height: usize,
) -> Vec<u32> {
    let mut result = Vec::with_capacity(dst_width * dst_height);

    let x_ratio = src_width as f32 / dst_width as f32;
    let y_ratio = src_height as f32 / dst_height as f32;

    for y in 0..dst_height {
        for x in 0..dst_width {
            let src_x = (x as f32 * x_ratio + 0.5).floor() as usize;
            let src_y = (y as f32 * y_ratio + 0.5).floor() as usize;

            // Clamp to valid indices
            let src_x = src_x.min(src_width - 1);
            let src_y = src_y.min(src_height - 1);

            result.push(buffer[src_y * src_width + src_x]);
        }
    }

    result
}
/// Interpolate between 2 colors linearly based on a scale of 0 to 1
pub fn interpolate_color_rgb_u32(
    color1: u32,
    color2: u32,
    progress: f32,
) -> u32 {
    let (r1, g1, b1) = u32_to_rgb_u32(color1);
    let (r2, g2, b2) = u32_to_rgb_u32(color2);
    let r = interpolate(r1 as f32, r2 as f32, progress);
    let g = interpolate(g1 as f32, g2 as f32, progress);
    let b = interpolate(b1 as f32, b2 as f32, progress);
    return rgb_to_u32(r as u8, g as u8, b as u8);
}
/// Manage textures easily, has extra features when using the `imagery` as well as the `texture_manager_cleanup` flags
/// Enable the `imagery` feature for automatic texture lookup -> Define a filepath for a texture and lazy load it
/// 
/// Enable the `texture_manager_cleanup` feature to gain access to cleanup_unused
pub struct TextureManager {
    textures: Vec<Option<RawImage>>,
    #[cfg(not(target_arch = "wasm32"))]
    lookup: ahash::AHashMap<String, usize>,
    #[cfg(target_arch = "wasm32")]
    lookup: std::collections::HashMap<String, usize>,
    free_list: Vec<usize>,
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(feature = "imagery")]
    texture_lookup: ahash::AHashMap<String, String>,
    #[cfg(target_arch = "wasm32")]
    #[cfg(feature = "imagery")]
    texture_lookup: std::collections::HashMap<String, String>,
    #[cfg(feature = "texture_manager_cleanup")]
    last_used: Vec<u64>, // frame number when last accessed
    #[cfg(feature = "texture_manager_cleanup")]
    current_frame: u64,
}

impl TextureManager {
    /// Create a texture manager -> Request textures for visual applications
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            lookup: ahash::AHashMap::new(),
            #[cfg(target_arch = "wasm32")]
            lookup: std::collections::HashMap::new(),
            free_list: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            #[cfg(feature = "imagery")]
            texture_lookup: ahash::AHashMap::new(),
            #[cfg(target_arch = "wasm32")]
            #[cfg(feature = "imagery")]
            texture_lookup: std::collections::HashMap::new(),
            #[cfg(feature = "texture_manager_cleanup")]
            last_used: Vec::new(),
            #[cfg(feature = "texture_manager_cleanup")]
            current_frame: 0,
        }
    }
    /// Registering a texture means being able to lazy load it upon request
    #[cfg(feature = "imagery")]
    pub fn register_texture(&mut self, name: String, file_path: String) {
        self.texture_lookup.insert(name, file_path);
    }
    /// Get a texture -> Enable 'imagery' feature for lazy loading
    /// Returns None if the requested image cannot be found
    pub fn get(
        &mut self,
        name: &str,
        #[cfg(feature = "imagery")] file_system: &dyn FileSystem,
        #[cfg(feature = "imagery")] remove_margins: bool,
    ) -> Option<&RawImage> {
        #[cfg(feature = "texture_manager_cleanup")]
        if let Some(&index) = self.lookup.get(name) {
            if index < self.last_used.len() {
                self.last_used[index] = self.current_frame;
            }
            return self.textures[index].as_ref();
        }

        // First check if it's already loaded
        if let Some(&index) = self.lookup.get(name) {
            return self.textures[index].as_ref();
        }

        #[cfg(feature = "imagery")]
        // If not loaded, try to load from file
        if let Some(file_path) = self.texture_lookup.get(name) {
            match self.load_texture_from_file(file_path, file_system) {
                Ok(mut raw_image) => {
                    if remove_margins {
                        raw_image.remove_margins();
                    }
                    self.insert_texture(name.to_string(), raw_image);
                    if let Some(&index) = self.lookup.get(name) {
                        return self.textures[index].as_ref();
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Failed to load texture '{}' from '{}': {}",
                        name, file_path, e
                    );
                    return None;
                }
            }
        }

        None
    }
    /// Load texture from file to memory
    #[cfg(feature = "imagery")]
    pub fn load_texture_from_file(
        &self,
        file_path: &str,
        file_system: &dyn FileSystem,
    ) -> Result<RawImage, Box<dyn std::error::Error>> {
        let file = file_system.get_file_contents(file_path)?;
        let img = file.as_image()?;
        Ok(img.into())
    }
    /// Manually insert a texture with a corresponding name into cache
    pub fn insert_texture(&mut self, name: String, texture: RawImage) {
        let index = if let Some(free) = self.free_list.pop() {
            self.textures[free] = Some(texture);
            free
        } else {
            self.textures.push(Some(texture));
            self.textures.len() - 1
        };
        self.lookup.insert(name, index);
    }
    /// Unloads/Deletes the specified image from cache if found
    pub fn unload_texture(&mut self, name: &str) {
        if let Some(&index) = self.lookup.get(name) {
            self.textures[index] = None;
            self.free_list.push(index);
            self.lookup.remove(name);
        }
    }
    /// Checks if a an image is already registered for lazy loading
    #[cfg(feature = "imagery")]
    pub fn is_texture_registered(&self, name: &str) -> bool {
        self.texture_lookup.contains_key(name)
    }
    /// Preload registered image instead of letting it lazy load
    #[cfg(feature = "imagery")]
    pub fn preload_texture(
        &mut self,
        name: &str,
        file_system: &dyn FileSystem,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !self.lookup.contains_key(name) {
            if let Some(file_path) = self.texture_lookup.get(name) {
                let raw_image =
                    self.load_texture_from_file(file_path, file_system)?;
                self.insert_texture(name.to_string(), raw_image);
            }
        }
        Ok(())
    }
    /// Remove textures that haven't been used in X ticks -> Call .tick() every frame for this to work properly
    /// Set to 0 if you only ever want the images you need in memory
    /// Setting it to at least 1 however is recommended
    #[cfg(feature = "texture_manager_cleanup")]
    #[allow(arithmetic_overflow)]
    pub fn cleanup_unused(&mut self, frames_unused: u64) {
        // This calculation is 100% wrong, future me; Fix it.
        let cutoff = self.current_frame.saturating_sub(frames_unused);

        // Collect names to remove (avoid borrowing issues)
        let to_remove: Vec<String> = self
            .lookup
            .iter()
            .filter(|(_, &index)| {
                index < self.last_used.len() && self.last_used[index] < cutoff
            })
            .map(|(name, _)| name.clone())
            .collect();

        for name in to_remove {
            self.unload_texture(&name);
        }
    }
    #[allow(arithmetic_overflow)]
    /// Tick the texture manager -> Only thing it does is increment a single value, required for .cleanup_unused()
    pub fn tick(&mut self) {
        self.current_frame += 1;
    }
}
