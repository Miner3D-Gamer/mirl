/// The inverse of [`FromPatch`]
pub const trait IntoPatch<T>: Sized {
    #[must_use]
    /// A custom from to bypass rusts orphan rule
    fn into_value(self) -> T;
}
impl<T: Sized + [const] FromPatch<O>, O> const IntoPatch<T> for O {
    fn into_value(self) -> T {
        T::from_value(self)
    }
}

/// Lets you convert from one value to another.
///
/// What's the difference between this and `core::convert::From`?
/// `core::convert::From` has many holes like being unable to convert a i32 to f32.
/// This trait "patches" the `core::convert::From` by recreating all safe existing conversions and extending on the original concept
pub const trait FromPatch<T>: Sized {
    #[must_use]
    /// A custom from to bypass rusts orphan rule
    fn from_value(value: T) -> Self;
}
mod numbers;
