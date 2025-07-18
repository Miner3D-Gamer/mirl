/// Rotate a direction by a multiple of 90°
pub trait RotateDirections: Sized {
    /// Rotate the direction 90° clockwise
    fn rotate_clockwise_90(&self) -> Self;

    /// Rotate the direction 90° counterclockwise (or 270° clockwise)
    fn rotate_counterclockwise_90(&self) -> Self;

    /// Rotate the direction 180°
    fn rotate_180(&self) -> Self {
        self.rotate_clockwise_90().rotate_clockwise_90()
    }

    /// Rotate the direction 270° clockwise (or 90° counterclockwise)
    fn rotate_clockwise_270(&self) -> Self {
        self.rotate_counterclockwise_90()
    }

    /// Rotate the direction 270° counterclockwise (or 90° clockwise)
    fn rotate_counterclockwise_270(&self) -> Self {
        self.rotate_clockwise_90()
    }
}

/// Rotate a direction by 45° increments
pub trait RotatePrecise: RotateDirections {
    /// Rotate the direction 45° clockwise
    fn rotate_clockwise_45(&self) -> Self;

    /// Rotate the direction 45° counterclockwise
    fn rotate_counterclockwise_45(&self) -> Self;

    /// Rotate the direction 135° clockwise (90° + 45°)
    fn rotate_clockwise_135(&self) -> Self {
        self.rotate_clockwise_90().rotate_clockwise_45()
    }

    /// Rotate the direction 135° counterclockwise (90° + 45°)
    fn rotate_counterclockwise_135(&self) -> Self {
        self.rotate_counterclockwise_90().rotate_counterclockwise_45()
    }
}

mod all;
mod cardinal;
mod direction;
mod directions_with_none;
mod extended;
mod special;

pub use all::*;
pub use cardinal::*;
pub use direction::*;
pub use directions_with_none::*;
pub use extended::*;
pub use special::*;
