#![allow(clippy::type_complexity)]
use crate::extensions::TryFromPatch;

/// Converts a tuple of size 1 into a another tuple of size 1 yet with a different unified output
pub trait Tuple1Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T,)>;
}
impl<Target, Source> Tuple1Into<Target> for (Source,)
where
    Target: TryFromPatch<Source>,
{
    fn try_tuple_into(self) -> Option<(Target,)> {
        Some((Target::try_from_value(self.0)?,))
    }
}
/// Converts a tuple of size 2 into a another tuple of size 2 yet with a different unified output
pub trait Tuple2Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T)>;
}
impl<Target, Source0, Source1> Tuple2Into<Target> for (Source0, Source1)
where
    Target: TryFromPatch<Source0> + TryFromPatch<Source1>,
{
    fn try_tuple_into(self) -> Option<(Target, Target)> {
        Some((Target::try_from_value(self.0)?, Target::try_from_value(self.1)?))
    }
}
/// Converts a tuple of size 3 into a another tuple of size 3 yet with a different unified output
pub trait Tuple3Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T)>;
}
impl<Target, Source0, Source1, Source2> Tuple3Into<Target>
    for (Source0, Source1, Source2)
where
    Target: TryFromPatch<Source0> + TryFromPatch<Source1> + TryFromPatch<Source2>,
{
    fn try_tuple_into(self) -> Option<(Target, Target, Target)> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
        ))
    }
}

/// Converts a tuple of size 4 into a another tuple of size 4 yet with a different unified output
pub trait Tuple4Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T)>;
}
impl<Target, Source0, Source1, Source2, Source3> Tuple4Into<Target>
    for (Source0, Source1, Source2, Source3)
where
    Target: TryFromPatch<Source0>
        + TryFromPatch<Source1>
        + TryFromPatch<Source2>
        + TryFromPatch<Source3>,
{
    fn try_tuple_into(self) -> Option<(Target, Target, Target, Target)> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
            Target::try_from_value(self.3)?,
        ))
    }
}

/// Converts a tuple of size 5 into a another tuple of size 5 yet with a different unified output
pub trait Tuple5Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T)>;
}
impl<Target, Source0, Source1, Source2, Source3, Source4> Tuple5Into<Target>
    for (Source0, Source1, Source2, Source3, Source4)
where
    Target: TryFromPatch<Source0>
        + TryFromPatch<Source1>
        + TryFromPatch<Source2>
        + TryFromPatch<Source3>
        + TryFromPatch<Source4>,
{
    fn try_tuple_into(self) -> Option<(Target, Target, Target, Target, Target)> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
            Target::try_from_value(self.3)?,
            Target::try_from_value(self.4)?,
        ))
    }
}

/// Converts a tuple of size 6 into a another tuple of size 6 yet with a different unified output
pub trait Tuple6Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T, T)>;
}
impl<Target, Source0, Source1, Source2, Source3, Source4, Source5>
    Tuple6Into<Target>
    for (Source0, Source1, Source2, Source3, Source4, Source5)
where
    Target: TryFromPatch<Source0>
        + TryFromPatch<Source1>
        + TryFromPatch<Source2>
        + TryFromPatch<Source3>
        + TryFromPatch<Source4>
        + TryFromPatch<Source5>,
{
    fn try_tuple_into(self) -> Option<(Target, Target, Target, Target, Target, Target)> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
            Target::try_from_value(self.3)?,
            Target::try_from_value(self.4)?,
            Target::try_from_value(self.5)?,
        ))
    }
}

/// Converts a tuple of size 7 into a another tuple of size 7 yet with a different unified output
pub trait Tuple7Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T, T, T)>;
}
impl<Target, Source0, Source1, Source2, Source3, Source4, Source5, Source6>
    Tuple7Into<Target>
    for (Source0, Source1, Source2, Source3, Source4, Source5, Source6)
where
    Target: TryFromPatch<Source0>
        + TryFromPatch<Source1>
        + TryFromPatch<Source2>
        + TryFromPatch<Source3>
        + TryFromPatch<Source4>
        + TryFromPatch<Source5>
        + TryFromPatch<Source6>,
{
    fn try_tuple_into(
        self,
    ) -> Option<(Target, Target, Target, Target, Target, Target, Target)> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
            Target::try_from_value(self.3)?,
            Target::try_from_value(self.4)?,
            Target::try_from_value(self.5)?,
            Target::try_from_value(self.6)?,
        ))
    }
}

/// Converts a tuple of size 8 into a another tuple of size 8 yet with a different unified output
pub trait Tuple8Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T, T, T, T)>;
}
impl<
        Target,
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
    > Tuple8Into<Target>
    for (Source0, Source1, Source2, Source3, Source4, Source5, Source6, Source7)
where
    Target: TryFromPatch<Source0>
        + TryFromPatch<Source1>
        + TryFromPatch<Source2>
        + TryFromPatch<Source3>
        + TryFromPatch<Source4>
        + TryFromPatch<Source5>
        + TryFromPatch<Source6>
        + TryFromPatch<Source7>,
{
    fn try_tuple_into(
        self,
    ) -> Option<(Target, Target, Target, Target, Target, Target, Target, Target)> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
            Target::try_from_value(self.3)?,
            Target::try_from_value(self.4)?,
            Target::try_from_value(self.5)?,
            Target::try_from_value(self.6)?,
            Target::try_from_value(self.7)?,
        ))
    }
}

/// Converts a tuple of size 9 into a another tuple of size 9 yet with a different unified output
pub trait Tuple9Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T, T, T, T, T)>;
}
impl<
        Target,
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
    > Tuple9Into<Target>
    for (
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
    )
where
    Target: TryFromPatch<Source0>
        + TryFromPatch<Source1>
        + TryFromPatch<Source2>
        + TryFromPatch<Source3>
        + TryFromPatch<Source4>
        + TryFromPatch<Source5>
        + TryFromPatch<Source6>
        + TryFromPatch<Source7>
        + TryFromPatch<Source8>,
{
    fn try_tuple_into(
        self,
    ) -> Option<(Target, Target, Target, Target, Target, Target, Target, Target, Target)>
    {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
            Target::try_from_value(self.3)?,
            Target::try_from_value(self.4)?,
            Target::try_from_value(self.5)?,
            Target::try_from_value(self.6)?,
            Target::try_from_value(self.7)?,
            Target::try_from_value(self.8)?,
        ))
    }
}
/// Converts a tuple of size 10 into a another tuple of size 10 yet with a different unified output
pub trait Tuple10Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T, T, T, T, T, T)>;
}
impl<
        Target,
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
        Source9,
    > Tuple10Into<Target>
    for (
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
        Source9,
    )
where
    Target: TryFromPatch<Source0>
        + TryFromPatch<Source1>
        + TryFromPatch<Source2>
        + TryFromPatch<Source3>
        + TryFromPatch<Source4>
        + TryFromPatch<Source5>
        + TryFromPatch<Source6>
        + TryFromPatch<Source7>
        + TryFromPatch<Source8>
        + TryFromPatch<Source9>,
{
    fn try_tuple_into(
        self,
    ) -> Option<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
            Target::try_from_value(self.3)?,
            Target::try_from_value(self.4)?,
            Target::try_from_value(self.5)?,
            Target::try_from_value(self.6)?,
            Target::try_from_value(self.7)?,
            Target::try_from_value(self.8)?,
            Target::try_from_value(self.9)?,
        ))
    }
}

/// Converts a tuple of size 11 into a another tuple of size 11 yet with a different unified output
pub trait Tuple11Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T, T, T, T, T, T, T)>;
}
impl<
        Target,
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
        Source9,
        Source10,
    > Tuple11Into<Target>
    for (
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
        Source9,
        Source10,
    )
where
    Target: TryFromPatch<Source0>
        + TryFromPatch<Source1>
        + TryFromPatch<Source2>
        + TryFromPatch<Source3>
        + TryFromPatch<Source4>
        + TryFromPatch<Source5>
        + TryFromPatch<Source6>
        + TryFromPatch<Source7>
        + TryFromPatch<Source8>
        + TryFromPatch<Source9>
        + TryFromPatch<Source10>,
{
    fn try_tuple_into(
        self,
    ) -> Option<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
            Target::try_from_value(self.3)?,
            Target::try_from_value(self.4)?,
            Target::try_from_value(self.5)?,
            Target::try_from_value(self.6)?,
            Target::try_from_value(self.7)?,
            Target::try_from_value(self.8)?,
            Target::try_from_value(self.9)?,
            Target::try_from_value(self.10)?,
        ))
    }
}

/// Converts a tuple of size 12 into a another tuple of size 12 yet with a different unified output
pub trait Tuple12Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T, T, T, T, T, T, T, T)>;
}
impl<
        Target,
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
        Source9,
        Source10,
        Source11,
    > Tuple12Into<Target>
    for (
        Source0,
        Source1,
        Source2,
        Source3,
        Source4,
        Source5,
        Source6,
        Source7,
        Source8,
        Source9,
        Source10,
        Source11,
    )
where
    Target: TryFromPatch<Source0>
        + TryFromPatch<Source1>
        + TryFromPatch<Source2>
        + TryFromPatch<Source3>
        + TryFromPatch<Source4>
        + TryFromPatch<Source5>
        + TryFromPatch<Source6>
        + TryFromPatch<Source7>
        + TryFromPatch<Source8>
        + TryFromPatch<Source9>
        + TryFromPatch<Source10>
        + TryFromPatch<Source11>,
{
    fn try_tuple_into(
        self,
    ) -> Option<(
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
        Target,
    )> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
            Target::try_from_value(self.3)?,
            Target::try_from_value(self.4)?,
            Target::try_from_value(self.5)?,
            Target::try_from_value(self.6)?,
            Target::try_from_value(self.7)?,
            Target::try_from_value(self.8)?,
            Target::try_from_value(self.9)?,
            Target::try_from_value(self.10)?,
            Target::try_from_value(self.11)?,
        ))
    }
}