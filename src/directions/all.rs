use super::{
    Directions, ExtendedDirections, RotateDirections, RotatePrecise,
    SpecialDirections,
};
/// N E S W + NE SE SW NW + None
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            Self::Base(direction) => {
                Self::Base(direction.rotate_clockwise_90())
            }
            Self::Extended(direction) => {
                Self::Extended(direction.rotate_clockwise_90())
            }
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            Self::Base(direction) => {
                Self::Base(direction.rotate_counterclockwise_90())
            }
            Self::Extended(direction) => {
                Self::Extended(direction.rotate_counterclockwise_90())
            }
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }
    fn rotate_180(&self) -> Self {
        match self {
            Self::Base(direction) => {
                Self::Base(direction.rotate_180())
            }
            Self::Extended(direction) => {
                Self::Extended(direction.rotate_180())
            }
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }
}
impl RotatePrecise for AllDirections {
    fn rotate_clockwise_45(&self) -> Self {
        match self {
            Self::Base(direction) => {
                Self::Extended(match direction {
                    Directions::North => ExtendedDirections::NorthEast,
                    Directions::East => ExtendedDirections::SouthEast,
                    Directions::South => ExtendedDirections::SouthWest,
                    Directions::West => ExtendedDirections::NorthWest,
                })
            }
            Self::Extended(direction) => {
                Self::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::East,
                    ExtendedDirections::SouthEast => Directions::South,
                    ExtendedDirections::SouthWest => Directions::West,
                    ExtendedDirections::NorthWest => Directions::North,
                })
            }
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }

    fn rotate_counterclockwise_45(&self) -> Self {
        match self {
            Self::Base(direction) => {
                Self::Extended(match direction {
                    Directions::North => ExtendedDirections::NorthWest,
                    Directions::East => ExtendedDirections::NorthEast,
                    Directions::South => ExtendedDirections::SouthEast,
                    Directions::West => ExtendedDirections::SouthWest,
                })
            }
            Self::Extended(direction) => {
                Self::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::North,
                    ExtendedDirections::SouthEast => Directions::East,
                    ExtendedDirections::SouthWest => Directions::South,
                    ExtendedDirections::NorthWest => Directions::West,
                })
            }
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }

    fn rotate_clockwise_135(&self) -> Self {
        match self {
            Self::Base(direction) => {
                Self::Extended(match direction {
                    Directions::North => ExtendedDirections::SouthEast,
                    Directions::East => ExtendedDirections::SouthWest,
                    Directions::South => ExtendedDirections::NorthWest,
                    Directions::West => ExtendedDirections::NorthEast,
                })
            }
            Self::Extended(direction) => {
                Self::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::South,
                    ExtendedDirections::SouthEast => Directions::West,
                    ExtendedDirections::SouthWest => Directions::North,
                    ExtendedDirections::NorthWest => Directions::East,
                })
            }
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }

    fn rotate_counterclockwise_135(&self) -> Self {
        match self {
            Self::Base(direction) => {
                Self::Extended(match direction {
                    Directions::North => ExtendedDirections::SouthWest,
                    Directions::East => ExtendedDirections::NorthWest,
                    Directions::South => ExtendedDirections::NorthEast,
                    Directions::West => ExtendedDirections::SouthEast,
                })
            }
            Self::Extended(direction) => {
                Self::Base(match direction {
                    ExtendedDirections::NorthEast => Directions::West,
                    ExtendedDirections::SouthEast => Directions::North,
                    ExtendedDirections::SouthWest => Directions::East,
                    ExtendedDirections::NorthWest => Directions::South,
                })
            }
            Self::Special(SpecialDirections::None) => {
                Self::Special(SpecialDirections::None)
            }
        }
    }
}
