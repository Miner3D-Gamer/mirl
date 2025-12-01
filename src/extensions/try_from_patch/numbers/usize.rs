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
use crate::extensions::TryFromPatch;
