use crate::extensions::TryFromPatch;

// Vec<T> conversions

impl<T, const N: usize> TryFromPatch<Vec<T>> for [T; N] {
    fn try_from_value(v: Vec<T>) -> Option<Self> {
        v.try_into().ok()
    }
}
