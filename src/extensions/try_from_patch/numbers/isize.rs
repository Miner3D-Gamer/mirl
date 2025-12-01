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
use crate::extensions::TryFromPatch;
