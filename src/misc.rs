use std::hash::Hasher;
/// Combine 2 strings
pub fn concatenate<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> String {
    let mut result = String::from(a.as_ref());
    result.push_str(b.as_ref());
    return result;
}
/// Hash a value
pub fn hash_value<T: std::hash::Hash>(value: &T) -> u64 {
    let mut s = std::hash::DefaultHasher::new();
    value.hash(&mut s);
    s.finish()
}
/// Repeat the given data X times
pub fn repeat_data<T: Clone>(data: T, amount: usize) -> Vec<T> {
    vec![data; amount]
}
