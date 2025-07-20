use super::Buffer;

impl Buffer {
    /// Flip the buffer vertically (top becomes bottom)
    pub fn flip_vertically(&self) -> Self {
        let result = Self::new_empty(self.width, self.height);

        unsafe {
            for y in 0..self.height {
                let src_row = self.pointer.add(y * self.width);
                let dst_row =
                    result.pointer.add((self.height - 1 - y) * self.width);
                std::ptr::copy_nonoverlapping(src_row, dst_row, self.width);
            }
        }

        result
    }

    /// Flip the buffer horizontally (left becomes right)
    pub fn flip_horizontally(&self) -> Self {
        let result = Self::new_empty(self.width, self.height);

        unsafe {
            for y in 0..self.height {
                for x in 0..self.width {
                    let src_pixel = *self.pointer.add(y * self.width + x);
                    let dst_idx = y * self.width + (self.width - 1 - x);
                    *result.pointer.add(dst_idx) = src_pixel;
                }
            }
        }

        result
    }

    /// Rotate the buffer 90°
    pub fn rotate_90(&self) -> Self {
        let result = Self::new_empty(self.height, self.width);

        unsafe {
            for y in 0..self.height {
                for x in 0..self.width {
                    let src_pixel = *self.pointer.add(y * self.width + x);
                    // For 90° clockwise: new_x = old_y, new_y = width - 1 - old_x
                    let new_x = y;
                    let new_y = self.width - 1 - x;
                    let dst_idx = new_y * self.height + new_x;
                    *result.pointer.add(dst_idx) = src_pixel;
                }
            }
        }

        result
    }

    /// Rotate the buffer 180°
    pub fn rotate_180(&self) -> Self {
        let result = Self::new_empty(self.width, self.height);

        unsafe {
            for i in 0..self.total_size {
                let src_pixel = *self.pointer.add(i);
                let dst_idx = self.total_size - 1 - i;
                *result.pointer.add(dst_idx) = src_pixel;
            }
        }

        result
    }

    /// Rotate the buffer 270° (or -90°)
    pub fn rotate_270(&self) -> Self {
        let result = Self::new_empty(self.height, self.width);

        unsafe {
            for y in 0..self.height {
                for x in 0..self.width {
                    let src_pixel = *self.pointer.add(y * self.width + x);
                    let new_x = self.height - 1 - y;
                    let new_y = x;
                    let dst_idx = new_y * self.height + new_x;
                    *result.pointer.add(dst_idx) = src_pixel;
                }
            }
        }

        result
    }
}
