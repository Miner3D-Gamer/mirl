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
