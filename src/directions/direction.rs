use super::RotateDirections;

/// N E S W
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
            Directions::North => Directions::East,
            Directions::East => Directions::South,
            Directions::South => Directions::West,
            Directions::West => Directions::North,
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            Directions::North => Directions::West,
            Directions::East => Directions::North,
            Directions::South => Directions::East,
            Directions::West => Directions::South,
        }
    }
    fn rotate_180(&self) -> Self {
        match self {
            Directions::North => Directions::South,
            Directions::East => Directions::West,
            Directions::South => Directions::North,
            Directions::West => Directions::East,
        }
    }
}
