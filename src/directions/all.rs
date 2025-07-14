use super::{
    Directions, ExtendedDirections, RotateDirections, RotatePrecise,
    SpecialDirections,
};
/// A unifier to have every direction available
pub enum AllDirections {
    /// N E S W
    Base(Directions),
    /// NE SE SW NW
    Extended(ExtendedDirections),
    /// No direction
    Special(SpecialDirections),
}
impl RotateDirections for AllDirections {
    fn rotate_clockwise_90(&self) -> Self {
        match self {
            AllDirections::Base(direction) => {
                AllDirections::Base(direction.rotate_clockwise_90())
            }
            AllDirections::Extended(direction) => {
                AllDirections::Extended(direction.rotate_clockwise_90())
            }
            AllDirections::Special(SpecialDirections::None) => {
                AllDirections::Special(SpecialDirections::None)
            }
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            AllDirections::Base(direction) => {
                AllDirections::Base(direction.rotate_counterclockwise_90())
            }
            AllDirections::Extended(direction) => {
                AllDirections::Extended(direction.rotate_counterclockwise_90())
            }
            AllDirections::Special(SpecialDirections::None) => {
                AllDirections::Special(SpecialDirections::None)
            }
        }
    }
    fn rotate_180(&self) -> Self {
        match self {
            AllDirections::Base(direction) => {
                AllDirections::Base(direction.rotate_180())
            }
            AllDirections::Extended(direction) => {
                AllDirections::Extended(direction.rotate_180())
            }
            AllDirections::Special(SpecialDirections::None) => {
                AllDirections::Special(SpecialDirections::None)
            }
        }
    }
}
impl RotatePrecise for AllDirections {
    fn rotate_clockwise_45(&self) -> Self {
        match self {
            AllDirections::Base(direction) => {
                AllDirections::Extended(match direction {
                    Directions::North => ExtendedDirections::NorthEast,
                    Directions::East => ExtendedDirections::SouthEast,
                    Directions::South => ExtendedDirections::SouthWest,
                    Directions::West => ExtendedDirections::NorthWest,
                })
            }
            AllDirections::Extended(direction) => {
                AllDirections::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::East,
                    ExtendedDirections::SouthEast => Directions::South,
                    ExtendedDirections::SouthWest => Directions::West,
                    ExtendedDirections::NorthWest => Directions::North,
                })
            }
            AllDirections::Special(SpecialDirections::None) => {
                AllDirections::Special(SpecialDirections::None)
            }
        }
    }

    fn rotate_counterclockwise_45(&self) -> Self {
        match self {
            AllDirections::Base(direction) => {
                AllDirections::Extended(match direction {
                    Directions::North => ExtendedDirections::NorthWest,
                    Directions::East => ExtendedDirections::NorthEast,
                    Directions::South => ExtendedDirections::SouthEast,
                    Directions::West => ExtendedDirections::SouthWest,
                })
            }
            AllDirections::Extended(direction) => {
                AllDirections::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::North,
                    ExtendedDirections::SouthEast => Directions::East,
                    ExtendedDirections::SouthWest => Directions::South,
                    ExtendedDirections::NorthWest => Directions::West,
                })
            }
            AllDirections::Special(SpecialDirections::None) => {
                AllDirections::Special(SpecialDirections::None)
            }
        }
    }

    fn rotate_clockwise_135(&self) -> Self {
        match self {
            AllDirections::Base(direction) => {
                AllDirections::Extended(match direction {
                    Directions::North => ExtendedDirections::SouthEast,
                    Directions::East => ExtendedDirections::SouthWest,
                    Directions::South => ExtendedDirections::NorthWest,
                    Directions::West => ExtendedDirections::NorthEast,
                })
            }
            AllDirections::Extended(direction) => {
                AllDirections::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::South,
                    ExtendedDirections::SouthEast => Directions::West,
                    ExtendedDirections::SouthWest => Directions::North,
                    ExtendedDirections::NorthWest => Directions::East,
                })
            }
            AllDirections::Special(SpecialDirections::None) => {
                AllDirections::Special(SpecialDirections::None)
            }
        }
    }

    fn rotate_counterclockwise_135(&self) -> Self {
        match self {
            AllDirections::Base(direction) => {
                AllDirections::Extended(match direction {
                    Directions::North => ExtendedDirections::SouthWest,
                    Directions::East => ExtendedDirections::NorthWest,
                    Directions::South => ExtendedDirections::NorthEast,
                    Directions::West => ExtendedDirections::SouthEast,
                })
            }
            AllDirections::Extended(direction) => {
                AllDirections::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::West,
                    ExtendedDirections::SouthEast => Directions::North,
                    ExtendedDirections::SouthWest => Directions::East,
                    ExtendedDirections::NorthWest => Directions::South,
                })
            }
            AllDirections::Special(SpecialDirections::None) => {
                AllDirections::Special(SpecialDirections::None)
            }
        }
    }
}
