/// A trait for supporting any kind of drawable buffer
///
/// Warning: Do not use this trait. It will be removed in the future
///
/// TODO: Split this trait into several sub traits with more specialized functions
pub const trait BufferMisc {
    /// Get the stored data as a slice
    fn data(&mut self) -> &mut [u32];
}
/// Get data pointers for the given buffer
pub const trait BufferPointers {
    #[must_use]
    /// Get the pointer to the stored data
    fn pointer(&self) -> *const u32;
    #[must_use]
    /// Get the pointer to the stored data mutably
    fn mut_pointer(&mut self) -> *mut u32;
}
/// Get image size for the given buffer
pub const trait BufferMetrics: [const] BufferMetricsHelper {
    /// Get the width of the stored image
    fn width(&self) -> usize;
    /// Get the height of the stored image
    fn height(&self) -> usize;
}
/// A helper trait for [`BufferMetrics`]
pub const trait BufferMetricsHelper {
    /// Get the current size of the buffer in a tuple
    fn get_size(&self) -> (usize, usize);
}
impl<T: [const] BufferMetrics> const BufferMetricsHelper for T {
    fn get_size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
}

/// Get the pixel color at a position
pub const trait BufferGetPixel: BufferGetPixelHelper {
    /// Safely get the pixel color of the buffer at the specified x and y, returns 0 if the pixel is out of bounds
    fn get_pixel(&self, xy: (usize, usize)) -> u32;
    /// Safely get the pixel color of the buffer at the specified x and y, returns None if the pixel is out of bounds
    fn get_pixel_option(&self, xy: (usize, usize)) -> Option<u32>;
    /// Get the pixel color at a position in a buffer without checking if the pixel is on screen (which will crash the program if it isn't)
    fn get_pixel_unsafe(&self, xy: (usize, usize)) -> u32;
}
/// A helper trait with automatic isize support
pub const trait BufferGetPixelHelper {
    /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
    fn get_pixel_isize(&self, xy: (isize, isize)) -> u32;
    /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
    fn get_pixel_isize_option(&self, xy: (isize, isize)) -> Option<u32>;
}

impl<T: BufferGetPixel> BufferGetPixelHelper for T {
    /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
    #[inline(always)]
    #[allow(clippy::cast_sign_loss)]
    fn get_pixel_isize(&self, xy: (isize, isize)) -> u32 {
        if xy.0 < 0 || xy.1 < 0 {
            return 0;
        }
        self.get_pixel((xy.0 as usize, xy.1 as usize))
    }
    /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
    #[inline(always)]
    #[allow(clippy::cast_sign_loss)]
    fn get_pixel_isize_option(&self, xy: (isize, isize)) -> Option<u32> {
        if xy.0 < 0 || xy.1 < 0 {
            return None;
        }
        self.get_pixel_option((xy.0 as usize, xy.1 as usize))
    }
}
/// Set the pixel color at a position - Automatically implemented for structs that implement [`BufferPointers`] and [`BufferMetrics`]
pub const trait BufferSetPixel {
    /// Set the pixel at the specified position, it'll check if the pixel is in bounds for you
    fn set_pixel_safe(&mut self, xy: (usize, usize), color: u32);
    /// Set the pixel at the specified position without checking if it is within the allowed memory
    fn set_pixel_unsafe(&mut self, xy: (usize, usize), color: u32);
}

impl<T: BufferPointers + BufferMetrics> BufferSetPixel for T {
    /// Set the pixel at the specified position, it'll check if the pixel is in bounds for you
    fn set_pixel_safe(&mut self, xy: (usize, usize), color: u32) {
        crate::render::draw_pixel_safe(self, xy, color);
    }
    /// Set the pixel at the specified position without checking if it is within the allowed memory
    fn set_pixel_unsafe(&mut self, xy: (usize, usize), color: u32) {
        crate::render::draw_pixel_unsafe(self, xy, color);
    }
}
use crate::prelude::*;
/// Create a collision -> [Pos2<Rectangle>>](crate::math::geometry::Pos2D) / [Rectange](crate::math::geometry::d2::Rectangle) from the buffer metrics -> Automatically implemented for structs that implement [`BufferMetrics`]
pub const trait BufferCollision {
    /// Create a collision instance for the current buffer
    fn to_rectangle<const CS: bool, T: [const] FromPatch<usize> + Copy>(
        &self,
    ) -> crate::math::collision::Rectangle<T, CS>;
    /// Create a collision instance for the current buffer
    fn try_to_rectangle<
        const CS: bool,
        T: [const] TryFromPatch<usize>
            + core::ops::Add<Output = T>
            + PartialOrd
            + Copy
            + core::ops::Div<Output = T>,
    >(
        &self,
    ) -> Option<crate::math::collision::Rectangle<T, CS>>;
    #[must_use]
    /// Create a collision instance for the current buffer
    fn to_collision<const CS: bool, T: [const] FromPatch<usize> + Copy>(
        &self,
        pos: (T, T),
    ) -> crate::math::geometry::Pos2D<T, crate::math::collision::Rectangle<T, CS>>;
    /// Create a collision instance for the current buffer
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    fn try_to_collision<
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
    >;
}

impl<B: [const] BufferMetrics> const BufferCollision for B {
    /// Create a collision instance for the current buffer
    fn to_rectangle<const CS: bool, T: [const] FromPatch<usize> + Copy>(
        &self,
    ) -> crate::math::collision::Rectangle<T, CS> {
        crate::math::collision::Rectangle::new(self.get_size().tuple_into())
    }
    /// Create a collision instance for the current buffer using isize coordinates
    #[allow(clippy::cast_possible_wrap)]
    fn try_to_rectangle<
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
            T::try_from_value(self.width())?,
            T::try_from_value(self.height())?,
        )))
    }
    /// Create a collision instance for the current buffer
    fn to_collision<const CS: bool, T: [const] FromPatch<usize> + Copy>(
        &self,
        pos: (T, T),
    ) -> crate::math::geometry::Pos2D<T, crate::math::collision::Rectangle<T, CS>>
    {
        crate::math::geometry::Pos2D::new(
            pos,
            crate::math::collision::Rectangle::new((
                self.width().into_value(),
                self.height().into_value(),
            )),
        )
    }
    /// Create a collision instance for the current buffer using isize coordinates
    #[allow(clippy::cast_possible_wrap)]
    fn try_to_collision<
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
                T::try_from_value(self.width())?,
                T::try_from_value(self.height())?,
            )),
        ))
    }
}

// BufferMetrics
// BufferPointers
// BufferData
// BufferSetPixel
// BufferGetPixel
