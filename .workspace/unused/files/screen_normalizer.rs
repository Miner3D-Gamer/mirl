#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// A struct to convert between 0.0-1.0 and the metrics of the screen
pub struct ScreenNormalizer<S> {
    screen_width: S,
    screen_height: S,
}
impl<S: Copy + core::ops::Mul<Output = S> + core::ops::Div<Output = S>> ScreenNormalizer<S> {
    /// Recommended is using [`crate::math::UniformRange`] in conjunction with this struct
    ///
    /// # Panics
    /// If the dimensions are invalid
    pub const fn new(screen_size: (S, S)) -> Self {
        Self {
            screen_width: screen_size.0,
            screen_height: screen_size.1,
        }
    }
    /// Convert a percentage into screen a coordinate horizontally
    pub fn try_percentile_to_x<T>(&self, p: S) -> Option<T>
    where
        S: TryInto<T>,
    {
        (p * self.screen_width).try_into().ok()
    }
    /// Convert a percentage into screen a coordinate vertically
    pub fn try_percentile_to_y<T>(&self, p: S) -> Option<T>
    where
        S: TryInto<T>,
    {
        (p * self.screen_height).try_into().ok()
    }
    /// Convert a percentage into screen a coordinate
    pub fn try_percentile_to_xy<T>(&self, pxy: (S, S)) -> Option<(T, T)>
    where
        S: TryInto<T>,
    {
        let x = (pxy.0 * self.screen_width).try_into().ok()?;
        let y = (pxy.1 * self.screen_height).try_into().ok()?;
        Some((x, y))
    }
    /// Convert a horizontal screen coordinate into a percentage
    pub fn try_x_to_percentile<T: TryInto<S>>(&self, x: T) -> Option<S> {
        if let Some(value) = (x).try_into().ok() {
            return Some(value / self.screen_width);
        }
        core::intrinsics::cold_path();
        None
    }
    /// Convert a vertical screen coordinate into a percentage
    pub fn try_y_to_percentile<T: TryInto<S>>(&self, y: T) -> Option<S> {
        if let Some(value) = y.try_into().ok() {
            return Some(value / self.screen_height);
        }
        core::intrinsics::cold_path();
        None
    }

    /// Convert a percentage into screen a coordinate horizontally
    pub fn percentile_to_x<T>(&self, p: S) -> T
    where
        S: Into<T>,
    {
        (p * self.screen_width).into()
    }
    /// Convert a percentage into screen a coordinate vertically
    pub fn percentile_to_y<T>(&self, p: S) -> T
    where
        S: Into<T>,
    {
        (p * self.screen_height).into()
    }
    /// Convert a percentage into screen a coordinate
    pub fn percentile_to_xy<T>(&self, pxy: (S, S)) -> (T, T)
    where
        S: Into<T>,
    {
        let x = (pxy.0 * self.screen_width).into();
        let y = (pxy.1 * self.screen_height).into();
        (x, y)
    }
    /// Convert a horizontal screen coordinate into a percentage
    pub fn x_to_percentile<T: Into<S>>(&self, x: T) -> S {
        (x).into() / self.screen_width
    }
    /// Convert a vertical screen coordinate into a percentage
    pub fn y_to_percentile<T: Into<S>>(&self, y: T) -> S {
        y.into() / self.screen_height
    }
}
