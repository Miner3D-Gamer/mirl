use super::RotateDirections;

/// NE SE SW NW
#[allow(missing_docs)]
pub enum ExtendedDirections {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}
impl RotateDirections for ExtendedDirections {
    fn rotate_clockwise_90(&self) -> Self {
        match self {
            ExtendedDirections::NorthEast => ExtendedDirections::SouthEast,
            ExtendedDirections::SouthEast => ExtendedDirections::SouthWest,
            ExtendedDirections::SouthWest => ExtendedDirections::NorthWest,
            ExtendedDirections::NorthWest => ExtendedDirections::NorthEast,
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            ExtendedDirections::NorthEast => ExtendedDirections::NorthWest,
            ExtendedDirections::SouthEast => ExtendedDirections::NorthEast,
            ExtendedDirections::SouthWest => ExtendedDirections::SouthEast,
            ExtendedDirections::NorthWest => ExtendedDirections::SouthWest,
        }
    }
    fn rotate_180(&self) -> Self {
        match self {
            ExtendedDirections::NorthEast => ExtendedDirections::SouthWest,
            ExtendedDirections::SouthEast => ExtendedDirections::NorthWest,
            ExtendedDirections::SouthWest => ExtendedDirections::SouthEast,
            ExtendedDirections::NorthWest => ExtendedDirections::NorthEast,
        }
    }
}
