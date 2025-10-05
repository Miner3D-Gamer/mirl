#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
/// A trait for numbers to support `sign()`
#[const_trait]
pub trait Sign {
    /// Returns the sign of the number -> -1, 0, 1
    #[must_use]
    fn sign(self) -> Self;
}

macro_rules! impl_sign {
    ($($t:ty),*) => {
        $(impl Sign for $t {
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
        $(impl Sign for $t {
            fn sign(self) -> Self {
                if self > 0.0 { 1.0  }
                else if self < 0.0  { -1.0 }
                else { 0.0  }
            }
        })*
    };
}

impl_sign!(i8, i16, i32, i64, i128, isize);
impl_sign_float!(f32, f64);

// use core::f32;
// use std::convert::TryFrom;

// pub fn from_u8<T: TryFrom<u8>>(value: u8) -> T {
//     T::try_from(value).ok().expect("constant u8 conversion failed")
// }
/// A trait for making numbers support `sqrt()`
#[const_trait]
pub trait Sqrt {
    /// Return the square root of a number
    #[must_use]
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt {
    ($($t:ty),*) => {
        $(
            impl Sqrt for $t {
                fn sqrt(self) -> Self {
                    // There has to be a better way
                    (self as f64).sqrt() as Self
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

use num_traits::{bounds, cast, identities, sign};

#[const_trait]
/// Add a signed number to a non signed number
pub trait AddSign<T> {
    /// Simple addition of the possibly negative number (mutating)
    fn set_add_sign(&mut self, value: T);
    /// Addition of the possibly negative number without over/underflowing (mutating)
    fn set_saturated_add_sign(&mut self, value: T);
    /// Simple addition of the possibly negative number (returning)
    #[must_use]
    fn add_sign(&self, value: T) -> Self;
    /// Addition of the possibly negative number without over/underflowing (returning)
    #[must_use]
    fn saturated_add_sign(&self, value: T) -> Self;
}

impl<U, S> AddSign<S> for U
where
    U: sign::Unsigned
        + bounds::Bounded
        + identities::Zero
        + Copy
        + num_traits::WrappingAdd
        + num_traits::WrappingSub
        + num_traits::SaturatingAdd
        + num_traits::SaturatingSub,
    S: sign::Signed + Copy + std::cmp::PartialOrd + num_traits::NumCast,
    U: std::ops::Add<U, Output = U>
        + std::ops::Sub<U, Output = U>
        + num_traits::NumCast,
{
    fn set_add_sign(&mut self, value: S) {
        if value >= S::zero() {
            if let Some(pos_val) = cast::cast::<S, U>(value) {
                *self = self.wrapping_add(&pos_val);
            } else {
                *self = self.wrapping_add(&U::max_value());
            }
        } else {
            let abs_val = value.abs();
            if let Some(sub_val) = cast::cast::<S, U>(abs_val) {
                *self = self.wrapping_sub(&sub_val);
            } else {
                *self = self.wrapping_sub(&U::max_value());
            }
        }
    }

    fn set_saturated_add_sign(&mut self, value: S) {
        if value >= S::zero() {
            if let Some(pos_val) = cast::cast::<S, U>(value) {
                *self = self.saturating_add(&pos_val);
            } else {
                *self = U::max_value();
            }
        } else {
            let abs_val = value.abs();
            if let Some(sub_val) = cast::cast::<S, U>(abs_val) {
                *self = self.saturating_sub(&sub_val);
            } else {
                *self = U::zero();
            }
        }
    }

    fn add_sign(&self, value: S) -> Self {
        if value >= S::zero() {
            cast::cast::<S, U>(value).map_or_else(
                || self.wrapping_add(&U::max_value()),
                |pos_val| self.wrapping_add(&pos_val),
            )
        } else {
            let abs_val = value.abs();
            cast::cast::<S, U>(abs_val).map_or_else(
                || self.wrapping_sub(&U::max_value()),
                |sub_val| self.wrapping_sub(&sub_val),
            )
        }
    }

    fn saturated_add_sign(&self, value: S) -> Self {
        if value >= S::zero() {
            cast::cast::<S, U>(value).map_or_else(U::max_value, |pos_val| {
                self.saturating_add(&pos_val)
            })
        } else {
            let abs_val = value.abs();
            cast::cast::<S, U>(abs_val)
                .map_or_else(U::zero, |sub_val| self.saturating_sub(&sub_val))
        }
    }
}

#[const_trait]
/// Trait for mapping between signed and unsigned integer types
pub trait MapToSign {
    /// Signed Version
    type Signed;
    /// Unsigned Version
    type Unsigned;

    /// Map from unsigned back to signed by flipping the sign bit
    fn map_non_sign_to_sign(&self) -> Self::Signed;
}
#[const_trait]
/// Trait for mapping between signed and unsigned integer types
pub trait MapToUnSign {
    /// Signed Version
    type Signed;
    /// Unsigned Version
    type Unsigned;

    /// Map from signed to unsigned by flipping the sign bit
    fn map_sign_to_non_sign(&self) -> Self::Unsigned;
}

/// Macro to implement `SignMapping` for integer type pairs
macro_rules! impl_sign_mapping {
    ($signed:ty, $unsigned:ty) => {
        impl MapToUnSign for $signed {
            type Signed = $signed;
            type Unsigned = $unsigned;

            #[allow(clippy::cast_sign_loss)]
            fn map_sign_to_non_sign(&self) -> Self::Unsigned {
                (*self as Self::Unsigned).wrapping_add(<$unsigned>::MAX / 2 + 1)
            }
        }
        impl MapToSign for $unsigned {
            type Signed = $signed;
            type Unsigned = $unsigned;

            #[allow(clippy::cast_possible_wrap)]
            fn map_non_sign_to_sign(&self) -> Self::Signed {
                self.wrapping_sub(<$unsigned>::MAX / 2 + 1) as Self::Signed
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

// pub trait Modular<Rhs = Self> {
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

// /// I always try to write `desaturating_add` instead of `saturating_sub`, idk why but now I'm fixing this problem in the stupidest I can
// pub trait Desaturation
// where
//     Self: num_traits::SaturatingAdd + num_traits::SaturatingSub,
// {
//     /// Alias for `saturating_sub`
//     #[must_use]
//     fn desaturating_add(&self, other: Self) -> Self {
//         self.saturating_sub(&other)
//     }
//     /// Alias for `saturating_add`
//     #[must_use]
//     fn desaturating_sub(&self, other: Self) -> Self {
//         self.saturating_add(&other)
//     }
// }
// impl<T: num_traits::SaturatingAdd + num_traits::SaturatingSub> Desaturation
//     for T
// {
// }
