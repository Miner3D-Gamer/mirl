use crate::misc::EasyUnwrapUnchecked;
// TODO: Add this trait to ahash and indexmap

/// A trait that combines standart capabilities across Maps
pub const trait Map<K, V>: core::hash::Hash {
    /// Insert a value using a key, return `Some(old_value)` if key already existed
    fn insert(&mut self, key_val: (K, V)) -> Option<V>;
    /// Get a value using the key
    fn get(&self, key: &K) -> Option<&V>;
    /// Get a mutable value using the key
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    /// Remove a value using the key
    fn remove(&mut self, key: &K) -> Option<V>;
    /// Get all keys and all values
    fn get_all(&self) -> Vec<(&K, &V)>;
    /// Find the key of a value
    fn find(&self, value: &V) -> Option<&K>;
    /// Index the key of a value, return the index at which the value was found
    fn index(&self, value: &V) -> Option<usize>;
    /// Get the value at the index
    fn get_at_index(&self, index: usize) -> Option<&V>;
    /// Get the mutable value at the index
    fn get_at_index_mut(&mut self, index: usize) -> Option<&mut V>;
    /// If the given map supports placing at taking from explicit indexes
    fn supports_indexing(&self) -> bool;
}
impl<K: core::hash::Hash + core::cmp::Ord, V: core::hash::Hash> Map<K, V>
    for std::collections::BTreeMap<K, V>
{
    fn insert(&mut self, key_val: (K, V)) -> Option<V> {
        self.insert(key_val.0, key_val.1)
    }
    fn get(&self, key: &K) -> Option<&V> {
        self.get(key)
    }
    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.get_mut(key)
    }
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
    fn get_all(&self) -> Vec<(&K, &V)> {
        let keys = self.keys();

        let mut output = Vec::new();
        for i in keys {
            output.push((i, self.get(i).easy_unwrap_unchecked()));
        }

        output
    }
    fn find(&self, _value: &V) -> Option<&K> {
        None
    }
    fn index(&self, _value: &V) -> Option<usize> {
        None
    }
    fn get_at_index(&self, _index: usize) -> Option<&V> {
        None
    }
    fn get_at_index_mut(&mut self, _index: usize) -> Option<&mut V> {
        None
    }
    fn supports_indexing(&self) -> bool {
        false
    }
}
// #[allow(clippy::implicit_hasher)]
// #[cfg(feature = "std")]
// /// Instead of key -> value, value -> key
// pub fn find_key_by_value<K: Eq + core::hash::Hash + Clone, V: PartialEq>(
//     map: &std::collections::HashMap<K, V>,
//     value: &V,
// ) -> Option<K> {
//     for (k, v) in map {
//         if v == value {
//             return Some(k.clone());
//         }
//     }
//     None
// }
