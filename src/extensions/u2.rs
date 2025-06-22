use super::U4;
use num_traits::{NumCast, ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, std::cmp::PartialOrd)]
pub struct U2 {
    pub b0: bool,
    pub b1: bool,
}
impl ToPrimitive for U2 {
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

impl NumCast for U2 {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Some(Self::new(n.to_u8().unwrap()))
    }
}


impl U2 {
    /// Create a U2 from a u8, panicking if value > 3
    pub fn new(val: u8) -> Self {
        assert!(val <= 0b11, "Value out of range for U2 (must be 0..=3)");
        U2 {
            b0: (val & 0b01) != 0,
            b1: (val & 0b10) != 0,
        }
    }

    /// Create a U2 without checking (masking to lower 2 bits)
    pub const fn from_u8_trunc(val: u8) -> Self {
        U2 {
            b0: (val & 0b01) != 0,
            b1: (val & 0b10) != 0,
        }
    }

    /// Get the integer value of this U2
    pub const fn value(self) -> u8 {
        (self.b0 as u8) | ((self.b1 as u8) << 1)
    }

    /// Returns true if the value is zero
    pub const fn is_zero(self) -> bool {
        !self.b0 && !self.b1
    }

    /// Returns true if the value is the maximum (3)
    pub const fn is_max(self) -> bool {
        self.b0 && self.b1
    }

    /// Wrapping add: (self + other) mod 4
    pub const fn wrapping_add(self, other: U2) -> U2 {
        U2::from_u8_trunc(self.value().wrapping_add(other.value()))
    }

    /// Wrapping sub: (self - other) mod 4
    pub const fn wrapping_sub(self, other: U2) -> U2 {
        U2::from_u8_trunc(self.value().wrapping_sub(other.value()))
    }
    /// Convert to U4 (with upper bits set to 0)
    pub fn to_u4(self) -> U4 {
        self.into()
    }

    /// Combine two U2 values into a U4
    /// self will occupy the high bits (2-3), other will occupy the low bits (0-1)
    pub fn combine_high_with(self, other: U2) -> U4 {
        U4::from_u2_pair(self, other)
    }

    /// Combine two U2 values into a U4
    /// self will occupy the low bits (0-1), other will occupy the high bits (2-3)
    pub fn combine_low_with(self, other: U2) -> U4 {
        U4::from_u2_pair(other, self)
    }
}

// Bitwise and arithmetic traits
impl std::ops::Not for U2 {
    type Output = U2;
    fn not(self) -> U2 {
        U2::from_u8_trunc(!self.value())
    }
}

impl std::ops::BitAnd for U2 {
    type Output = U2;
    fn bitand(self, rhs: U2) -> U2 {
        U2::from_u8_trunc(self.value() & rhs.value())
    }
}

impl std::ops::BitOr for U2 {
    type Output = U2;
    fn bitor(self, rhs: U2) -> U2 {
        U2::from_u8_trunc(self.value() | rhs.value())
    }
}

impl std::ops::BitXor for U2 {
    type Output = U2;
    fn bitxor(self, rhs: U2) -> U2 {
        U2::from_u8_trunc(self.value() ^ rhs.value())
    }
}

impl std::ops::Shl<usize> for U2 {
    type Output = U2;
    fn shl(self, rhs: usize) -> U2 {
        U2::from_u8_trunc(self.value() << rhs)
    }
}

impl std::ops::Shr<usize> for U2 {
    type Output = U2;
    fn shr(self, rhs: usize) -> U2 {
        U2::from_u8_trunc(self.value() >> rhs)
    }
}

impl std::ops::Add for U2 {
    type Output = U2;
    fn add(self, rhs: U2) -> U2 {
        self.wrapping_add(rhs)
    }
}

impl std::ops::Sub for U2 {
    type Output = U2;
    fn sub(self, rhs: U2) -> U2 {
        self.wrapping_sub(rhs)
    }
}

impl std::ops::Mul for U2 {
    type Output = U2;
    fn mul(self, rhs: U2) -> U2 {
        U2::from_u8_trunc(self.value().wrapping_mul(rhs.value()))
    }
}
impl std::ops::Div for U2 {
    type Output = U2;
    fn div(self, rhs: U2) -> U2 {
        U2::from_u8_trunc(self.value().wrapping_div(rhs.value()))
    }
}

#[macro_export]
macro_rules! u2 {
    ($val:expr) => {
        U2::new($val)
    };
}

macro_rules! impl_u2_conversion {
    ($($t:ty),* $(,)?) => {
        $(
            // From {type} -> U2
            impl From<$t> for U2 {
                fn from(val: $t) -> Self {
                    // Handle signed and unsigned differently
                    let as_u8 = val as i128; // Safe for both signed/unsigned
                    assert!((0..=3).contains(&as_u8), "Value out of range for U2 (must be 0..=3)");
                    let val = as_u8 as u8;
                    U2 {
                        b0: (val & 0b01) != 0,
                        b1: (val & 0b10) != 0,
                    }
                }
            }

            // From U2 -> {type}
            impl From<U2> for $t {
                fn from(val: U2) -> Self {
                    let raw = (val.b0 as u8) | ((val.b1 as u8) << 1);
                    raw as $t
                }
            }
        )*
    };
}

macro_rules! impl_u2_float_conversion {
    ($($f:ty),* $(,)?) => {
        $(
            impl From<$f> for U2 {
                fn from(val: $f) -> Self {
                    assert!(val.is_finite(), "Cannot convert non-finite float to U2");
                    assert!(val.fract() == 0.0, "Cannot convert non-integer float to U2");
                    let as_int = val as i128;
                    assert!((0..=3).contains(&as_int), "Float value out of U2 range (must be 0.0 to 3.0)");
                    let val = as_int as u8;
                    U2 {
                        b0: (val & 0b01) != 0,
                        b1: (val & 0b10) != 0,
                    }
                }
            }

            impl From<U2> for $f {
                fn from(val: U2) -> Self {
                    let raw = (val.b0 as u8) | ((val.b1 as u8) << 1);
                    raw as $f
                }
            }
        )*
    };
}

impl_u2_conversion!(u8, u16, u32, u64, u128, usize);
impl_u2_conversion!(i8, i16, i32, i64, i128, isize);
impl_u2_float_conversion!(f32, f64);
