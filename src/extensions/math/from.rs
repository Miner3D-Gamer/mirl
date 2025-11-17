/// Lets you convert from one value to another.
///
/// What's the difference between this and `std::convert::TryFrom`?
/// `std::convert::TryFrom` has many holes covered by `std::convert::From`, yet having 2 traits inconveniences things.
/// This trait "patches" the `std::convert::TryFrom` by combining both implementations
pub const trait TryFromPatch<T>: Sized {
    #[must_use]
    /// A custom from to bypass rusts orphan rule
    fn try_from_value(value: T) -> Option<Self>;
}

// i8
impl const TryFromPatch<Self> for i8 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<i8> for i16 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for i32 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for i64 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for i128 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for isize {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for u8 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for u16 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for u32 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for u64 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for u128 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for usize {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for f32 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i8> for f64 {
    fn try_from_value(v: i8) -> Option<Self> {
        Some(v as Self)
    }
}

// i16
impl const TryFromPatch<i16> for i8 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for i16 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<i16> for i32 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for i64 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for i128 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for isize {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for u8 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for u16 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for u32 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for u64 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for u128 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for usize {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for f32 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i16> for f64 {
    fn try_from_value(v: i16) -> Option<Self> {
        Some(v as Self)
    }
}

// i32
impl const TryFromPatch<i32> for i8 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for i16 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for i32 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<i32> for i64 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for i128 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for isize {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for u8 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for u16 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for u32 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for u64 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for u128 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for usize {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for f32 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i32> for f64 {
    fn try_from_value(v: i32) -> Option<Self> {
        Some(v as Self)
    }
}

// i64
impl const TryFromPatch<i64> for i8 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for i16 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for i32 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for i64 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<i64> for i128 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for isize {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for u8 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for u16 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for u32 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for u64 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for u128 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for usize {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for f32 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i64> for f64 {
    fn try_from_value(v: i64) -> Option<Self> {
        Some(v as Self)
    }
}

// i128
impl const TryFromPatch<i128> for i8 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for i16 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for i32 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for i64 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for i128 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<i128> for isize {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for u8 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for u16 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for u32 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for u64 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for u128 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for usize {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for f32 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for f64 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}

// isize
impl const TryFromPatch<isize> for i8 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for i16 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for i32 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for i64 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for i128 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for isize {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<isize> for u8 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for u16 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for u32 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for u64 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for u128 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for usize {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for f32 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<isize> for f64 {
    fn try_from_value(v: isize) -> Option<Self> {
        Some(v as Self)
    }
}

// u8
impl const TryFromPatch<u8> for i8 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for i16 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for i32 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for i64 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for i128 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for isize {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for u8 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<u8> for u16 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for u32 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for u64 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for u128 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for usize {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for f32 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u8> for f64 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}

// u16
impl const TryFromPatch<u16> for i8 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for i16 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for i32 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for i64 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for i128 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for isize {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for u8 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for u16 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<u16> for u32 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for u64 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for u128 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for usize {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for f32 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u16> for f64 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}

// u32
impl const TryFromPatch<u32> for i8 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for i16 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for i32 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for i64 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for i128 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for isize {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for u8 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for u16 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for u32 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<u32> for u64 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for u128 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for usize {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for f32 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u32> for f64 {
    fn try_from_value(v: u32) -> Option<Self> {
        Some(v as Self)
    }
}

// u64
impl const TryFromPatch<u64> for i8 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for i16 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for i32 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for i64 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for i128 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for isize {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for u8 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for u16 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for u32 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for u64 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<u64> for u128 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for usize {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for f32 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u64> for f64 {
    fn try_from_value(v: u64) -> Option<Self> {
        Some(v as Self)
    }
}

// u128
impl const TryFromPatch<u128> for i8 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for i16 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for i32 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for i64 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for i128 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for isize {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for u8 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for u16 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for u32 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for u64 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for u128 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<u128> for usize {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for f32 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<u128> for f64 {
    fn try_from_value(v: u128) -> Option<Self> {
        Some(v as Self)
    }
}

// usize
impl const TryFromPatch<usize> for i8 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for i16 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for i32 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for i64 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for i128 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for isize {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for u8 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for u16 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for u32 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for u64 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for u128 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for usize {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<usize> for f32 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<usize> for f64 {
    fn try_from_value(v: usize) -> Option<Self> {
        Some(v as Self)
    }
}

// f32
impl const TryFromPatch<f32> for i8 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for i16 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for i32 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for i64 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for i128 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for isize {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for u8 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for u16 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for u32 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for u64 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for u128 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f32> for usize {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for f32 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<f32> for f64 {
    fn try_from_value(v: f32) -> Option<Self> {
        Some(v as Self)
    }
}

// f64
impl const TryFromPatch<f64> for i8 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for i16 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for i32 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for i64 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for i128 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for isize {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for u8 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for u16 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for u32 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for u64 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for u128 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for usize {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<f64> for f32 {
    fn try_from_value(v: f64) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for f64 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
