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
pub fn normalize_vector(x: f32, y: f32, z: f32) -> (f32, f32, f32) {
    let v = (x * x + y * y + z * z).abs().sqrt();
    return (x / v, y / v, z / v);
}

use num_traits::NumCast;

use crate::render::{U1, U2, U4};

pub trait WholeNumber: std::cmp::PartialOrd + NumCast {}
impl WholeNumber for U1 {}
impl WholeNumber for U2 {}
impl WholeNumber for U4 {}

impl WholeNumber for u8 {}
impl WholeNumber for u16 {}
impl WholeNumber for u32 {}
impl WholeNumber for u64 {}
impl WholeNumber for u128 {}
impl WholeNumber for usize {}

impl WholeNumber for i8 {}
impl WholeNumber for i16 {}
impl WholeNumber for i32 {}
impl WholeNumber for i64 {}
impl WholeNumber for i128 {}
impl WholeNumber for isize {}

pub trait FloatNumber: std::cmp::PartialOrd + NumCast {}

impl FloatNumber for f32 {}
impl FloatNumber for f64 {}

pub trait Number:
    std::cmp::PartialOrd
    + NumCast
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
{
}


impl Number for U1 {}
impl Number for U2 {}
impl Number for U4 {}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for usize {}

impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for isize {}

impl Number for f32 {}
impl Number for f64 {}
