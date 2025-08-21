use super::Buffer;
use crate::graphics::{rgb_to_u32, rgba_to_u32};
impl Buffer {
    
    #[track_caller]
    /// Create a new color
    /// 
    /// # Errors
    /// When not enough data was provided, an error is returned instead of a Buffer
    pub fn new(
        data: Vec<u32>,
        width: usize,
        height: usize,
    ) -> Result<Self, String> {
        let total_size = width * height;
        if data.len() != total_size {
            return Err(format!(
                "Data length does not match dimensions - Expected: {}, Got: {}",
                total_size,
                data.len()
            ));
        }
        let mut buffer = data.into_boxed_slice();
        let buffer_pointer = buffer.as_mut_ptr();
        Ok(Self {
            data: buffer,
            pointer: buffer_pointer,
            width,
            height,
            total_size,
        })
    }
    #[must_use]
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
    #[must_use]
    /// Create a new [Buffer] filled with the specified color
    pub fn new_empty_with_color(
        width: usize,
        height: usize,
        color: u32,
    ) -> Self {
        let total_size = width * height;
        let mut buffer = vec![color; total_size].into_boxed_slice();
        let buffer_pointer = buffer.as_mut_ptr();
        Self {
            data: buffer,
            pointer: buffer_pointer,
            width,
            height,
            total_size,
        }
    }
    #[must_use]
    #[allow(clippy::unwrap_used)]
    #[allow(clippy::missing_panics_doc)]
    /// Generate a error texture with the desired size
    pub fn generate_fallback(
        width: usize,
        height: usize,
        squares: usize,
    ) -> Self {
        let mut data = Vec::with_capacity(width * height);
        let square_size = width.midpoint(height) / squares;

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

        Self::new(data, width, height).unwrap()
    }

    
    /// Create a buffer from a rgba &[u8]
    ///
    /// # Errors
    /// When not enough data was provided, an error is returned instead of a Buffer
    #[track_caller]
    pub fn from_u8_rgba(
        rgba: &[u8],
        width: usize,
        height: usize,
    ) -> Result<Self, String> {
        let mut return_list = Vec::new();
        for i in rgba.chunks(3) {
            let color = rgba_to_u32(i[0], i[1], i[2], i[3]);
            return_list.push(color);
        }
        Self::new(return_list, width, height)
    }
    
    #[track_caller]
    /// Create a buffer from an RGB &[u8]
    ///
    /// # Errors
    /// When not enough data was provided, an error is returned instead of a Buffer
    pub fn from_u8_rgb(
        rgba: &[u8],
        width: usize,
        height: usize,
    ) -> Result<Self, String> {
        let mut return_list = Vec::with_capacity(rgba.len() / 3);
        for chunk in rgba.chunks(3) {
            if chunk.len() == 3 {
                let color = rgb_to_u32(chunk[0], chunk[1], chunk[2]);
                return_list.push(color);
            }
        }
        Self::new(return_list, width, height)
    }
}
