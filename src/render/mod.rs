use crate::graphics::{rgb_to_u32, u32_to_rgb};
use crate::math::{abs, radians, sign};

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

pub fn draw_line(
    buffer: &mut Vec<u32>,
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
    let sign_x = sign(difference_x as i64) as i16;
    let sign_y = sign(difference_y as i64) as i16;
    let abs_difference_x: i16 = abs(difference_x as i64) as i16;
    let abs_difference_y: i16 = abs(difference_y as i64) as i16;
    if abs_difference_x > abs_difference_y {
        let mut error: i16 = abs_difference_x / 2;
        while start_x != end_x {
            start_x += sign_x;
            error -= abs_difference_y;
            if error < 0 {
                start_y += sign_y;
                error += abs_difference_x;
            }
            set_pixel(
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
            set_pixel(
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

pub fn set_pixel(
    buffer: &mut Vec<u32>,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    color: u32,
) {
    if x >= width || y >= height {
        return;
    }
    let position = y * width + x;
    buffer[position] = color;
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
    point1: Vertex3D,
    point2: Vertex3D,
    point3: Vertex3D,
}
#[derive(Copy, Clone)]
pub struct Vertex3D {
    x: f64,
    y: f64,
    z: f64,
    u: f32,
    v: f32,
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
pub fn clear_screen(width: usize, height: usize) -> Vec<u32> {
    return vec![0; width * height];
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
