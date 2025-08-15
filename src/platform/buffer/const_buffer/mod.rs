#[derive(PartialEq, Debug, Eq)]
/// A buffer that utilized known metrics for more compile time optimizations
pub struct ConstBuffer<const WIDTH: usize, const HEIGHT: usize> {
    /// Actual color data
    pub data: Box<[u32]>,
    /// Pointer to the color data
    pub pointer: *mut u32,
}

impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT> {
    /// The total size, to use, use; `Buffer::<WIDTH, HEIGHT>::TOTAL_SIZE`
    pub const TOTAL_SIZE: usize = WIDTH * HEIGHT;
}
