use super::RotateDirections;

/// N E S W
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Directions {
    /// It's North, what'd you expect?
    North,
    /// It's East, what'd you expect?
    East,
    /// It's South, what'd you expect?
    South,
    /// It's West, what'd you expect?
    West,
}

impl RotateDirections for Directions {
    fn rotate_clockwise_90(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }
    fn rotate_180(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}
