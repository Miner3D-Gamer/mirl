use std::{collections::HashMap, hash::BuildHasher};

use crate::extensions::TryFromPatch;

impl<
        K1: core::cmp::Eq + core::hash::Hash + TryFromPatch<K2>,
        K2: Clone,
        V1: TryFromPatch<V2>,
        V2: Clone,
        S: BuildHasher + Default,
    > TryFromPatch<HashMap<K2, V2, S>> for HashMap<K1, V1, S>
{
    fn try_from_value(value: HashMap<K2, V2, S>) -> Option<Self> {
        let mut new = Self::with_hasher(S::default());
        for (key, item) in value {
            new.insert(K1::try_from_value(key)?, V1::try_from_value(item)?);
        }
        Some(new)
    }
}
use std::collections::BTreeMap;

impl<
        K1: Ord + TryFromPatch<K2>,
        K2: Clone,
        V1: TryFromPatch<V2>,
        V2: Clone,
    > TryFromPatch<BTreeMap<K2, V2>> for BTreeMap<K1, V1>
{
    fn try_from_value(value: BTreeMap<K2, V2>) -> Option<Self> {
        let mut new = Self::new();
        for (key, item) in value {
            new.insert(K1::try_from_value(key)?, V1::try_from_value(item)?);
        }
        Some(new)
    }
}
