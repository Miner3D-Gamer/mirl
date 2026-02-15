/// 2D shapes
pub mod d2;
/// Positional data
pub mod positioning;
pub use positioning::{Pos1D, Pos2D, Pos3D, Pos4D};

use crate::directions::ShapeDirectionType;

/// Rotational data
pub mod rotation;
/// When the bottom of the collision is mathematically higher
pub const BOTTOM_HIGHER: bool = true;
/// When the top of the collision is mathematically higher
pub const BOTTOM_LOWER: bool = false;

/// A marker signifying that a struct is a raw shape
pub const trait Shape<T> {}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
/// Sometimes we just need a path without an associating shape
pub struct EmptyShape {}

impl<T> Shape<T> for EmptyShape {}
/// When the given position is at an edge or corner within the range, return the type of the closest [`ShapeDirectionType`](crate::directions::ShapeDirectionType)
pub const trait GetShapeDirectionType<T, R: ShapeDirectionType> {
    /// Get the edge type for the closest edge/corner
    fn get_edge_or_corner_type(&self, point: (T, T), margin: T) -> R;
    /// Get the closest edge type
    fn get_edge_type(&self, point: (T, T), margin: T) -> R;
    /// Get the closest corner type
    fn get_corner_type(&self, point: (T, T), margin: T) -> R;
}

// pub const  trait CollisionDetection<T> {
//     fn collides(&self, other: T) -> bool;
// }

// pub  const trait CollisionResolution<T> {
//     fn collide(&mut self, other: T);
// }
