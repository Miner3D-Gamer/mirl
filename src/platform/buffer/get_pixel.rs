use super::Buffer;
impl Buffer {
    /// Safely get the pixel color of the buffer at the specified x and y, returns 0 if the pixel is out of bounce
    /// For a custom return number use [get_pixel_fallback]
    /// For getting the pixel without bounds checking use [get_pixel_unsafe]
    #[inline(always)]
    pub fn get_pixel(&self, xy: (usize, usize)) -> u32 {
        if xy.0 >= self.width || xy.1 >= self.height {
            return 0;
        }
        let index = xy.1 * self.width + xy.0;
        unsafe {
            return *self.pointer.add(index);
        }
    }
    /// Safely get the pixel color of the buffer at the specified x and y, returns the fallback input if the pixel is out of bounce
    #[inline(always)]
    pub fn get_pixel_fallback(&self, xy: (usize, usize), fallback: u32) -> u32 {
        if xy.0 >= self.width || xy.1 >= self.height {
            return fallback;
        }
        let index = xy.1 * self.width + xy.0;
        unsafe {
            return *self.pointer.add(index);
        }
    }
    /// Get the pixel color at a position in a buffer without checking if the pixel is on screen (which will crash the program if it isn't)
    /// The function for getting a pixel safely is [get_pixel] or [get_pixel_isize]
    #[inline(always)]
    pub fn get_pixel_unsafe(&self, xy: (usize, usize)) -> u32 {
        let index = xy.1 * self.width + xy.0;
        unsafe {
            return *self.pointer.add(index);
        }
    }
    /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
    #[inline(always)]
    pub fn get_pixel_isize(&self, xy: (isize, isize)) -> Option<u32> {
        if xy.0 < 0 || xy.1 < 0 {
            return None;
        }
        let _y = xy.1 as usize;
        let _x = xy.0 as usize;
        if _x >= self.width || _y >= self.height {
            return None;
        }
        let index = _y * self.width + _x;
        return Some(self.data[index]);
    }
    /// Get the pixel color at a position in a buffer yet before that check if it is in the range of the buffer
    /// Instead of returning None if the result isn't in the buffer, it will return the specified fallback value
    #[inline(always)]
    pub fn get_pixel_isize_fallback(
        &self,
        x: isize,
        y: isize,
        fallback: u32,
    ) -> u32 {
        if x < 0 || y < 0 {
            return fallback;
        }
        let _y = y as usize;
        let _x = x as usize;
        if _x >= self.width || _y >= self.height {
            return fallback;
        }
        let index = _y as usize * self.width + _x;
        return self.data[index];
    }
}
