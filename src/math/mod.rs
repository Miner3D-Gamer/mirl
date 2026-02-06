/// A collision extension focusing on 2d rectangles
pub mod geometry;
#[deprecated = "This is outdated, the new path is `geometry::d2`"]
pub use geometry::d2 as collision;

/// Mathematical equations
pub mod expressions;

//#[cfg(feature = "std")]
mod uniform_range;
//#[cfg(feature = "std")]
pub use uniform_range::*;

// /// Structs for positioning data in 2d and 3d space
// pub mod positioning;

/// Convert angle degrees into angle radians
pub fn radians<T: TryFromPatch<f64> + core::ops::Mul<Output = T>>(
    angle_degrees: T,
) -> Option<T> {
    Some(angle_degrees * T::try_from_value(0.017_453_292_519_943_295)?) //PI / 180.0
}

/// Convert angle radians into angle degrees
pub fn degrees<T: TryFromPatch<f64> + core::ops::Mul<Output = T>>(
    angle_radians: T,
) -> Option<T> {
    Some(angle_radians * T::try_from_value(57.295_779_513_082_32)?) //180.0 / PI
}

/// Sets the length of the vector to 1 changing the direction it's facing
pub fn normalize_vector<T: NumberWithMonotoneOps + Abs + Sqrt + Copy>(
    x: T,
    y: T,
    z: T,
) -> (T, T, T) {
    let v = (x * x + y * y + z * z).abs().sqrt();
    (x / v, y / v, z / v)
}

/// A trait for defining a number
pub const trait NumberWithMonotoneOps:
    core::cmp::PartialOrd
    + core::marker::Sized
    + [const] core::ops::Add<Output = Self>
    + [const] core::ops::Sub<Output = Self>
    + [const] core::ops::Mul<Output = Self>
    + [const] core::ops::Div<Output = Self>
    + [const] core::ops::Rem<Output = Self>
{
}

// What's up with this formatting?
impl<
        T: core::cmp::PartialOrd
            + core::ops::Add<Output = Self>
            + core::ops::Sub<Output = Self>
            + core::ops::Mul<Output = Self>
            + core::ops::Div<Output = Self>
            + core::ops::Rem<Output = Self>,
    > NumberWithMonotoneOps for T
{
}

/// A trait for simple but useful operations that weirdly enough do not exist in std
pub const trait ConvenientOps:
    Bounded + Copy + core::cmp::PartialOrd
{
    /// Get the half of a value
    #[must_use]
    fn half(&self) -> Self;
    #[must_use]
    /// Get the double of the current value
    fn double(&self) -> Self;
    #[must_use]
    /// Checks if a value is more than half its maximum
    fn more_than_half(&self) -> bool;
    #[must_use]
    /// Checks if a value is less than half its maximum
    fn less_than_half(&self) -> bool;
    #[must_use]
    /// Checks if a value is half its maximum
    fn equals_half(&self) -> bool;
}

impl<
        T: [const] ConstNumbers128
            + [const] NumberWithMonotoneOps
            + Copy
            + [const] Bounded
            + [const] PartialOrd,
    > const ConvenientOps for T
{
    fn half(&self) -> Self {
        *self / Self::CONST_2
    }
    fn more_than_half(&self) -> bool {
        *self > Self::half(&Self::max_value())
    }
    fn less_than_half(&self) -> bool {
        *self < Self::half(&Self::max_value())
    }
    fn equals_half(&self) -> bool {
        *self == Self::half(&Self::max_value())
    }
    fn double(&self) -> Self {
        *self * Self::CONST_2
    }
}

/// Progress must be between 0 and 1 for this to work as intended most of the times
pub const fn interpolate<
    T: Copy
        + ConstOne
        + [const] core::ops::Add<Output = T>
        + [const] core::ops::Sub<Output = T>
        + [const] core::ops::Mul<Output = T>,
>(
    start: T,
    end: T,
    progress: T,
) -> T {
    start * (T::ONE - progress) + end * progress
}
/// Get the position of area A if you wanted to center it in area B
pub fn get_center_position_of_object_for_object<
    T: core::ops::Div<Output = T> + core::ops::Sub<Output = T> + ConstNumbers128,
>(
    inner_width: T,
    inner_height: T,
    outer_width: T,
    outer_height: T,
) -> (T, T) {
    // This is one hell of a way of getting the number 2 for a type
    (
        outer_width.div(T::CONST_2) - inner_width.div(T::CONST_2),
        outer_height.div(T::CONST_2) - inner_height.div(T::CONST_2),
    )
}

/// A trait for getting the smallest value >0 and the biggest value <1
pub const trait UniformPreviousNext {
    /// Get the smallest value >0
    #[must_use]
    fn smallest_bigger_than_zero(&self) -> Self;
    /// Get the biggest value <0
    #[must_use]
    fn biggest_smaller_than_one(&self) -> Self;
}

impl const UniformPreviousNext for f32 {
    fn biggest_smaller_than_one(&self) -> Self {
        1.0f32.next_down()
    }
    fn smallest_bigger_than_zero(&self) -> Self {
        Self::MIN_POSITIVE
    }
}
impl const UniformPreviousNext for f64 {
    fn biggest_smaller_than_one(&self) -> Self {
        1.0f64.next_down()
    }
    fn smallest_bigger_than_zero(&self) -> Self {
        Self::MIN_POSITIVE
    }
}

impl const UniformPreviousNext for f128 {
    fn biggest_smaller_than_one(&self) -> Self {
        1.0f128.next_down()
    }
    fn smallest_bigger_than_zero(&self) -> Self {
        Self::MIN_POSITIVE
    }
}
impl const UniformPreviousNext for f16 {
    fn biggest_smaller_than_one(&self) -> Self {
        1.0f16.next_down()
    }
    fn smallest_bigger_than_zero(&self) -> Self {
        Self::MIN_POSITIVE
    }
}

use core::{f32, f64};

#[allow(missing_docs)]
/// The upper and lower bound of a value
pub const trait Bounded {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

macro_rules! bounded_impl {
    ($t:ty, $min:expr, $max:expr) => {
        impl const Bounded for $t {
            #[inline]
            fn min_value() -> $t {
                $min
            }

            #[inline]
            fn max_value() -> $t {
                $max
            }
        }
    };
}

bounded_impl!(usize, usize::MIN, usize::MAX);
bounded_impl!(u8, u8::MIN, u8::MAX);
bounded_impl!(u16, u16::MIN, u16::MAX);
bounded_impl!(u32, u32::MIN, u32::MAX);
bounded_impl!(u64, u64::MIN, u64::MAX);
bounded_impl!(u128, u128::MIN, u128::MAX);

bounded_impl!(isize, isize::MIN, isize::MAX);
bounded_impl!(i8, i8::MIN, i8::MAX);
bounded_impl!(i16, i16::MIN, i16::MAX);
bounded_impl!(i32, i32::MIN, i32::MAX);
bounded_impl!(i64, i64::MIN, i64::MAX);
bounded_impl!(i128, i128::MIN, i128::MAX);

bounded_impl!(f32, f32::MIN, f32::MAX);
bounded_impl!(f64, f64::MIN, f64::MAX);
bounded_impl!(f128, f128::MIN, f128::MAX);
bounded_impl!(f16, f16::MIN, f16::MAX);

use crate::prelude::{Abs, Sqrt, TryFromPatch};

/// A function that smoothly transitions from 0 to 1
/// At x 1, the y will be 0.5 if the offset is 0
/// If the offset is 2 the y will be 2 at x 0.5
#[must_use]
pub fn smooth_0_to_1(x: f32, steepness: f32, offset: f32) -> f32 {
    1.0 / (1.0
        + { core::intrinsics::expf32((x / offset) * steepness) }
        + (-steepness))
}

/// A value of 10 with a variance of 5 will return (5, 15)
pub fn range_with_variance<T: NumberWithMonotoneOps + Copy>(
    value: T,
    variance: T,
) -> (T, T) {
    (value - variance, value + variance)
}

/// A custom [`ConstOne`] as [num-traits](crate::math::ConstOne) does not support f16 or f128
pub const trait ConstOne {
    /// The value of 1 in the respective type
    const ONE: Self;
}
macro_rules! impl_const_one {
    ($t:ty, $v:expr) => {
        impl ConstOne for $t {
            const ONE: Self = $v;
        }
    };
}

impl_const_one!(usize, 1);
impl_const_one!(u8, 1);
impl_const_one!(u16, 1);
impl_const_one!(u32, 1);
impl_const_one!(u64, 1);
impl_const_one!(u128, 1);

impl_const_one!(isize, 1);
impl_const_one!(i8, 1);
impl_const_one!(i16, 1);
impl_const_one!(i32, 1);
impl_const_one!(i64, 1);
impl_const_one!(i128, 1);

impl_const_one!(f16, 1.0);
impl_const_one!(f32, 1.0);
impl_const_one!(f64, 1.0);
impl_const_one!(f128, 1.0);

/// A custom [`ConstZero`] as [num-traits](crate::math::ConstZero) does not support f16 or f128
pub const trait ConstZero {
    /// The value of 0 in the respective type
    const ZERO: Self;
}
macro_rules! impl_const_zero {
    ($t:ty, $v:expr) => {
        impl ConstZero for $t {
            const ZERO: Self = $v;
        }
    };
}

impl_const_zero!(usize, 0);
impl_const_zero!(u8, 0);
impl_const_zero!(u16, 0);
impl_const_zero!(u32, 0);
impl_const_zero!(u64, 0);
impl_const_zero!(u128, 0);

impl_const_zero!(isize, 0);
impl_const_zero!(i8, 0);
impl_const_zero!(i16, 0);
impl_const_zero!(i32, 0);
impl_const_zero!(i64, 0);
impl_const_zero!(i128, 0);

impl_const_zero!(f16, 0.0);
impl_const_zero!(f32, 0.0);
impl_const_zero!(f64, 0.0);
impl_const_zero!(f128, 0.0);

/// A trait that sets the current value to 0
pub trait SetZero {
    /// Set the current value to 0
    fn set_zero(&mut self);
}
/// A trait that sets the current value to 1
pub trait SetOne {
    /// Set the current value to 1
    fn set_one(&mut self);
}
impl<T: ConstZero> SetZero for T {
    fn set_zero(&mut self) {
        *self = Self::ZERO;
    }
}
impl<T: ConstOne> SetOne for T {
    fn set_one(&mut self) {
        *self = Self::ONE;
    }
}

/// Numbers that support the number range 0-127 (2^8)/2
pub const trait SupportsRange128 {}
/// Numbers that support the number range 0-255 (2^8)
pub const trait SupportsRange256 {}
/// Numbers that support the number range 0-32767 (2^16)/2
pub const trait SupportsRange32768 {}
/// Numbers that support the number range 0-65535 (2^16)
pub const trait SupportsRange65536 {}

crate::impl_empty_trait!(SupportsRange128 for u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f16 f32 f64 f128);

crate::impl_empty_trait!(SupportsRange256 for u8 u16 u32 u64 u128 usize i16 i32 i64 i128 isize f16 f32 f64 f128);

crate::impl_empty_trait!(SupportsRange32768 for u16 u32 u64 u128 usize i16 i32 i64 i128 isize f16 f32 f64 f128);

crate::impl_empty_trait!(SupportsRange65536 for u16 u32 u64 u128 usize i32 i64 i128 isize f16 f32 f64 f128);
#[allow(missing_docs)]
/// An extended version of the [`ConstOne`](crate::math::ConstOne)/[`ConstZero`](crate::math::ConstZero) traits covering all numbers from 0 to 127
pub const trait ConstNumbers128: SupportsRange128 {
    const CONST_0: Self;
    const CONST_1: Self;
    const CONST_2: Self;
    const CONST_3: Self;
    const CONST_4: Self;
    const CONST_5: Self;
    const CONST_6: Self;
    const CONST_7: Self;
    const CONST_8: Self;
    const CONST_9: Self;
    const CONST_10: Self;
    const CONST_11: Self;
    const CONST_12: Self;
    const CONST_13: Self;
    const CONST_14: Self;
    const CONST_15: Self;
    const CONST_16: Self;
    const CONST_17: Self;
    const CONST_18: Self;
    const CONST_19: Self;
    const CONST_20: Self;
    const CONST_21: Self;
    const CONST_22: Self;
    const CONST_23: Self;
    const CONST_24: Self;
    const CONST_25: Self;
    const CONST_26: Self;
    const CONST_27: Self;
    const CONST_28: Self;
    const CONST_29: Self;
    const CONST_30: Self;
    const CONST_31: Self;
    const CONST_32: Self;
    const CONST_33: Self;
    const CONST_34: Self;
    const CONST_35: Self;
    const CONST_36: Self;
    const CONST_37: Self;
    const CONST_38: Self;
    const CONST_39: Self;
    const CONST_40: Self;
    const CONST_41: Self;
    const CONST_42: Self;
    const CONST_43: Self;
    const CONST_44: Self;
    const CONST_45: Self;
    const CONST_46: Self;
    const CONST_47: Self;
    const CONST_48: Self;
    const CONST_49: Self;
    const CONST_50: Self;
    const CONST_51: Self;
    const CONST_52: Self;
    const CONST_53: Self;
    const CONST_54: Self;
    const CONST_55: Self;
    const CONST_56: Self;
    const CONST_57: Self;
    const CONST_58: Self;
    const CONST_59: Self;
    const CONST_60: Self;
    const CONST_61: Self;
    const CONST_62: Self;
    const CONST_63: Self;
    const CONST_64: Self;
    const CONST_65: Self;
    const CONST_66: Self;
    const CONST_67: Self;
    const CONST_68: Self;
    const CONST_69: Self;
    const CONST_70: Self;
    const CONST_71: Self;
    const CONST_72: Self;
    const CONST_73: Self;
    const CONST_74: Self;
    const CONST_75: Self;
    const CONST_76: Self;
    const CONST_77: Self;
    const CONST_78: Self;
    const CONST_79: Self;
    const CONST_80: Self;
    const CONST_81: Self;
    const CONST_82: Self;
    const CONST_83: Self;
    const CONST_84: Self;
    const CONST_85: Self;
    const CONST_86: Self;
    const CONST_87: Self;
    const CONST_88: Self;
    const CONST_89: Self;
    const CONST_90: Self;
    const CONST_91: Self;
    const CONST_92: Self;
    const CONST_93: Self;
    const CONST_94: Self;
    const CONST_95: Self;
    const CONST_96: Self;
    const CONST_97: Self;
    const CONST_98: Self;
    const CONST_99: Self;
    const CONST_100: Self;
    const CONST_101: Self;
    const CONST_102: Self;
    const CONST_103: Self;
    const CONST_104: Self;
    const CONST_105: Self;
    const CONST_106: Self;
    const CONST_107: Self;
    const CONST_108: Self;
    const CONST_109: Self;
    const CONST_110: Self;
    const CONST_111: Self;
    const CONST_112: Self;
    const CONST_113: Self;
    const CONST_114: Self;
    const CONST_115: Self;
    const CONST_116: Self;
    const CONST_117: Self;
    const CONST_118: Self;
    const CONST_119: Self;
    const CONST_120: Self;
    const CONST_121: Self;
    const CONST_122: Self;
    const CONST_123: Self;
    const CONST_124: Self;
    const CONST_125: Self;
    const CONST_126: Self;
    const CONST_127: Self;
}

impl<
        T: crate::math::ConstOne
            + crate::math::ConstZero
            + SupportsRange128
            + const core::ops::Add<Output = T>,
    > const ConstNumbers128 for T
{
    const CONST_0: Self = T::ZERO;
    const CONST_1: Self = T::ONE;
    const CONST_2: Self = T::CONST_1 + T::CONST_1;
    const CONST_3: Self = T::CONST_2 + T::CONST_1;
    const CONST_4: Self = T::CONST_3 + T::CONST_1;
    const CONST_5: Self = T::CONST_4 + T::CONST_1;
    const CONST_6: Self = T::CONST_5 + T::CONST_1;
    const CONST_7: Self = T::CONST_6 + T::CONST_1;
    const CONST_8: Self = T::CONST_7 + T::CONST_1;
    const CONST_9: Self = T::CONST_8 + T::CONST_1;
    const CONST_10: Self = T::CONST_9 + T::CONST_1;
    const CONST_11: Self = T::CONST_10 + T::CONST_1;
    const CONST_12: Self = T::CONST_11 + T::CONST_1;
    const CONST_13: Self = T::CONST_12 + T::CONST_1;
    const CONST_14: Self = T::CONST_13 + T::CONST_1;
    const CONST_15: Self = T::CONST_14 + T::CONST_1;
    const CONST_16: Self = T::CONST_15 + T::CONST_1;
    const CONST_17: Self = T::CONST_16 + T::CONST_1;
    const CONST_18: Self = T::CONST_17 + T::CONST_1;
    const CONST_19: Self = T::CONST_18 + T::CONST_1;
    const CONST_20: Self = T::CONST_19 + T::CONST_1;
    const CONST_21: Self = T::CONST_20 + T::CONST_1;
    const CONST_22: Self = T::CONST_21 + T::CONST_1;
    const CONST_23: Self = T::CONST_22 + T::CONST_1;
    const CONST_24: Self = T::CONST_23 + T::CONST_1;
    const CONST_25: Self = T::CONST_24 + T::CONST_1;
    const CONST_26: Self = T::CONST_25 + T::CONST_1;
    const CONST_27: Self = T::CONST_26 + T::CONST_1;
    const CONST_28: Self = T::CONST_27 + T::CONST_1;
    const CONST_29: Self = T::CONST_28 + T::CONST_1;
    const CONST_30: Self = T::CONST_29 + T::CONST_1;
    const CONST_31: Self = T::CONST_30 + T::CONST_1;
    const CONST_32: Self = T::CONST_31 + T::CONST_1;
    const CONST_33: Self = T::CONST_32 + T::CONST_1;
    const CONST_34: Self = T::CONST_33 + T::CONST_1;
    const CONST_35: Self = T::CONST_34 + T::CONST_1;
    const CONST_36: Self = T::CONST_35 + T::CONST_1;
    const CONST_37: Self = T::CONST_36 + T::CONST_1;
    const CONST_38: Self = T::CONST_37 + T::CONST_1;
    const CONST_39: Self = T::CONST_38 + T::CONST_1;
    const CONST_40: Self = T::CONST_39 + T::CONST_1;
    const CONST_41: Self = T::CONST_40 + T::CONST_1;
    const CONST_42: Self = T::CONST_41 + T::CONST_1;
    const CONST_43: Self = T::CONST_42 + T::CONST_1;
    const CONST_44: Self = T::CONST_43 + T::CONST_1;
    const CONST_45: Self = T::CONST_44 + T::CONST_1;
    const CONST_46: Self = T::CONST_45 + T::CONST_1;
    const CONST_47: Self = T::CONST_46 + T::CONST_1;
    const CONST_48: Self = T::CONST_47 + T::CONST_1;
    const CONST_49: Self = T::CONST_48 + T::CONST_1;
    const CONST_50: Self = T::CONST_49 + T::CONST_1;
    const CONST_51: Self = T::CONST_50 + T::CONST_1;
    const CONST_52: Self = T::CONST_51 + T::CONST_1;
    const CONST_53: Self = T::CONST_52 + T::CONST_1;
    const CONST_54: Self = T::CONST_53 + T::CONST_1;
    const CONST_55: Self = T::CONST_54 + T::CONST_1;
    const CONST_56: Self = T::CONST_55 + T::CONST_1;
    const CONST_57: Self = T::CONST_56 + T::CONST_1;
    const CONST_58: Self = T::CONST_57 + T::CONST_1;
    const CONST_59: Self = T::CONST_58 + T::CONST_1;
    const CONST_60: Self = T::CONST_59 + T::CONST_1;
    const CONST_61: Self = T::CONST_60 + T::CONST_1;
    const CONST_62: Self = T::CONST_61 + T::CONST_1;
    const CONST_63: Self = T::CONST_62 + T::CONST_1;
    const CONST_64: Self = T::CONST_63 + T::CONST_1;
    const CONST_65: Self = T::CONST_64 + T::CONST_1;
    const CONST_66: Self = T::CONST_65 + T::CONST_1;
    const CONST_67: Self = T::CONST_66 + T::CONST_1;
    const CONST_68: Self = T::CONST_67 + T::CONST_1;
    const CONST_69: Self = T::CONST_68 + T::CONST_1;
    const CONST_70: Self = T::CONST_69 + T::CONST_1;
    const CONST_71: Self = T::CONST_70 + T::CONST_1;
    const CONST_72: Self = T::CONST_71 + T::CONST_1;
    const CONST_73: Self = T::CONST_72 + T::CONST_1;
    const CONST_74: Self = T::CONST_73 + T::CONST_1;
    const CONST_75: Self = T::CONST_74 + T::CONST_1;
    const CONST_76: Self = T::CONST_75 + T::CONST_1;
    const CONST_77: Self = T::CONST_76 + T::CONST_1;
    const CONST_78: Self = T::CONST_77 + T::CONST_1;
    const CONST_79: Self = T::CONST_78 + T::CONST_1;
    const CONST_80: Self = T::CONST_79 + T::CONST_1;
    const CONST_81: Self = T::CONST_80 + T::CONST_1;
    const CONST_82: Self = T::CONST_81 + T::CONST_1;
    const CONST_83: Self = T::CONST_82 + T::CONST_1;
    const CONST_84: Self = T::CONST_83 + T::CONST_1;
    const CONST_85: Self = T::CONST_84 + T::CONST_1;
    const CONST_86: Self = T::CONST_85 + T::CONST_1;
    const CONST_87: Self = T::CONST_86 + T::CONST_1;
    const CONST_88: Self = T::CONST_87 + T::CONST_1;
    const CONST_89: Self = T::CONST_88 + T::CONST_1;
    const CONST_90: Self = T::CONST_89 + T::CONST_1;
    const CONST_91: Self = T::CONST_90 + T::CONST_1;
    const CONST_92: Self = T::CONST_91 + T::CONST_1;
    const CONST_93: Self = T::CONST_92 + T::CONST_1;
    const CONST_94: Self = T::CONST_93 + T::CONST_1;
    const CONST_95: Self = T::CONST_94 + T::CONST_1;
    const CONST_96: Self = T::CONST_95 + T::CONST_1;
    const CONST_97: Self = T::CONST_96 + T::CONST_1;
    const CONST_98: Self = T::CONST_97 + T::CONST_1;
    const CONST_99: Self = T::CONST_98 + T::CONST_1;
    const CONST_100: Self = T::CONST_99 + T::CONST_1;
    const CONST_101: Self = T::CONST_100 + T::CONST_1;
    const CONST_102: Self = T::CONST_101 + T::CONST_1;
    const CONST_103: Self = T::CONST_102 + T::CONST_1;
    const CONST_104: Self = T::CONST_103 + T::CONST_1;
    const CONST_105: Self = T::CONST_104 + T::CONST_1;
    const CONST_106: Self = T::CONST_105 + T::CONST_1;
    const CONST_107: Self = T::CONST_106 + T::CONST_1;
    const CONST_108: Self = T::CONST_107 + T::CONST_1;
    const CONST_109: Self = T::CONST_108 + T::CONST_1;
    const CONST_110: Self = T::CONST_109 + T::CONST_1;
    const CONST_111: Self = T::CONST_110 + T::CONST_1;
    const CONST_112: Self = T::CONST_111 + T::CONST_1;
    const CONST_113: Self = T::CONST_112 + T::CONST_1;
    const CONST_114: Self = T::CONST_113 + T::CONST_1;
    const CONST_115: Self = T::CONST_114 + T::CONST_1;
    const CONST_116: Self = T::CONST_115 + T::CONST_1;
    const CONST_117: Self = T::CONST_116 + T::CONST_1;
    const CONST_118: Self = T::CONST_117 + T::CONST_1;
    const CONST_119: Self = T::CONST_118 + T::CONST_1;
    const CONST_120: Self = T::CONST_119 + T::CONST_1;
    const CONST_121: Self = T::CONST_120 + T::CONST_1;
    const CONST_122: Self = T::CONST_121 + T::CONST_1;
    const CONST_123: Self = T::CONST_122 + T::CONST_1;
    const CONST_124: Self = T::CONST_123 + T::CONST_1;
    const CONST_125: Self = T::CONST_124 + T::CONST_1;
    const CONST_126: Self = T::CONST_125 + T::CONST_1;
    const CONST_127: Self = T::CONST_126 + T::CONST_1;
}
