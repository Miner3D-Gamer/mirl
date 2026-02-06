use super::ConstBuffer;

impl<const WIDTH: usize, const HEIGHT: usize> ConstBuffer<WIDTH, HEIGHT>
where
    [(); WIDTH * HEIGHT]:,
{
    #[must_use]
    /// Flip the `ConstBuffer` vertically (top becomes bottom)
    pub fn flip_vertically(&self) -> Self {
        let mut result = Self::new_empty();

        unsafe {
            for y in 0..HEIGHT {
                let src_row = self.as_ptr().add(y * WIDTH);
                let dst_row =
                    result.data.as_mut_ptr().add((HEIGHT - 1 - y) * WIDTH);
                core::ptr::copy_nonoverlapping(src_row, dst_row, WIDTH);
            }
        }

        result
    }
    #[must_use]
    /// Flip the `ConstBuffer` horizontally (left becomes right)
    pub fn flip_horizontally(&self) -> Self {
        let mut result = Self::new_empty();

        unsafe {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let dst_idx = y * WIDTH + (WIDTH - 1 - x);
                    *result.data.as_mut_ptr().add(dst_idx) =
                        *self.data.as_ptr().add(y * WIDTH + x);
                }
            }
        }

        result
    }
    #[must_use]
    /// Rotate the `ConstBuffer` 90°
    pub fn rotate_90(&self) -> ConstBuffer<HEIGHT, WIDTH>
    where
        [(); WIDTH * HEIGHT]:,
        [(); HEIGHT * WIDTH]:,
    {
        let mut result: ConstBuffer<HEIGHT, WIDTH> = ConstBuffer::new_empty();

        unsafe {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let src_pixel = *self.data.as_ptr().add(y * WIDTH + x);
                    // For 90° clockwise: new_x = old_y, new_y = width - 1 - old_x
                    let new_x = y;
                    let new_y = WIDTH - 1 - x;
                    let dst_idx = new_y * HEIGHT + new_x;
                    *result.data.as_mut_ptr().add(dst_idx) = src_pixel;
                }
            }
        }

        result
    }
    #[must_use]
    /// Rotate the `ConstBuffer` 180°
    pub fn rotate_180(&self) -> Self {
        let mut result = Self::new_empty();

        unsafe {
            for i in 0..Self::TOTAL_SIZE {
                let src_pixel = *self.data.as_ptr().add(i);
                let dst_idx = Self::TOTAL_SIZE - 1 - i;
                *result.data.as_mut_ptr().add(dst_idx) = src_pixel;
            }
        }

        result
    }
    #[must_use]
    /// Rotate the `ConstBuffer` 270° (or -90°)
    pub fn rotate_270(&self) -> ConstBuffer<HEIGHT, WIDTH>
    where
        [(); WIDTH * HEIGHT]:,
        [(); HEIGHT * WIDTH]:,
    {
        let mut result: ConstBuffer<HEIGHT, WIDTH> = ConstBuffer::new_empty();

        unsafe {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let src_pixel = *self.data.as_ptr().add(y * WIDTH + x);
                    let new_x = HEIGHT - 1 - y;
                    let new_y = x;
                    let dst_idx = new_y * HEIGHT + new_x;
                    *result.data.as_mut_ptr().add(dst_idx) = src_pixel;
                }
            }
        }

        result
    }
}
