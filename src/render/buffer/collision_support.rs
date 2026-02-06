#![allow(clippy::inline_always)]
use super::Buffer;
use crate::{extensions::TryFromPatch, render::BufferCollision};

impl Buffer {
    #[deprecated = "Use `to_collision` instead"]
    #[must_use]
    /// Fallback function
    pub const fn create_collision_usize<const CS: bool>(
        &self,
        pos: (usize, usize),
    ) -> crate::math::geometry::Pos2D<
        usize,
        crate::math::collision::Rectangle<usize, CS>,
    > {
        self.to_collision(pos)
    }
    #[deprecated = "Use `try_to_collision` instead"]
    /// Fallback function
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub const fn create_collision<
        const CS: bool,
        T: [const] TryFromPatch<usize>
            + core::ops::Add<Output = T>
            + PartialOrd
            + Copy
            + core::ops::Div<Output = T>,
    >(
        &self,
        pos: (T, T),
    ) -> Option<
        crate::math::geometry::Pos2D<
            T,
            crate::math::collision::Rectangle<T, CS>,
        >,
    > {
        self.try_to_collision(pos)
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
        // Really? Checking if the value is positive after casting to usize? Good thing there is no record of this code
        x > 0
            && y > 0
            && (x as usize) < self.width
            && (y as usize) < self.height
    }
}
