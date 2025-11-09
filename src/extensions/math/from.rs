/// For example: i32 does not implement `from<usize>`
/// 
/// Why is that?
/// 
/// That isn't a rhetorical question, why can I not convert from one number to another if when the implementation is not that difficult 
/// 
/// This fixes that annoyance by using a custom from
pub const trait FromPatch<T>: Sized {
    #[must_use]
    /// A custom from to bypass rusts orphan rule
    fn from_value(value: T) -> Self;
}

// i8
impl const FromPatch<Self> for i8 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<i8> for i16 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for i32 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for i64 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for i128 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for isize {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for u8 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for u16 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for u32 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for u64 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for u128 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for usize {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for f32 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}
impl const FromPatch<i8> for f64 {
    fn from_value(v: i8) -> Self {
        v as Self
    }
}

// i16
impl const FromPatch<i16> for i8 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for i16 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<i16> for i32 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for i64 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for i128 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for isize {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for u8 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for u16 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for u32 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for u64 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for u128 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for usize {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for f32 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}
impl const FromPatch<i16> for f64 {
    fn from_value(v: i16) -> Self {
        v as Self
    }
}

// i32
impl const FromPatch<i32> for i8 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for i16 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for i32 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<i32> for i64 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for i128 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for isize {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for u8 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for u16 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for u32 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for u64 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for u128 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for usize {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for f32 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}
impl const FromPatch<i32> for f64 {
    fn from_value(v: i32) -> Self {
        v as Self
    }
}

// i64
impl const FromPatch<i64> for i8 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for i16 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for i32 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for i64 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<i64> for i128 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for isize {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for u8 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for u16 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for u32 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for u64 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for u128 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for usize {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for f32 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}
impl const FromPatch<i64> for f64 {
    fn from_value(v: i64) -> Self {
        v as Self
    }
}

// i128
impl const FromPatch<i128> for i8 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for i16 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for i32 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for i64 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for i128 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<i128> for isize {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for u8 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for u16 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for u32 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for u64 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for u128 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for usize {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for f32 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}
impl const FromPatch<i128> for f64 {
    fn from_value(v: i128) -> Self {
        v as Self
    }
}

// isize
impl const FromPatch<isize> for i8 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for i16 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for i32 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for i64 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for i128 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for isize {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<isize> for u8 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for u16 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for u32 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for u64 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for u128 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for usize {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for f32 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}
impl const FromPatch<isize> for f64 {
    fn from_value(v: isize) -> Self {
        v as Self
    }
}

// u8
impl const FromPatch<u8> for i8 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for i16 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for i32 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for i64 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for i128 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for isize {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for u8 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<u8> for u16 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for u32 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for u64 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for u128 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for usize {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for f32 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}
impl const FromPatch<u8> for f64 {
    fn from_value(v: u8) -> Self {
        v as Self
    }
}

// u16
impl const FromPatch<u16> for i8 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for i16 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for i32 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for i64 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for i128 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for isize {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for u8 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for u16 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<u16> for u32 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for u64 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for u128 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for usize {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for f32 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}
impl const FromPatch<u16> for f64 {
    fn from_value(v: u16) -> Self {
        v as Self
    }
}

// u32
impl const FromPatch<u32> for i8 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for i16 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for i32 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for i64 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for i128 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for isize {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for u8 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for u16 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for u32 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<u32> for u64 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for u128 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for usize {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for f32 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}
impl const FromPatch<u32> for f64 {
    fn from_value(v: u32) -> Self {
        v as Self
    }
}

// u64
impl const FromPatch<u64> for i8 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for i16 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for i32 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for i64 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for i128 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for isize {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for u8 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for u16 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for u32 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for u64 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<u64> for u128 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for usize {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for f32 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}
impl const FromPatch<u64> for f64 {
    fn from_value(v: u64) -> Self {
        v as Self
    }
}

// u128
impl const FromPatch<u128> for i8 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for i16 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for i32 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for i64 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for i128 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for isize {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for u8 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for u16 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for u32 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for u64 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for u128 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<u128> for usize {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for f32 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}
impl const FromPatch<u128> for f64 {
    fn from_value(v: u128) -> Self {
        v as Self
    }
}

// usize
impl const FromPatch<usize> for i8 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for i16 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for i32 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for i64 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for i128 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for isize {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for u8 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for u16 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for u32 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for u64 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for u128 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for usize {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<usize> for f32 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}
impl const FromPatch<usize> for f64 {
    fn from_value(v: usize) -> Self {
        v as Self
    }
}

// f32
impl const FromPatch<f32> for i8 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for i16 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for i32 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for i64 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for i128 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for isize {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for u8 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for u16 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for u32 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for u64 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for u128 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<f32> for usize {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for f32 {
    fn from_value(v: Self) -> Self {
        v
    }
}
impl const FromPatch<f32> for f64 {
    fn from_value(v: f32) -> Self {
        v as Self
    }
}

// f64
impl const FromPatch<f64> for i8 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for i16 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for i32 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for i64 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for i128 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for isize {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for u8 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for u16 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for u32 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for u64 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for u128 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for usize {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<f64> for f32 {
    fn from_value(v: f64) -> Self {
        v as Self
    }
}
impl const FromPatch<Self> for f64 {
    fn from_value(v: Self) -> Self {
        v
    }
}
