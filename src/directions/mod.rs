pub trait RotateDirections: Sized {
    fn rotate_clockwise_90(&self) -> Self;
    fn rotate_counterclockwise_90(&self) -> Self;
    fn rotate_180(&self) -> Self {
        self.rotate_clockwise_90().rotate_clockwise_90()
    }
    fn rotate_clockwise_270(&self) -> Self {
        self.rotate_counterclockwise_90()
    }
    fn rotate_counterclockwise_270(&self) -> Self {
        self.rotate_clockwise_90()
    }
}
pub trait RotatePrecise: RotateDirections {
    fn rotate_clockwise_45(&self) -> Self;
    fn rotate_counterclockwise_45(&self) -> Self;
    fn rotate_clockwise_135(&self) -> Self {
        self.rotate_clockwise_90().rotate_clockwise_45()
    }
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
