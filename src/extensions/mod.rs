// /// A trait for getting the average X of Y
// pub trait Average<T> {
//     /// Get the average value of X
//     fn average(&self) -> Option<T>;
// }
// impl<V: num_traits::Num + num_traits::NumCast + Copy> Average<V> for Vec<V> {
//     fn average(&self) -> Option<V> {
//         crate::lists::average(self)
//     }
// }
/// A dummy enum to be used as [`Option<NoneOnly>`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoneOnly {}

/// More tuple functions
mod tuple;

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

mod math;
pub use math::*;

mod cell;
pub use cell::*;

mod error;
pub use error::*;

mod repeat;
pub use repeat::*;

/// List operations
pub mod lists;
pub use lists::traits::*;

#[const_trait]
/// Writing out {variable} = `std::default::Default::default()`; is annoying, if only there was a function you could call from the variable itself.
pub trait SetToDefault {
    /// Set the value to its default form
    fn restore_default(&mut self);
}
impl<T: Default> SetToDefault for T {
    fn restore_default(&mut self) {
        *self = std::default::Default::default();
    }
}
/// An extension to `std::ops::Range`
pub mod range;
pub use range::*;
