/// Rotate a direction by a multiple of 90°
pub const trait RotateDirections: [const] RotateDirectionsHelper {
    #[must_use]
    /// Rotate the direction 90° clockwise
    fn rotate_clockwise_90(&self) -> Self;
    #[must_use]
    /// Rotate the direction 90° counterclockwise (or 270° clockwise)
    fn rotate_counterclockwise_90(&self) -> Self;
}
/// Rotate a direction by a multiple of 90°
pub const trait RotateDirectionsHelper: Sized {
    #[must_use]
    /// Rotate the direction 180°
    fn rotate_180(&self) -> Self;
    #[must_use]
    /// Rotate the direction 270° clockwise (or 90° counterclockwise)
    fn rotate_clockwise_270(&self) -> Self;
    #[must_use]
    /// Rotate the direction 270° counterclockwise (or 90° clockwise)
    fn rotate_counterclockwise_270(&self) -> Self;
}
impl<T: [const] RotateDirections + Copy> const RotateDirectionsHelper for T {
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
pub const trait RotatePrecise: [const] RotatePreciseHelper {
    #[must_use]
    /// Rotate the direction 45° clockwise
    fn rotate_clockwise_45(&self) -> Self;
    #[must_use]
    /// Rotate the direction 45° counterclockwise
    fn rotate_counterclockwise_45(&self) -> Self;
}
/// Rotate a direction by 45° increments
pub const trait RotatePreciseHelper: [const] RotateDirections {
    #[must_use]
    /// Rotate the direction 135° clockwise (90° + 45°)
    fn rotate_clockwise_135(&self) -> Self;
    #[must_use]
    /// Rotate the direction 135° counterclockwise (90° + 45°)
    fn rotate_counterclockwise_135(&self) -> Self;
}
impl<T: [const] RotatePrecise + Copy> const RotatePreciseHelper for T {
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

/// Functions that are somewhat related to directions yet do not have direct relation to it
pub mod misc;

#[allow(clippy::struct_excessive_bools, missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
/// A boolean for each simple Direction
pub struct NormalDirections {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub top_left: bool,
    pub top_right: bool,
    pub bottom_left: bool,
    pub bottom_right: bool,
}
impl NormalDirections {
    #[must_use]
    #[allow(clippy::fn_params_excessive_bools)] // Really clippy? 4 booleans is excessive in your eyes?
    /// Create a simple directional boolean struct
    pub const fn new(top: bool, bottom: bool, left: bool, right: bool) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            top_left: top && left,
            top_right: top && right,
            bottom_left: bottom && left,
            bottom_right: bottom && right,
        }
    }
    /// "Yes"
    #[must_use]
    pub const fn all_true() -> Self {
        Self {
            top: true,
            bottom: true,
            left: true,
            right: true,
            top_left: true,
            top_right: true,
            bottom_left: true,
            bottom_right: true,
        }
    }
    /// "No"
    #[must_use]
    pub const fn all_false() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
            top_left: false,
            top_right: false,
            bottom_left: false,
            bottom_right: false,
        }
    }
    #[must_use]
    /// Check if the given direction is true
    pub fn is_direction_true<T: IsDirectionTrue>(&self, direction: &T) -> bool {
        direction.is_direction_true(self)
    }
}
/// Check if the given direction is true in [`NormalDirections`]
pub const trait IsDirectionTrue {
    /// Check if the given direction is true in [`NormalDirections`]
    fn is_direction_true(&self, directions: &NormalDirections) -> bool;
}
impl const IsDirectionTrue for u8 {
    fn is_direction_true(&self, directions: &NormalDirections) -> bool {
        match self {
            0 => directions.top_left,
            1 => directions.top,
            2 => directions.top_right,
            3 => directions.right,
            4 => directions.bottom_right,
            5 => directions.bottom,
            6 => directions.bottom_left,
            7 => directions.left,
            _ => false,
        }
    }
}

// TODO: Add this trait to other directions
impl const IsDirectionTrue for Directions {
    fn is_direction_true(&self, directions: &NormalDirections) -> bool {
        match self {
            Self::North => directions.top,
            Self::South => directions.bottom,
            Self::West => directions.left,
            Self::East => directions.right,
        }
    }
}

/// When implemented, return the value corresponding with the location
///
/// TODO: Add unicode arrows for ever location
pub const trait ShapeDirectionType {
    /// The top left corner
    fn top_left_direction() -> Self;
    /// The top right corner
    fn top_right_direction() -> Self;
    /// The bottom left corner
    fn bottom_left_direction() -> Self;
    /// The bottom right corner
    fn bottom_right_direction() -> Self;
    /// The top edge
    fn top_direction() -> Self;
    /// The right edge
    fn right_direction() -> Self;
    /// The left edge
    fn left_direction() -> Self;
    /// The bottom edge
    fn bottom_direction() -> Self;
    /// Not near an edge
    fn none_directional() -> Self;
}
use crate::math::{ConstOne, ConstNumbers128, ConstZero, Bounded};
impl<T: ConstNumbers128 + ConstOne + ConstZero + [const] Bounded> const
    ShapeDirectionType for T
{
    fn bottom_direction() -> Self {
        Self::CONST_5
    }
    fn bottom_left_direction() -> Self {
        Self::CONST_6
    }
    fn bottom_right_direction() -> Self {
        Self::CONST_4
    }
    fn left_direction() -> Self {
        Self::CONST_7
    }
    fn none_directional() -> Self {
        Self::max_value()
    }
    fn right_direction() -> Self {
        Self::CONST_3
    }
    fn top_direction() -> Self {
        Self::ONE
    }
    fn top_left_direction() -> Self {
        Self::ZERO
    }
    fn top_right_direction() -> Self {
        Self::CONST_2
    }
}
