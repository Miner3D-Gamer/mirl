pub use crate::extensions::*;
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

#[inline(always)]
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

#[inline]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Polygon {
    pub point1: Vertex3D,
    pub point2: Vertex3D,
    pub point3: Vertex3D,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub u: f32,
    pub v: f32,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}
pub fn vertex_3d_to_2d(
    vertex: &Vertex3D,
    width: usize,
    height: usize,
) -> (isize, isize) {
    //return (vertex.x as u16, vertex.y as u16);
    let half_width = (width / 2) as f64;
    let half_height = (height / 2) as f64;
    let x = vertex.x;
    let y = vertex.y;
    let z = vertex.z;

    let screen_x = (x - half_width) / z + half_width;
    let screen_y = (y - half_height) / z + half_height;
    return (screen_x as isize, screen_y as isize);
}


// fn useable(
//     p: &Vertex3D,
//     width: usize,
//     height: usize,
// ) -> (isize, isize, f32, f32) {
//     let position = vertex_3d_to_2d(p, width, height);

//     return (position.0, position.1, p.u, p.v);
// }

// #[cfg(feature = "imagery")]
// mod image_support;
// #[cfg(feature = "imagery")]
// pub use image_support::*;

// ---------------------------------------------------------------------
mod rendering;
pub use rendering::*;
