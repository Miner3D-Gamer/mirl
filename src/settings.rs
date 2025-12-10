/// The map type used around the lib
#[cfg(any(target_arch = "wasm32", not(feature = "ahash")))]
#[cfg(feature = "std")]
pub type MapType<K, V> = std::collections::HashMap<K, V>;
#[cfg(feature = "std")]
#[cfg(all(not(target_arch = "wasm32"), feature = "ahash"))]
pub type MapType<K, V> = ahash::AHashMap<K, V>;
