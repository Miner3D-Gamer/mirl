#![allow(clippy::inline_always)]
use super::ConstBuffer;
use crate::extensions::TryFromPatch;

impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    #[must_use]
    /// Create a collision instance for the current buffer
    pub const fn to_rectangle_usize<const CS: bool>(
        &self,
    ) -> crate::math::collision::Rectangle<usize, CS> {
        crate::math::collision::Rectangle::new(self.get_size())
    }
    /// Create a collision instance for the current buffer using isize coordinates
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub const fn to_rectangle<
        const CS: bool,
        T: [const] TryFromPatch<usize>
            + core::ops::Add<Output = T>
            + PartialOrd
            + Copy
            + core::ops::Div<Output = T>,
    >(
        &self,
    ) -> Option<crate::math::collision::Rectangle<T, CS>> {
        Some(crate::math::collision::Rectangle::new((
            T::try_from_value(WIDTH)?,
            T::try_from_value(HEIGHT)?,
        )))
    }
    #[must_use]
    /// Create a collision instance for the current buffer
    pub const fn to_collision_usize<const CS: bool>(
        &self,
        pos: (usize, usize),
    ) -> crate::math::geometry::Pos2D<
        usize,
        crate::math::collision::Rectangle<usize, CS>,
    > {
        crate::math::geometry::Pos2D::new(
            pos,
            crate::math::collision::Rectangle::new(self.get_size()),
        )
    }
    /// Create a collision instance for the current buffer using isize coordinates
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    pub const fn to_collision<
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
        Some(crate::math::geometry::Pos2D::new(
            pos,
            crate::math::collision::Rectangle::new((
                T::try_from_value(WIDTH)?,
                T::try_from_value(HEIGHT)?,
            )),
        ))
    }
    #[must_use]
    #[inline(always)]
    /// Simple function checking if pixel a coordinate falls within the buffer metrics
    pub const fn is_pixel_position_in_buffer(
        &self,
        x: usize,
        y: usize,
    ) -> bool {
        x < WIDTH && y < HEIGHT
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
        (x as usize) < WIDTH && (y as usize) < HEIGHT && x > 0 && y > 0
    }
}
