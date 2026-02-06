use crate::render::BufferGetPixel;

use super::Buffer;
impl Buffer {
    #[allow(clippy::cast_precision_loss)]
    /// A helper function to be used inside a `execute_at` render function
    ///
    /// Inverts the color at the given coordinates
    pub fn invert_color_below<const SAFE: bool>(
        &mut self,
        xy: (usize, usize),
        color: u32,
    ) {
        let old = if SAFE {
            let Some(old) = self.get_pixel_option(xy) else {
                return;
            };
            old
        } else {
            self.get_pixel_unsafe(xy)
        };
        let inverted = crate::graphics::invert_color(old);
        let new = crate::graphics::interpolate_color_rgb_u32_f32(
            inverted,
            old,
            crate::graphics::get_alpha_of_u32(color) as f32 / 255.0,
        );

        crate::render::draw_pixel_unsafe(self, xy, new);
    }
    /// A helper function to be used inside a `execute_at` render function
    ///
    /// Inverts the color below if it matches the input number
    pub const fn invert_color_if_same<const SAFE: bool>(
        &mut self,
        xy: (usize, usize),
        color: u32,
    ) {
        let old = if SAFE {
            let Some(old) = self.get_pixel_option(xy) else {
                return;
            };
            old
        } else {
            self.get_pixel_unsafe(xy)
        };
        if old == color {
            let inverted = crate::graphics::invert_color(old);

            crate::render::draw_pixel_unsafe(self, xy, inverted);
        }
        crate::render::draw_pixel_unsafe(self, xy, color);
    }
}
