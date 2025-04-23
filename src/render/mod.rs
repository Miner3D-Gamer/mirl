use crate::extensions::*;
use crate::graphics::{rgb_to_u32, u32_to_rgb};
use crate::math::radians;

pub fn rotate_x_vertex_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    vertex: &mut Vertex3D,
) {
    let angle_radians = radians(angle_degrees);
    let cos_a = angle_radians.cos();
    let sin_a = angle_radians.sin();
    let (cx, cy, cz) =
        (rotation_center.x, rotation_center.y, rotation_center.z);

    // Adjust the vertex relative to the rotation center
    let x = vertex.x - cx;
    let y = vertex.y - cy;
    let z = vertex.z - cz;

    // Apply the rotation
    let new_y = cos_a * (y as f64) - sin_a * (z as f64);
    let new_z = sin_a * (y as f64) + cos_a * (z as f64);

    // Adjust the vertex back
    vertex.x = x + cx;
    vertex.y = new_y + cy;
    vertex.z = new_z + cz;
}
pub fn rotate_x_polygon_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    polygon: &mut Polygon,
) {
    rotate_x_vertex_3d(
        angle_degrees,
        rotation_center.clone(),
        &mut polygon.point1,
    );
    rotate_x_vertex_3d(
        angle_degrees,
        rotation_center.clone(),
        &mut polygon.point2,
    );
    rotate_x_vertex_3d(
        angle_degrees,
        rotation_center.clone(),
        &mut polygon.point3,
    );
}
pub fn rotate_y_polygon_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    polygon: &mut Polygon,
) {
    rotate_y_vertex_3d(
        angle_degrees,
        rotation_center.clone(),
        &mut polygon.point1,
    );
    rotate_y_vertex_3d(
        angle_degrees,
        rotation_center.clone(),
        &mut polygon.point2,
    );
    rotate_y_vertex_3d(
        angle_degrees,
        rotation_center.clone(),
        &mut polygon.point3,
    );
}
pub fn rotate_z_polygon_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    polygon: &mut Polygon,
) {
    rotate_z_vertex_3d(
        angle_degrees,
        rotation_center.clone(),
        &mut polygon.point1,
    );
    rotate_z_vertex_3d(
        angle_degrees,
        rotation_center.clone(),
        &mut polygon.point2,
    );
    rotate_z_vertex_3d(
        angle_degrees,
        rotation_center.clone(),
        &mut polygon.point3,
    );
}

pub fn rotate_y_vertex_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    vertex: &mut Vertex3D,
) {
    let angle_radians = radians(angle_degrees);
    let cos_a = angle_radians.cos();
    let sin_a = angle_radians.sin();
    let (cx, cy, cz) =
        (rotation_center.x, rotation_center.y, rotation_center.z);

    // Adjust the vertex relative to the rotation center
    let x = vertex.x - cx;
    let y = vertex.y - cy;
    let z = vertex.z - cz;

    // Apply the rotation
    let new_x = cos_a * (x as f64) + sin_a * (z as f64);
    let new_z = -sin_a * (x as f64) + cos_a * (z as f64);

    // Adjust the vertex back
    vertex.x = new_x + cx;
    vertex.y = y + cy;
    vertex.z = new_z + cz;
}

pub fn rotate_z_vertex_3d(
    angle_degrees: f64,
    rotation_center: Point3D,
    vertex: &mut Vertex3D,
) {
    let angle_radians = radians(angle_degrees);
    let cos_a = angle_radians.cos();
    let sin_a = angle_radians.sin();
    let (cx, cy, cz) =
        (rotation_center.x, rotation_center.y, rotation_center.z);

    // Adjust the vertex relative to the rotation center
    let x = vertex.x - cx;
    let y = vertex.y - cy;
    let z = vertex.z - cz;

    // Apply the rotation
    let new_x = cos_a * (x as f64) - sin_a * (y as f64);
    let new_y = sin_a * (x as f64) + cos_a * (y as f64);

    // Adjust the vertex back
    vertex.x = new_x + cx;
    vertex.y = new_y + cy;
    vertex.z = z + cz;
}

pub fn set_pixel_safe(
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
pub fn set_pixel_unsafe(
    buffer: *mut u32,
    width: usize,
    x: usize,
    y: usize,
    color: u32,
) {
    unsafe {
        *buffer.add(y * width + x) = color;
    }
}

pub fn uv_interpolate(
    target_y: f32,
    start_y: f32,
    start_val: f32,
    end_y: f32,
    end_val: f32,
) -> f32 {
    if start_y == end_y {
        return start_val;
    }
    return start_val
        + (end_val - start_val) * (target_y - start_y) / (end_y - start_y);
}

#[derive(Copy, Clone)]
pub struct Polygon {
    pub point1: Vertex3D,
    pub point2: Vertex3D,
    pub point3: Vertex3D,
}
#[derive(Copy, Clone)]
pub struct Vertex3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u: f32,
    pub v: f32,
}
#[derive(Copy, Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Point2D {
    pub x: f64,
    pub y: f64,
}
pub fn vertex_3d_to_2d(
    vertex: &Vertex3D,
    width: usize,
    height: usize,
) -> (u16, u16) {
    //return (vertex.x as u16, vertex.y as u16);
    let half_width = (width / 2) as f64;
    let half_height = (height / 2) as f64;
    let x = vertex.x;
    let y = vertex.y;
    let z = vertex.z;

    let screen_x = (x - half_width) / z + half_width;
    let screen_y = (y - half_height) / z + half_height;
    return (screen_x as u16, screen_y as u16);
}

#[inline(always)]
pub fn get_empty_buffer(width: usize, height: usize) -> Vec<u32> {
    return vec![0; width * height];
}
#[inline(always)]
pub fn clear_screen(pointer: *mut u32, total_size: usize) {
    unsafe {
        std::ptr::write_bytes(pointer, 0, total_size);
    }
}
pub fn color_screen(
    pointer: *mut u32,
    width: usize,
    height: usize,
    color: u32,
) {
    for y in 0..height {
        for x in 0..width {
            unsafe {
                *pointer.offset((y * width + x) as isize) = color;
            }
        }
    }
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

#[cfg(feature = "imagery")]
pub mod image_support;
#[cfg(feature = "imagery")]
pub use image_support::*;

#[inline]
fn round_float_key(value: f32) -> (i32, i32) {
    let multiplier = 10.0_f32.powi(4);
    let rounded_int_x = (value * multiplier).round() as i32;
    let rounded_int_y = (value * multiplier).fract() as i32;
    (rounded_int_x, rounded_int_y)
}

use std::collections::HashMap;
use std::sync::RwLock;

static GLYPH_CACHE: once_cell::sync::Lazy<
    RwLock<HashMap<(char, (i32, i32)), (fontdue::Metrics, Vec<u8>)>>,
> = once_cell::sync::Lazy::new(|| RwLock::new(HashMap::new()));

#[inline]
fn get_glyph_cache(
) -> &'static RwLock<HashMap<(char, (i32, i32)), (fontdue::Metrics, Vec<u8>)>> {
    &GLYPH_CACHE
}

use fontdue::Font;
pub trait RenderSettings {
    #[inline]
    fn draw_pixel(
        &self,
        buffer: *mut u32,
        width: usize,
        _height: usize,
        x: usize,
        y: usize,
        color: u32,
    ) {
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
        font: &Font,
    ) {
        let mut pen_x = x;
        let pen_y = y;
        let font_metrics = font.horizontal_line_metrics(size).unwrap();
        let ascent = font_metrics.ascent as usize;

        let rounded_size_key = round_float_key(size);

        for ch in text.chars() {
            // Try to get the glyph from cache first
            let cached_glyph = {
                let cache = get_glyph_cache().read().unwrap();
                cache.get(&(ch, rounded_size_key)).cloned()
            };

            // If not in cache, rasterize and insert
            let (metrics, bitmap) = cached_glyph.unwrap_or_else(|| {
                let rasterized = font.rasterize(ch, size);

                // Insert into cache
                let mut cache_mut = get_glyph_cache().write().unwrap();
                cache_mut.insert((ch, rounded_size_key), rasterized.clone());

                rasterized
            });

            let offset_y = ascent.saturating_sub(metrics.height);
            let w = metrics.width;
            let h = metrics.height;
            let advance_x = metrics.advance_width as usize;

            for gy in 0..h {
                let py = pen_y + gy + offset_y;
                if py >= height {
                    continue;
                }

                let row_start = gy * w;
                for gx in 0..w {
                    let px = pen_x + gx;
                    if px >= width {
                        continue;
                    }

                    if bitmap[row_start + gx] > 0 {
                        self.draw_pixel(buffer, width, height, px, py, color);
                    }
                }
            }
            pen_x += advance_x;
        }
    }
    #[inline]
    fn draw_circle(
        &self,
        buffer: *mut u32,
        width: usize,
        height: usize,
        pos_x: usize,
        pos_y: usize,
        radius: isize,
        color: u32,
    ) {
        let mut x = 0;
        let mut y = 0 - radius;
        let mut p = -radius;

        while (x) < (-y) {
            if p > 0 {
                y += 1;
                p += 2 * (x + y) + 1
            } else {
                p += 2 * x + 1
            }
            let temp_x = x as usize;
            let temp_y = y as usize;
            self.draw_pixel(
                buffer,
                width,
                height,
                pos_x + temp_x,
                pos_y + temp_y,
                color,
            );
            self.draw_pixel(
                buffer,
                width,
                height,
                pos_x - temp_x,
                pos_y + temp_y,
                color,
            );
            self.draw_pixel(
                buffer,
                width,
                height,
                pos_x + temp_x,
                pos_y - temp_y,
                color,
            );
            self.draw_pixel(
                buffer,
                width,
                height,
                pos_x - temp_x,
                pos_y - temp_y,
                color,
            );
            self.draw_pixel(
                buffer,
                width,
                height,
                pos_x + temp_y,
                pos_y + temp_x,
                color,
            );
            self.draw_pixel(
                buffer,
                width,
                height,
                pos_x + temp_y,
                pos_y - temp_x,
                color,
            );
            self.draw_pixel(
                buffer,
                width,
                height,
                pos_x - temp_y,
                pos_y + temp_x,
                color,
            );
            self.draw_pixel(
                buffer,
                width,
                height,
                pos_x - temp_y,
                pos_y - temp_x,
                color,
            );

            x += 1
        }
    }
    #[inline]
    fn adjust_brightness(&self, color: u32, x: i32) -> u32 {
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
    fn desaturate(&self, color: u32, amount: f32) -> u32 {
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
    #[inline]
    fn get_pixel(
        buffer: &Vec<u32>,
        width: &isize,
        height: &isize,
        x: isize,
        y: isize,
    ) -> u32 {
        if x < 0 || y < 0 {
            return 0;
        }
        if x >= *width || y >= *height {
            return 0;
        }
        let index = y * width + x;
        return buffer[index as usize];
    }

    fn draw_line(
        &self,
        buffer: *mut u32,
        width: usize,
        height: usize,
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        color: u32,
    ) {
        let mut start_x = x1 as i16;
        let mut start_y = y1 as i16;
        let end_x = x2 as i16;
        let end_y = y2 as i16;

        let difference_x: i16 = end_x - start_x;
        let difference_y: i16 = end_y - start_y;
        let sign_x = difference_x.sign();
        let sign_y = difference_y.sign();
        let abs_difference_x: i16 = difference_x.abs();
        let abs_difference_y: i16 = difference_y.abs();
        if abs_difference_x > abs_difference_y {
            let mut error: i16 = abs_difference_x / 2;
            while start_x != end_x {
                start_x += sign_x;
                error -= abs_difference_y;
                if error < 0 {
                    start_y += sign_y;
                    error += abs_difference_x;
                }
                self.draw_pixel(
                    buffer,
                    width,
                    height,
                    start_x as usize,
                    start_y as usize,
                    color,
                );
            }
        } else {
            let mut error: i16 = abs_difference_y / 2;
            while start_y != end_y {
                start_y += sign_y;
                error -= abs_difference_x;
                if error < 0 {
                    start_x += sign_x;
                    error += abs_difference_y;
                }
                self.draw_pixel(
                    buffer,
                    width,
                    height,
                    start_x as usize,
                    start_y as usize,
                    color,
                );
            }
        }
    }
}

pub mod fancy;
pub use fancy::RenderSettingsPretty;

pub mod fast;

pub use fast::RenderSettingsFast;
