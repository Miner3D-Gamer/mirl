#[derive(Debug, Clone, Copy, PartialEq, Eq, std::cmp::PartialOrd)]
/// U1 - Just a fancy bool honestly
pub struct U1 {
    /// Yep, that's the bool right there
    pub b0: bool,
}
use num_traits::{NumCast, ToPrimitive};

impl ToPrimitive for U1 {
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

impl NumCast for U1 {
    fn from<T: num_traits::ToPrimitive>(n: T) -> Option<Self> {
        Some(Self::new(n.to_u8().unwrap()))
    }
}

impl U1 {
    /// Create a U1 from a u8, panicking if value > 1
    pub fn new(val: u8) -> Self {
        assert!(val <= 0b1, "Value out of range for U1 (must be 0..=1)");
        U1 {
            b0: (val & 0b1) != 0,
        }
    }

    /// Create a U1 without checking (masking to lowest bit)
    pub const fn from_u8_trunc(val: u8) -> Self {
        U1 {
            b0: (val & 0b1) != 0,
        }
    }

    /// Get the integer value of this U1
    pub const fn value(self) -> u8 {
        self.b0 as u8
    }

    /// Returns true if the value is zero
    pub const fn is_zero(self) -> bool {
        !self.b0
    }

    /// Returns true if the value is the maximum (1)
    pub const fn is_max(self) -> bool {
        self.b0
    }

    /// Wrapping add: (self + other) mod 2
    pub const fn wrapping_add(self, other: U1) -> U1 {
        U1::from_u8_trunc(self.value().wrapping_add(other.value()))
    }

    /// Wrapping sub: (self - other) mod 2
    pub const fn wrapping_sub(self, other: U1) -> U1 {
        U1::from_u8_trunc(self.value().wrapping_sub(other.value()))
    }

    /// Create a U1 directly from a boolean
    pub const fn from_bool(val: bool) -> Self {
        U1 {
            b0: val,
        }
    }

    /// Convert to boolean
    pub const fn to_bool(self) -> bool {
        self.b0
    }
}

// Bitwise and arithmetic traits
impl std::ops::Not for U1 {
    type Output = U1;
    fn not(self) -> U1 {
        U1::from_u8_trunc(!self.value())
    }
}

impl std::ops::BitAnd for U1 {
    type Output = U1;
    fn bitand(self, rhs: U1) -> U1 {
        U1::from_u8_trunc(self.value() & rhs.value())
    }
}

impl std::ops::BitOr for U1 {
    type Output = U1;
    fn bitor(self, rhs: U1) -> U1 {
        U1::from_u8_trunc(self.value() | rhs.value())
    }
}

impl std::ops::BitXor for U1 {
    type Output = U1;
    fn bitxor(self, rhs: U1) -> U1 {
        U1::from_u8_trunc(self.value() ^ rhs.value())
    }
}

impl std::ops::Shl<usize> for U1 {
    type Output = U1;
    fn shl(self, rhs: usize) -> U1 {
        U1::from_u8_trunc(self.value() << rhs)
    }
}

impl std::ops::Shr<usize> for U1 {
    type Output = U1;
    fn shr(self, rhs: usize) -> U1 {
        U1::from_u8_trunc(self.value() >> rhs)
    }
}

impl std::ops::Add for U1 {
    type Output = U1;
    fn add(self, rhs: U1) -> U1 {
        self.wrapping_add(rhs)
    }
}

impl std::ops::Sub for U1 {
    type Output = U1;
    fn sub(self, rhs: U1) -> U1 {
        self.wrapping_sub(rhs)
    }
}

impl std::ops::Mul for U1 {
    type Output = U1;
    fn mul(self, rhs: U1) -> U1 {
        U1::from_u8_trunc(self.value().wrapping_mul(rhs.value()))
    }
}
impl std::ops::Div for U1 {
    type Output = U1;
    fn div(self, rhs: U1) -> U1 {
        U1::from_u8_trunc(self.value().wrapping_div(rhs.value()))
    }
}

// Direct conversion between U1 and bool
impl From<bool> for U1 {
    fn from(val: bool) -> Self {
        U1 {
            b0: val,
        }
    }
}

impl From<U1> for bool {
    fn from(val: U1) -> Self {
        val.b0
    }
}
/// Convert a number into a u1
#[macro_export]
macro_rules! u1 {
    ($val:expr) => {
        U1::new($val)
    };
}

macro_rules! impl_u1_conversion {
    ($($t:ty),* $(,)?) => {
        $(
            // From {type} -> U1
            impl From<$t> for U1 {
                fn from(val: $t) -> Self {
                    // Handle signed and unsigned differently
                    let as_u8 = val as i128; // Safe for both signed/unsigned
                    assert!((0..=1).contains(&as_u8), "Value out of range for U1 (must be 0..=1)");
                    let val = as_u8 as u8;
                    U1 {
                        b0: (val & 0b1) != 0,
                    }
                }
            }

            // From U1 -> {type}
            impl From<U1> for $t {
                fn from(val: U1) -> Self {
                    let raw = val.b0 as u8;
                    raw as $t
                }
            }
        )*
    };
}

macro_rules! impl_u1_float_conversion {
    ($($f:ty),* $(,)?) => {
        $(
            impl From<$f> for U1 {
                fn from(val: $f) -> Self {
                    assert!(val.is_finite(), "Cannot convert non-finite float to U1");
                    assert!(val.fract() == 0.0, "Cannot convert non-integer float to U1");
                    let as_int = val as i128;
                    assert!((0..=1).contains(&as_int), "Float value out of U1 range (must be 0.0 to 1.0)");
                    let val = as_int as u8;
                    U1 {
                        b0: (val & 0b1) != 0,
                    }
                }
            }

            impl From<U1> for $f {
                fn from(val: U1) -> Self {
                    let raw = val.b0 as u8;
                    raw as $f
                }
            }
        )*
    };
}

impl_u1_conversion!(u8, u16, u32, u64, u128, usize);
impl_u1_conversion!(i8, i16, i32, i64, i128, isize);
impl_u1_float_conversion!(f32, f64);

impl std::ops::Rem for U1 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::from_u8_trunc(self.value() % rhs.value())
    }
}
impl num_traits::One for U1 {
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
impl num_traits::Zero for U1 {
    fn zero() -> Self {
        Self::from_u8_trunc(0)
    }
    fn is_zero(&self) -> bool {
        self.value() == 0
    }
}

impl num_traits::Num for U1 {
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
