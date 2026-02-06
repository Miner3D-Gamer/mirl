#![allow(clippy::type_complexity)]
use crate::extensions::FromPatch;

/// Converts a tuple of size 1 into a another tuple of size 1 yet with a different unified output
pub const trait Tuple1Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T,);
}

impl<Target, Source> const Tuple1Into<Target> for (Source,)
where
    Source: Copy,
    Target: Copy + [const] FromPatch<Source>,
{
    fn tuple_into(self) -> (Target,) {
        (Target::from_value(self.0),)
    }
}

/// Converts a tuple of size 2 into a another tuple of size 2 yet with a different unified output
pub const trait Tuple2Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T);
}

impl<Target, Source0, Source1> const Tuple2Into<Target>
    for (Source0, Source1)
where
    Source0: Copy,
    Source1: Copy,
    Target:
        Copy + [const] FromPatch<Source0> + [const] FromPatch<Source1>,
{
    fn tuple_into(self) -> (Target, Target) {
        (Target::from_value(self.0), Target::from_value(self.1))
    }
}

/// Converts a tuple of size 3 into a another tuple of size 3 yet with a different unified output
pub const trait Tuple3Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T);
}

impl<Target, Source0, Source1, Source2> const Tuple3Into<Target>
    for (Source0, Source1, Source2)
where
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Target: Copy
        + [const] FromPatch<Source0>
        + [const] FromPatch<Source1>
        + [const] FromPatch<Source2>,
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
pub const trait Tuple4Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T);
}

impl<Target, Source0, Source1, Source2, Source3> const Tuple4Into<Target>
    for (Source0, Source1, Source2, Source3)
where
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Target: Copy
        + [const] FromPatch<Source0>
        + [const] FromPatch<Source1>
        + [const] FromPatch<Source2>
        + [const] FromPatch<Source3>,
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
pub const trait Tuple5Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T);
}

impl<Target, Source0, Source1, Source2, Source3, Source4> const
    Tuple5Into<Target> for (Source0, Source1, Source2, Source3, Source4)
where
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Source4: Copy,
    Target: Copy
        + [const] FromPatch<Source0>
        + [const] FromPatch<Source1>
        + [const] FromPatch<Source2>
        + [const] FromPatch<Source3>
        + [const] FromPatch<Source4>,
{
    fn tuple_into(
        self,
    ) -> (Target, Target, Target, Target, Target) {
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
pub const trait Tuple6Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T, T);
}

impl<Target, Source0, Source1, Source2, Source3, Source4, Source5> const
    Tuple6Into<Target>
    for (Source0, Source1, Source2, Source3, Source4, Source5)
where
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Source4: Copy,
    Source5: Copy,
    Target: Copy
        + [const] FromPatch<Source0>
        + [const] FromPatch<Source1>
        + [const] FromPatch<Source2>
        + [const] FromPatch<Source3>
        + [const] FromPatch<Source4>
        + [const] FromPatch<Source5>,
{
    fn tuple_into(
        self,
    ) -> (Target, Target, Target, Target, Target, Target) {
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
pub const trait Tuple7Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(self) -> (T, T, T, T, T, T, T);
}

impl<Target, Source0, Source1, Source2, Source3, Source4, Source5, Source6> const
    Tuple7Into<Target>
    for (Source0, Source1, Source2, Source3, Source4, Source5, Source6)
where
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Source4: Copy,
    Source5: Copy,
    Source6: Copy,
    Target: Copy
        + [const] FromPatch<Source0>
        + [const] FromPatch<Source1>
        + [const] FromPatch<Source2>
        + [const] FromPatch<Source3>
        + [const] FromPatch<Source4>
        + [const] FromPatch<Source5>
        + [const] FromPatch<Source6>,
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
pub const trait Tuple8Into<T> {
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
    > const Tuple8Into<Target>
    for (Source0, Source1, Source2, Source3, Source4, Source5, Source6, Source7)
where
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Source4: Copy,
    Source5: Copy,
    Source6: Copy,
    Source7: Copy,
    Target: Copy
        + [const] FromPatch<Source0>
        + [const] FromPatch<Source1>
        + [const] FromPatch<Source2>
        + [const] FromPatch<Source3>
        + [const] FromPatch<Source4>
        + [const] FromPatch<Source5>
        + [const] FromPatch<Source6>
        + [const] FromPatch<Source7>,
{
    fn tuple_into(
        self,
    ) -> (Target, Target, Target, Target, Target, Target, Target, Target)
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
        )
    }
}

/// Converts a tuple of size 9 into a another tuple of size 9 yet with a different unified output
pub const trait Tuple9Into<T> {
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
    > const Tuple9Into<Target>
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
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Source4: Copy,
    Source5: Copy,
    Source6: Copy,
    Source7: Copy,
    Source8: Copy,
    Target: Copy
        + [const] FromPatch<Source0>
        + [const] FromPatch<Source1>
        + [const] FromPatch<Source2>
        + [const] FromPatch<Source3>
        + [const] FromPatch<Source4>
        + [const] FromPatch<Source5>
        + [const] FromPatch<Source6>
        + [const] FromPatch<Source7>
        + [const] FromPatch<Source8>,
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
        )
    }
}

/// Converts a tuple of size 10 into a another tuple of size 10 yet with a different unified output
pub const trait Tuple10Into<T> {
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
    > const Tuple10Into<Target>
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
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Source4: Copy,
    Source5: Copy,
    Source6: Copy,
    Source7: Copy,
    Source8: Copy,
    Source9: Copy,
    Target: Copy
        + [const] FromPatch<Source0>
        + [const] FromPatch<Source1>
        + [const] FromPatch<Source2>
        + [const] FromPatch<Source3>
        + [const] FromPatch<Source4>
        + [const] FromPatch<Source5>
        + [const] FromPatch<Source6>
        + [const] FromPatch<Source7>
        + [const] FromPatch<Source8>
        + [const] FromPatch<Source9>,
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
pub const trait Tuple11Into<T> {
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
    > const Tuple11Into<Target>
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
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Source4: Copy,
    Source5: Copy,
    Source6: Copy,
    Source7: Copy,
    Source8: Copy,
    Source9: Copy,
    Source10: Copy,
    Target: Copy
        + [const] FromPatch<Source0>
        + [const] FromPatch<Source1>
        + [const] FromPatch<Source2>
        + [const] FromPatch<Source3>
        + [const] FromPatch<Source4>
        + [const] FromPatch<Source5>
        + [const] FromPatch<Source6>
        + [const] FromPatch<Source7>
        + [const] FromPatch<Source8>
        + [const] FromPatch<Source9>
        + [const] FromPatch<Source10>,
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
pub const trait Tuple12Into<T> {
    /// The tuple version of `into()`
    fn tuple_into(
        self,
    ) -> (T, T, T, T, T, T, T, T, T, T, T, T);
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
    > const Tuple12Into<Target>
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
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Source4: Copy,
    Source5: Copy,
    Source6: Copy,
    Source7: Copy,
    Source8: Copy,
    Source9: Copy,
    Source10: Copy,
    Source11: Copy,
    Target: Copy
        + [const] FromPatch<Source0>
        + [const] FromPatch<Source1>
        + [const] FromPatch<Source2>
        + [const] FromPatch<Source3>
        + [const] FromPatch<Source4>
        + [const] FromPatch<Source5>
        + [const] FromPatch<Source6>
        + [const] FromPatch<Source7>
        + [const] FromPatch<Source8>
        + [const] FromPatch<Source9>
        + [const] FromPatch<Source10>
        + [const] FromPatch<Source11>,
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
