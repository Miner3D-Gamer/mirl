// use num_traits::{PrimInt, ToPrimitive};

// pub fn range<T>(start: T, end: T) -> Vec<T>
// where
//     T: PrimInt + ToPrimitive,
// {
//     (start.to_u64().unwrap()..=end.to_u64().unwrap())
//         .map(|i| T::from(i).unwrap())
//         .collect()
// }
/// Convert angle degrees into angle radians
pub fn radians<T: num_traits::Float>(angle_degrees: T) -> T {
    angle_degrees * NumCast::from(0.017453292519943295).unwrap() //PI / 180.0
}
/// Convert angle radians into angle degrees
pub fn degrees<T: num_traits::Float>(angle_radians: T) -> T {
    angle_radians * NumCast::from(57.29577951308232).unwrap() //180.0 / PI
}
/// Sets the length of the vector to 1 changing the direction it's facing
pub fn normalize_vector<T: num_traits::Float>(x: T, y: T, z: T) -> (T, T, T) {
    let v = (x * x + y * y + z * z).abs().sqrt();
    return (x / v, y / v, z / v);
}

use num_traits::NumCast;

use crate::extensions::*;

/// A trait for defining a number that does not have fractions
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

/// A trait for defining a number that does have fractions
pub trait FloatNumber: std::cmp::PartialOrd + NumCast {}

impl FloatNumber for f32 {}
impl FloatNumber for f64 {}

/// A trait for defining a number
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

/// A collision extension focusing on 2d rectangles
pub mod collision;

/// Progress must be between 0 and 1 for this to work as intended most of the times
pub fn interpolate<T: Number + Copy + num_traits::One>(
    start: T,
    end: T,
    progress: T,
) -> T {
    return start * (T::one() - progress) + end * progress;
}
/// Get the position of object A if you wanted to center it in object B
pub fn get_center_position_of_object_for_object<
    T: std::ops::Div<Output = T> + std::ops::Sub<Output = T> + num_traits::NumCast,
>(
    inner_width: T,
    inner_height: T,
    outer_width: T,
    outer_height: T,
) -> (T, T) {
    (
        outer_width.div(num_traits::NumCast::from(2).unwrap())
            - inner_width.div(num_traits::NumCast::from(2).unwrap()),
        outer_height.div(num_traits::NumCast::from(2).unwrap())
            - inner_height.div(num_traits::NumCast::from(2).unwrap()),
    )
}

mod uniform_range;
pub use uniform_range::*;