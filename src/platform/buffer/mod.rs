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
mod new;
mod set_color;
mod trim;
mod manipulate;

use std::ops::Deref;
// Automatically convert the usage of Buffer to Buffer.data
impl Deref for Buffer {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Clone for Buffer {
    fn clone(&self) -> Self {
        let data = self.data.clone();

        let pointer = data.as_ptr() as *mut u32;

        Self {
            data,
            pointer,
            width: self.width,
            height: self.height,
            total_size: self.total_size,
        }
    }
}
