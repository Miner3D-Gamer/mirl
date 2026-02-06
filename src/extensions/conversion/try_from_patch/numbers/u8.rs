// u8
impl const TryFromPatch<u8> for i8 {
    fn try_from_value(v: u8) -> Option<Self> {
        Some(v as Self)
    }
}
use crate::extensions::TryFromPatch;
