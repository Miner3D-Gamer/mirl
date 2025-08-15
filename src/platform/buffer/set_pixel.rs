#![allow(clippy::inline_always)]
use super::Buffer;
impl Buffer {
    #[inline(always)]
    /// Set the pixel at the specified position safely
    pub fn set_pixel_safe(&self, x: usize, y: usize, color: u32) {
        crate::render::draw_pixel_safe(self, x, y, color);
    }
    #[inline(always)]
    /// Set the pixel at the specified position without checking if it is within the allowed memory
    pub fn set_pixel_unsafe(&self, x: usize, y: usize, color: u32) {
        crate::render::draw_pixel_unsafe(self, x, y, color);
    }
}
