

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
}use crate::extensions::TryFromPatch;
