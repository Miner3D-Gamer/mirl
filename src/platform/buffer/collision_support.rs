#![allow(clippy::inline_always)]
use super::Buffer;
use crate::extensions::FromPatch;

impl Buffer {
    #[must_use]
    /// Create a collision instance for the current buffer
    pub const fn create_collision_usize<const CS: bool>(
        &self,
        x: usize,
        y: usize,
    ) -> crate::math::collision::Rectangle<usize, CS> {
        crate::math::collision::Rectangle::new(x, y, self.width, self.height)
    }
    /// Create a collision instance for the current buffer using isize coordinates
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub const fn create_collision<
        const CS: bool,
        T: [const] FromPatch<usize>
            + std::ops::Add<Output = T>
            + PartialOrd
            + Copy,
    >(
        &self,
        x: T,
        y: T,
    ) -> crate::math::collision::Rectangle<T, CS> {
        crate::math::collision::Rectangle::new(
            x,
            y,
            T::from_value(self.width),
            T::from_value(self.height),
        )
    }
    #[must_use]
    #[inline(always)]
    /// Simple function checking if pixel a coordinate falls within the buffer metrics
    pub const fn is_pixel_position_in_buffer(
        &self,
        x: usize,
        y: usize,
    ) -> bool {
        x < self.width && y < self.height
    }
    #[must_use]
    #[inline(always)]
    #[allow(clippy::cast_sign_loss)]
    /// Simple function checking if pixel a possibly negative coordinate falls within the buffer metrics
    pub const fn is_pixel_position_in_buffer_isize(
        &self,
        x: isize,
        y: isize,
    ) -> bool {
        (x as usize) < self.width
            && (y as usize) < self.height
            && x > 0
            && y > 0
    }
}
