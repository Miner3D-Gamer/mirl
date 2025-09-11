use crate::math::{ConvenientOps, NumberWithMonotoneOps};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A simple Rectangle defining computational limits
#[allow(missing_docs)]
pub struct Circle<T, const CS: bool> {
    pub x: T,
    pub y: T,
    pub radius: T,
    pub half_radius: T,
}
impl<const CS: bool, T: NumberWithMonotoneOps + Copy + ConvenientOps>
    Circle<T, CS>
{
    #[allow(missing_docs)]
    pub fn new(x: T, y: T, radius: T) -> Self {
        Self {
            x,
            y,
            radius,
            half_radius: radius.half(),
        }
    }
}

impl<const CS: bool, T: NumberWithMonotoneOps + Copy> Circle<T, CS> {
    /// Checks if a point is inside the radius of the circle
    pub fn does_area_contain_point(&self, point: (T, T)) -> bool {
        let dx = point.0 - (self.x + self.half_radius);
        let dy = point.1 - (self.y + self.half_radius);
        dx * dx + dy * dy <= self.radius * self.radius
    }
}
