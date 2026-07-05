#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "mirl_derive", mirl_derive::derive_all(zerocopy = false))]
#[must_use = "It is expected that the return value of a `FoundValue` is used for something"]
/// A enum that will return the value it beholds when a ? is used - The opposite of [Option]
pub enum FoundValue<V> {
    #[default]
    /// The item was not found, continue execution
    NotFound,
    /// The item was found, exit the current function!
    Found(V),
}
impl<V: core::ops::Residual<()>> core::ops::Try for FoundValue<V> {
    type Output = ();
    type Residual = V;

    fn from_output((): ()) -> Self {
        Self::NotFound
    }

    fn branch(self) -> core::ops::ControlFlow<V, ()> {
        match self {
            Self::NotFound => core::ops::ControlFlow::Continue(()),
            Self::Found(v) => core::ops::ControlFlow::Break(v),
        }
    }
}
impl<V> Unwrap<V> for FoundValue<V> {
    fn unwrap(self) -> V {
        match self {
            Self::Found(val) => val,
            Self::NotFound => {
                panic!("called `Unwrap::unwrap()` on a `NotFound` value")
            }
        }
    }
    fn unwrap_or(self, default: V) -> V {
        match self {
            Self::Found(val) => val,
            Self::NotFound => default,
        }
    }
    unsafe fn unwrap_unchecked(self) -> V {
        match self {
            Self::Found(val) => val,
            Self::NotFound => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
impl<V: Default> UnwrapDefault<V> for FoundValue<V> {
    fn unwrap_or_default(self) -> V {
        match self {
            Self::Found(val) => val,
            Self::NotFound => V::default(),
        }
    }
}

/// Convert to [`FoundValue`] which returns upon having a value instead of not having a value (opposite of [`Option`])
pub const trait ToFoundValue<T> {
    /// Convert to [`FoundValue`] which returns upon having a value instead of not having a value (opposite of [`Option`])
    fn found_error(self) -> FoundValue<T>;
}
impl<T, E> ToFoundValue<E> for Result<T, E> {
    fn found_error(self) -> FoundValue<E> {
        match self {
            Self::Ok(_) => FoundValue::NotFound,
            Self::Err(val) => FoundValue::Found(val),
        }
    }
}
impl<E> ToFoundValue<E> for Option<E> {
    fn found_error(self) -> FoundValue<E> {
        match self {
            Self::None => FoundValue::NotFound,
            Self::Some(val) => FoundValue::Found(val),
        }
    }
}

impl<V> core::ops::FromResidual<V> for FoundValue<V> {
    fn from_residual(residual: V) -> Self {
        Self::Found(residual)
    }
}

impl<E> core::ops::FromResidual<Result<core::convert::Infallible, E>> for FoundValue<E> {
    fn from_residual(residual: Result<core::convert::Infallible, E>) -> Self {
        match residual {
            Err(e) => Self::Found(e),
        }
    }
}
impl<T> From<Option<T>> for FoundValue<T> {
    fn from(val: Option<T>) -> Self {
        val.map_or_else(|| Self::NotFound, |val| Self::Found(val))
    }
}
impl<T> FromPatch<Option<T>> for FoundValue<T> {
    fn from_value(val: Option<T>) -> Self {
        val.map_or_else(|| Self::NotFound, |val| Self::Found(val))
    }
}
impl<T> From<T> for FoundValue<T> {
    fn from(val: T) -> Self {
        Self::Found(val)
    }
}
