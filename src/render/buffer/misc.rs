use super::Buffer;
use crate::{graphics::ColorManipulation, render::BufferGetPixel};
impl Buffer {
    #[must_use]
    #[cfg(feature = "font_support")]
    /// Create a buffer with some pre-placed text
    ///
    /// # Errors
    /// When the total visual text width or height exceeds [`usize::MAX`]
    pub fn new_with_text(
        string: &str,
        size: usize,
        font: &fontdue::Font,
        text_color: u32,
        background_color: u32,
        antialiased: Option<u8>,
    ) -> Option<Self> {
        use crate::extensions::*;
        let height = crate::render::get_text_height(string, size as f32, font);
        let width = crate::render::get_text_width(string, size as f32, font);
        let mut buffer = Self::new_empty_with_color(
            (height, width).try_tuple_into()?,
            background_color,
        );
        crate::render::draw_text_switch::<false>(
            &mut buffer,
            string,
            (0, 0),
            text_color,
            size as f32,
            font,
            antialiased,
        );

        Some(buffer)
    }
    /// Apply a simple filter to every pixel of the buffer
    pub fn apply_filter(&mut self, function: impl Fn(u32) -> u32) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel_unsafe(
                    (x, y),
                    function(self.get_pixel_unsafe((x, y))),
                );
            }
        }
    }
    #[must_use]
    #[inline(always)]
    /// Get the current size in a tuple
    pub const fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    /// A steepness of 15 and offset of 0.8 makes a nice looking icon (Rough estimates based on trail and error)
    pub fn fade_out_edges(&mut self, steepness: f32, offset: f32) {
        let cx = self.width as f32 / 2.0;
        let cy = self.height as f32 / 2.0;
        let max_dist = cx.hypot(cy);

        for y in 0..self.height {
            for x in 0..self.width {
                let dx = x as f32 - cx;
                let dy = y as f32 - cy;
                let dist = dx.hypot(dy) / max_dist;
                let fade = 1.0 - dist;
                let fade =
                    1.0 - crate::math::smooth_0_to_1(fade, steepness, offset);
                unsafe {
                    let color = *self.pointer().add(y * self.width + x);
                    *self.mut_pointer().add(y * self.width + x) = color
                        .with_alpha(crate::math::interpolate(
                            0_f32, 255_f32, fade,
                        ) as u32);
                };
            }
        }
    }
}
