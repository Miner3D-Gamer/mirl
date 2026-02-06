#![allow(clippy::type_complexity)]
use crate::extensions::TryFromPatch;

/// Converts a tuple of size 1 into a another tuple of size 1 yet with a different unified output
pub const trait TryTuple1Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T,)>;
}

impl<Target, Source> const TryTuple1Into<Target> for (Source,)
where
    Source: Copy,
    Target: Copy + [const] TryFromPatch<Source>,
{
    fn try_tuple_into(self) -> Option<(Target,)> {
        Some((Target::try_from_value(self.0)?,))
    }
}

/// Converts a tuple of size 2 into a another tuple of size 2 yet with a different unified output
pub const trait TryTuple2Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T)>;
}

impl<Target, Source0, Source1> const TryTuple2Into<Target>
    for (Source0, Source1)
where
    Source0: Copy,
    Source1: Copy,
    Target:
        Copy + [const] TryFromPatch<Source0> + [const] TryFromPatch<Source1>,
{
    fn try_tuple_into(self) -> Option<(Target, Target)> {
        Some((Target::try_from_value(self.0)?, Target::try_from_value(self.1)?))
    }
}

/// Converts a tuple of size 3 into a another tuple of size 3 yet with a different unified output
pub const trait TryTuple3Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T)>;
}

impl<Target, Source0, Source1, Source2> const TryTuple3Into<Target>
    for (Source0, Source1, Source2)
where
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Target: Copy
        + [const] TryFromPatch<Source0>
        + [const] TryFromPatch<Source1>
        + [const] TryFromPatch<Source2>,
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
pub const trait TryTuple4Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T)>;
}

impl<Target, Source0, Source1, Source2, Source3> const TryTuple4Into<Target>
    for (Source0, Source1, Source2, Source3)
where
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Target: Copy
        + [const] TryFromPatch<Source0>
        + [const] TryFromPatch<Source1>
        + [const] TryFromPatch<Source2>
        + [const] TryFromPatch<Source3>,
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
pub const trait TryTuple5Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T)>;
}

impl<Target, Source0, Source1, Source2, Source3, Source4> const
    TryTuple5Into<Target> for (Source0, Source1, Source2, Source3, Source4)
where
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Source4: Copy,
    Target: Copy
        + [const] TryFromPatch<Source0>
        + [const] TryFromPatch<Source1>
        + [const] TryFromPatch<Source2>
        + [const] TryFromPatch<Source3>
        + [const] TryFromPatch<Source4>,
{
    fn try_tuple_into(
        self,
    ) -> Option<(Target, Target, Target, Target, Target)> {
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
pub const trait TryTuple6Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T, T)>;
}

impl<Target, Source0, Source1, Source2, Source3, Source4, Source5> const
    TryTuple6Into<Target>
    for (Source0, Source1, Source2, Source3, Source4, Source5)
where
    Source0: Copy,
    Source1: Copy,
    Source2: Copy,
    Source3: Copy,
    Source4: Copy,
    Source5: Copy,
    Target: Copy
        + [const] TryFromPatch<Source0>
        + [const] TryFromPatch<Source1>
        + [const] TryFromPatch<Source2>
        + [const] TryFromPatch<Source3>
        + [const] TryFromPatch<Source4>
        + [const] TryFromPatch<Source5>,
{
    fn try_tuple_into(
        self,
    ) -> Option<(Target, Target, Target, Target, Target, Target)> {
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
pub const trait TryTuple7Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(self) -> Option<(T, T, T, T, T, T, T)>;
}

impl<Target, Source0, Source1, Source2, Source3, Source4, Source5, Source6> const
    TryTuple7Into<Target>
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
        + [const] TryFromPatch<Source0>
        + [const] TryFromPatch<Source1>
        + [const] TryFromPatch<Source2>
        + [const] TryFromPatch<Source3>
        + [const] TryFromPatch<Source4>
        + [const] TryFromPatch<Source5>
        + [const] TryFromPatch<Source6>,
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
pub const trait TryTuple8Into<T> {
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
    > const TryTuple8Into<Target>
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
        + [const] TryFromPatch<Source0>
        + [const] TryFromPatch<Source1>
        + [const] TryFromPatch<Source2>
        + [const] TryFromPatch<Source3>
        + [const] TryFromPatch<Source4>
        + [const] TryFromPatch<Source5>
        + [const] TryFromPatch<Source6>
        + [const] TryFromPatch<Source7>,
{
    fn try_tuple_into(
        self,
    ) -> Option<(Target, Target, Target, Target, Target, Target, Target, Target)>
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
        ))
    }
}

/// Converts a tuple of size 9 into a another tuple of size 9 yet with a different unified output
pub const trait TryTuple9Into<T> {
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
    > const TryTuple9Into<Target>
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
        + [const] TryFromPatch<Source0>
        + [const] TryFromPatch<Source1>
        + [const] TryFromPatch<Source2>
        + [const] TryFromPatch<Source3>
        + [const] TryFromPatch<Source4>
        + [const] TryFromPatch<Source5>
        + [const] TryFromPatch<Source6>
        + [const] TryFromPatch<Source7>
        + [const] TryFromPatch<Source8>,
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
        ))
    }
}

/// Converts a tuple of size 10 into a another tuple of size 10 yet with a different unified output
pub const trait TryTuple10Into<T> {
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
    > const TryTuple10Into<Target>
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
        + [const] TryFromPatch<Source0>
        + [const] TryFromPatch<Source1>
        + [const] TryFromPatch<Source2>
        + [const] TryFromPatch<Source3>
        + [const] TryFromPatch<Source4>
        + [const] TryFromPatch<Source5>
        + [const] TryFromPatch<Source6>
        + [const] TryFromPatch<Source7>
        + [const] TryFromPatch<Source8>
        + [const] TryFromPatch<Source9>,
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
pub const trait TryTuple11Into<T> {
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
    > const TryTuple11Into<Target>
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
        + [const] TryFromPatch<Source0>
        + [const] TryFromPatch<Source1>
        + [const] TryFromPatch<Source2>
        + [const] TryFromPatch<Source3>
        + [const] TryFromPatch<Source4>
        + [const] TryFromPatch<Source5>
        + [const] TryFromPatch<Source6>
        + [const] TryFromPatch<Source7>
        + [const] TryFromPatch<Source8>
        + [const] TryFromPatch<Source9>
        + [const] TryFromPatch<Source10>,
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
pub const trait TryTuple12Into<T> {
    /// The tuple version of `into()`
    fn try_tuple_into(
        self,
    ) -> Option<(T, T, T, T, T, T, T, T, T, T, T, T)>;
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
    > const TryTuple12Into<Target>
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
        + [const] TryFromPatch<Source0>
        + [const] TryFromPatch<Source1>
        + [const] TryFromPatch<Source2>
        + [const] TryFromPatch<Source3>
        + [const] TryFromPatch<Source4>
        + [const] TryFromPatch<Source5>
        + [const] TryFromPatch<Source6>
        + [const] TryFromPatch<Source7>
        + [const] TryFromPatch<Source8>
        + [const] TryFromPatch<Source9>
        + [const] TryFromPatch<Source10>
        + [const] TryFromPatch<Source11>,
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
