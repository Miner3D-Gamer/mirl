use crate::extensions::TryFromPatch;

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
