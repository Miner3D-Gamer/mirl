#![allow(clippy::type_complexity)]
use crate::extensions::TryFromPatch;

/// Converts a tuple of size 1 into a another tuple of size 1 yet with a different unified output
pub const trait ConstTuple1Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T,)>;
}

impl<Target, Source> const ConstTuple1Into<Target> for (Source,)
where
    Source: Copy,
    Target: Copy + [const] TryFromPatch<Source>,
{
    fn const_try_tuple_into(self) -> Option<(Target,)> {
        Some((Target::try_from_value(self.0)?,))
    }
}

/// Converts a tuple of size 2 into a another tuple of size 2 yet with a different unified output
pub const trait ConstTuple2Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T)>;
}

impl<Target, Source0, Source1> const ConstTuple2Into<Target>
    for (Source0, Source1)
where
    Source0: Copy,
    Source1: Copy,
    Target:
        Copy + [const] TryFromPatch<Source0> + [const] TryFromPatch<Source1>,
{
    fn const_try_tuple_into(self) -> Option<(Target, Target)> {
        Some((Target::try_from_value(self.0)?, Target::try_from_value(self.1)?))
    }
}

/// Converts a tuple of size 3 into a another tuple of size 3 yet with a different unified output
pub const trait ConstTuple3Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T, T)>;
}

impl<Target, Source0, Source1, Source2> const ConstTuple3Into<Target>
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
    fn const_try_tuple_into(self) -> Option<(Target, Target, Target)> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
        ))
    }
}

/// Converts a tuple of size 4 into a another tuple of size 4 yet with a different unified output
pub const trait ConstTuple4Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T, T, T)>;
}

impl<Target, Source0, Source1, Source2, Source3> const ConstTuple4Into<Target>
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
    fn const_try_tuple_into(self) -> Option<(Target, Target, Target, Target)> {
        Some((
            Target::try_from_value(self.0)?,
            Target::try_from_value(self.1)?,
            Target::try_from_value(self.2)?,
            Target::try_from_value(self.3)?,
        ))
    }
}

/// Converts a tuple of size 5 into a another tuple of size 5 yet with a different unified output
pub const trait ConstTuple5Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T, T, T, T)>;
}

impl<Target, Source0, Source1, Source2, Source3, Source4> const
    ConstTuple5Into<Target> for (Source0, Source1, Source2, Source3, Source4)
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
    fn const_try_tuple_into(
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
pub const trait ConstTuple6Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T, T, T, T, T)>;
}

impl<Target, Source0, Source1, Source2, Source3, Source4, Source5> const
    ConstTuple6Into<Target>
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
    fn const_try_tuple_into(
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
pub const trait ConstTuple7Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T, T, T, T, T, T)>;
}

impl<Target, Source0, Source1, Source2, Source3, Source4, Source5, Source6> const
    ConstTuple7Into<Target>
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
    fn const_try_tuple_into(
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
pub const trait ConstTuple8Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T, T, T, T, T, T, T)>;
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
    > const ConstTuple8Into<Target>
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
    fn const_try_tuple_into(
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
pub const trait ConstTuple9Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T, T, T, T, T, T, T, T)>;
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
    > const ConstTuple9Into<Target>
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
    fn const_try_tuple_into(
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
pub const trait ConstTuple10Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T, T, T, T, T, T, T, T, T)>;
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
    > const ConstTuple10Into<Target>
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
    fn const_try_tuple_into(
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
pub const trait ConstTuple11Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T, T, T, T, T, T, T, T, T, T)>;
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
    > const ConstTuple11Into<Target>
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
    fn const_try_tuple_into(
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
pub const trait ConstTuple12Into<T> {
    /// The tuple version of `into()`
    fn const_try_tuple_into(self) -> Option<(T, T, T, T, T, T, T, T, T, T, T, T)>;
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
    > const ConstTuple12Into<Target>
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
    fn const_try_tuple_into(
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
