#[derive(Debug, Clone, Copy)]
/// A 2D point with an x and a y
pub struct Point2D<T> {
    /// Coordinate on the x axis
    pub x: T,
    /// Coordinate on the y axis
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
/// A 3D point with an x, y, and z
pub struct Point3D<T> {
    /// Coordinate on the x axis
    pub x: T,
    /// Coordinate on the y axis
    pub y: T,
    /// Coordinate on the z axis
    pub z: T,
}
