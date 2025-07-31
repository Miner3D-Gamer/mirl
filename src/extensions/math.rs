#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
/// A trait for numbers to support `sign()`
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

/// Add a signed number to a non signed number
pub trait AddSign<T> {
    /// Simple addition of the possibly negative number
    fn add_sign(&mut self, value: T);
    /// Addition of the possibly negative number without over/underflowing
    fn saturated_add_sign(&mut self, value: T);
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
    fn add_sign(&mut self, value: S) {
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

    fn saturated_add_sign(&mut self, value: S) {
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
}


/// Trait for mapping between signed and unsigned integer types
pub trait SignMapping {
    /// Signed Version
    type Signed;
    /// Unsigned Version
    type Unsigned;
    
    /// Map from signed to unsigned by flipping the sign bit
    fn map_sign_to_non_sign(v: Self::Signed) -> Self::Unsigned;
    
    /// Map from unsigned back to signed by flipping the sign bit
    fn map_non_sign_to_sign(v: Self::Unsigned) -> Self::Signed;
}

/// Macro to implement `SignMapping` for integer type pairs
macro_rules! impl_sign_mapping {
    ($signed:ty, $unsigned:ty) => {
        impl SignMapping for $signed {
            type Signed = $signed;
            type Unsigned = $unsigned;
            
            #[allow(clippy::cast_sign_loss)]
            fn map_sign_to_non_sign(v: Self::Signed) -> Self::Unsigned {
                (v as Self::Unsigned).wrapping_add(<$unsigned>::MAX / 2 + 1)
            }
            #[allow(clippy::cast_possible_wrap)]
            fn map_non_sign_to_sign(v: Self::Unsigned) -> Self::Signed {
                v.wrapping_sub(<$unsigned>::MAX / 2 + 1) as Self::Signed
            }
        }
        impl SignMapping for $unsigned {
            type Signed = $signed;
            type Unsigned = $unsigned;
            
            #[allow(clippy::cast_sign_loss)]
            fn map_sign_to_non_sign(v: Self::Signed) -> Self::Unsigned {
                (v as Self::Unsigned).wrapping_add(<$unsigned>::MAX / 2 + 1)
            }
            #[allow(clippy::cast_possible_wrap)]
            fn map_non_sign_to_sign(v: Self::Unsigned) -> Self::Signed {
                v.wrapping_sub(<$unsigned>::MAX / 2 + 1) as Self::Signed
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

/// Map a value to their non signed counterpart
pub fn map_sign_to_non_sign<T: SignMapping>(v: T::Signed) -> T::Unsigned {
    T::map_sign_to_non_sign(v)
}

/// Map a value to their signed counterpart
pub fn map_non_sign_to_sign<T: SignMapping>(v: T::Unsigned) -> T::Signed {
    T::map_non_sign_to_sign(v)
}


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
