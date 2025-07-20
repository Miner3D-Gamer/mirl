use crate::graphics::rgb_to_u32;

use super::Buffer;
impl Buffer {
    /// Create a new color
    pub fn new(data: Vec<u32>, width: usize, height: usize) -> Self {
        let total_size = width * height;
        let mut buffer = data.into_boxed_slice();
        let buffer_pointer = buffer.as_mut_ptr();
        Self {
            data: buffer,
            pointer: buffer_pointer,
            width,
            height,
            total_size,
        }
    }
    /// Create a new, empty, [Buffer]
    pub fn new_empty(width: usize, height: usize) -> Self {
        let total_size = width * height;
        let mut buffer = vec![0u32; total_size].into_boxed_slice();
        let buffer_pointer = buffer.as_mut_ptr();
        Self {
            data: buffer,
            pointer: buffer_pointer,
            width,
            height,
            total_size,
        }
    }
    /// Generate a error texture with the desired size
    pub fn generate_fallback(
        width: usize,
        height: usize,
        square_size: usize,
    ) -> Self {
        let mut data = Vec::with_capacity(width * height);

        let purple = rgb_to_u32(128, 0, 128);
        let black = rgb_to_u32(0, 0, 0);

        for y in 0..height {
            for x in 0..width {
                let square_x = x / square_size;
                let square_y = y / square_size;

                let color = if (square_x + square_y) % 2 == 0 {
                    purple
                } else {
                    black
                };

                data.push(color);
            }
        }

        Self::new(data, width, height)
    }
}
