#[cfg(feature = "num_traits")]
use crate::prelude::TryFromPatch;

/// Additional functions for `std::ops::Range` (val..val)
pub trait RangeExtension<T, F> {
    /// If the range goes from 10 to 20 and a 0.5 is inputted, 15 is returned
    fn get_value_from_percent(&self, percentage: F) -> Option<T>;
    /// If the range goes from 10 to 20 and a 15 is inputted, 0.5 is returned
    fn get_percent_from_value(&self, value: T) -> Option<F>;
}
#[cfg(feature = "num_traits")]
impl<T: TryFromPatch<F> + Copy, F: num_traits::Float + TryFromPatch<T>>
    RangeExtension<T, F> for std::ops::Range<T>
{
    fn get_value_from_percent(&self, percentage: F) -> Option<T> {
        T::try_from_value((F::try_from_value(self.end)? * percentage).round())
    }
    fn get_percent_from_value(&self, value: T) -> Option<F> {
        Some(F::try_from_value(value)? / (F::try_from_value(self.end)?))
    }
}
