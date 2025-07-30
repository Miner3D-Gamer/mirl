// Rewrite to use copyable list instead of Box<[u32]>?
/// A raw color buffer to be modified and read quickly
#[derive(PartialEq, Debug, Eq)]
pub struct Buffer {
    /// Actual color data
    pub data: Box<[u32]>,
    /// Pointer to the color data
    pub pointer: *mut u32,
    /// Width of the buffer
    pub width: usize,
    /// Height of the buffer
    pub height: usize,
    /// The total size -> width*height
    pub total_size: usize,
}

mod get_converted;
mod get_pixel;
mod manipulate;
mod new;
mod set_color;
mod trim;

use std::ops::Deref;
// Automatically convert the usage of Buffer to Buffer.data
impl Deref for Buffer {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Buffer {
    /// Update internal pointer
    pub fn update_pointer(&mut self) {
        self.pointer = self.data.as_mut_ptr();
    }
    /// Update the total size of the buffer
    pub const fn update_total_size(&mut self) {
        self.total_size = self.width * self.height;
    }
}

impl Clone for Buffer {
    #[allow(clippy::as_ptr_cast_mut)]
    fn clone(&self) -> Self {
        let data = self.data.clone();

        let pointer = data.as_ptr().cast_mut();

        Self {
            data,
            pointer,
            width: self.width,
            height: self.height,
            total_size: self.total_size,
        }
    }
}
