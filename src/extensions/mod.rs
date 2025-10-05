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

#[cfg(not(feature = "do_not_compile_extension_tuple_support"))]
/// More tuple functions
mod tuple;
#[cfg(not(feature = "do_not_compile_extension_tuple_support"))]
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

#[const_trait]
/// A trait that allows for data to repeat
pub trait RepeatData
where
    Self: Sized + Clone,
{
    /// Repeat the given data X times and return a Vec containing the repeated data
    fn repeat_value(self, times: usize) -> Vec<Self>;
}
impl<T: Sized + Clone> RepeatData for T {
    fn repeat_value(self, amount: usize) -> Vec<T> {
        std::vec::from_elem(self, amount)
    }
}
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
