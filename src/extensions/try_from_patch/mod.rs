/// The inverse of [`TryFromPatch`]
pub const trait TryIntoPatch<T>: Sized {
    #[must_use]
    /// A custom from to bypass rusts orphan rule
    fn try_into_value(self) -> Option<T>;
}
impl<T: Sized + TryFromPatch<O>, O> TryIntoPatch<T> for O {
    fn try_into_value(self) -> Option<T> {
        T::try_from_value(self)
    }
}

/// Lets you convert from one value to another.
///
/// What's the difference between this and `std::convert::TryFrom`?
/// `std::convert::TryFrom` has many holes covered by `std::convert::From`, yet having 2 traits inconveniences things.
/// This trait "patches" the `std::convert::TryFrom` by combining both implementations
pub const trait TryFromPatch<T>: Sized {
    #[must_use]
    /// A custom from to bypass rusts orphan rule
    fn try_from_value(value: T) -> Option<Self>;
}

#[cfg(feature = "std")]
mod map;
mod numbers;
#[cfg(feature = "std")]
mod string;
#[cfg(feature = "std")]
mod vec;
