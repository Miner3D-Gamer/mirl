#![allow(clippy::inline_always)]
use super::Buffer;

impl Buffer {
    #[must_use]
    /// Create a collision instance for the current buffer
    pub const fn create_collision<const CS: bool>(
        &self,
        x: usize,
        y: usize,
    ) -> crate::math::collision::Rectangle<usize, CS> {
        crate::math::collision::Rectangle::new(x, y, self.width, self.height)
    }
    /// Create a collision instance for the current buffer using isize coordinates
    pub const fn create_collision_isize<const CS: bool>(
        &self,
        x: isize,
        y: isize,
    ) -> crate::math::collision::Rectangle<isize, CS> {
        crate::math::collision::Rectangle::new(
            x,
            y,
            self.width as isize,
            self.height as isize,
        )
    }
    #[must_use]
    #[inline(always)]
    /// Simple function of pixel coordinate falls within the buffer metrics
    pub const fn is_pixel_position_in_buffer(
        &self,
        x: usize,
        y: usize,
    ) -> bool {
        x < self.width && y < self.height
    }
}
