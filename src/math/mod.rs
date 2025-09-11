// use num_traits::{PrimInt, ToPrimitive};

// pub fn range<T>(start: T, end: T) -> Vec<T>
// where
//     T: PrimInt + ToPrimitive,
// {
//     (start.to_u64().unwrap()..=end.to_u64().unwrap())
//         .map(|i| T::from(i).unwrap())
//         .collect()
// }

#[allow(clippy::unwrap_used)]
/// Convert angle degrees into angle radians
///
/// # Panics
/// If somehow `T` doesn't support casting from a float this errors
pub fn radians<T: num_traits::Float>(angle_degrees: T) -> T {
    angle_degrees
        * num_traits::NumCast::from(0.017_453_292_519_943_295).unwrap() //PI / 180.0
}
#[allow(clippy::unwrap_used)]
/// Convert angle radians into angle degrees
///
/// # Panics
/// If somehow `T` doesn't support casting from a float this errors
pub fn degrees<T: num_traits::Float>(angle_radians: T) -> T {
    angle_radians * num_traits::NumCast::from(57.295_779_513_082_32).unwrap() //180.0 / PI
}
/// Sets the length of the vector to 1 changing the direction it's facing
pub fn normalize_vector<T: num_traits::Float>(x: T, y: T, z: T) -> (T, T, T) {
    let v = (x * x + y * y + z * z).abs().sqrt();
    (x / v, y / v, z / v)
}

/// A trait for defining a number
pub trait NumberWithMonotoneOps:
    std::cmp::PartialOrd
    + num_traits::NumCast
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + std::ops::Rem<Output = Self>
{
}

// What's up with this formatting?
impl<
        T: std::cmp::PartialOrd
            + num_traits::NumCast
            + std::ops::Add<Output = Self>
            + std::ops::Sub<Output = Self>
            + std::ops::Mul<Output = Self>
            + std::ops::Div<Output = Self>
            + std::ops::Rem<Output = Self>,
    > NumberWithMonotoneOps for T
{
}
#[allow(missing_docs)]
/// An extended version of `num_traits::One` trait going all the way to Ten
pub trait TwoTillTen<T: num_traits::One + std::ops::Add<Output = T>> {
    #[must_use]
    fn two() -> T {
        T::one() + T::one()
    }
    #[must_use]
    fn three() -> T {
        T::one() + T::one() + T::one()
    }
    #[must_use]
    fn four() -> T {
        T::one() + T::one() + T::one() + T::one()
    }
    #[must_use]
    fn five() -> T {
        T::one() + T::one() + T::one() + T::one() + T::one()
    }
    #[must_use]
    fn six() -> T {
        T::one() + T::one() + T::one() + T::one() + T::one() + T::one()
    }
    #[must_use]
    fn seven() -> T {
        T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
    }
    #[must_use]
    fn eight() -> T {
        T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
    }
    #[must_use]
    fn nine() -> T {
        T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
    }
    #[must_use]
    fn ten() -> T {
        T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
            + T::one()
    }
}
#[allow(missing_docs)]
#[const_trait]
/// An extended version of `num_traits::ConstOne` trait going from 2 to 10
pub trait ConstTwoTillTen {
    const TWO: Self;
    const THREE: Self;
    const FOUR: Self;
    const FIVE: Self;
    const SIX: Self;
    const SEVEN: Self;
    const EIGHT: Self;
    const NINE: Self;
    const TEN: Self;
}
impl<T: num_traits::ConstOne + const std::ops::Add<Output = T>> const
    ConstTwoTillTen for T
{
    const TWO: Self = T::ONE + T::ONE;

    const THREE: Self = T::ONE + T::ONE + T::ONE;

    const FOUR: Self = T::ONE + T::ONE + T::ONE + T::ONE;

    const FIVE: Self = T::ONE + T::ONE + T::ONE + T::ONE + T::ONE;

    const SIX: Self = T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE;

    const SEVEN: Self =
        T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE;

    const EIGHT: Self =
        T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE;

    const NINE: Self = T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE;

    const TEN: Self = T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE
        + T::ONE;
}

impl<T: num_traits::One + std::ops::Add<Output = T>> TwoTillTen<T> for T {}

/// A trait for simple but useful operations that weirdly enough do not exist in std
pub trait ConvenientOps:
    num_traits::bounds::UpperBounded + Copy + std::cmp::PartialOrd
{
    /// Get the half of a value
    #[must_use]
    fn half(&self) -> Self;
    /// Checks if a value is more than half its maximum
    fn more_than_half(&self) -> bool {
        *self > Self::half(&Self::max_value())
    }
}
impl<
        T: TwoTillTen<T>
            + NumberWithMonotoneOps
            + Copy
            + num_traits::bounds::UpperBounded
            + std::cmp::PartialOrd
            + num_traits::One,
    > ConvenientOps for T
{
    fn half(&self) -> Self {
        *self / (Self::two())
    }
}

/// A collision extension focusing on 2d rectangles
pub mod collision;

/// Progress must be between 0 and 1 for this to work as intended most of the times
pub fn interpolate<T: NumberWithMonotoneOps + Copy + num_traits::One>(
    start: T,
    end: T,
    progress: T,
) -> T {
    start * (T::one() - progress) + end * progress
}
/// Get the position of area A if you wanted to center it in area B
pub fn get_center_position_of_object_for_object<
    T: std::ops::Div<Output = T> + std::ops::Sub<Output = T> + ConstTwoTillTen,
>(
    inner_width: T,
    inner_height: T,
    outer_width: T,
    outer_height: T,
) -> (T, T) {
    // This is one hell of a way of getting the number 2 for a type
    (
        outer_width.div(T::TWO) - inner_width.div(T::TWO),
        outer_height.div(T::TWO) - inner_height.div(T::TWO),
    )
}

mod uniform_range;
pub use uniform_range::*;

/// A trait for getting the smallest value >0 and the biggest value <1
pub trait UniformPreviousNext {
    /// Get the smallest value >0
    #[must_use]
    fn smallest_bigger_than_zero(&self) -> Self;
    /// Get the biggest value <0
    #[must_use]
    fn biggest_smaller_than_one(&self) -> Self;
}

impl UniformPreviousNext for f32 {
    fn biggest_smaller_than_one(&self) -> Self {
        1.0f32.next_down()
    }
    fn smallest_bigger_than_zero(&self) -> Self {
        Self::MIN_POSITIVE
    }
}
impl UniformPreviousNext for f64 {
    fn biggest_smaller_than_one(&self) -> Self {
        1.0f64.next_down()
    }
    fn smallest_bigger_than_zero(&self) -> Self {
        Self::MIN_POSITIVE
    }
}
#[cfg(feature = "f128")]
impl UniformPreviousNext for f128 {
    fn biggest_smaller_than_one(&self) -> Self {
        1.0f128.next_down()
    }
    fn smallest_bigger_than_zero(&self) -> Self {
        Self::MIN_POSITIVE
    }
}
