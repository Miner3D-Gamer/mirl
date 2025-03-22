use crate::graphics::{rgb_to_u32, rgba_to_u32, u32_to_rgb};
use crate::math::{abs, radians, range, sign, top_clamp};
use image::{self, GenericImageView};
use std::mem;

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
pub fn draw_triangle(
    buffer: &mut Vec<u32>,
    width: usize,
    height: usize,
    point1: (u16, u16, f32, f32),
    point2: (u16, u16, f32, f32),
    point3: (u16, u16, f32, f32),
    texture: &image::DynamicImage,
) {
    // points: screen_x, screen_y, texture_x, texture_y
    let x1 = point1.0;
    let y1 = point1.1;
    let u1 = point1.2;
    let v1 = point1.3;

    let x2 = point2.0;
    let y2 = point2.1;
    let u2 = point2.2;
    let v2 = point2.3;

    let x3 = point3.0;
    let y3 = point3.1;
    let u3 = point3.2;
    let v3 = point3.3;
    let mut x_start;
    let mut x_end;
    let mut u_start;
    let mut u_end;
    let mut v_start;
    let mut v_end;

    let mut temp;
    let mut u;
    let mut v;

    let mut tex_x;
    let mut tex_y;
    let mut color;

    for y in range(y1, y3 + 1) {
        if y < y2 {
            x_start = uv_interpolate(
                y as f32, y1 as f32, x1 as f32, y2 as f32, x2 as f32,
            );
            x_end = uv_interpolate(
                y as f32 as f32,
                y1 as f32,
                x1 as f32,
                y3 as f32,
                x3 as f32,
            );

            u_start = uv_interpolate(
                y as f32, y1 as f32, u1 as f32, y2 as f32, u2 as f32,
            );
            u_end = uv_interpolate(
                y as f32, y1 as f32, u1 as f32, y3 as f32, u3 as f32,
            );

            v_start = uv_interpolate(
                y as f32, y1 as f32, v1 as f32, y2 as f32, v2 as f32,
            );
            v_end = uv_interpolate(
                y as f32, y1 as f32, v1 as f32, y3 as f32, v3 as f32,
            );
        } else {
            x_start = uv_interpolate(
                y as f32, y2 as f32, x2 as f32, y3 as f32, x3 as f32,
            );
            x_end = uv_interpolate(
                y as f32, y1 as f32, x1 as f32, y3 as f32, x3 as f32,
            );

            u_start = uv_interpolate(
                y as f32, y2 as f32, u2 as f32, y3 as f32, u3 as f32,
            );
            u_end = uv_interpolate(
                y as f32, y1 as f32, u1 as f32, y3 as f32, u3 as f32,
            );

            v_start = uv_interpolate(
                y as f32, y2 as f32, v2 as f32, y3 as f32, v3 as f32,
            );
            v_end = uv_interpolate(
                y as f32, y1 as f32, v1 as f32, y3 as f32, v3 as f32,
            );
        }
        if x_start > x_end {
            mem::swap(&mut x_start, &mut x_end);
            mem::swap(&mut u_start, &mut u_end);
            mem::swap(&mut v_start, &mut v_end);
        }
        for x in range(x_start.floor() as u16, (x_end.floor() + 1.0) as u16) {
            if x_start != x_end {
                temp = (x as f32 - x_start) / (x_end - x_start)
            } else {
                temp = 0.0
            }

            u = u_start + temp * (u_end - u_start);
            v = v_start + temp * (v_end - v_start);

            tex_x = top_clamp(
                (u * (texture.width() - 1) as f32) as u16,
                (texture.width() - 1) as u16,
            );
            tex_y = top_clamp(
                (v * (texture.height() - 1) as f32) as u16,
                (texture.width() - 1) as u16,
            );
            color = texture.get_pixel(tex_x as u32, tex_y as u32);
            set_pixel(
                buffer,
                width,
                height,
                x as usize,
                y as usize,
                rgba_to_u32(color),
            );
        }
    }
}

pub fn draw_polygon3d(
    buffer: &mut Vec<u32>,
    width: usize,
    height: usize,
    polygon: Polygon,
    texture: &image::DynamicImage,
) {
    let (point1_position_x, point1_position_y) =
        vertex_3d_to_2d(&polygon.point1, width, height);
    let (point2_position_x, point2_position_y) =
        vertex_3d_to_2d(&polygon.point2, width, height);
    let (point3_position_x, point3_position_y) =
        vertex_3d_to_2d(&polygon.point3, width, height);
    // print("Points1");
    // print(point1_position_x.to_string().as_str());
    // print(point1_position_y.to_string().as_str());
    // print("Points2");
    // print(point2_position_x.to_string().as_str());
    // print(point2_position_y.to_string().as_str());
    // print("Points3");
    // print(point3_position_x.to_string().as_str());
    // print(point3_position_y.to_string().as_str());

    draw_triangle(
        buffer,
        width,
        height,
        (
            point1_position_x,
            point1_position_y,
            polygon.point1.u,
            polygon.point1.v,
        ),
        (
            point2_position_x,
            point2_position_y,
            polygon.point2.u,
            polygon.point2.v,
        ),
        (
            point3_position_x,
            point3_position_y,
            polygon.point3.u,
            polygon.point3.v,
        ),
        texture,
    );
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

pub fn clear_screen(width: usize, height: usize) -> Vec<u32> {
    return vec![0; width * height];
}
pub fn draw_image(
    buffer: &mut Vec<u32>,
    width: usize,
    height: usize,
    texture_width: u16,
    texture_height: u16,
    texture_x: isize,
    texture_y: isize,
    texture: &image::DynamicImage,
) {
    let safe_width = texture.width() as f32 - 1.0;
    let safe_height = texture.height() as f32 - 1.0;

    // Clamping drawing bounds to the screen
    let start_x = texture_x.max(0) as u32;
    let start_y = texture_y.max(0) as u32;
    let end_x = (texture_x + texture_width as isize).min(width as isize) as u32;
    let end_y =
        (texture_y + texture_height as isize).min(height as isize) as u32;

    for x in start_x..end_x {
        let texture_uv_x = ((x as isize - texture_x) as f32
            / texture_width as f32)
            * safe_width;
        for y in start_y..end_y {
            let texture_uv_y = ((y as isize - texture_y) as f32
                / texture_height as f32)
                * safe_height;

            let clamped_uv_x = texture_uv_x.min(safe_width).max(0.0);
            let clamped_uv_y = texture_uv_y.min(safe_height).max(0.0);

            let pixel =
                texture.get_pixel(clamped_uv_x as u32, clamped_uv_y as u32);

            set_pixel(
                buffer,
                width,
                height,
                x as usize,
                y as usize,
                rgba_to_u32(pixel),
            );
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
