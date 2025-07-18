/// A trait for getting the average X of Y
pub trait Average<T> {
    /// Get the average value of X
    fn average(&self) -> T;
}
impl<V: num_traits::Num + num_traits::NumCast + Copy> Average<V> for Vec<V> {
    fn average(&self) -> V {
        crate::lists::average(&self)
    }
}
/// A dummy enum to be used as Option<NoneOnly>
pub enum NoneOnly {}

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
#[allow(unused_imports)]
pub use small_u_support::*;

mod math;
pub use math::*;
