/// Circular collision math
pub mod circle;
/// Rectangular collision math
pub mod rectangle;

pub use circle::Circle;
pub use rectangle::Rectangle;

/// When the bottom of the collision is mathematically higher
pub const BOTTOM_HIGHER: bool = true;
/// When the top of the collision is mathematically higher
pub const BOTTOM_LOWER: bool = false;
