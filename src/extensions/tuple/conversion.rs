#![allow(clippy::type_complexity)]
use crate::extensions::FromPatch;

/// Converts a tuple of size 1 into a another tuple of size 1 yet with a different unified output
pub trait Tuple1Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T,);
}
impl<Target, Source> Tuple1Into<Target> for (Source,)
where
    Target: FromPatch<Source>,
{
    fn tuple_into(self) -> (Target,) {
        (Target::from_value(self.0),)
    }
}
/// Converts a tuple of size 2 into a another tuple of size 2 yet with a different unified output
pub trait Tuple2Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T);
}
impl<Target, Source0, Source1> Tuple2Into<Target> for (Source0, Source1)
where
    Target: FromPatch<Source0> + FromPatch<Source1>,
{
    fn tuple_into(self) -> (Target, Target) {
        (Target::from_value(self.0), Target::from_value(self.1))
    }
}
/// Converts a tuple of size 3 into a another tuple of size 3 yet with a different unified output
pub trait Tuple3Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T);
}
impl<Target, Source0, Source1, Source2> Tuple3Into<Target>
    for (Source0, Source1, Source2)
where
    Target: FromPatch<Source0> + FromPatch<Source1> + FromPatch<Source2>,
{
    fn tuple_into(self) -> (Target, Target, Target) {
        (
            Target::from_value(self.0),
            Target::from_value(self.1),
            Target::from_value(self.2),
        )
    }
}

/// Converts a tuple of size 4 into a another tuple of size 4 yet with a different unified output
pub trait Tuple4Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T);
}
impl<Target, Source0, Source1, Source2, Source3> Tuple4Into<Target>
    for (Source0, Source1, Source2, Source3)
where
    Target: FromPatch<Source0>
        + FromPatch<Source1>
        + FromPatch<Source2>
        + FromPatch<Source3>,
{
    fn tuple_into(self) -> (Target, Target, Target, Target) {
        (
            Target::from_value(self.0),
            Target::from_value(self.1),
            Target::from_value(self.2),
            Target::from_value(self.3),
        )
    }
}

/// Converts a tuple of size 5 into a another tuple of size 5 yet with a different unified output
pub trait Tuple5Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T);
}
impl<Target, Source0, Source1, Source2, Source3, Source4> Tuple5Into<Target>
    for (Source0, Source1, Source2, Source3, Source4)
where
    Target: FromPatch<Source0>
        + FromPatch<Source1>
        + FromPatch<Source2>
        + FromPatch<Source3>
        + FromPatch<Source4>,
{
    fn tuple_into(self) -> (Target, Target, Target, Target, Target) {
        (
            Target::from_value(self.0),
            Target::from_value(self.1),
            Target::from_value(self.2),
            Target::from_value(self.3),
            Target::from_value(self.4),
        )
    }
}

/// Converts a tuple of size 6 into a another tuple of size 6 yet with a different unified output
pub trait Tuple6Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T, T);
}
impl<Target, Source0, Source1, Source2, Source3, Source4, Source5>
    Tuple6Into<Target>
    for (Source0, Source1, Source2, Source3, Source4, Source5)
where
    Target: FromPatch<Source0>
        + FromPatch<Source1>
        + FromPatch<Source2>
        + FromPatch<Source3>
        + FromPatch<Source4>
        + FromPatch<Source5>,
{
    fn tuple_into(self) -> (Target, Target, Target, Target, Target, Target) {
        (
            Target::from_value(self.0),
            Target::from_value(self.1),
            Target::from_value(self.2),
            Target::from_value(self.3),
            Target::from_value(self.4),
            Target::from_value(self.5),
        )
    }
}

/// Converts a tuple of size 7 into a another tuple of size 7 yet with a different unified output
pub trait Tuple7Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T, T, T);
}
impl<Target, Source0, Source1, Source2, Source3, Source4, Source5, Source6>
    Tuple7Into<Target>
    for (Source0, Source1, Source2, Source3, Source4, Source5, Source6)
where
    Target: FromPatch<Source0>
        + FromPatch<Source1>
        + FromPatch<Source2>
        + FromPatch<Source3>
        + FromPatch<Source4>
        + FromPatch<Source5>
        + FromPatch<Source6>,
{
    fn tuple_into(
        self,
    ) -> (Target, Target, Target, Target, Target, Target, Target) {
        (
            Target::from_value(self.0),
            Target::from_value(self.1),
            Target::from_value(self.2),
            Target::from_value(self.3),
            Target::from_value(self.4),
            Target::from_value(self.5),
            Target::from_value(self.6),
        )
    }
}

/// Converts a tuple of size 8 into a another tuple of size 8 yet with a different unified output
pub trait Tuple8Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T, T, T, T);
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
    Target: FromPatch<Source0>
        + FromPatch<Source1>
        + FromPatch<Source2>
        + FromPatch<Source3>
        + FromPatch<Source4>
        + FromPatch<Source5>
        + FromPatch<Source6>
        + FromPatch<Source7>,
{
    fn tuple_into(
        self,
    ) -> (Target, Target, Target, Target, Target, Target, Target, Target) {
        (
            Target::from_value(self.0),
            Target::from_value(self.1),
            Target::from_value(self.2),
            Target::from_value(self.3),
            Target::from_value(self.4),
            Target::from_value(self.5),
            Target::from_value(self.6),
            Target::from_value(self.7),
        )
    }
}

/// Converts a tuple of size 9 into a another tuple of size 9 yet with a different unified output
pub trait Tuple9Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T, T, T, T, T);
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
    Target: FromPatch<Source0>
        + FromPatch<Source1>
        + FromPatch<Source2>
        + FromPatch<Source3>
        + FromPatch<Source4>
        + FromPatch<Source5>
        + FromPatch<Source6>
        + FromPatch<Source7>
        + FromPatch<Source8>,
{
    fn tuple_into(
        self,
    ) -> (Target, Target, Target, Target, Target, Target, Target, Target, Target)
    {
        (
            Target::from_value(self.0),
            Target::from_value(self.1),
            Target::from_value(self.2),
            Target::from_value(self.3),
            Target::from_value(self.4),
            Target::from_value(self.5),
            Target::from_value(self.6),
            Target::from_value(self.7),
            Target::from_value(self.8),
        )
    }
}
/// Converts a tuple of size 10 into a another tuple of size 10 yet with a different unified output
pub trait Tuple10Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T, T, T, T, T, T);
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
    Target: FromPatch<Source0>
        + FromPatch<Source1>
        + FromPatch<Source2>
        + FromPatch<Source3>
        + FromPatch<Source4>
        + FromPatch<Source5>
        + FromPatch<Source6>
        + FromPatch<Source7>
        + FromPatch<Source8>
        + FromPatch<Source9>,
{
    fn tuple_into(
        self,
    ) -> (
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
    ) {
        (
            Target::from_value(self.0),
            Target::from_value(self.1),
            Target::from_value(self.2),
            Target::from_value(self.3),
            Target::from_value(self.4),
            Target::from_value(self.5),
            Target::from_value(self.6),
            Target::from_value(self.7),
            Target::from_value(self.8),
            Target::from_value(self.9),
        )
    }
}

/// Converts a tuple of size 11 into a another tuple of size 11 yet with a different unified output
pub trait Tuple11Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T, T, T, T, T, T, T);
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
    Target: FromPatch<Source0>
        + FromPatch<Source1>
        + FromPatch<Source2>
        + FromPatch<Source3>
        + FromPatch<Source4>
        + FromPatch<Source5>
        + FromPatch<Source6>
        + FromPatch<Source7>
        + FromPatch<Source8>
        + FromPatch<Source9>
        + FromPatch<Source10>,
{
    fn tuple_into(
        self,
    ) -> (
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
    ) {
        (
            Target::from_value(self.0),
            Target::from_value(self.1),
            Target::from_value(self.2),
            Target::from_value(self.3),
            Target::from_value(self.4),
            Target::from_value(self.5),
            Target::from_value(self.6),
            Target::from_value(self.7),
            Target::from_value(self.8),
            Target::from_value(self.9),
            Target::from_value(self.10),
        )
    }
}

/// Converts a tuple of size 12 into a another tuple of size 12 yet with a different unified output
pub trait Tuple12Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T, T, T, T, T, T, T, T);
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
    Target: FromPatch<Source0>
        + FromPatch<Source1>
        + FromPatch<Source2>
        + FromPatch<Source3>
        + FromPatch<Source4>
        + FromPatch<Source5>
        + FromPatch<Source6>
        + FromPatch<Source7>
        + FromPatch<Source8>
        + FromPatch<Source9>
        + FromPatch<Source10>
        + FromPatch<Source11>,
{
    fn tuple_into(
        self,
    ) -> (
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
    ) {
        (
            Target::from_value(self.0),
            Target::from_value(self.1),
            Target::from_value(self.2),
            Target::from_value(self.3),
            Target::from_value(self.4),
            Target::from_value(self.5),
            Target::from_value(self.6),
            Target::from_value(self.7),
            Target::from_value(self.8),
            Target::from_value(self.9),
            Target::from_value(self.10),
            Target::from_value(self.11),
        )
    }
}
