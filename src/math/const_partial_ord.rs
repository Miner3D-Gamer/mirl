#[const_trait]
#[allow(missing_docs)]
/// A const implementation of `PartialOrd`
pub trait ConstPartialOrd: Copy {
    fn const_partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>;
    fn const_lt(&self, other: &Self) -> bool;
    fn const_le(&self, other: &Self) -> bool;
    fn const_gt(&self, other: &Self) -> bool;
    fn const_ge(&self, other: &Self) -> bool;
}
macro_rules! impl_const_partial_ord {
    ($($t:ty),+ $(,)?) => {
        $(
            impl const ConstPartialOrd for $t {
                fn const_partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    if *self < *other {
                        Some(std::cmp::Ordering::Less)
                    } else if *self > *other {
                        Some(std::cmp::Ordering::Greater)
                    } else {
                        Some(std::cmp::Ordering::Equal)
                    }
                }

                fn const_lt(&self, other: &Self) -> bool {
                    *self < *other
                }

                fn const_le(&self, other: &Self) -> bool {
                    *self <= *other
                }

                fn const_gt(&self, other: &Self) -> bool {
                    *self > *other
                }

                fn const_ge(&self, other: &Self) -> bool {
                    *self >= *other
                }
            }
        )+
    };
}
impl_const_partial_ord! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

impl_const_partial_ord! {
    f32, f64,
}
