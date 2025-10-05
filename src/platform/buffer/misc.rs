use super::Buffer;
impl Buffer {
    /// Apply a simple filter to every pixel of the buffer
    pub fn apply_filter(&self, function: impl Fn(u32) -> u32) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel_unsafe(
                    (x, y),
                    function(self.get_pixel_unsafe((x, y))),
                );
            }
        }
    }
}
