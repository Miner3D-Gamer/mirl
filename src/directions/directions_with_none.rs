use super::{Directions, RotateDirections, SpecialDirections};
/// A unifier to have the usual directions as well as the extended version
pub enum DirectionsWithNone {
    /// N E S W
    Base(Directions),
    /// NE SE SW NW
    Special(SpecialDirections),
}
impl RotateDirections for DirectionsWithNone {
    fn rotate_clockwise_90(&self) -> Self {
        match self {
            DirectionsWithNone::Base(direction) => {
                DirectionsWithNone::Base(direction.rotate_clockwise_90())
            }
            DirectionsWithNone::Special(SpecialDirections::None) => {
                DirectionsWithNone::Special(SpecialDirections::None)
            }
        }
    }
    fn rotate_counterclockwise_90(&self) -> Self {
        match self {
            DirectionsWithNone::Base(direction) => {
                DirectionsWithNone::Base(direction.rotate_counterclockwise_90())
            }
            DirectionsWithNone::Special(SpecialDirections::None) => {
                DirectionsWithNone::Special(SpecialDirections::None)
            }
        }
    }
    fn rotate_180(&self) -> Self {
        match self {
            DirectionsWithNone::Base(direction) => {
                DirectionsWithNone::Base(direction.rotate_180())
            }
            DirectionsWithNone::Special(SpecialDirections::None) => {
                DirectionsWithNone::Special(SpecialDirections::None)
            }
        }
    }
}
