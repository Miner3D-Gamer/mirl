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
use crate::extensions::TryFromPatch;
