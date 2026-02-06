#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]

use crate::math::{Bounded, ConstZero};
#[cfg(feature = "test")]
/// A list of tests
pub mod tests;

/// A helper trait for the people who are used to `.signum()`
pub const trait SigNum {
    /// Returns the sign of the number -> -1, 0, 1
    #[must_use]
    fn signum(self) -> Self;
}
impl<T: [const] Sign> const SigNum for T {
    fn signum(self) -> Self {
        self.sign()
    }
}

/// A trait for numbers to support `sign()`
pub const trait Sign {
    /// Returns the sign of the number -> -1, 0, 1
    #[must_use]
    fn sign(self) -> Self;
}

macro_rules! impl_sign_u {
    ($($t:ty),*) => {
        // See those two spaces between the `const` and `Sign`?
        $(impl const  Sign for $t {
            fn sign(self) -> Self {
                if self > 0 { 1  }
                else { 0  }
            }
        })*
    };
}
macro_rules! impl_sign {
    ($($t:ty),*) => {
        // See those two spaces between the `const` and `Sign`?
        $(impl const  Sign for $t {
            fn sign(self) -> Self {
                if self > 0 { 1  }
                else if self < 0  { -1 }
                else { 0  }
            }
        })*
    };
}
macro_rules! impl_sign_float {
    ($($t:ty),*) => {
        // See those two spaces between the `const` and `Sign`?
        $(impl const  Sign for $t {
            fn sign(self) -> Self {
                if self > 0.0 { 1.0  }
                else if self < 0.0  { -1.0 }
                else { 0.0  }
            }
        })*
    };
}

impl_sign!(i8, i16, i32, i64, i128, isize);
impl_sign_u!(u8, u16, u32, u64, u128, usize);
impl_sign_float!(f16, f32, f64, f128);

// use core::f32;
// use core::convert::TryFrom;

// pub fn from_u8<T: TryFrom<u8>>(value: u8) -> T {
//     T::try_from(value).ok().expect("constant u8 conversion failed")
// }

macro_rules! impl_sqrt {
    ($($t:ty),*) => {
        $(
            impl Sqrt for $t {
                fn sqrt(self) -> Self {
                    // There has to be a better way
                    core::f64::math::sqrt(self as f64) as Self
                }
            }
        )*
    };
}

impl_sqrt!(i8);
impl_sqrt!(i16);
impl_sqrt!(i32);
impl_sqrt!(i64);
impl_sqrt!(i128);
impl_sqrt!(isize);
impl_sqrt!(u8);
impl_sqrt!(u16);
impl_sqrt!(u32);
impl_sqrt!(u64);
impl_sqrt!(u128);
impl_sqrt!(usize);

// use crate::prelude::{U1, U2, U4};
/// Core trait: addition of a signed number to an unsigned number (returning)
pub const trait AddSignLogic<T> {
    /// Simple addition of the possibly negative number (returning)
    #[must_use]
    fn add_sign(&self, value: T) -> Self;

    /// Addition of the possibly negative number without over/underflowing (returning)
    #[must_use]
    fn saturated_add_sign(&self, value: T) -> Self;
}

/// Trait for mutating setters, automatically implemented using `AddSignLogic`
pub const trait AddSignSetter<T>: AddSignLogic<T> {
    /// Simple addition of the possibly negative number (mutating)
    fn set_add_sign(&mut self, value: T);

    /// Addition of the possibly negative number without over/underflowing (mutating)
    fn set_saturated_add_sign(&mut self, value: T);
}

impl<U, S> AddSignLogic<S> for U
where
    U: Copy
        + WrapOps
        + SaturatingAdd<Output = U>
        + SaturatingSub<Output = U>
        + core::ops::Add<U, Output = U>
        + Bounded
        + ConstZero
        + core::ops::Sub<U, Output = U>
        + TryFromPatch<S>,
    S: Copy + core::cmp::PartialOrd + Abs + ConstZero,
{
    fn add_sign(&self, value: S) -> Self {
        if value >= S::ZERO {
            U::try_from_value(value).map_or_else(
                || self.wrapping_add(U::max_value()),
                |pos_val| self.wrapping_add(pos_val),
            )
        } else {
            U::try_from_value(value.abs()).map_or_else(
                || self.wrapping_sub(U::max_value()),
                |sub_val| self.wrapping_sub(sub_val),
            )
        }
    }

    fn saturated_add_sign(&self, value: S) -> Self {
        if value >= S::ZERO {
            U::try_from_value(value).map_or_else(U::max_value, |pos_val| {
                self.saturating_add(pos_val)
            })
        } else {
            U::try_from_value(value.abs())
                .map_or_else(|| U::ZERO, |sub_val| self.saturating_sub(sub_val))
        }
    }
}

impl<U, S> AddSignSetter<S> for U
where
    U: AddSignLogic<S>,
{
    fn set_add_sign(&mut self, value: S) {
        *self = self.add_sign(value);
    }

    fn set_saturated_add_sign(&mut self, value: S) {
        *self = self.saturated_add_sign(value);
    }
}

/// Trait for mapping between signed and unsigned integer types
pub const trait MapToSign {
    /// Signed Version
    type Signed;
    /// Unsigned Version
    type Unsigned;

    /// Map from unsigned back to signed by flipping the sign bit
    fn map_non_sign_to_sign(self) -> Self::Signed;
}

/// Trait for mapping between signed and unsigned integer types
pub const trait MapToUnSign {
    /// Signed Version
    type Signed;
    /// Unsigned Version
    type Unsigned;

    /// Map from signed to unsigned by flipping the sign bit
    fn map_sign_to_non_sign(self) -> Self::Unsigned;
}
use crate::{math::ConstOne, prelude::TryFromPatch};
/// Macro to implement `SignMapping` for integer type pairs
macro_rules! impl_sign_mapping {
    ($signed:ty, $unsigned:ty) => {
        impl const MapToUnSign for $signed {
            type Signed = $signed;
            type Unsigned = $unsigned;

            #[allow(clippy::cast_sign_loss)]
            #[allow(trivial_numeric_casts)]
            fn map_sign_to_non_sign(self) -> Self::Unsigned {
                (self as Self::Unsigned).wrapping_add(
                    <$unsigned>::MAX / (<$unsigned>::ONE + <$unsigned>::ONE)
                        + <$unsigned>::ONE,
                )
            }
        }
        impl const MapToSign for $unsigned {
            type Signed = $signed;
            type Unsigned = $unsigned;

            #[allow(clippy::cast_possible_wrap)]
            #[allow(trivial_numeric_casts)]
            fn map_non_sign_to_sign(self) -> Self::Signed {
                self.wrapping_sub(
                    <$unsigned>::MAX / (<$unsigned>::ONE + <$unsigned>::ONE)
                        + <$unsigned>::ONE,
                ) as Self::Signed
            }
        }
    };
}

// Implement for all standard integer pairs
impl_sign_mapping!(i8, u8);
impl_sign_mapping!(i16, u16);
impl_sign_mapping!(i32, u32);
impl_sign_mapping!(i64, u64);
impl_sign_mapping!(i128, u128);
impl_sign_mapping!(isize, usize);
impl_sign_mapping!(f16, f16);
impl_sign_mapping!(f32, f32);
impl_sign_mapping!(f64, f64);
impl_sign_mapping!(f128, f128);
/// A unified trait providing wrapping arithmetic operations for all numeric types
pub const trait WrapOps {
    /// Wrapping addition. Computes `self + other`, wrapping around at the boundary of the type.
    #[must_use]
    fn wrapping_add(self, other: Self) -> Self;

    /// Wrapping subtraction. Computes `self - other`, wrapping around at the boundary of the type.
    #[must_use]
    fn wrapping_sub(self, other: Self) -> Self;

    /// Wrapping multiplication. Computes `self * other`, wrapping around at the boundary of the type.
    #[must_use]
    fn wrapping_mul(self, other: Self) -> Self;
}

macro_rules! impl_wrap_ops_int {
    ($($t:ty)*) => ($(
        impl const WrapOps for $t {
            #[inline]
            fn wrapping_add(self, other: Self) -> Self {
                <$t>::wrapping_add(self, other)
            }

            #[inline]
            fn wrapping_sub(self, other: Self) -> Self {
                <$t>::wrapping_sub(self, other)
            }

            #[inline]
            fn wrapping_mul(self, other: Self) -> Self {
                <$t>::wrapping_mul(self, other)
            }
        }
    )*)
}

macro_rules! impl_wrap_ops_float {
    ($($t:ty)*) => ($(
        impl const WrapOps for $t {
            #[inline]
            fn wrapping_add(self, other: Self) -> Self {
                self + other
            }

            #[inline]
            fn wrapping_sub(self, other: Self) -> Self {
                self - other
            }

            #[inline]
            fn wrapping_mul(self, other: Self) -> Self {
                self * other
            }
        }
    )*)
}

impl_wrap_ops_int! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }
impl_wrap_ops_float! {f16 f32 f64 f128}

// pub const trait Modular<Rhs = Self> {
//     type Output;
//     fn modular(&self, modulus: Rhs) -> Self::Output;
// }

// macro_rules! impl_modular_unsigned {
//     ($($t:ty),*) => {
//         $(
//             impl Modular for $t {
//                 type Output = $t;

//                 fn modular(&self, modulus: $t) -> $t {
//                     if modulus == 0 {
//                         panic!("Division by zero: modulus cannot be zero");
//                     }
//                     self % modulus
//                 }
//             }
//         )*
//     };
// }

// macro_rules! impl_modular_signed {
//     ($($t:ty),*) => {
//         $(
//             impl Modular for $t {
//                 type Output = $t;

//                 fn modular(&self, modulus: $t) -> $t {
//                     if modulus == 0 {
//                         panic!("Division by zero: modulus cannot be zero");
//                     }
//                     // Use rem_euclid to ensure positive remainder for negative numbers
//                     self.rem_euclid(modulus.abs())
//                 }
//             }
//         )*
//     };
// }

// macro_rules! impl_modular_float {
//     ($($t:ty),*) => {
//         $(
//             impl Modular for $t {
//                 type Output = $t;

//                 fn modular(&self, modulus: $t) -> $t {
//                     if modulus == 0.0 || modulus.is_nan() || self.is_nan() {
//                         panic!("Invalid modular operation: NaN or zero modulus");
//                     }
//                     // Use rem_euclid for consistent positive results
//                     self.rem_euclid(modulus.abs())
//                 }
//             }
//         )*
//     };
// }

// impl_modular_unsigned!(u8, u16, u32, u64, u128, usize, U4);
// impl_modular_signed!(i8, i16, i32, i64, i128, isize);
// impl_modular_float!(f32, f64);

/// Round a value to its nearest neighbor
pub const trait Round {
    #[must_use]
    /// Round the current value to its nearest neighbor and return the result
    fn round(self) -> Self;
}
impl const Round for f16 {
    fn round(self) -> Self {
        core::intrinsics::roundf16(self)
    }
}
impl const Round for f32 {
    fn round(self) -> Self {
        core::f32::math::round(self)
    }
}
impl const Round for f64 {
    fn round(self) -> Self {
        core::f64::math::round(self)
    }
}
impl const Round for f128 {
    fn round(self) -> Self {
        core::intrinsics::roundf128(self)
    }
}
impl const Round for u8 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for u16 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for u64 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for u32 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for u128 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for usize {
    fn round(self) -> Self {
        self
    }
}

impl const Round for i8 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for i16 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for i64 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for i32 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for i128 {
    fn round(self) -> Self {
        self
    }
}
impl const Round for isize {
    fn round(self) -> Self {
        self
    }
}
/// Take the sqrt of the given value
pub const trait Sqrt {
    #[must_use]
    /// Take the sqrt of the given value and return the result
    fn sqrt(self) -> Self;
}
impl Sqrt for f16 {
    fn sqrt(self) -> Self {
        {
            core::intrinsics::sqrtf16(self)
        }
    }
}
impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        core::f32::math::sqrt(self)
    }
}
impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        core::f64::math::sqrt(self)
    }
}
impl Sqrt for f128 {
    fn sqrt(self) -> Self {
        core::intrinsics::sqrtf128(self)
    }
}

/// Take the abs of the given value
pub const trait Abs {
    #[must_use]
    /// Take the sqrt of the given value and return the result
    fn abs(self) -> Self;
}
impl const Abs for f16 {
    fn abs(self) -> Self {
        Self::from_bits(self.to_bits() & !(1 << 15))
    }
}
impl const Abs for f32 {
    fn abs(self) -> Self {
        core::intrinsics::fabsf32(self)
    }
}
impl const Abs for f64 {
    fn abs(self) -> Self {
        core::intrinsics::fabsf64(self)
    }
}
impl const Abs for f128 {
    fn abs(self) -> Self {
        Self::from_bits(self.to_bits() & !(1 << 127))
    }
}
impl const Abs for i8 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl const Abs for i16 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl const Abs for i32 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl const Abs for i64 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl const Abs for i128 {
    fn abs(self) -> Self {
        self.abs()
    }
}
impl const Abs for isize {
    fn abs(self) -> Self {
        self.abs()
    }
}
/// Find the next number that is divisible by two
pub const trait NextPowerOfTwo {
    /// Find the next number that is divisible by two
    #[must_use]
    fn next_power_of_two(self) -> Self;
}

impl const NextPowerOfTwo for u8 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for u16 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for u32 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for u64 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for u128 {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for usize {
    fn next_power_of_two(self) -> Self {
        Self::next_power_of_two(self)
    }
}

impl const NextPowerOfTwo for i8 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl const NextPowerOfTwo for i16 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl const NextPowerOfTwo for i32 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl const NextPowerOfTwo for i64 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl const NextPowerOfTwo for i128 {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl const NextPowerOfTwo for isize {
    fn next_power_of_two(self) -> Self {
        self.unsigned_abs().next_power_of_two() as Self
    }
}

impl NextPowerOfTwo for f16 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}
impl NextPowerOfTwo for f32 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}

impl NextPowerOfTwo for f64 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}
impl NextPowerOfTwo for f128 {
    fn next_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        }
    }
}

/// Find the next number that is a power of two and return both the value and the exponent
pub const trait NextPowerOfTwoWithExponent {
    /// Find the next power of two and return (2^x, x)
    #[must_use]
    fn next_power_of_two_with_exponent(self) -> (Self, Self)
    where
        Self: core::marker::Sized;
}

impl const NextPowerOfTwoWithExponent for u8 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for u16 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for u32 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros();
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for u64 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for u128 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for usize {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = Self::next_power_of_two(self);
        let exponent = power.trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for i8 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for i16 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for i32 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for i64 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for i128 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl const NextPowerOfTwoWithExponent for isize {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = self.unsigned_abs().next_power_of_two() as Self;
        let exponent = power.unsigned_abs().trailing_zeros() as Self;
        (power, exponent)
    }
}

impl NextPowerOfTwoWithExponent for f16 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}

impl NextPowerOfTwoWithExponent for f32 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}

impl NextPowerOfTwoWithExponent for f64 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}

impl NextPowerOfTwoWithExponent for f128 {
    fn next_power_of_two_with_exponent(self) -> (Self, Self) {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        let exponent = power.log2();
        (power, exponent)
    }
}
/// Find the exponent x where 2^x is the next power of two
pub const trait NextPowerOfTwoExponent {
    /// Find the exponent x where 2^x is the next power of two
    #[must_use]
    fn next_power_of_two_exponent(self) -> Self;
}

impl const NextPowerOfTwoExponent for u8 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for u16 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for u32 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros()
    }
}

impl const NextPowerOfTwoExponent for u64 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for u128 {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for usize {
    fn next_power_of_two_exponent(self) -> Self {
        Self::next_power_of_two(self).trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for i8 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for i16 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for i32 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for i64 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for i128 {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl const NextPowerOfTwoExponent for isize {
    fn next_power_of_two_exponent(self) -> Self {
        self.unsigned_abs().next_power_of_two().trailing_zeros() as Self
    }
}

impl NextPowerOfTwoExponent for f16 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

impl NextPowerOfTwoExponent for f32 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

impl NextPowerOfTwoExponent for f64 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

impl NextPowerOfTwoExponent for f128 {
    fn next_power_of_two_exponent(self) -> Self {
        let power = if self <= 1.0 {
            1.0
        } else {
            self.log2().ceil().exp2()
        };
        power.log2()
    }
}

/// Calculate what 2^x will result in the given value
pub const trait Log2 {
    /// Calculate what 2^x will result in the given value
    #[must_use]
    fn log2(self) -> Self;
}

// TODO: Implement Log2 for all U and I types

impl Log2 for f16 {
    fn log2(self) -> Self {
        core::intrinsics::log2f16(self)
    }
}
impl Log2 for f32 {
    fn log2(self) -> Self {
        core::intrinsics::log2f32(self)
    }
}

impl Log2 for f64 {
    fn log2(self) -> Self {
        core::intrinsics::log2f64(self)
    }
}
impl Log2 for f128 {
    fn log2(self) -> Self {
        core::intrinsics::log2f128(self)
    }
}

/// Calculate what 10^x will result in the given value
pub const trait Log10 {
    /// Calculate what 10^x will result in the given value
    #[must_use]
    fn log10(self) -> Self;
}

// TODO: Implement Log10 for all U and I types

impl Log10 for f16 {
    fn log10(self) -> Self {
        core::intrinsics::log10f16(self)
    }
}
impl Log10 for f32 {
    fn log10(self) -> Self {
        core::intrinsics::log10f32(self)
    }
}

impl Log10 for f64 {
    fn log10(self) -> Self {
        core::intrinsics::log10f64(self)
    }
}
impl Log10 for f128 {
    fn log10(self) -> Self {
        core::intrinsics::log10f128(self)
    }
}

/// Calculate what e^x will result in the given value
pub const trait Log {
    /// Calculate what e^x will result in the given value
    #[must_use]
    fn log(self) -> Self;
}

// TODO: Implement Log for all U and I types

impl Log for f16 {
    fn log(self) -> Self {
        core::intrinsics::logf16(self)
    }
}
impl Log for f32 {
    fn log(self) -> Self {
        core::intrinsics::logf32(self)
    }
}

impl Log for f64 {
    fn log(self) -> Self {
        core::intrinsics::logf64(self)
    }
}
impl Log for f128 {
    fn log(self) -> Self {
        core::intrinsics::logf128(self)
    }
}

/// Calculate 2^x
pub const trait Exp2 {
    ///  Calculate 2^x
    #[must_use]
    fn exp2(self) -> Self;
}

// TODO: Implement Exp for all U and I types

impl Exp2 for f16 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f16(self)
    }
}
impl Exp2 for f32 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f32(self)
    }
}

impl Exp2 for f64 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f64(self)
    }
}
impl Exp2 for f128 {
    fn exp2(self) -> Self {
        core::intrinsics::exp2f128(self)
    }
}

/// ¯\_(ツ)_/¯
pub const trait Fract {
    /// ¯\_(ツ)_/¯
    #[must_use]
    fn fract(self) -> Self;
}

// TODO: Implement Fract for all U and I types

impl Fract for f16 {
    fn fract(self) -> Self {
        self - self.trunc()
    }
}
impl Fract for f32 {
    fn fract(self) -> Self {
        core::f32::math::fract(self)
    }
}

impl Fract for f64 {
    fn fract(self) -> Self {
        core::f64::math::fract(self)
    }
}
impl Fract for f128 {
    fn fract(self) -> Self {
        self - self.trunc()
    }
}

/// ¯\_(ツ)_/¯
pub const trait Trunc {
    /// ¯\_(ツ)_/¯
    #[must_use]
    fn tunc(self) -> Self;
}

// TODO: Implement Fract for all U and I types

impl Trunc for f16 {
    fn tunc(self) -> Self {
        core::intrinsics::truncf16(self)
    }
}
impl Trunc for f32 {
    fn tunc(self) -> Self {
        core::intrinsics::truncf32(self)
    }
}

impl Trunc for f64 {
    fn tunc(self) -> Self {
        core::intrinsics::truncf64(self)
    }
}
impl Trunc for f128 {
    fn tunc(self) -> Self {
        core::intrinsics::truncf128(self)
    }
}

/// Saturating addition
pub const trait SaturatingAdd<Rhs = Self> {
    /// The resulting type after applying saturating addition
    type Output;

    #[must_use]
    /// Add two values, saturating at the numeric bounds instead of overflowing
    fn saturating_add(self, rhs: Rhs) -> Self::Output;
}

impl const SaturatingAdd for i8 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for i16 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for i32 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for i64 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for i128 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for isize {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for u8 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for u16 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for u32 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for u64 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for u128 {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

impl const SaturatingAdd for usize {
    type Output = Self;
    fn saturating_add(self, rhs: Self) -> Self::Output {
        self.saturating_add(rhs)
    }
}

/// Saturating subtraction
pub const trait SaturatingSub<Rhs = Self> {
    /// The resulting type after applying saturating subtraction
    type Output;

    #[must_use]
    /// Subtract two values, saturating at the numeric bounds instead of overflowing
    fn saturating_sub(self, rhs: Rhs) -> Self::Output;
}

impl const SaturatingSub for i8 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for i16 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for i32 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for i64 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for i128 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for isize {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for u8 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for u16 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for u32 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for u64 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for u128 {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

impl const SaturatingSub for usize {
    type Output = Self;
    fn saturating_sub(self, rhs: Self) -> Self::Output {
        self.saturating_sub(rhs)
    }
}

/// Saturating multiplication
pub const trait SaturatingMul<Rhs = Self> {
    /// The resulting type after applying saturating multiplication
    type Output;

    #[must_use]
    /// Multiply two values, saturating at the numeric bounds instead of overflowing
    fn saturating_mul(self, rhs: Rhs) -> Self::Output;
}

impl const SaturatingMul for i8 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for i16 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for i32 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for i64 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for i128 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for isize {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for u8 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for u16 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for u32 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for u64 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for u128 {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

impl const SaturatingMul for usize {
    type Output = Self;
    fn saturating_mul(self, rhs: Self) -> Self::Output {
        self.saturating_mul(rhs)
    }
}

/// Saturating absolute value
pub const trait SaturatingAbs {
    /// The resulting type after applying saturating absolute value
    type Output;

    #[must_use]
    /// Take the absolute value, saturating at the numeric bounds instead of overflowing
    fn saturating_abs(self) -> Self::Output;
}

impl const SaturatingAbs for i8 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

impl const SaturatingAbs for i16 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

impl const SaturatingAbs for i32 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

impl const SaturatingAbs for i64 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

impl const SaturatingAbs for i128 {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

impl const SaturatingAbs for isize {
    type Output = Self;
    fn saturating_abs(self) -> Self::Output {
        self.saturating_abs()
    }
}

/// Saturating negation
pub const trait SaturatingNeg {
    /// The resulting type after applying saturating negation
    type Output;

    #[must_use]
    /// Negate the value, saturating at the numeric bounds instead of overflowing
    fn saturating_neg(self) -> Self::Output;
}

impl const SaturatingNeg for i8 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

impl const SaturatingNeg for i16 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

impl const SaturatingNeg for i32 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

impl const SaturatingNeg for i64 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

impl const SaturatingNeg for i128 {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}

impl const SaturatingNeg for isize {
    type Output = Self;
    fn saturating_neg(self) -> Self::Output {
        self.saturating_neg()
    }
}
/// Check if a number is a power of two
pub const trait IsPowerOfTwo {
    /// Returns true if the number is a power of two
    #[must_use]
    #[allow(clippy::wrong_self_convention)]
    fn is_power_of_two(self) -> bool;
}

impl const IsPowerOfTwo for u8 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for u16 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for u32 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for u64 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for u128 {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for usize {
    fn is_power_of_two(self) -> bool {
        Self::is_power_of_two(self)
    }
}

impl const IsPowerOfTwo for i8 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl const IsPowerOfTwo for i16 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl const IsPowerOfTwo for i32 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl const IsPowerOfTwo for i64 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl const IsPowerOfTwo for i128 {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl const IsPowerOfTwo for isize {
    fn is_power_of_two(self) -> bool {
        self > 0 && self.unsigned_abs().is_power_of_two()
    }
}

impl IsPowerOfTwo for f16 {
    #[allow(clippy::float_cmp)]
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

impl IsPowerOfTwo for f32 {
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

impl IsPowerOfTwo for f64 {
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

impl IsPowerOfTwo for f128 {
    #[allow(clippy::float_cmp)]
    fn is_power_of_two(self) -> bool {
        self > 0.0 && self.is_finite() && self.log2().fract() == 0.0
    }
}

/// Get the previous power of two (rounding down)
pub const trait PrevPowerOfTwo {
    /// Find the previous power of two (the largest power of two less than or equal to self)
    #[must_use]
    fn prev_power_of_two(self) -> Self;
}

impl const PrevPowerOfTwo for u8 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for u16 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for u32 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for u64 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for u128 {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for usize {
    fn prev_power_of_two(self) -> Self {
        if self == 0 {
            0
        } else {
            1 << (Self::BITS - 1 - self.leading_zeros())
        }
    }
}

impl const PrevPowerOfTwo for i8 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl const PrevPowerOfTwo for i16 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl const PrevPowerOfTwo for i32 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl const PrevPowerOfTwo for i64 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl const PrevPowerOfTwo for i128 {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl const PrevPowerOfTwo for isize {
    fn prev_power_of_two(self) -> Self {
        if self <= 0 {
            0
        } else {
            self.unsigned_abs().prev_power_of_two() as Self
        }
    }
}

impl PrevPowerOfTwo for f16 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}

impl PrevPowerOfTwo for f32 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}

impl PrevPowerOfTwo for f64 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}

impl PrevPowerOfTwo for f128 {
    fn prev_power_of_two(self) -> Self {
        if self <= 1.0 {
            1.0
        } else {
            self.log2().floor().exp2()
        }
    }
}
/// Round down the current number
pub const trait Floor {
    #[must_use]
    /// Round down the current number
    fn floor(self) -> Self;
}

/// Round up the current number
pub const trait Ceil {
    #[must_use]
    /// Round up the current number
    fn ceil(self) -> Self;
}

impl const Floor for f16 {
    fn floor(self) -> Self {
        core::intrinsics::floorf16(self)
    }
}
impl const Ceil for f16 {
    fn ceil(self) -> Self {
        core::intrinsics::ceilf16(self)
    }
}

impl const Floor for f32 {
    fn floor(self) -> Self {
        core::f32::math::floor(self)
    }
}
impl const Ceil for f32 {
    fn ceil(self) -> Self {
        core::f32::math::ceil(self)
    }
}

impl const Floor for f64 {
    fn floor(self) -> Self {
        core::f64::math::floor(self)
    }
}
impl const Ceil for f64 {
    fn ceil(self) -> Self {
        core::f64::math::ceil(self)
    }
}

impl const Floor for f128 {
    fn floor(self) -> Self {
        core::intrinsics::floorf128(self)
    }
}
impl const Ceil for f128 {
    fn ceil(self) -> Self {
        core::intrinsics::ceilf128(self)
    }
}

macro_rules! impl_int {
    ($($t:ty),*) => {
        $(
            impl const Floor for $t {
                fn floor(self) -> Self {
                    self
                }
            }
            impl const Ceil for $t {
                fn ceil(self) -> Self {
                    self
                }
            }
        )*
    };
}

impl_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

/// Trait providing trigonometric functions
pub const trait MathRotation {
    /// Computes the sine of self (in radians)
    #[must_use]
    fn sin(self) -> Self;

    /// Computes the cosine of self (in radians)
    #[must_use]
    fn cos(self) -> Self;
}

impl MathRotation for f16 {
    fn sin(self) -> Self {
        core::intrinsics::sinf16(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf16(self)
    }
}

impl MathRotation for f32 {
    fn sin(self) -> Self {
        core::intrinsics::sinf32(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf32(self)
    }
}

impl MathRotation for f64 {
    fn sin(self) -> Self {
        core::intrinsics::sinf64(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf64(self)
    }
}

impl MathRotation for f128 {
    fn sin(self) -> Self {
        core::intrinsics::sinf128(self)
    }
    fn cos(self) -> Self {
        core::intrinsics::cosf128(self)
    }
}

/// Trait providing fused multiply-add operation
pub const trait MulAdd {
    /// Computes `(self * a) + b` with only one rounding error if possible
    #[must_use]
    fn mul_add(self, a: Self, b: Self) -> Self;
}

impl const MulAdd for f16 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::intrinsics::fmaf16(self, a, b)
    }
}

impl const MulAdd for f32 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::f32::math::mul_add(self, a, b)
    }
}

impl const MulAdd for f64 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::f64::math::mul_add(self, a, b)
    }
}

impl const MulAdd for f128 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        core::intrinsics::fmaf128(self, a, b)
    }
}
macro_rules! impl_mul_add_plain {
    ($($t:ty),* $(,)?) => {
        $(
            impl const MulAdd for $t {
                #[inline(always)]
                fn mul_add(self, a: Self, b: Self) -> Self {
                    self * a + b
                }
            }
        )*
    };
}

impl_mul_add_plain!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize,
);

/// Clamp a value between a minimum and maximum bound
///
/// The function isn't called `clamp` for compatibility with `core::ops::Ord` which numbers like `core::f64` do not implement
pub const trait Clamp {
    /// Restricts a value to be within a specified range [min, max]
    #[must_use]
    fn clamped(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_clamp_int {
    ($($t:ty),*) => {
        $(
            impl const Clamp for $t {
                fn clamped(self, min: Self, max: Self) -> Self {
                    if self < min {
                        min
                    } else if self > max {
                        max
                    } else {
                        self
                    }
                }
            }
        )*
    };
}

macro_rules! impl_clamp_float {
    ($($t:ty),*) => {
        $(
            impl const Clamp for $t {
                fn clamped(self, min: Self, max: Self) -> Self {
                    if self.is_nan() || min.is_nan() || max.is_nan() {
                        return self;
                    }

                    if self < min {
                        min
                    } else if self > max {
                        max
                    } else {
                        self
                    }
                }
            }
        )*
    };
}

impl_clamp_int!(i8, i16, i32, i64, i128, isize);

impl_clamp_int!(u8, u16, u32, u64, u128, usize);

impl_clamp_float!(f16, f32, f64, f128);

/// Get the hypotenuse of a value
pub const trait Hypot {
    #[must_use]
    /// Get the hypotenuse of a value
    fn hypot(self, other: Self) -> Self;
}
impl<
        T: Copy + Sqrt + core::ops::Add<Output = T> + core::ops::Mul<Output = T>,
    > Hypot for T
{
    fn hypot(self, other: Self) -> Self {
        (self * self + other * other).sqrt()
    }
}

// impl Hypot for f16 {
//     fn hypot(self, other: Self) -> Self {
//         cmath::hypotf(self as f32, other as f32) as f16
//     }
// }

// impl Hypot for f32 {
//     fn hypot(self, other: Self) -> Self {
//         #[cfg(feature = "std")]
//         return self.hypot(other);
//         #[cfg(not(feature = "std"))]
//         (self * self + other * other).sqrt()
//     }
// }

// impl Hypot for f16 {
//     fn hypot(self, other: Self) -> Self {
//         #[cfg(feature = "std")]
//         return self.hypot(other);
//         #[cfg(not(feature = "std"))]
//         (self * self + other * other).sqrt()
//     }
// }

// impl Hypot for f64 {
//     fn hypot(self, other: Self) -> Self {
//         #[cfg(feature = "std")]
//         return self.hypot(other);
//         #[cfg(not(feature = "std"))]
//         (self * self + other * other).sqrt()
//     }
// }

// impl Hypot for f128 {
//     fn hypot(self, other: Self) -> Self {
//         #[cfg(feature = "std")]
//         return self.hypot(other);
//         #[cfg(not(feature = "std"))]
//         (self * self + other * other).sqrt()
//     }
// }

/// Returns the exp or exp2 of a value
pub trait Exp {
    #[must_use]
    /// Returns e^self.
    fn exp(self) -> Self;
}
impl Exp for f16 {
    fn exp(self) -> Self {
        core::intrinsics::expf16(self)
    }
}

impl Exp for f32 {
    fn exp(self) -> Self {
        core::intrinsics::expf32(self)
    }
}

impl Exp for f64 {
    fn exp(self) -> Self {
        core::intrinsics::expf64(self)
    }
}

impl Exp for f128 {
    fn exp(self) -> Self {
        core::intrinsics::expf128(self)
    }
}
