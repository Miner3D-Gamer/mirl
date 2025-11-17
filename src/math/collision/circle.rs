#[cfg(feature = "std")]
use crate::math::ConvenientOps;
#[cfg(feature = "std")]
use crate::math::NumberWithMonotoneOps;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A simple Rectangle defining computational limits
#[allow(missing_docs)]
pub struct Circle<T, const CS: bool> {
    pub x: T,
    pub y: T,
    pub radius: T,
    pub half_radius: T,
}
#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
impl<const CS: bool, T: NumberWithMonotoneOps + Copy> Circle<T, CS> {
    /// Checks if a point is inside the radius of the circle
    pub fn does_area_contain_point(&self, point: (T, T)) -> bool {
        let dx = point.0 - (self.x + self.half_radius);
        let dy = point.1 - (self.y + self.half_radius);
        dx * dx + dy * dy <= self.radius * self.radius
    }
}

#[cfg(feature = "std")]
impl<
        const CS: bool,
        T: NumberWithMonotoneOps + Copy + num_traits::Zero + num_traits::Float,
    > Circle<T, CS>
{
    /// Get the closest point on the edge to the defined point
    pub fn get_closest_point_on_edge(&self, point: (T, T)) -> (T, T) {
        let cx = self.x + self.half_radius;
        let cy = self.y + self.half_radius;
        let dx = point.0 - cx;
        let dy = point.1 - cy;
        let dist_sq = dx * dx + dy * dy;
        if dist_sq == T::zero() {
            return (cx + self.radius, cy); // arbitrary edge point when point == center
        }
        let dist = dist_sq.sqrt();
        let scale = self.radius / dist;
        (cx + dx * scale, cy + dy * scale)
    }
}
