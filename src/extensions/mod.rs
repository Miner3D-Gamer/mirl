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

#[cfg(feature = "std")]
mod string;
#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
mod cell;
#[cfg(feature = "std")]
pub use cell::*;

#[cfg(feature = "std")]
mod error;
#[cfg(feature = "std")]
pub use error::*;

#[cfg(feature = "std")]
mod repeat;
#[cfg(feature = "std")]
pub use repeat::*;

/// List operations
#[cfg(feature = "std")]
pub mod lists;
#[cfg(feature = "std")]
pub use lists::traits::*;

#[cfg(feature = "std")]
#[const_trait]
/// Writing out {variable} = `std::default::Default::default()`; is annoying, if only there was a function you could call from the variable itself.
pub trait SetToDefault {
    /// Set the value to its default form
    fn restore_default(&mut self);
}
#[cfg(feature = "std")]
impl<T: Default> SetToDefault for T {
    fn restore_default(&mut self) {
        *self = std::default::Default::default();
    }
}
/// An extension to `std::ops::Range`
#[cfg(feature = "std")]
pub mod range;
#[cfg(feature = "std")]
pub use range::*;

mod try_from_patch;
pub use try_from_patch::*;
mod macro_fun;
