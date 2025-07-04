use num_traits::{NumCast, ToPrimitive};

use super::U2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, std::cmp::PartialOrd)]
pub struct U4 {
    pub b0: bool,
    pub b1: bool,
    pub b2: bool,
    pub b3: bool,
}
impl ToPrimitive for U4 {
    fn to_f32(&self) -> Option<f32> {
        return Some(self.value().into());
    }
    fn to_f64(&self) -> Option<f64> {
        return Some(self.value().into());
    }
    fn to_i128(&self) -> Option<i128> {
        return Some(self.value().into());
    }
    fn to_i16(&self) -> Option<i16> {
        return Some(self.value().into());
    }
    fn to_i32(&self) -> Option<i32> {
        return Some(self.value().into());
    }
    fn to_i64(&self) -> Option<i64> {
        return Some(self.value().into());
    }
    fn to_i8(&self) -> Option<i8> {
        return Some(self.value().to_i8().unwrap());
    }
    fn to_isize(&self) -> Option<isize> {
        return Some(self.value().into());
    }
    fn to_u128(&self) -> Option<u128> {
        return Some(self.value().into());
    }
    fn to_u16(&self) -> Option<u16> {
        return Some(self.value().into());
    }
    fn to_u32(&self) -> Option<u32> {
        return Some(self.value().into());
    }
    fn to_u64(&self) -> Option<u64> {
        return Some(self.value().into());
    }
    fn to_u8(&self) -> Option<u8> {
        return Some(self.value());
    }
}

impl NumCast for U4 {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Some(Self::new(n.to_u8().unwrap()))
    }
}

impl U4 {
    /// Create a U4 from a u8, panicking if value > 15
    pub fn new(val: u8) -> Self {
        assert!(val <= 0b1111, "Value out of range for U4 (must be 0..=15)");
        U4 {
            b0: (val & 0b0001) != 0,
            b1: (val & 0b0010) != 0,
            b2: (val & 0b0100) != 0,
            b3: (val & 0b1000) != 0,
        }
    }
    /// Create a U4 without checking (masking to lower 4 bits)
    pub const fn from_u8_trunc(val: u8) -> Self {
        U4 {
            b0: (val & 0b0001) != 0,
            b1: (val & 0b0010) != 0,
            b2: (val & 0b0100) != 0,
            b3: (val & 0b1000) != 0,
        }
    }

    /// Get the integer value of this U4
    pub const fn value(self) -> u8 {
        (self.b0 as u8)
            | ((self.b1 as u8) << 1)
            | ((self.b2 as u8) << 2)
            | ((self.b3 as u8) << 3)
    }

    /// Returns true if the value is zero
    pub const fn is_zero(self) -> bool {
        !self.b0 && !self.b1 && !self.b2 && !self.b3
    }

    /// Returns true if the value is the maximum (15)
    pub const fn is_max(self) -> bool {
        self.b0 && self.b1 && self.b2 && self.b3
    }
    /// Wrapping add: (self + other) mod 16
    pub const fn wrapping_add(self, other: U4) -> U4 {
        U4::from_u8_trunc(self.value().wrapping_add(other.value()))
    }

    /// Wrapping sub: (self - other) mod 16
    pub const fn wrapping_sub(self, other: U4) -> U4 {
        U4::from_u8_trunc(self.value().wrapping_sub(other.value()))
    }
    /// Convert to U2 by truncating to lower 2 bits
    pub fn to_u2(self) -> U2 {
        self.into()
    }

    /// Create a U4 from high and low U2 values
    /// The high U2 will occupy bits 2-3, and the low U2 will occupy bits 0-1
    pub fn from_u2_pair(high: U2, low: U2) -> Self {
        U4 {
            b0: low.b0,
            b1: low.b1,
            b2: high.b0,
            b3: high.b1,
        }
    }

    /// Split a U4 into high and low U2 values
    /// Returns (high_bits, low_bits) where high_bits are bits 2-3 and low_bits are bits 0-1
    pub fn split_to_u2_pair(self) -> (U2, U2) {
        let high = U2 {
            b0: self.b2,
            b1: self.b3,
        };

        let low = U2 {
            b0: self.b0,
            b1: self.b1,
        };

        (high, low)
    }
}

// Bitwise and arithmetic traits
impl std::ops::Not for U4 {
    type Output = U4;
    fn not(self) -> U4 {
        U4::from_u8_trunc(!self.value())
    }
}
impl std::ops::BitAnd for U4 {
    type Output = U4;
    fn bitand(self, rhs: U4) -> U4 {
        U4::from_u8_trunc(self.value() & rhs.value())
    }
}
impl std::ops::BitOr for U4 {
    type Output = U4;
    fn bitor(self, rhs: U4) -> U4 {
        U4::from_u8_trunc(self.value() | rhs.value())
    }
}
impl std::ops::BitXor for U4 {
    type Output = U4;
    fn bitxor(self, rhs: U4) -> U4 {
        U4::from_u8_trunc(self.value() ^ rhs.value())
    }
}
impl std::ops::Shl<usize> for U4 {
    type Output = U4;
    fn shl(self, rhs: usize) -> U4 {
        U4::from_u8_trunc(self.value() << rhs)
    }
}
impl std::ops::Shr<usize> for U4 {
    type Output = U4;
    fn shr(self, rhs: usize) -> U4 {
        U4::from_u8_trunc(self.value() >> rhs)
    }
}
impl std::ops::Add for U4 {
    type Output = Self;
    fn add(self, rhs: U4) -> U4 {
        self.wrapping_add(rhs)
    }
}
impl std::ops::Sub for U4 {
    type Output = U4;
    fn sub(self, rhs: U4) -> U4 {
        self.wrapping_sub(rhs)
    }
}
impl std::ops::Mul for U4 {
    type Output = U4;
    fn mul(self, rhs: U4) -> U4 {
        U4::from_u8_trunc(self.value().wrapping_mul(rhs.value()))
    }
}
impl std::ops::Div for U4 {
    type Output = U4;
    fn div(self, rhs: U4) -> U4 {
        U4::from_u8_trunc(self.value().wrapping_div(rhs.value()))
    }
}
#[macro_export]
macro_rules! u4 {
    ($val:expr) => {
        U4::new($val)
    };
}
macro_rules! impl_u4_conversion {
    ($($t:ty),* $(,)?) => {
        $(
            // From {type} -> U4
            impl From<$t> for U4 {
                fn from(val: $t) -> Self {
                    // Handle signed and unsigned differently
                    let as_u8 = val as i128; // Safe for both signed/unsigned
                    assert!((0..=15).contains(&as_u8), "Value out of range for U4 (must be 0..=15)");
                    let val = as_u8 as u8;
                    U4 {
                        b0: (val & 0b0001) != 0,
                        b1: (val & 0b0010) != 0,
                        b2: (val & 0b0100) != 0,
                        b3: (val & 0b1000) != 0,
                    }
                }
            }

            // From U4 -> {type}
            impl From<U4> for $t {
                fn from(val: U4) -> Self {
                    let raw = (val.b0 as u8)
                        | ((val.b1 as u8) << 1)
                        | ((val.b2 as u8) << 2)
                        | ((val.b3 as u8) << 3);
                    raw as $t
                }
            }
        )*
    };
}

macro_rules! impl_u4_float_conversion {
    ($($f:ty),* $(,)?) => {
        $(
            impl From<$f> for U4 {
                fn from(val: $f) -> Self {
                    assert!(val.is_finite(), "Cannot convert non-finite float to U4");
                    assert!(val.fract() == 0.0, "Cannot convert non-integer float to U4");
                    let as_int = val as i128;
                    assert!((0..=15).contains(&as_int), "Float value out of U4 range (must be 0.0 to 15.0)");
                    let val = as_int as u8;
                    U4 {
                        b0: (val & 0b0001) != 0,
                        b1: (val & 0b0010) != 0,
                        b2: (val & 0b0100) != 0,
                        b3: (val & 0b1000) != 0,
                    }
                }
            }

            impl From<U4> for $f {
                fn from(val: U4) -> Self {
                    let raw = (val.b0 as u8)
                        | ((val.b1 as u8) << 1)
                        | ((val.b2 as u8) << 2)
                        | ((val.b3 as u8) << 3);
                    raw as $f
                }
            }
        )*
    };
}
impl_u4_conversion!(u8, u16, u32, u64, u128, usize);
impl_u4_conversion!(i8, i16, i32, i64, i128, isize);
impl_u4_float_conversion!(f32, f64);

impl std::ops::Rem for U4 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_u8_trunc(self.value() % rhs.value())
    }
}
impl num_traits::One for U4 {
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.value() == 1
    }
    fn one() -> Self {
        Self::from_u8_trunc(1)
    }
}
impl num_traits::Zero for U4 {
    fn zero() -> Self {
        Self::from_u8_trunc(0)
    }
    fn is_zero(&self) -> bool {
        self.value() == 0
    }
}

impl num_traits::Num for U4 {
    fn from_str_radix(
        str: &str,
        radix: u32,
    ) -> Result<Self, Self::FromStrRadixErr> {
        let result = <u8 as num_traits::Num>::from_str_radix(str, radix);
        if let Ok(r) = result {
            return Result::Ok(Self::from_u8_trunc(r));
        } else if let Err(e) = result {
            return Result::Err(e);
        } else {
            panic!("This panic will never hit")
        }
    }

    type FromStrRadixErr = ::core::num::ParseIntError;
}
