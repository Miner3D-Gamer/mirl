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
impl const TryFromPatch<u16> for u8 {
    fn try_from_value(v: u16) -> Option<Self> {
        Some(v as Self)
    }
}
use crate::extensions::TryFromPatch;
