use crate::render::{BufferMetrics, BufferMisc, BufferPointers};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A buffer that utilized known metrics for more compile time optimizations
pub struct ConstBuffer<const WIDTH: usize, const HEIGHT: usize>
where
    [(); WIDTH * HEIGHT]:,
{
    /// Actual color data
    pub data: [u32; WIDTH * HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    /// The total size, to use, use; `Buffer::<WIDTH, HEIGHT>::TOTAL_SIZE`
    pub const TOTAL_SIZE: usize = WIDTH * HEIGHT;
    #[must_use]
    /// Get the pointer to self.data
    pub const fn pointer(&self) -> *const u32 {
        self.data.as_ptr()
    }
    #[must_use]
    /// Get the pointer to self.data mutably
    pub const fn mut_pointer(&mut self) -> *mut u32 {
        self.data.as_mut_ptr()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> core::ops::Deref
    for ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
mod collision_support;
mod draw;
mod get_converted;
mod get_pixel;
mod manipulate;
mod misc;
mod new;
mod set_color;
mod set_pixel;
mod trim;

impl<const WIDTH: usize, const HEIGHT: usize> BufferMisc
    for ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    fn data(&mut self) -> &mut [u32] {
        &mut self.data
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> const BufferMetrics
    for ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> const BufferPointers
    for ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    fn pointer(&self) -> *const u32 {
        self.pointer()
    }

    fn mut_pointer(&mut self) -> *mut u32 {
        self.mut_pointer()
    }
}
