#[cfg(any(target_arch = "wasm32", not(feature = "internal_use_ahash")))]
#[cfg(feature = "std")]
/// The default hasher
pub type HashBuilder = std::hash::RandomState;
#[cfg(all(not(target_arch = "wasm32"), feature = "internal_use_ahash"))]
#[cfg(feature = "std")]
/// The hasher from ahash
pub type HashBuilder = std::hash::BuildHasherDefault<ahash::AHasher>;

#[cfg(any(target_arch = "wasm32", not(feature = "internal_use_indexmap")))]
#[cfg(feature = "std")]
/// The map type used around the lib
pub type MapType<K, V> = std::collections::HashMap<K, V, HashBuilder>;
#[cfg(feature = "std")]
/// The map type used around the lib
#[cfg(all(not(target_arch = "wasm32"), feature = "internal_use_indexmap"))]
pub type MapType<K, V> = indexmap::IndexMap<K, V, HashBuilder>;
#[cfg(feature = "std")]
/// Functions to unify the functionality of [`MapType`]
pub trait SettingsMapType<K, V> {
    /// Create a new map
    fn new_map() -> Self;
    /// Remove an object from the given map
    fn remove_thingy(&mut self, key: &K) -> Option<V>;
}
#[cfg(feature = "std")]
#[cfg(all(not(target_arch = "wasm32"), feature = "internal_use_indexmap"))]
impl<K: Sized + core::hash::Hash + indexmap::Equivalent<K>, V>
    SettingsMapType<K, V> for MapType<K, V>
{
    fn new_map() -> Self {
        Self::with_capacity_and_hasher(0, HashBuilder::default())
    }
    fn remove_thingy(&mut self, key: &K) -> Option<V> {
        self.swap_remove(key)
    }
}
#[cfg(feature = "std")]
#[allow(clippy::implicit_hasher)]
#[cfg(any(target_arch = "wasm32", not(feature = "internal_use_indexmap")))]
impl<K: Sized + core::cmp::Eq + core::hash::Hash, V> SettingsMapType<K, V>
    for MapType<K, V>
{
    fn new_map() -> Self {
        Self::with_capacity_and_hasher(0, HashBuilder::default())
    }
    fn remove_thingy(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }
}
