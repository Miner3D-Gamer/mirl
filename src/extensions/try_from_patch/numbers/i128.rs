
// i128
impl const TryFromPatch<i128> for i8 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for i16 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for i32 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for i64 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<Self> for i128 {
    fn try_from_value(v: Self) -> Option<Self> {
        Some(v)
    }
}
impl const TryFromPatch<i128> for isize {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for u8 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for u16 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for u32 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for u64 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for u128 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for usize {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for f32 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}
impl const TryFromPatch<i128> for f64 {
    fn try_from_value(v: i128) -> Option<Self> {
        Some(v as Self)
    }
}use crate::extensions::TryFromPatch;
