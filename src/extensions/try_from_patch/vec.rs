use crate::extensions::TryFromPatch;

// Vec<T> conversions

impl<T: Clone, S: TryFromPatch<T>, const N: usize> TryFromPatch<[T; N]>
    for Vec<S>
{
    fn try_from_value(v: [T; N]) -> Option<Self> {
        Some(
            v.iter()
                .filter_map(|x| TryFromPatch::try_from_value(x.clone()))
                .collect(),
        )
    }
}
impl<T: Clone, S: TryFromPatch<T>> TryFromPatch<&[T]> for Vec<S> {
    fn try_from_value(v: &[T]) -> Option<Self> {
        v.iter().map(|x| S::try_from_value(x.clone())).collect()
    }
}

impl<T, const N: usize> TryFromPatch<Vec<T>> for [T; N] {
    fn try_from_value(v: Vec<T>) -> Option<Self> {
        v.try_into().ok()
    }
}

// Vec<T> with element conversion
impl<T, S: TryFromPatch<T>> TryFromPatch<Vec<T>> for Vec<S> {
    fn try_from_value(v: Vec<T>) -> Option<Self> {
        v.into_iter().map(|x| S::try_from_value(x)).collect()
    }
}

// Array to array with element conversion
impl<T: Clone, S: TryFromPatch<T>, const N: usize> TryFromPatch<[T; N]>
    for [S; N]
{
    fn try_from_value(v: [T; N]) -> Option<Self> {
        let vec: Vec<S> =
            v.iter().filter_map(|x| S::try_from_value(x.clone())).collect();

        if vec.len() == N {
            vec.try_into().ok()
        } else {
            None
        }
    }
}

impl<T: Clone, S: TryFromPatch<T>, const N: usize> TryFromPatch<&[T; N]>
    for [S; N]
{
    fn try_from_value(v: &[T; N]) -> Option<Self> {
        let vec: Vec<S> =
            v.iter().filter_map(|x| S::try_from_value(x.clone())).collect();

        if vec.len() == N {
            vec.try_into().ok()
        } else {
            None
        }
    }
}
