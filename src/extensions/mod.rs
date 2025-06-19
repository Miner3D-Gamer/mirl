pub trait Sign {
    /// Returns the sign of the number -> -1, 0, 1
    fn sign(self) -> Self;
}

macro_rules! impl_sign {
    ($($t:ty),*) => {
        $(impl Sign for $t {
            fn sign(self) -> Self {
                if self > 0 as $t { 1 as $t }
                else if self < 0 as $t { -1 as $t }
                else { 0 as $t }
            }
        })*
    };
}

impl_sign!(i8, i16, i32, i64, i128, isize, f32, f64);

use core::f32;
use std::convert::TryFrom;

pub fn from_u8<T: TryFrom<u8>>(value: u8) -> T {
    T::try_from(value).ok().expect("constant u8 conversion failed")
}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

macro_rules! impl_sqrt {
    ($($t:ty),*) => {
        $(
            impl Sqrt for $t {
                fn sqrt(self) -> Self {
                    // There has to be a better way
                    (self as f64).sqrt() as Self
                }
            }
        )*
    };
}

impl_sqrt!(i8);
impl_sqrt!(i16);
impl_sqrt!(i32);
impl_sqrt!(i64);
impl_sqrt!(i128);
impl_sqrt!(isize);

#[macro_export]
macro_rules! convert_tuple_elements {
    // 1-tuple (rare but possible)
    (($x1:expr,), $target_type:ty) => {
        ($x1 as $target_type,)
    };
    // 2-tuple
    (($x1:expr, $x2:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type)
    };
    // 3-tuple
    (($x1:expr, $x2:expr, $x3:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type, $x3 as $target_type)
    };
    // 4-tuple
    (($x1:expr, $x2:expr, $x3:expr, $x4:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type, $x3 as $target_type, $x4 as $target_type)
    };
    // 5-tuple
    (($x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type, $x3 as $target_type, $x4 as $target_type, $x5 as $target_type)
    };
    // 6-tuple
    (($x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type, $x3 as $target_type, $x4 as $target_type, $x5 as $target_type, $x6 as $target_type)
    };
    // 7-tuple
    (($x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr, $x7:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type, $x3 as $target_type, $x4 as $target_type, $x5 as $target_type, $x6 as $target_type, $x7 as $target_type)
    };
    // 8-tuple
    (($x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr, $x7:expr, $x8:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type, $x3 as $target_type, $x4 as $target_type, $x5 as $target_type, $x6 as $target_type, $x7 as $target_type, $x8 as $target_type)
    };
    // 9-tuple
    (($x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr, $x7:expr, $x8:expr, $x9:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type, $x3 as $target_type, $x4 as $target_type, $x5 as $target_type, $x6 as $target_type, $x7 as $target_type, $x8 as $target_type, $x9 as $target_type)
    };
    // 10-tuple
    (($x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr, $x7:expr, $x8:expr, $x9:expr, $x10:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type, $x3 as $target_type, $x4 as $target_type, $x5 as $target_type, $x6 as $target_type, $x7 as $target_type, $x8 as $target_type, $x9 as $target_type, $x10 as $target_type)
    };
    // 11-tuple
    (($x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr, $x7:expr, $x8:expr, $x9:expr, $x10:expr, $x11:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type, $x3 as $target_type, $x4 as $target_type, $x5 as $target_type, $x6 as $target_type, $x7 as $target_type, $x8 as $target_type, $x9 as $target_type, $x10 as $target_type, $x11 as $target_type)
    };
    // 12-tuple
    (($x1:expr, $x2:expr, $x3:expr, $x4:expr, $x5:expr, $x6:expr, $x7:expr, $x8:expr, $x9:expr, $x10:expr, $x11:expr, $x12:expr), $target_type:ty) => {
        ($x1 as $target_type, $x2 as $target_type, $x3 as $target_type, $x4 as $target_type, $x5 as $target_type, $x6 as $target_type, $x7 as $target_type, $x8 as $target_type, $x9 as $target_type, $x10 as $target_type, $x11 as $target_type, $x12 as $target_type)
    };
}

#[macro_export]
macro_rules! convert_tuple_literal {
    (($($x:expr),+ $(,)?), $target_type:ty) => {
        ($($x as $target_type),+)
    };
}

#[macro_export]
macro_rules! convert_tuple {
    ($tuple_expr:expr, $target_type:ty) => {{
        let tuple = $tuple_expr;
        $crate::extensions::convert_tuple_elements!(tuple, $target_type)
    }};
}





mod tuple;


use std::i128;

pub use tuple::*;

mod string;
pub use string::*;

mod u4;
pub use u4::*;

mod u2;
pub use u2::*;

mod u1;
pub use u1::*;

mod small_u_support;
#[allow(unused_imports)]
pub use small_u_support::*;
