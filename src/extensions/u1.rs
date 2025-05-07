#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U1 {
    pub b0: bool,
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
