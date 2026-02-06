/// A dummy enum to be used as [`Option<NoneOnly>`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

mod cell;
pub use cell::*;

mod error;
pub use error::*;

#[cfg(feature = "std")]
mod repeat;
#[cfg(feature = "std")]
pub use repeat::*;

/// List operations
#[cfg(feature = "std")]
mod lists;
#[cfg(feature = "std")]
pub use lists::*;

/// Writing out {variable} = `core::default::Default::default()`; is annoying, if only there was a function you could call from the variable itself.
pub const trait SetToDefault {
    /// Set the value to its default form
    fn restore_default(&mut self);
}
impl<T: Default> SetToDefault for T {
    fn restore_default(&mut self) {
        *self = core::default::Default::default();
    }
}
/// An extension to `core::ops::Range`
pub mod range;
pub use range::*;

mod conversion;
pub use conversion::*;
mod macro_fun;

mod char;
pub use char::*;

// #[cfg(not(feature = "std"))]
// mod without_std;
// #[cfg(not(feature = "std"))]
// pub use without_std::*;

/// A trait for quickly printing to console by just calling [`.println_self()`](LogToConsole::println_self) instead of [`println!("{:?}", self)`](std::println)
pub const trait LogToConsole: LogToConsoleHelper {
    /// Equivalent to println!("{self:?}")
    fn println_self(&self);
    /// Equivalent to print!("{self:?}")
    fn print_self(&self);
}
/// A helper trait for [`LogToConsole`] which automatically implements prefix and suffix functions
pub const trait LogToConsoleHelper {
    /// Print the given value with a prefix
    fn println_self_prefixed(&self, prefix: &str);
    /// Print the given value with a suffix
    fn println_self_suffixed(&self, suffix: &str);
}
#[cfg(feature = "std")]
impl<T: LogToConsole + core::fmt::Debug> LogToConsoleHelper for T {
    fn println_self_prefixed(&self, prefix: &str) {
        String::println_self(&format!("{prefix}{self:?}"));
    }
    fn println_self_suffixed(&self, suffix: &str) {
        String::println_self(&format!("{self:?}{suffix}"));
    }
}
#[cfg(feature = "std")]
impl<T: core::fmt::Debug> LogToConsole for T {
    fn println_self(&self) {
        println!("{self:?}");
    }
    fn print_self(&self) {
        print!("{self:?}");
    }
}
