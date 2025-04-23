// use num_traits::{PrimInt, ToPrimitive};

// pub fn range<T>(start: T, end: T) -> Vec<T>
// where
//     T: PrimInt + ToPrimitive,
// {
//     (start.to_u64().unwrap()..=end.to_u64().unwrap())
//         .map(|i| T::from(i).unwrap())
//         .collect()
// }

pub fn radians(angle_degrees: f64) -> f64 {
    angle_degrees * 0.017453292519943295 //PI / 180.0
}
pub fn degrees(angle_radians: f64) -> f64 {
    angle_radians * 57.29577951308232 //180.0 / PI
}
fn normalize_vector(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let v = (x * x + y * y + z * z).abs().sqrt();
    return (x / v, y / v, z / v);
}
