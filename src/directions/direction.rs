use super::RotateDirections;

pub enum Directions {
    North,
    East,
    South,
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
