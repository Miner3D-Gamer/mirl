#![allow(clippy::similar_names)]
#![allow(clippy::many_single_char_names)]

use crate::{
    graphics::{rgba_to_u32, u32_to_rgba_u8, u32_to_rgba},
    math::interpolate,
};

/// The interpolation mode for resizing a buffer-like object
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InterpolationMode {
    #[default]
    /// Nearest neighbor - Best for pixel art and sharp edges
    Nearest,
    /// Linear (bilinear) - Simple smoothing, fast but can blur
    Linear,
    /// Cubic (bicubic) - Smoother than linear, preserves detail better
    Cubic,
    /// Lanczos - High quality, sharp, good for downscaling, more expensive
    Lanczos,
    /// Area (box/average) - Good for reducing aliasing when downscaling
    Area,
    /// Catmull–Rom spline - Cubic variant, sharper than bicubic
    CatmullRom,
    /// Gaussian - Blurry but effective at noise reduction
    Gaussian,
    /// Mitchell–Netravali - Balanced sharpness and smoothness
    MitchellNetravali,
}


#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
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
#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
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
            let src_x = (x as f32).mul_add(x_ratio, 0.5).floor() as usize;
            let src_y = (y as f32).mul_add(y_ratio, 0.5).floor() as usize;

            // Clamp to valid indices
            let src_x = src_x.min(src_width - 1);
            let src_y = src_y.min(src_height - 1);

            result.push(buffer[src_y * src_width + src_x]);
        }
    }

    result
}

#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
/// Resize u32 list using bicubic interpolation
pub fn resize_buffer_cubic(
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

            let interpolated =
                bicubic_sample(buffer, src_width, src_height, src_x, src_y);
            result.push(interpolated);
        }
    }

    result
}

#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
/// Resize u32 list using Lanczos interpolation
pub fn resize_buffer_lanczos(
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

            let interpolated =
                lanczos_sample(buffer, src_width, src_height, src_x, src_y);
            result.push(interpolated);
        }
    }

    result
}

#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
/// Resize u32 list using area averaging (good for downscaling)
pub fn resize_buffer_area(
    buffer: &[u32],
    src_width: usize,
    src_height: usize,
    dst_width: usize,
    dst_height: usize,
) -> Vec<u32> {
    let mut result = Vec::with_capacity(dst_width * dst_height);

    let x_scale = src_width as f32 / dst_width as f32;
    let y_scale = src_height as f32 / dst_height as f32;

    for y in 0..dst_height {
        for x in 0..dst_width {
            let src_x_start = x as f32 * x_scale;
            let src_y_start = y as f32 * y_scale;
            let src_x_end = ((x + 1) as f32 * x_scale).min(src_width as f32);
            let src_y_end = ((y + 1) as f32 * y_scale).min(src_height as f32);

            let interpolated = area_average_sample(
                buffer,
                src_width,
                src_height,
                src_x_start,
                src_y_start,
                src_x_end,
                src_y_end,
            );
            result.push(interpolated);
        }
    }

    result
}

#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
/// Resize u32 list using Catmull-Rom spline interpolation
pub fn resize_buffer_catmull_rom(
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

            let interpolated =
                catmull_rom_sample(buffer, src_width, src_height, src_x, src_y);
            result.push(interpolated);
        }
    }

    result
}

#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
/// Resize u32 list using Gaussian interpolation
pub fn resize_buffer_gaussian(
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

            let interpolated =
                gaussian_sample(buffer, src_width, src_height, src_x, src_y);
            result.push(interpolated);
        }
    }

    result
}

#[must_use]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
/// Resize u32 list using Mitchell-Netravali interpolation
pub fn resize_buffer_mitchell_netravali(
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

            let interpolated = mitchell_netravali_sample(
                buffer, src_width, src_height, src_x, src_y,
            );
            result.push(interpolated);
        }
    }

    result
}

#[must_use]
/// Resize a list of u32 to a list of u32s with a different visual size
pub fn resize_buffer(
    buffer: &[u32],
    src_width: usize,
    src_height: usize,
    dst_width: usize,
    dst_height: usize,
    resizing_method: InterpolationMode,
) -> Vec<u32> {
    match resizing_method {
        InterpolationMode::Nearest => resize_buffer_nearest(
            buffer, src_width, src_height, dst_width, dst_height,
        ),
        InterpolationMode::Linear => resize_buffer_linear(
            buffer, src_width, src_height, dst_width, dst_height,
        ),
        InterpolationMode::Cubic => resize_buffer_cubic(
            buffer, src_width, src_height, dst_width, dst_height,
        ),
        InterpolationMode::Lanczos => resize_buffer_lanczos(
            buffer, src_width, src_height, dst_width, dst_height,
        ),
        InterpolationMode::Area => resize_buffer_area(
            buffer, src_width, src_height, dst_width, dst_height,
        ),
        InterpolationMode::CatmullRom => resize_buffer_catmull_rom(
            buffer, src_width, src_height, dst_width, dst_height,
        ),
        InterpolationMode::Gaussian => resize_buffer_gaussian(
            buffer, src_width, src_height, dst_width, dst_height,
        ),
        InterpolationMode::MitchellNetravali => {
            resize_buffer_mitchell_netravali(
                buffer, src_width, src_height, dst_width, dst_height,
            )
        }
    }
}

// Helper functions for interpolation

/// Bilinear interpolation for u32 pixels
fn bilinear_interpolate_u32(
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    dx: f32,
    dy: f32,
) -> u32 {
    let (r1, g1, b1, a1) = u32_to_rgba(p1);
    let (r2, g2, b2, a2) = u32_to_rgba(p2);
    let (r3, g3, b3, a3) = u32_to_rgba(p3);
    let (r4, g4, b4, a4) = u32_to_rgba(p4);

    let r = interpolate(
        interpolate(r1 as f32, r2 as f32, dx),
        interpolate(r3 as f32, r4 as f32, dx),
        dy,
    );
    let g = interpolate(
        interpolate(g1 as f32, g2 as f32, dx),
        interpolate(g3 as f32, g4 as f32, dx),
        dy,
    );
    let b = interpolate(
        interpolate(b1 as f32, b2 as f32, dx),
        interpolate(b3 as f32, b4 as f32, dx),
        dy,
    );
    let a = interpolate(
        interpolate(a1 as f32, a2 as f32, dx),
        interpolate(a3 as f32, a4 as f32, dx),
        dy,
    );

    rgba_to_u32(r as u32, g as u32, b as u32, a as u32)
}

fn get_pixel_safe(
    buffer: &[u32],
    width: usize,
    height: usize,
    x: i32,
    y: i32,
) -> u32 {
    let x = x.max(0).min(width as i32 - 1) as usize;
    let y = y.max(0).min(height as i32 - 1) as usize;
    buffer[y * width + x]
}

/// Bicubic interpolation sampling
fn bicubic_sample(
    buffer: &[u32],
    width: usize,
    height: usize,
    x: f32,
    y: f32,
) -> u32 {
    let x_int = x.floor() as i32;
    let y_int = y.floor() as i32;
    let dx = x - x_int as f32;
    let dy = y - y_int as f32;

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 0.0;

    for j in -1..3 {
        for i in -1..3 {
            let pixel =
                get_pixel_safe(buffer, width, height, x_int + i, y_int + j);
            let (pr, pg, pb, pa) = u32_to_rgba(pixel);

            let weight_x = cubic_hermite(dx, i as f32);
            let weight_y = cubic_hermite(dy, j as f32);
            let weight = weight_x * weight_y;

            r += pr as f32 * weight;
            g += pg as f32 * weight;
            b += pb as f32 * weight;
            a += pa as f32 * weight;
        }
    }

    rgba_to_u32(
        r.clamp(0.0, 255.0) as u32,
        g.clamp(0.0, 255.0) as u32,
        b.clamp(0.0, 255.0) as u32,
        a.clamp(0.0, 255.0) as u32,
    )
}
#[must_use]
/// Lanczos interpolation sampling
pub fn lanczos_sample(
    buffer: &[u32],
    width: usize,
    height: usize,
    x: f32,
    y: f32,
) -> u32 {
    const RADIUS: i32 = 3;
    let x_int = x.floor() as i32;
    let y_int = y.floor() as i32;

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 0.0;
    let mut weight_sum = 0.0;

    for j in -RADIUS..=RADIUS {
        for i in -RADIUS..=RADIUS {
            let pixel =
                get_pixel_safe(buffer, width, height, x_int + i, y_int + j);
            let (pr, pg, pb, pa) = u32_to_rgba(pixel);

            let weight_x = lanczos_kernel(x - (x_int + i) as f32, RADIUS);
            let weight_y = lanczos_kernel(y - (y_int + j) as f32, RADIUS);
            let weight = weight_x * weight_y;

            r += pr as f32 * weight;
            g += pg as f32 * weight;
            b += pb as f32 * weight;
            a += pa as f32 * weight;
            weight_sum += weight;
        }
    }

    if weight_sum > 0.0 {
        r /= weight_sum;
        g /= weight_sum;
        b /= weight_sum;
        a /= weight_sum;
    }

    rgba_to_u32(
        r.clamp(0.0, 255.0) as u32,
        g.clamp(0.0, 255.0) as u32,
        b.clamp(0.0, 255.0) as u32,
        a.clamp(0.0, 255.0) as u32,
    )
}
#[must_use]
/// Area averaging sample
pub fn area_average_sample(
    buffer: &[u32],
    width: usize,
    height: usize,
    x_start: f32,
    y_start: f32,
    x_end: f32,
    y_end: f32,
) -> u32 {
    let x1 = x_start.floor() as usize;
    let y1 = y_start.floor() as usize;
    let x2 = (x_end.ceil() as usize).min(width);
    let y2 = (y_end.ceil() as usize).min(height);

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 0.0;
    let mut total_weight = 0.0;

    for y in y1..y2 {
        for x in x1..x2 {
            let pixel = buffer[y * width + x];
            let (pr, pg, pb, pa) = u32_to_rgba_u8(pixel);

            let weight = calculate_overlap(
                x as f32,
                (x + 1) as f32,
                y as f32,
                (y + 1) as f32,
                x_start,
                x_end,
                y_start,
                y_end,
            );

            r += pr as f32 * weight;
            g += pg as f32 * weight;
            b += pb as f32 * weight;
            a += pa as f32 * weight;
            total_weight += weight;
        }
    }

    if total_weight > 0.0 {
        r /= total_weight;
        g /= total_weight;
        b /= total_weight;
        a /= total_weight;
    }

    rgba_to_u32(
        r.clamp(0.0, 255.0) as u32,
        g.clamp(0.0, 255.0) as u32,
        b.clamp(0.0, 255.0) as u32,
        a.clamp(0.0, 255.0) as u32,
    )
}

/// Catmull-Rom spline interpolation
#[must_use]
pub fn catmull_rom_sample(
    buffer: &[u32],
    width: usize,
    height: usize,
    x: f32,
    y: f32,
) -> u32 {
    let x_int = x.floor() as i32;
    let y_int = y.floor() as i32;
    let dx = x - x_int as f32;
    let dy = y - y_int as f32;

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 0.0;

    for j in -1..3 {
        for i in -1..3 {
            let pixel =
                get_pixel_safe(buffer, width, height, x_int + i, y_int + j);
            let (pr, pg, pb, pa) = u32_to_rgba(pixel);

            let weight_x = catmull_rom_kernel(dx - i as f32);
            let weight_y = catmull_rom_kernel(dy - j as f32);
            let weight = weight_x * weight_y;

            r += pr as f32 * weight;
            g += pg as f32 * weight;
            b += pb as f32 * weight;
            a += pa as f32 * weight;
        }
    }

    rgba_to_u32(
        r.clamp(0.0, 255.0) as u32,
        g.clamp(0.0, 255.0) as u32,
        b.clamp(0.0, 255.0) as u32,
        a.clamp(0.0, 255.0) as u32,
    )
}

/// Gaussian interpolation sampling
#[must_use]
pub fn gaussian_sample(
    buffer: &[u32],
    width: usize,
    height: usize,
    x: f32,
    y: f32,
) -> u32 {
    const RADIUS: i32 = 2;
    const SIGMA: f32 = 1.0;
    let x_int = x.round() as i32;
    let y_int = y.round() as i32;

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 0.0;
    let mut weight_sum = 0.0;

    for j in -RADIUS..=RADIUS {
        for i in -RADIUS..=RADIUS {
            let pixel =
                get_pixel_safe(buffer, width, height, x_int + i, y_int + j);
            let (pr, pg, pb, pa) = u32_to_rgba(pixel);

            let dx = x - (x_int + i) as f32;
            let dy = y - (y_int + j) as f32;
            let distance_sq = dx.mul_add(dx, dy * dy);
            let weight = (-distance_sq / (2.0 * SIGMA * SIGMA)).exp();

            r += pr as f32 * weight;
            g += pg as f32 * weight;
            b += pb as f32 * weight;
            a += pa as f32 * weight;
            weight_sum += weight;
        }
    }

    if weight_sum > 0.0 {
        r /= weight_sum;
        g /= weight_sum;
        b /= weight_sum;
        a /= weight_sum;
    }

    rgba_to_u32(
        r.clamp(0.0, 255.0) as u32,
        g.clamp(0.0, 255.0) as u32,
        b.clamp(0.0, 255.0) as u32,
        a.clamp(0.0, 255.0) as u32,
    )
}

#[must_use]
/// Mitchell-Netravali interpolation sampling
pub fn mitchell_netravali_sample(
    buffer: &[u32],
    width: usize,
    height: usize,
    x: f32,
    y: f32,
) -> u32 {
    let x_int = x.floor() as i32;
    let y_int = y.floor() as i32;
    let dx = x - x_int as f32;
    let dy = y - y_int as f32;

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 0.0;

    for j in -1..3 {
        for i in -1..3 {
            let pixel =
                get_pixel_safe(buffer, width, height, x_int + i, y_int + j);
            let (pr, pg, pb, pa) = u32_to_rgba_u8(pixel);

            let weight_x = mitchell_netravali_kernel(dx - i as f32);
            let weight_y = mitchell_netravali_kernel(dy - j as f32);
            let weight = weight_x * weight_y;

            r += pr as f32 * weight;
            g += pg as f32 * weight;
            b += pb as f32 * weight;
            a += pa as f32 * weight;
        }
    }

    rgba_to_u32(
        r.clamp(0.0, 255.0) as u32,
        g.clamp(0.0, 255.0) as u32,
        b.clamp(0.0, 255.0) as u32,
        a.clamp(0.0, 255.0) as u32,
    )
}
/// Cubic Hermite spline kernel
#[must_use]
pub fn cubic_hermite(t: f32, p: f32) -> f32 {
    let t = (t - p).abs();
    if t < 1.0 {
        (2.0 * t * t).mul_add(t, -(3.0 * t * t)) + 1.0
    } else if t < 2.0 {
        8.0f32.mul_add(-t, (-t * t).mul_add(t, 5.0 * t * t)) + 4.0
    } else {
        0.0
    }
}

/// Lanczos resampling kernel
#[must_use]
pub fn lanczos_kernel(x: f32, a: i32) -> f32 {
    let x_abs = x.abs();
    if x_abs < a as f32 {
        if x_abs == 0.0 {
            1.0
        } else {
            let pi_x = std::f32::consts::PI * x_abs;
            let pi_x_a = pi_x / a as f32;
            (pi_x.sin() / pi_x) * (pi_x_a.sin() / pi_x_a)
        }
    } else {
        0.0
    }
}

/// Catmull-Rom spline kernel
#[must_use]
pub fn catmull_rom_kernel(t: f32) -> f32 {
    let t_abs = t.abs();
    if t_abs < 1.0 {
        (1.5 * t_abs * t_abs).mul_add(t_abs, -(2.5 * t_abs * t_abs)) + 1.0
    } else if t_abs < 2.0 {
        4.0f32.mul_add(
            -t_abs,
            (-0.5 * t_abs * t_abs).mul_add(t_abs, 2.5 * t_abs * t_abs),
        ) + 2.0
    } else {
        0.0
    }
}

/// Mitchell–Netravali resampling kernel
#[must_use]
pub fn mitchell_netravali_kernel(t: f32) -> f32 {
    const B: f32 = 1.0 / 3.0;
    const C: f32 = 1.0 / 3.0;
    let t_abs = t.abs();

    if t_abs < 1.0 {
        ((6.0f32.mul_add(-C, 9.0f32.mul_add(-B, 12.0)) * t_abs * t_abs)
            .mul_add(
                t_abs,
                6.0f32.mul_add(C, 12.0f32.mul_add(B, -18.0)) * t_abs * t_abs,
            )
            + 2.0f32.mul_add(-B, 6.0))
            / 6.0
    } else if t_abs < 2.0 {
        ((-12.0f32).mul_add(B, -(48.0 * C)).mul_add(
            t_abs,
            (6.0f32.mul_add(-C, -B) * t_abs * t_abs)
                .mul_add(t_abs, 6.0f32.mul_add(B, 30.0 * C) * t_abs * t_abs),
        ) + 8.0f32.mul_add(B, 24.0 * C))
            / 6.0
    } else {
        0.0
    }
}

/// Calculates overlap area between two rectangles
#[must_use]
#[allow(clippy::too_many_arguments)]
pub fn calculate_overlap(
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    rx1: f32,
    rx2: f32,
    ry1: f32,
    ry2: f32,
) -> f32 {
    let overlap_x = (x2.min(rx2) - x1.max(rx1)).max(0.0);
    let overlap_y = (y2.min(ry2) - y1.max(ry1)).max(0.0);
    overlap_x * overlap_y
}
