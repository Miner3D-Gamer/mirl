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
use crate::extensions::TryFromPatch;
