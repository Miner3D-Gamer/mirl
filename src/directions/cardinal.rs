use super::{Directions, ExtendedDirections, RotateDirections, RotatePrecise};
/// A unifier for all real directions
pub enum AllCardinalDirections {
    /// N E S W
    Base(Directions),
    /// NE SE SW NW
    Extended(ExtendedDirections),
}
impl RotateDirections for AllCardinalDirections {
    fn rotate_clockwise_90(&self) -> Self {
        match self {
            AllCardinalDirections::Base(direction) => {
                AllCardinalDirections::Base(direction.rotate_clockwise_90())
            }
            AllCardinalDirections::Extended(direction) => {
                AllCardinalDirections::Extended(direction.rotate_clockwise_90())
            }
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            AllCardinalDirections::Base(direction) => {
                AllCardinalDirections::Base(
                    direction.rotate_counterclockwise_90(),
                )
            }
            AllCardinalDirections::Extended(direction) => {
                AllCardinalDirections::Extended(
                    direction.rotate_counterclockwise_90(),
                )
            }
        }
    }
    fn rotate_180(&self) -> Self {
        match self {
            AllCardinalDirections::Base(direction) => {
                AllCardinalDirections::Base(direction.rotate_180())
            }
            AllCardinalDirections::Extended(direction) => {
                AllCardinalDirections::Extended(direction.rotate_180())
            }
        }
    }
}

impl RotatePrecise for AllCardinalDirections {
    fn rotate_clockwise_45(&self) -> Self {
        match self {
            AllCardinalDirections::Base(direction) => {
                AllCardinalDirections::Extended(match direction {
                    Directions::North => ExtendedDirections::NorthEast,
                    Directions::East => ExtendedDirections::SouthEast,
                    Directions::South => ExtendedDirections::SouthWest,
                    Directions::West => ExtendedDirections::NorthWest,
                })
            }
            AllCardinalDirections::Extended(direction) => {
                AllCardinalDirections::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::East,
                    ExtendedDirections::SouthEast => Directions::South,
                    ExtendedDirections::SouthWest => Directions::West,
                    ExtendedDirections::NorthWest => Directions::North,
                })
            }
        }
    }
    fn rotate_counterclockwise_45(&self) -> Self {
        match self {
            AllCardinalDirections::Base(direction) => {
                AllCardinalDirections::Extended(match direction {
                    Directions::North => ExtendedDirections::NorthWest,
                    Directions::East => ExtendedDirections::NorthEast,
                    Directions::South => ExtendedDirections::SouthWest,
                    Directions::West => ExtendedDirections::SouthEast,
                })
            }
            AllCardinalDirections::Extended(direction) => {
                AllCardinalDirections::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::North,
                    ExtendedDirections::SouthEast => Directions::East,
                    ExtendedDirections::SouthWest => Directions::South,
                    ExtendedDirections::NorthWest => Directions::West,
                })
            }
        }
    }
    fn rotate_clockwise_135(&self) -> Self {
        match self {
            AllCardinalDirections::Base(direction) => {
                AllCardinalDirections::Extended(match direction {
                    Directions::North => ExtendedDirections::SouthEast,
                    Directions::East => ExtendedDirections::SouthWest,
                    Directions::South => ExtendedDirections::NorthWest,
                    Directions::West => ExtendedDirections::NorthEast,
                })
            }
            AllCardinalDirections::Extended(direction) => {
                AllCardinalDirections::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::South,
                    ExtendedDirections::SouthEast => Directions::West,
                    ExtendedDirections::SouthWest => Directions::North,
                    ExtendedDirections::NorthWest => Directions::East,
                })
            }
        }
    }
    fn rotate_counterclockwise_135(&self) -> Self {
        match self {
            AllCardinalDirections::Base(direction) => {
                AllCardinalDirections::Extended(match direction {
                    Directions::North => ExtendedDirections::NorthWest,
                    Directions::East => ExtendedDirections::NorthEast,
                    Directions::South => ExtendedDirections::SouthEast,
                    Directions::West => ExtendedDirections::SouthWest,
                })
            }
            AllCardinalDirections::Extended(direction) => {
                AllCardinalDirections::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::North,
                    ExtendedDirections::SouthEast => Directions::East,
                    ExtendedDirections::SouthWest => Directions::South,
                    ExtendedDirections::NorthWest => Directions::West,
                })
            }
        }
    }
}
