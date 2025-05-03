#[inline]
pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

#[inline]
pub fn rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
    (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32
}
#[inline]
pub fn u32_to_rgb(color: u32) -> (u8, u8, u8) {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    (r, g, b)
}

#[cfg(feature = "imagery")]
pub mod imagery;
#[cfg(feature = "imagery")]
pub use imagery::*;

#[inline]
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
pub fn adjust_brightness_hsl(color: u32, x: i32) -> u32 {
    let a = (color >> 24) & 0xFF;
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;

    let (h, s, l) = rgb_to_hsl(r, g, b);

    // Adjust lightness in HSL space (most perceptually accurate)
    let l_new = (l + x as f32).clamp(0.0, 100.0);

    let (r_new, g_new, b_new) = hsl_to_rgb_u32(h, s, l_new);

    (a << 24) | (r_new << 16) | (g_new << 8) | b_new
}
#[inline]
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
pub fn rgb_to_hsl(r: u32, g: u32, b: u32) -> (f32, f32, f32) {
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

pub fn hsl_to_rgb_u32(h: f32, s: f32, l: f32) -> (u32, u32, u32) {
    let h_norm = h / 360.0;
    let s_norm = s / 100.0;
    let l_norm = l / 100.0;

    if s_norm < 0.0001 {
        let gray = (l_norm * 255.0).round() as u32;
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

    let r_8bit = (r * 255.0).round() as u32;
    let g_8bit = (g * 255.0).round() as u32;
    let b_8bit = (b * 255.0).round() as u32;

    (r_8bit, g_8bit, b_8bit)
}

/// Higher-level function that provides both perceptual models
pub enum BrightnessModel {
    LinearWeighted, // Uses RGB with perceptual weights
    HSL,            // Uses HSL color space
}
#[inline]

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
        BrightnessModel::HSL => adjust_brightness_hsl(color, x),
    }
}

#[inline]
pub fn shift_color_rgb(
    r: u32,
    g: u32,
    b: u32,
    hue_shift: f32,
) -> (u32, u32, u32) {
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

#[inline]
pub fn shift_color_u32(color: u32, hue_shift: f32) -> u32 {
    let a = (color >> 24) & 0xFF;
    let r = (color >> 16) & 0xFF;
    let g = (color >> 8) & 0xFF;
    let b = color & 0xFF;

    let (r_new, g_new, b_new) = shift_color_rgb(r, g, b, hue_shift);

    (a << 24) | (r_new << 16) | (g_new << 8) | b_new
}

#[inline]
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

pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn pixel_to_u32(pixel: Pixel) -> u32 {
    rgb_to_u32(pixel.r, pixel.g, pixel.b)
}

pub fn u32_to_pixel(color: u32) -> Pixel {
    let (r, g, b) = u32_to_rgb(color);
    Pixel {
        r,
        g,
        b,
    }
}
