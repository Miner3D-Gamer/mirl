use crate::{extensions::FromPatch, misc::EasyUnwrapUnchecked};

// Vec<T> conversions

impl<T: Clone, S: FromPatch<T>, const N: usize> FromPatch<[T; N]> for Vec<S> {
    fn from_value(v: [T; N]) -> Self {
        v.iter().map(|x| FromPatch::from_value(x.clone())).collect()
    }
}
impl<T: Clone, S: FromPatch<T>> FromPatch<&[T]> for Vec<S> {
    fn from_value(v: &[T]) -> Self {
        v.iter().map(|x| S::from_value(x.clone())).collect()
    }
}

// Vec<T> with element conversion
impl<T, S: FromPatch<T>> FromPatch<Vec<T>> for Vec<S> {
    fn from_value(v: Vec<T>) -> Self {
        v.into_iter().map(|x| S::from_value(x)).collect()
    }
}

// Array to array with element conversion
impl<T: Clone, S: FromPatch<T>, const N: usize> FromPatch<[T; N]> for [S; N] {
    fn from_value(v: [T; N]) -> Self {
        let vec: Vec<S> = v.iter().map(|x| S::from_value(x.clone())).collect();

        vec.try_into().easy_unwrap_unchecked()
    }
}

impl<T: Clone, S: FromPatch<T>, const N: usize> FromPatch<&[T; N]> for [S; N] {
    fn from_value(v: &[T; N]) -> Self {
        let vec: Vec<S> = v.iter().map(|x| S::from_value(x.clone())).collect();

        vec.try_into().easy_unwrap_unchecked()
    }
}

#[cfg(feature = "indexmap")]
impl<K, V> FromPatch<indexmap::IndexMap<K, V>> for Vec<(K, V)> {
    fn from_value(v: indexmap::IndexMap<K, V>) -> Self {
        let mut new = Self::new();
        for i in v {
            new.push(i);
        }
        new
    }
}

#[cfg(feature = "indexmap")]
impl<K: core::hash::Hash + Eq, V> FromPatch<Vec<(K, V)>> for indexmap::IndexMap<K, V> {
    fn from_value(v: Vec<(K, V)>) -> Self {
        let mut new = Self::new();
        for i in v {
            new.insert(i.0, i.1);
        }
        new
    }
}
