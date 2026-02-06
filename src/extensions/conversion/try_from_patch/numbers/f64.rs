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
impl const TryFromPatch<Self> for f16 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<Self> for f128 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
use crate::extensions::TryFromPatch;
