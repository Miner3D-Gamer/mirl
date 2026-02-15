#[derive(Debug, Clone, Copy, PartialEq, Default, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A camera that is bound to a region
pub struct ScrollableCamera {
    /// The size of the horizontally scrollable region
    pub container_width: f32,
    /// The size of the vertically scrollable region
    pub container_height: f32,
    /// The size of the "thing" moving horizontally in the scrollable region
    pub content_width: f32,
    /// The size of the "thing" moving vertically in the scrollable region
    pub content_height: f32,
    /// The camera offset in the X direction
    pub offset_x: f32,
    /// The camera offset in the Y direction
    pub offset_y: f32,
    /// When scrolling, by what should the incoming scroll be affected horizontally
    pub scroll_multiplier_x: f32,
    /// When scrolling, by what should the incoming scroll be affected vertically
    pub scroll_multiplier_y: f32,
    /// When scrolling horizontally instead of vertically, should the scroll multipliers be switched (`x_mul` -> `y_mul`, `y_mul` -> `x_mul`)
    pub horizontal_context_switch_multipliers: bool,
    /// If the content inside is also scrollable when the bounds are bigger than the content
    pub allow_free_scroll: bool,
}
impl ScrollableCamera {
    /// Scroll within the contained region
    pub fn scroll(&mut self, by: (f32, f32), vertical: bool) {
        if vertical {
            self.offset_x += by.0 * self.scroll_multiplier_x;
            self.offset_y += by.1 * self.scroll_multiplier_y;
        } else if !self.horizontal_context_switch_multipliers {
            self.offset_x += by.1 * self.scroll_multiplier_y;
            self.offset_y += by.0 * self.scroll_multiplier_x;
        } else {
            self.offset_x += by.1 * self.scroll_multiplier_x;
            self.offset_y += by.0 * self.scroll_multiplier_y;
        }

        self.clamp_to_bounds();
    }
    #[allow(clippy::tuple_array_conversions)]
    /// Clamp the content to the bounds of the defined region
    pub fn clamp_to_bounds(&mut self) {
        let y_range = if self.allow_free_scroll {
            [self.container_height - self.content_height, 0.0]
        } else {
            [(self.container_height - self.content_height).min(0.0), 0.0]
        };

        let (y_min, y_max) = if y_range[0] <= y_range[1] {
            (y_range[0], y_range[1])
        } else {
            (y_range[1], y_range[0])
        };
        self.offset_y = self.offset_y.clamp(y_min, y_max);

        let x_range = if self.allow_free_scroll {
            [self.container_width - self.content_width, 0.0]
        } else {
            [(self.container_width - self.content_width).min(0.0), 0.0]
        };
        let (x_min, x_max) = if x_range[0] <= x_range[1] {
            (x_range[0], x_range[1])
        } else {
            (x_range[1], x_range[0])
        };
        self.offset_x = self.offset_x.clamp(x_min, x_max);
    }
}
