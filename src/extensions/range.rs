use crate::extensions::FromPatch;

/// Additional functions for `std::ops::Range` (val..val)
pub trait RangeExtension<T, F> {
    /// If the range goes from 10 to 20 and a 0.5 is inputted, 15 is returned
    fn get_value_from_percent(&self, percentage: F) -> T;
    /// If the range goes from 10 to 20 and a 15 is inputted, 0.5 is returned
    fn get_percent_from_value(&self, value: T) -> F;
}

impl<T: FromPatch<F> + Copy, F: num_traits::Float + FromPatch<T>>
    RangeExtension<T, F> for std::ops::Range<T>
{
    fn get_value_from_percent(&self, percentage: F) -> T {
        T::from_value((F::from_value(self.end) * percentage).round())
    }
    fn get_percent_from_value(&self, value: T) -> F {
        F::from_value(value) / (F::from_value(self.end))
    }
}
