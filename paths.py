# extracted using 'cargo public-api --features all'
# Skip to line ~6k for actual code
file = """
pub mod mirl
pub mod mirl::console
pub fn mirl::console::clear_console() -> std::io::error::Result<()>
pub fn mirl::console::clear_lines(n: usize) -> std::io::error::Result<()>
pub fn mirl::console::color(msg: &str, r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) -> alloc::string::String
pub fn mirl::console::color_background(msg: &str, r: u8, g: u8, b: u8) -> alloc::string::String
pub fn mirl::console::color_text(msg: &str, r: u8, g: u8, b: u8) -> alloc::string::String
pub fn mirl::console::get_console_content(max_lines: usize) -> alloc::vec::Vec<alloc::string::String>
pub fn mirl::console::input(msg: &str) -> std::io::error::Result<alloc::string::String>
pub fn mirl::console::move_to_top() -> std::io::error::Result<()>
pub fn mirl::console::print_color(buffer: &[mirl::graphics::Pixel])
pub fn mirl::console::print_color_v(buffer: &[mirl::graphics::Pixel], width: usize)
pub fn mirl::console::reset_color() -> alloc::string::String
pub mod mirl::constants
pub mod mirl::constants::bytes
pub const mirl::constants::bytes::BB: u128
pub const mirl::constants::bytes::BYTE: usize
pub const mirl::constants::bytes::EB: usize
pub const mirl::constants::bytes::GB: usize
pub const mirl::constants::bytes::GPB: u128
pub const mirl::constants::bytes::KB: usize
pub const mirl::constants::bytes::MB: usize
pub const mirl::constants::bytes::PB: usize
pub const mirl::constants::bytes::QB: u128
pub const mirl::constants::bytes::RB: u128
pub const mirl::constants::bytes::TB: usize
pub const mirl::constants::bytes::YB: u128
pub const mirl::constants::bytes::ZB: u128
pub mod mirl::directions
pub enum mirl::directions::AllCardinalDirections
pub mirl::directions::AllCardinalDirections::Base(mirl::directions::Directions)
pub mirl::directions::AllCardinalDirections::Extended(mirl::directions::ExtendedDirections)
impl core::clone::Clone for mirl::directions::AllCardinalDirections
pub fn mirl::directions::AllCardinalDirections::clone(&self) -> mirl::directions::AllCardinalDirections
impl core::cmp::Eq for mirl::directions::AllCardinalDirections
impl core::cmp::PartialEq for mirl::directions::AllCardinalDirections
pub fn mirl::directions::AllCardinalDirections::eq(&self, other: &mirl::directions::AllCardinalDirections) -> bool
impl core::fmt::Debug for mirl::directions::AllCardinalDirections
pub fn mirl::directions::AllCardinalDirections::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::directions::AllCardinalDirections
impl core::marker::StructuralPartialEq for mirl::directions::AllCardinalDirections
impl mirl::directions::RotateDirections for mirl::directions::AllCardinalDirections
pub fn mirl::directions::AllCardinalDirections::rotate_180(&self) -> Self
pub fn mirl::directions::AllCardinalDirections::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::AllCardinalDirections::rotate_counterclockwise_90(&self) -> Self
impl mirl::directions::RotatePrecise for mirl::directions::AllCardinalDirections
pub fn mirl::directions::AllCardinalDirections::rotate_clockwise_135(&self) -> Self
pub fn mirl::directions::AllCardinalDirections::rotate_clockwise_45(&self) -> Self
pub fn mirl::directions::AllCardinalDirections::rotate_counterclockwise_135(&self) -> Self
pub fn mirl::directions::AllCardinalDirections::rotate_counterclockwise_45(&self) -> Self
impl core::marker::Freeze for mirl::directions::AllCardinalDirections
impl core::marker::Send for mirl::directions::AllCardinalDirections
impl core::marker::Sync for mirl::directions::AllCardinalDirections
impl core::marker::Unpin for mirl::directions::AllCardinalDirections
impl core::panic::unwind_safe::RefUnwindSafe for mirl::directions::AllCardinalDirections
impl core::panic::unwind_safe::UnwindSafe for mirl::directions::AllCardinalDirections
impl<Q, K> equivalent::Equivalent<K> for mirl::directions::AllCardinalDirections where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::directions::AllCardinalDirections::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::directions::AllCardinalDirections where U: core::convert::From<T>
pub fn mirl::directions::AllCardinalDirections::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::directions::AllCardinalDirections where U: core::convert::Into<T>
pub type mirl::directions::AllCardinalDirections::Error = core::convert::Infallible
pub fn mirl::directions::AllCardinalDirections::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::directions::AllCardinalDirections where U: core::convert::TryFrom<T>
pub type mirl::directions::AllCardinalDirections::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::directions::AllCardinalDirections::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::directions::AllCardinalDirections where T: core::clone::Clone
pub type mirl::directions::AllCardinalDirections::Owned = T
pub fn mirl::directions::AllCardinalDirections::clone_into(&self, target: &mut T)
pub fn mirl::directions::AllCardinalDirections::to_owned(&self) -> T
impl<T> core::any::Any for mirl::directions::AllCardinalDirections where T: 'static + ?core::marker::Sized
pub fn mirl::directions::AllCardinalDirections::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::directions::AllCardinalDirections where T: ?core::marker::Sized
pub fn mirl::directions::AllCardinalDirections::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::directions::AllCardinalDirections where T: ?core::marker::Sized
pub fn mirl::directions::AllCardinalDirections::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::directions::AllCardinalDirections where T: core::clone::Clone
pub unsafe fn mirl::directions::AllCardinalDirections::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::directions::AllCardinalDirections
pub fn mirl::directions::AllCardinalDirections::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::directions::AllCardinalDirections
pub type mirl::directions::AllCardinalDirections::Init = T
pub const mirl::directions::AllCardinalDirections::ALIGN: usize
pub unsafe fn mirl::directions::AllCardinalDirections::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::directions::AllCardinalDirections::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::directions::AllCardinalDirections::drop(ptr: usize)
pub unsafe fn mirl::directions::AllCardinalDirections::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::directions::AllCardinalDirections
impl<T> mirl::extensions::RepeatData for mirl::directions::AllCardinalDirections where T: core::clone::Clone
pub fn mirl::directions::AllCardinalDirections::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::directions::AllDirections
pub mirl::directions::AllDirections::Base(mirl::directions::Directions)
pub mirl::directions::AllDirections::Extended(mirl::directions::ExtendedDirections)
pub mirl::directions::AllDirections::Special(mirl::directions::SpecialDirections)
impl core::clone::Clone for mirl::directions::AllDirections
pub fn mirl::directions::AllDirections::clone(&self) -> mirl::directions::AllDirections
impl core::cmp::Eq for mirl::directions::AllDirections
impl core::cmp::PartialEq for mirl::directions::AllDirections
pub fn mirl::directions::AllDirections::eq(&self, other: &mirl::directions::AllDirections) -> bool
impl core::fmt::Debug for mirl::directions::AllDirections
pub fn mirl::directions::AllDirections::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::directions::AllDirections
impl core::marker::StructuralPartialEq for mirl::directions::AllDirections
impl mirl::directions::RotateDirections for mirl::directions::AllDirections
pub fn mirl::directions::AllDirections::rotate_180(&self) -> Self
pub fn mirl::directions::AllDirections::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::AllDirections::rotate_counterclockwise_90(&self) -> Self
impl mirl::directions::RotatePrecise for mirl::directions::AllDirections
pub fn mirl::directions::AllDirections::rotate_clockwise_135(&self) -> Self
pub fn mirl::directions::AllDirections::rotate_clockwise_45(&self) -> Self
pub fn mirl::directions::AllDirections::rotate_counterclockwise_135(&self) -> Self
pub fn mirl::directions::AllDirections::rotate_counterclockwise_45(&self) -> Self
impl core::marker::Freeze for mirl::directions::AllDirections
impl core::marker::Send for mirl::directions::AllDirections
impl core::marker::Sync for mirl::directions::AllDirections
impl core::marker::Unpin for mirl::directions::AllDirections
impl core::panic::unwind_safe::RefUnwindSafe for mirl::directions::AllDirections
impl core::panic::unwind_safe::UnwindSafe for mirl::directions::AllDirections
impl<Q, K> equivalent::Equivalent<K> for mirl::directions::AllDirections where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::directions::AllDirections::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::directions::AllDirections where U: core::convert::From<T>
pub fn mirl::directions::AllDirections::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::directions::AllDirections where U: core::convert::Into<T>
pub type mirl::directions::AllDirections::Error = core::convert::Infallible
pub fn mirl::directions::AllDirections::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::directions::AllDirections where U: core::convert::TryFrom<T>
pub type mirl::directions::AllDirections::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::directions::AllDirections::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::directions::AllDirections where T: core::clone::Clone
pub type mirl::directions::AllDirections::Owned = T
pub fn mirl::directions::AllDirections::clone_into(&self, target: &mut T)
pub fn mirl::directions::AllDirections::to_owned(&self) -> T
impl<T> core::any::Any for mirl::directions::AllDirections where T: 'static + ?core::marker::Sized
pub fn mirl::directions::AllDirections::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::directions::AllDirections where T: ?core::marker::Sized
pub fn mirl::directions::AllDirections::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::directions::AllDirections where T: ?core::marker::Sized
pub fn mirl::directions::AllDirections::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::directions::AllDirections where T: core::clone::Clone
pub unsafe fn mirl::directions::AllDirections::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::directions::AllDirections
pub fn mirl::directions::AllDirections::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::directions::AllDirections
pub type mirl::directions::AllDirections::Init = T
pub const mirl::directions::AllDirections::ALIGN: usize
pub unsafe fn mirl::directions::AllDirections::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::directions::AllDirections::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::directions::AllDirections::drop(ptr: usize)
pub unsafe fn mirl::directions::AllDirections::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::directions::AllDirections
impl<T> mirl::extensions::RepeatData for mirl::directions::AllDirections where T: core::clone::Clone
pub fn mirl::directions::AllDirections::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::directions::Directions
pub mirl::directions::Directions::East
pub mirl::directions::Directions::North
pub mirl::directions::Directions::South
pub mirl::directions::Directions::West
impl core::clone::Clone for mirl::directions::Directions
pub fn mirl::directions::Directions::clone(&self) -> mirl::directions::Directions
impl core::cmp::Eq for mirl::directions::Directions
impl core::cmp::PartialEq for mirl::directions::Directions
pub fn mirl::directions::Directions::eq(&self, other: &mirl::directions::Directions) -> bool
impl core::fmt::Debug for mirl::directions::Directions
pub fn mirl::directions::Directions::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::directions::Directions
impl core::marker::StructuralPartialEq for mirl::directions::Directions
impl mirl::directions::RotateDirections for mirl::directions::Directions
pub fn mirl::directions::Directions::rotate_180(&self) -> Self
pub fn mirl::directions::Directions::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::Directions::rotate_counterclockwise_90(&self) -> Self
impl core::marker::Freeze for mirl::directions::Directions
impl core::marker::Send for mirl::directions::Directions
impl core::marker::Sync for mirl::directions::Directions
impl core::marker::Unpin for mirl::directions::Directions
impl core::panic::unwind_safe::RefUnwindSafe for mirl::directions::Directions
impl core::panic::unwind_safe::UnwindSafe for mirl::directions::Directions
impl<Q, K> equivalent::Equivalent<K> for mirl::directions::Directions where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::directions::Directions::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::directions::Directions where U: core::convert::From<T>
pub fn mirl::directions::Directions::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::directions::Directions where U: core::convert::Into<T>
pub type mirl::directions::Directions::Error = core::convert::Infallible
pub fn mirl::directions::Directions::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::directions::Directions where U: core::convert::TryFrom<T>
pub type mirl::directions::Directions::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::directions::Directions::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::directions::Directions where T: core::clone::Clone
pub type mirl::directions::Directions::Owned = T
pub fn mirl::directions::Directions::clone_into(&self, target: &mut T)
pub fn mirl::directions::Directions::to_owned(&self) -> T
impl<T> core::any::Any for mirl::directions::Directions where T: 'static + ?core::marker::Sized
pub fn mirl::directions::Directions::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::directions::Directions where T: ?core::marker::Sized
pub fn mirl::directions::Directions::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::directions::Directions where T: ?core::marker::Sized
pub fn mirl::directions::Directions::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::directions::Directions where T: core::clone::Clone
pub unsafe fn mirl::directions::Directions::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::directions::Directions
pub fn mirl::directions::Directions::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::directions::Directions
pub type mirl::directions::Directions::Init = T
pub const mirl::directions::Directions::ALIGN: usize
pub unsafe fn mirl::directions::Directions::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::directions::Directions::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::directions::Directions::drop(ptr: usize)
pub unsafe fn mirl::directions::Directions::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::directions::Directions
impl<T> mirl::extensions::RepeatData for mirl::directions::Directions where T: core::clone::Clone
pub fn mirl::directions::Directions::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::directions::DirectionsWithNone
pub mirl::directions::DirectionsWithNone::Base(mirl::directions::Directions)
pub mirl::directions::DirectionsWithNone::Special(mirl::directions::SpecialDirections)
impl core::clone::Clone for mirl::directions::DirectionsWithNone
pub fn mirl::directions::DirectionsWithNone::clone(&self) -> mirl::directions::DirectionsWithNone
impl core::cmp::Eq for mirl::directions::DirectionsWithNone
impl core::cmp::PartialEq for mirl::directions::DirectionsWithNone
pub fn mirl::directions::DirectionsWithNone::eq(&self, other: &mirl::directions::DirectionsWithNone) -> bool
impl core::fmt::Debug for mirl::directions::DirectionsWithNone
pub fn mirl::directions::DirectionsWithNone::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::directions::DirectionsWithNone
impl core::marker::StructuralPartialEq for mirl::directions::DirectionsWithNone
impl mirl::directions::RotateDirections for mirl::directions::DirectionsWithNone
pub fn mirl::directions::DirectionsWithNone::rotate_180(&self) -> Self
pub fn mirl::directions::DirectionsWithNone::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::DirectionsWithNone::rotate_counterclockwise_90(&self) -> Self
impl core::marker::Freeze for mirl::directions::DirectionsWithNone
impl core::marker::Send for mirl::directions::DirectionsWithNone
impl core::marker::Sync for mirl::directions::DirectionsWithNone
impl core::marker::Unpin for mirl::directions::DirectionsWithNone
impl core::panic::unwind_safe::RefUnwindSafe for mirl::directions::DirectionsWithNone
impl core::panic::unwind_safe::UnwindSafe for mirl::directions::DirectionsWithNone
impl<Q, K> equivalent::Equivalent<K> for mirl::directions::DirectionsWithNone where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::directions::DirectionsWithNone::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::directions::DirectionsWithNone where U: core::convert::From<T>
pub fn mirl::directions::DirectionsWithNone::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::directions::DirectionsWithNone where U: core::convert::Into<T>
pub type mirl::directions::DirectionsWithNone::Error = core::convert::Infallible
pub fn mirl::directions::DirectionsWithNone::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::directions::DirectionsWithNone where U: core::convert::TryFrom<T>
pub type mirl::directions::DirectionsWithNone::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::directions::DirectionsWithNone::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::directions::DirectionsWithNone where T: core::clone::Clone
pub type mirl::directions::DirectionsWithNone::Owned = T
pub fn mirl::directions::DirectionsWithNone::clone_into(&self, target: &mut T)
pub fn mirl::directions::DirectionsWithNone::to_owned(&self) -> T
impl<T> core::any::Any for mirl::directions::DirectionsWithNone where T: 'static + ?core::marker::Sized
pub fn mirl::directions::DirectionsWithNone::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::directions::DirectionsWithNone where T: ?core::marker::Sized
pub fn mirl::directions::DirectionsWithNone::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::directions::DirectionsWithNone where T: ?core::marker::Sized
pub fn mirl::directions::DirectionsWithNone::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::directions::DirectionsWithNone where T: core::clone::Clone
pub unsafe fn mirl::directions::DirectionsWithNone::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::directions::DirectionsWithNone
pub fn mirl::directions::DirectionsWithNone::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::directions::DirectionsWithNone
pub type mirl::directions::DirectionsWithNone::Init = T
pub const mirl::directions::DirectionsWithNone::ALIGN: usize
pub unsafe fn mirl::directions::DirectionsWithNone::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::directions::DirectionsWithNone::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::directions::DirectionsWithNone::drop(ptr: usize)
pub unsafe fn mirl::directions::DirectionsWithNone::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::directions::DirectionsWithNone
impl<T> mirl::extensions::RepeatData for mirl::directions::DirectionsWithNone where T: core::clone::Clone
pub fn mirl::directions::DirectionsWithNone::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::directions::ExtendedDirections
pub mirl::directions::ExtendedDirections::NorthEast
pub mirl::directions::ExtendedDirections::NorthWest
pub mirl::directions::ExtendedDirections::SouthEast
pub mirl::directions::ExtendedDirections::SouthWest
impl core::clone::Clone for mirl::directions::ExtendedDirections
pub fn mirl::directions::ExtendedDirections::clone(&self) -> mirl::directions::ExtendedDirections
impl core::cmp::Eq for mirl::directions::ExtendedDirections
impl core::cmp::PartialEq for mirl::directions::ExtendedDirections
pub fn mirl::directions::ExtendedDirections::eq(&self, other: &mirl::directions::ExtendedDirections) -> bool
impl core::fmt::Debug for mirl::directions::ExtendedDirections
pub fn mirl::directions::ExtendedDirections::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::directions::ExtendedDirections
impl core::marker::StructuralPartialEq for mirl::directions::ExtendedDirections
impl mirl::directions::RotateDirections for mirl::directions::ExtendedDirections
pub fn mirl::directions::ExtendedDirections::rotate_180(&self) -> Self
pub fn mirl::directions::ExtendedDirections::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::ExtendedDirections::rotate_counterclockwise_90(&self) -> Self
impl core::marker::Freeze for mirl::directions::ExtendedDirections
impl core::marker::Send for mirl::directions::ExtendedDirections
impl core::marker::Sync for mirl::directions::ExtendedDirections
impl core::marker::Unpin for mirl::directions::ExtendedDirections
impl core::panic::unwind_safe::RefUnwindSafe for mirl::directions::ExtendedDirections
impl core::panic::unwind_safe::UnwindSafe for mirl::directions::ExtendedDirections
impl<Q, K> equivalent::Equivalent<K> for mirl::directions::ExtendedDirections where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::directions::ExtendedDirections::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::directions::ExtendedDirections where U: core::convert::From<T>
pub fn mirl::directions::ExtendedDirections::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::directions::ExtendedDirections where U: core::convert::Into<T>
pub type mirl::directions::ExtendedDirections::Error = core::convert::Infallible
pub fn mirl::directions::ExtendedDirections::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::directions::ExtendedDirections where U: core::convert::TryFrom<T>
pub type mirl::directions::ExtendedDirections::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::directions::ExtendedDirections::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::directions::ExtendedDirections where T: core::clone::Clone
pub type mirl::directions::ExtendedDirections::Owned = T
pub fn mirl::directions::ExtendedDirections::clone_into(&self, target: &mut T)
pub fn mirl::directions::ExtendedDirections::to_owned(&self) -> T
impl<T> core::any::Any for mirl::directions::ExtendedDirections where T: 'static + ?core::marker::Sized
pub fn mirl::directions::ExtendedDirections::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::directions::ExtendedDirections where T: ?core::marker::Sized
pub fn mirl::directions::ExtendedDirections::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::directions::ExtendedDirections where T: ?core::marker::Sized
pub fn mirl::directions::ExtendedDirections::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::directions::ExtendedDirections where T: core::clone::Clone
pub unsafe fn mirl::directions::ExtendedDirections::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::directions::ExtendedDirections
pub fn mirl::directions::ExtendedDirections::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::directions::ExtendedDirections
pub type mirl::directions::ExtendedDirections::Init = T
pub const mirl::directions::ExtendedDirections::ALIGN: usize
pub unsafe fn mirl::directions::ExtendedDirections::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::directions::ExtendedDirections::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::directions::ExtendedDirections::drop(ptr: usize)
pub unsafe fn mirl::directions::ExtendedDirections::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::directions::ExtendedDirections
impl<T> mirl::extensions::RepeatData for mirl::directions::ExtendedDirections where T: core::clone::Clone
pub fn mirl::directions::ExtendedDirections::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::directions::SpecialDirections
pub mirl::directions::SpecialDirections::None
impl core::clone::Clone for mirl::directions::SpecialDirections
pub fn mirl::directions::SpecialDirections::clone(&self) -> mirl::directions::SpecialDirections
impl core::cmp::Eq for mirl::directions::SpecialDirections
impl core::cmp::PartialEq for mirl::directions::SpecialDirections
pub fn mirl::directions::SpecialDirections::eq(&self, other: &mirl::directions::SpecialDirections) -> bool
impl core::fmt::Debug for mirl::directions::SpecialDirections
pub fn mirl::directions::SpecialDirections::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::directions::SpecialDirections
impl core::marker::StructuralPartialEq for mirl::directions::SpecialDirections
impl core::marker::Freeze for mirl::directions::SpecialDirections
impl core::marker::Send for mirl::directions::SpecialDirections
impl core::marker::Sync for mirl::directions::SpecialDirections
impl core::marker::Unpin for mirl::directions::SpecialDirections
impl core::panic::unwind_safe::RefUnwindSafe for mirl::directions::SpecialDirections
impl core::panic::unwind_safe::UnwindSafe for mirl::directions::SpecialDirections
impl<Q, K> equivalent::Equivalent<K> for mirl::directions::SpecialDirections where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::directions::SpecialDirections::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::directions::SpecialDirections where U: core::convert::From<T>
pub fn mirl::directions::SpecialDirections::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::directions::SpecialDirections where U: core::convert::Into<T>
pub type mirl::directions::SpecialDirections::Error = core::convert::Infallible
pub fn mirl::directions::SpecialDirections::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::directions::SpecialDirections where U: core::convert::TryFrom<T>
pub type mirl::directions::SpecialDirections::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::directions::SpecialDirections::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::directions::SpecialDirections where T: core::clone::Clone
pub type mirl::directions::SpecialDirections::Owned = T
pub fn mirl::directions::SpecialDirections::clone_into(&self, target: &mut T)
pub fn mirl::directions::SpecialDirections::to_owned(&self) -> T
impl<T> core::any::Any for mirl::directions::SpecialDirections where T: 'static + ?core::marker::Sized
pub fn mirl::directions::SpecialDirections::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::directions::SpecialDirections where T: ?core::marker::Sized
pub fn mirl::directions::SpecialDirections::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::directions::SpecialDirections where T: ?core::marker::Sized
pub fn mirl::directions::SpecialDirections::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::directions::SpecialDirections where T: core::clone::Clone
pub unsafe fn mirl::directions::SpecialDirections::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::directions::SpecialDirections
pub fn mirl::directions::SpecialDirections::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::directions::SpecialDirections
pub type mirl::directions::SpecialDirections::Init = T
pub const mirl::directions::SpecialDirections::ALIGN: usize
pub unsafe fn mirl::directions::SpecialDirections::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::directions::SpecialDirections::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::directions::SpecialDirections::drop(ptr: usize)
pub unsafe fn mirl::directions::SpecialDirections::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::directions::SpecialDirections
impl<T> mirl::extensions::RepeatData for mirl::directions::SpecialDirections where T: core::clone::Clone
pub fn mirl::directions::SpecialDirections::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub trait mirl::directions::RotateDirections: core::marker::Sized
pub fn mirl::directions::RotateDirections::rotate_180(&self) -> Self
pub fn mirl::directions::RotateDirections::rotate_clockwise_270(&self) -> Self
pub fn mirl::directions::RotateDirections::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::RotateDirections::rotate_counterclockwise_270(&self) -> Self
pub fn mirl::directions::RotateDirections::rotate_counterclockwise_90(&self) -> Self
impl mirl::directions::RotateDirections for mirl::directions::AllCardinalDirections
pub fn mirl::directions::AllCardinalDirections::rotate_180(&self) -> Self
pub fn mirl::directions::AllCardinalDirections::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::AllCardinalDirections::rotate_counterclockwise_90(&self) -> Self
impl mirl::directions::RotateDirections for mirl::directions::AllDirections
pub fn mirl::directions::AllDirections::rotate_180(&self) -> Self
pub fn mirl::directions::AllDirections::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::AllDirections::rotate_counterclockwise_90(&self) -> Self
impl mirl::directions::RotateDirections for mirl::directions::Directions
pub fn mirl::directions::Directions::rotate_180(&self) -> Self
pub fn mirl::directions::Directions::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::Directions::rotate_counterclockwise_90(&self) -> Self
impl mirl::directions::RotateDirections for mirl::directions::DirectionsWithNone
pub fn mirl::directions::DirectionsWithNone::rotate_180(&self) -> Self
pub fn mirl::directions::DirectionsWithNone::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::DirectionsWithNone::rotate_counterclockwise_90(&self) -> Self
impl mirl::directions::RotateDirections for mirl::directions::ExtendedDirections
pub fn mirl::directions::ExtendedDirections::rotate_180(&self) -> Self
pub fn mirl::directions::ExtendedDirections::rotate_clockwise_90(&self) -> Self
pub fn mirl::directions::ExtendedDirections::rotate_counterclockwise_90(&self) -> Self
pub trait mirl::directions::RotatePrecise: mirl::directions::RotateDirections
pub fn mirl::directions::RotatePrecise::rotate_clockwise_135(&self) -> Self
pub fn mirl::directions::RotatePrecise::rotate_clockwise_45(&self) -> Self
pub fn mirl::directions::RotatePrecise::rotate_counterclockwise_135(&self) -> Self
pub fn mirl::directions::RotatePrecise::rotate_counterclockwise_45(&self) -> Self
impl mirl::directions::RotatePrecise for mirl::directions::AllCardinalDirections
pub fn mirl::directions::AllCardinalDirections::rotate_clockwise_135(&self) -> Self
pub fn mirl::directions::AllCardinalDirections::rotate_clockwise_45(&self) -> Self
pub fn mirl::directions::AllCardinalDirections::rotate_counterclockwise_135(&self) -> Self
pub fn mirl::directions::AllCardinalDirections::rotate_counterclockwise_45(&self) -> Self
impl mirl::directions::RotatePrecise for mirl::directions::AllDirections
pub fn mirl::directions::AllDirections::rotate_clockwise_135(&self) -> Self
pub fn mirl::directions::AllDirections::rotate_clockwise_45(&self) -> Self
pub fn mirl::directions::AllDirections::rotate_counterclockwise_135(&self) -> Self
pub fn mirl::directions::AllDirections::rotate_counterclockwise_45(&self) -> Self
pub mod mirl::extensions
pub enum mirl::extensions::NoneOnly
impl core::clone::Clone for mirl::extensions::NoneOnly
pub fn mirl::extensions::NoneOnly::clone(&self) -> mirl::extensions::NoneOnly
impl core::cmp::Eq for mirl::extensions::NoneOnly
impl core::cmp::PartialEq for mirl::extensions::NoneOnly
pub fn mirl::extensions::NoneOnly::eq(&self, other: &mirl::extensions::NoneOnly) -> bool
impl core::fmt::Debug for mirl::extensions::NoneOnly
pub fn mirl::extensions::NoneOnly::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::extensions::NoneOnly
impl core::marker::StructuralPartialEq for mirl::extensions::NoneOnly
impl core::marker::Freeze for mirl::extensions::NoneOnly
impl core::marker::Send for mirl::extensions::NoneOnly
impl core::marker::Sync for mirl::extensions::NoneOnly
impl core::marker::Unpin for mirl::extensions::NoneOnly
impl core::panic::unwind_safe::RefUnwindSafe for mirl::extensions::NoneOnly
impl core::panic::unwind_safe::UnwindSafe for mirl::extensions::NoneOnly
impl<Q, K> equivalent::Equivalent<K> for mirl::extensions::NoneOnly where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::extensions::NoneOnly::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::extensions::NoneOnly where U: core::convert::From<T>
pub fn mirl::extensions::NoneOnly::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::extensions::NoneOnly where U: core::convert::Into<T>
pub type mirl::extensions::NoneOnly::Error = core::convert::Infallible
pub fn mirl::extensions::NoneOnly::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::extensions::NoneOnly where U: core::convert::TryFrom<T>
pub type mirl::extensions::NoneOnly::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::extensions::NoneOnly::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::extensions::NoneOnly where T: core::clone::Clone
pub type mirl::extensions::NoneOnly::Owned = T
pub fn mirl::extensions::NoneOnly::clone_into(&self, target: &mut T)
pub fn mirl::extensions::NoneOnly::to_owned(&self) -> T
impl<T> core::any::Any for mirl::extensions::NoneOnly where T: 'static + ?core::marker::Sized
pub fn mirl::extensions::NoneOnly::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::extensions::NoneOnly where T: ?core::marker::Sized
pub fn mirl::extensions::NoneOnly::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::extensions::NoneOnly where T: ?core::marker::Sized
pub fn mirl::extensions::NoneOnly::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::extensions::NoneOnly where T: core::clone::Clone
pub unsafe fn mirl::extensions::NoneOnly::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::extensions::NoneOnly
pub fn mirl::extensions::NoneOnly::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::extensions::NoneOnly
pub type mirl::extensions::NoneOnly::Init = T
pub const mirl::extensions::NoneOnly::ALIGN: usize
pub unsafe fn mirl::extensions::NoneOnly::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::extensions::NoneOnly::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::extensions::NoneOnly::drop(ptr: usize)
pub unsafe fn mirl::extensions::NoneOnly::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::extensions::NoneOnly
impl<T> mirl::extensions::RepeatData for mirl::extensions::NoneOnly where T: core::clone::Clone
pub fn mirl::extensions::NoneOnly::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::extensions::U1
pub mirl::extensions::U1::b0: bool
impl mirl::extensions::U1
pub const fn mirl::extensions::U1::from_bool(val: bool) -> Self
pub const fn mirl::extensions::U1::from_u8_trunc(val: u8) -> Self
pub const fn mirl::extensions::U1::is_max(self) -> bool
pub const fn mirl::extensions::U1::is_zero(self) -> bool
pub fn mirl::extensions::U1::new(val: u8) -> Self
pub const fn mirl::extensions::U1::to_bool(self) -> bool
pub const fn mirl::extensions::U1::value(self) -> u8
pub const fn mirl::extensions::U1::wrapping_add(self, other: Self) -> Self
pub const fn mirl::extensions::U1::wrapping_sub(self, other: Self) -> Self
impl mirl::extensions::U1
pub fn mirl::extensions::U1::to_u2(self) -> mirl::extensions::U2
pub fn mirl::extensions::U1::to_u4(self) -> mirl::extensions::U4
impl core::clone::Clone for mirl::extensions::U1
pub fn mirl::extensions::U1::clone(&self) -> mirl::extensions::U1
impl core::cmp::Eq for mirl::extensions::U1
impl core::cmp::PartialEq for mirl::extensions::U1
pub fn mirl::extensions::U1::eq(&self, other: &mirl::extensions::U1) -> bool
impl core::cmp::PartialOrd for mirl::extensions::U1
pub fn mirl::extensions::U1::partial_cmp(&self, other: &mirl::extensions::U1) -> core::option::Option<core::cmp::Ordering>
impl core::convert::From<bool> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: bool) -> Self
impl core::convert::From<f32> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: f32) -> Self
impl core::convert::From<f64> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: f64) -> Self
impl core::convert::From<i128> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: i128) -> Self
impl core::convert::From<i16> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: i16) -> Self
impl core::convert::From<i32> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: i32) -> Self
impl core::convert::From<i64> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: i64) -> Self
impl core::convert::From<i8> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: i8) -> Self
impl core::convert::From<isize> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: isize) -> Self
impl core::convert::From<mirl::extensions::U1> for bool
pub fn bool::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for f32
pub fn f32::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for f64
pub fn f64::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for i128
pub fn i128::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for i16
pub fn i16::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for i32
pub fn i32::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for i64
pub fn i64::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for i8
pub fn i8::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for isize
pub fn isize::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for u128
pub fn u128::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for u16
pub fn u16::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for u32
pub fn u32::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for u64
pub fn u64::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for u8
pub fn u8::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U1> for usize
pub fn usize::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U2> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U4> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<u128> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: u128) -> Self
impl core::convert::From<u16> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: u16) -> Self
impl core::convert::From<u32> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: u32) -> Self
impl core::convert::From<u64> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: u64) -> Self
impl core::convert::From<u8> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: u8) -> Self
impl core::convert::From<usize> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: usize) -> Self
impl core::fmt::Debug for mirl::extensions::U1
pub fn mirl::extensions::U1::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::extensions::U1
impl core::marker::StructuralPartialEq for mirl::extensions::U1
impl core::ops::arith::Add for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::add(self, rhs: Self) -> Self
impl core::ops::arith::Div for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::div(self, rhs: Self) -> Self
impl core::ops::arith::Mul for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::mul(self, rhs: Self) -> Self
impl core::ops::arith::Rem for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::rem(self, rhs: Self) -> Self::Output
impl core::ops::arith::Sub for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::sub(self, rhs: Self) -> Self
impl core::ops::bit::BitAnd for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::bitand(self, rhs: Self) -> Self
impl core::ops::bit::BitOr for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::bitor(self, rhs: Self) -> Self
impl core::ops::bit::BitXor for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::bitxor(self, rhs: Self) -> Self
impl core::ops::bit::Not for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::not(self) -> Self
impl core::ops::bit::Shl<usize> for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::shl(self, rhs: usize) -> Self
impl core::ops::bit::Shr<usize> for mirl::extensions::U1
pub type mirl::extensions::U1::Output = mirl::extensions::U1
pub fn mirl::extensions::U1::shr(self, rhs: usize) -> Self
impl num_traits::Num for mirl::extensions::U1
pub type mirl::extensions::U1::FromStrRadixErr = core::num::error::ParseIntError
pub fn mirl::extensions::U1::from_str_radix(str: &str, radix: u32) -> core::result::Result<Self, Self::FromStrRadixErr>
impl num_traits::cast::NumCast for mirl::extensions::U1
pub fn mirl::extensions::U1::from<T: num_traits::cast::ToPrimitive>(n: T) -> core::option::Option<Self>
impl num_traits::cast::ToPrimitive for mirl::extensions::U1
pub fn mirl::extensions::U1::to_f32(&self) -> core::option::Option<f32>
pub fn mirl::extensions::U1::to_f64(&self) -> core::option::Option<f64>
pub fn mirl::extensions::U1::to_i128(&self) -> core::option::Option<i128>
pub fn mirl::extensions::U1::to_i16(&self) -> core::option::Option<i16>
pub fn mirl::extensions::U1::to_i32(&self) -> core::option::Option<i32>
pub fn mirl::extensions::U1::to_i64(&self) -> core::option::Option<i64>
pub fn mirl::extensions::U1::to_i8(&self) -> core::option::Option<i8>
pub fn mirl::extensions::U1::to_isize(&self) -> core::option::Option<isize>
pub fn mirl::extensions::U1::to_u128(&self) -> core::option::Option<u128>
pub fn mirl::extensions::U1::to_u16(&self) -> core::option::Option<u16>
pub fn mirl::extensions::U1::to_u32(&self) -> core::option::Option<u32>
pub fn mirl::extensions::U1::to_u64(&self) -> core::option::Option<u64>
pub fn mirl::extensions::U1::to_u8(&self) -> core::option::Option<u8>
impl num_traits::identities::One for mirl::extensions::U1
pub fn mirl::extensions::U1::is_one(&self) -> bool where Self: core::cmp::PartialEq
pub fn mirl::extensions::U1::one() -> Self
impl num_traits::identities::Zero for mirl::extensions::U1
pub fn mirl::extensions::U1::is_zero(&self) -> bool
pub fn mirl::extensions::U1::zero() -> Self
impl core::marker::Freeze for mirl::extensions::U1
impl core::marker::Send for mirl::extensions::U1
impl core::marker::Sync for mirl::extensions::U1
impl core::marker::Unpin for mirl::extensions::U1
impl core::panic::unwind_safe::RefUnwindSafe for mirl::extensions::U1
impl core::panic::unwind_safe::UnwindSafe for mirl::extensions::U1
impl<Q, K> equivalent::Equivalent<K> for mirl::extensions::U1 where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::extensions::U1::equivalent(&self, key: &K) -> bool
impl<T, Rhs, Output> num_traits::NumOps<Rhs, Output> for mirl::extensions::U1 where T: core::ops::arith::Sub<Rhs, Output = Output> + core::ops::arith::Mul<Rhs, Output = Output> + core::ops::arith::Div<Rhs, Output = Output> + core::ops::arith::Add<Rhs, Output = Output> + core::ops::arith::Rem<Rhs, Output = Output>
impl<T, U> core::convert::Into<U> for mirl::extensions::U1 where U: core::convert::From<T>
pub fn mirl::extensions::U1::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::extensions::U1 where U: core::convert::Into<T>
pub type mirl::extensions::U1::Error = core::convert::Infallible
pub fn mirl::extensions::U1::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::extensions::U1 where U: core::convert::TryFrom<T>
pub type mirl::extensions::U1::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::extensions::U1::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::extensions::U1 where T: core::clone::Clone
pub type mirl::extensions::U1::Owned = T
pub fn mirl::extensions::U1::clone_into(&self, target: &mut T)
pub fn mirl::extensions::U1::to_owned(&self) -> T
impl<T> core::any::Any for mirl::extensions::U1 where T: 'static + ?core::marker::Sized
pub fn mirl::extensions::U1::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::extensions::U1 where T: ?core::marker::Sized
pub fn mirl::extensions::U1::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::extensions::U1 where T: ?core::marker::Sized
pub fn mirl::extensions::U1::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::extensions::U1 where T: core::clone::Clone
pub unsafe fn mirl::extensions::U1::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::extensions::U1
pub type mirl::extensions::U1::Init = T
pub const mirl::extensions::U1::ALIGN: usize
pub unsafe fn mirl::extensions::U1::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::extensions::U1::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::extensions::U1::drop(ptr: usize)
pub unsafe fn mirl::extensions::U1::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::extensions::U1
impl<T> mirl::extensions::RepeatData for mirl::extensions::U1 where T: core::clone::Clone
pub fn mirl::extensions::U1::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
impl<T> mirl::math::NumberWithMonotoneOps for mirl::extensions::U1 where T: core::ops::arith::Sub<Output = T> + core::cmp::PartialOrd + core::ops::arith::Mul<Output = T> + num_traits::cast::NumCast + core::ops::arith::Div<Output = T> + core::ops::arith::Add<Output = T>
impl<T> mirl::math::TwoTillTen<T> for mirl::extensions::U1 where T: num_traits::identities::One + core::ops::arith::Add<Output = T>
pub struct mirl::extensions::U2
pub mirl::extensions::U2::b0: bool
pub mirl::extensions::U2::b1: bool
impl mirl::extensions::U2
pub const fn mirl::extensions::U2::combine_high_with(self, other: Self) -> mirl::extensions::U4
pub const fn mirl::extensions::U2::combine_low_with(self, other: Self) -> mirl::extensions::U4
pub const fn mirl::extensions::U2::from_u8_trunc(val: u8) -> Self
pub const fn mirl::extensions::U2::is_max(self) -> bool
pub const fn mirl::extensions::U2::is_zero(self) -> bool
pub fn mirl::extensions::U2::new(val: u8) -> Self
pub fn mirl::extensions::U2::to_u4(self) -> mirl::extensions::U4
pub const fn mirl::extensions::U2::value(self) -> u8
pub const fn mirl::extensions::U2::wrapping_add(self, other: Self) -> Self
pub const fn mirl::extensions::U2::wrapping_sub(self, other: Self) -> Self
impl mirl::extensions::U2
pub const fn mirl::extensions::U2::from_u1_pair(high: mirl::extensions::U1, low: mirl::extensions::U1) -> Self
pub const fn mirl::extensions::U2::split_to_u1_pair(self) -> (mirl::extensions::U1, mirl::extensions::U1)
pub fn mirl::extensions::U2::to_u1(self) -> mirl::extensions::U1
impl core::clone::Clone for mirl::extensions::U2
pub fn mirl::extensions::U2::clone(&self) -> mirl::extensions::U2
impl core::cmp::Eq for mirl::extensions::U2
impl core::cmp::PartialEq for mirl::extensions::U2
pub fn mirl::extensions::U2::eq(&self, other: &mirl::extensions::U2) -> bool
impl core::cmp::PartialOrd for mirl::extensions::U2
pub fn mirl::extensions::U2::partial_cmp(&self, other: &mirl::extensions::U2) -> core::option::Option<core::cmp::Ordering>
impl core::convert::From<f32> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: f32) -> Self
impl core::convert::From<f64> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: f64) -> Self
impl core::convert::From<i128> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: i128) -> Self
impl core::convert::From<i16> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: i16) -> Self
impl core::convert::From<i32> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: i32) -> Self
impl core::convert::From<i64> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: i64) -> Self
impl core::convert::From<i8> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: i8) -> Self
impl core::convert::From<isize> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: isize) -> Self
impl core::convert::From<mirl::extensions::U1> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U2> for f32
pub fn f32::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for f64
pub fn f64::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for i128
pub fn i128::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for i16
pub fn i16::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for i32
pub fn i32::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for i64
pub fn i64::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for i8
pub fn i8::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for isize
pub fn isize::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for u128
pub fn u128::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for u16
pub fn u16::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for u32
pub fn u32::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for u64
pub fn u64::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for u8
pub fn u8::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U2> for usize
pub fn usize::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U4> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<u128> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: u128) -> Self
impl core::convert::From<u16> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: u16) -> Self
impl core::convert::From<u32> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: u32) -> Self
impl core::convert::From<u64> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: u64) -> Self
impl core::convert::From<u8> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: u8) -> Self
impl core::convert::From<usize> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: usize) -> Self
impl core::fmt::Debug for mirl::extensions::U2
pub fn mirl::extensions::U2::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::extensions::U2
impl core::marker::StructuralPartialEq for mirl::extensions::U2
impl core::ops::arith::Add for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::add(self, rhs: Self) -> Self
impl core::ops::arith::Div for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::div(self, rhs: Self) -> Self
impl core::ops::arith::Mul for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::mul(self, rhs: Self) -> Self
impl core::ops::arith::Rem for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::rem(self, rhs: Self) -> Self::Output
impl core::ops::arith::Sub for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::sub(self, rhs: Self) -> Self
impl core::ops::bit::BitAnd for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::bitand(self, rhs: Self) -> Self
impl core::ops::bit::BitOr for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::bitor(self, rhs: Self) -> Self
impl core::ops::bit::BitXor for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::bitxor(self, rhs: Self) -> Self
impl core::ops::bit::Not for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::not(self) -> Self
impl core::ops::bit::Shl<usize> for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::shl(self, rhs: usize) -> Self
impl core::ops::bit::Shr<usize> for mirl::extensions::U2
pub type mirl::extensions::U2::Output = mirl::extensions::U2
pub fn mirl::extensions::U2::shr(self, rhs: usize) -> Self
impl num_traits::Num for mirl::extensions::U2
pub type mirl::extensions::U2::FromStrRadixErr = core::num::error::ParseIntError
pub fn mirl::extensions::U2::from_str_radix(str: &str, radix: u32) -> core::result::Result<Self, Self::FromStrRadixErr>
impl num_traits::cast::NumCast for mirl::extensions::U2
pub fn mirl::extensions::U2::from<T: num_traits::cast::ToPrimitive>(n: T) -> core::option::Option<Self>
impl num_traits::cast::ToPrimitive for mirl::extensions::U2
pub fn mirl::extensions::U2::to_f32(&self) -> core::option::Option<f32>
pub fn mirl::extensions::U2::to_f64(&self) -> core::option::Option<f64>
pub fn mirl::extensions::U2::to_i128(&self) -> core::option::Option<i128>
pub fn mirl::extensions::U2::to_i16(&self) -> core::option::Option<i16>
pub fn mirl::extensions::U2::to_i32(&self) -> core::option::Option<i32>
pub fn mirl::extensions::U2::to_i64(&self) -> core::option::Option<i64>
pub fn mirl::extensions::U2::to_i8(&self) -> core::option::Option<i8>
pub fn mirl::extensions::U2::to_isize(&self) -> core::option::Option<isize>
pub fn mirl::extensions::U2::to_u128(&self) -> core::option::Option<u128>
pub fn mirl::extensions::U2::to_u16(&self) -> core::option::Option<u16>
pub fn mirl::extensions::U2::to_u32(&self) -> core::option::Option<u32>
pub fn mirl::extensions::U2::to_u64(&self) -> core::option::Option<u64>
pub fn mirl::extensions::U2::to_u8(&self) -> core::option::Option<u8>
impl num_traits::identities::One for mirl::extensions::U2
pub fn mirl::extensions::U2::is_one(&self) -> bool where Self: core::cmp::PartialEq
pub fn mirl::extensions::U2::one() -> Self
impl num_traits::identities::Zero for mirl::extensions::U2
pub fn mirl::extensions::U2::is_zero(&self) -> bool
pub fn mirl::extensions::U2::zero() -> Self
impl core::marker::Freeze for mirl::extensions::U2
impl core::marker::Send for mirl::extensions::U2
impl core::marker::Sync for mirl::extensions::U2
impl core::marker::Unpin for mirl::extensions::U2
impl core::panic::unwind_safe::RefUnwindSafe for mirl::extensions::U2
impl core::panic::unwind_safe::UnwindSafe for mirl::extensions::U2
impl<Q, K> equivalent::Equivalent<K> for mirl::extensions::U2 where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::extensions::U2::equivalent(&self, key: &K) -> bool
impl<T, Rhs, Output> num_traits::NumOps<Rhs, Output> for mirl::extensions::U2 where T: core::ops::arith::Sub<Rhs, Output = Output> + core::ops::arith::Mul<Rhs, Output = Output> + core::ops::arith::Div<Rhs, Output = Output> + core::ops::arith::Add<Rhs, Output = Output> + core::ops::arith::Rem<Rhs, Output = Output>
impl<T, U> core::convert::Into<U> for mirl::extensions::U2 where U: core::convert::From<T>
pub fn mirl::extensions::U2::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::extensions::U2 where U: core::convert::Into<T>
pub type mirl::extensions::U2::Error = core::convert::Infallible
pub fn mirl::extensions::U2::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::extensions::U2 where U: core::convert::TryFrom<T>
pub type mirl::extensions::U2::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::extensions::U2::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::extensions::U2 where T: core::clone::Clone
pub type mirl::extensions::U2::Owned = T
pub fn mirl::extensions::U2::clone_into(&self, target: &mut T)
pub fn mirl::extensions::U2::to_owned(&self) -> T
impl<T> core::any::Any for mirl::extensions::U2 where T: 'static + ?core::marker::Sized
pub fn mirl::extensions::U2::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::extensions::U2 where T: ?core::marker::Sized
pub fn mirl::extensions::U2::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::extensions::U2 where T: ?core::marker::Sized
pub fn mirl::extensions::U2::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::extensions::U2 where T: core::clone::Clone
pub unsafe fn mirl::extensions::U2::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::extensions::U2
pub type mirl::extensions::U2::Init = T
pub const mirl::extensions::U2::ALIGN: usize
pub unsafe fn mirl::extensions::U2::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::extensions::U2::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::extensions::U2::drop(ptr: usize)
pub unsafe fn mirl::extensions::U2::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::extensions::U2
impl<T> mirl::extensions::RepeatData for mirl::extensions::U2 where T: core::clone::Clone
pub fn mirl::extensions::U2::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
impl<T> mirl::math::NumberWithMonotoneOps for mirl::extensions::U2 where T: core::ops::arith::Sub<Output = T> + core::cmp::PartialOrd + core::ops::arith::Mul<Output = T> + num_traits::cast::NumCast + core::ops::arith::Div<Output = T> + core::ops::arith::Add<Output = T>
impl<T> mirl::math::TwoTillTen<T> for mirl::extensions::U2 where T: num_traits::identities::One + core::ops::arith::Add<Output = T>
pub struct mirl::extensions::U4
pub mirl::extensions::U4::b0: bool
pub mirl::extensions::U4::b1: bool
pub mirl::extensions::U4::b2: bool
pub mirl::extensions::U4::b3: bool
impl mirl::extensions::U4
pub const fn mirl::extensions::U4::from_u1_quad(b3: mirl::extensions::U1, b2: mirl::extensions::U1, b1: mirl::extensions::U1, b0: mirl::extensions::U1) -> Self
pub const fn mirl::extensions::U4::split_to_u1_quad(self) -> (mirl::extensions::U1, mirl::extensions::U1, mirl::extensions::U1, mirl::extensions::U1)
pub fn mirl::extensions::U4::to_u1(self) -> mirl::extensions::U1
impl mirl::extensions::U4
pub const fn mirl::extensions::U4::from_u2_pair(high: mirl::extensions::U2, low: mirl::extensions::U2) -> Self
pub const fn mirl::extensions::U4::from_u8_trunc(val: u8) -> Self
pub const fn mirl::extensions::U4::is_max(self) -> bool
pub const fn mirl::extensions::U4::is_zero(self) -> bool
pub fn mirl::extensions::U4::new(val: u8) -> Self
pub const fn mirl::extensions::U4::split_to_u2_pair(self) -> (mirl::extensions::U2, mirl::extensions::U2)
pub fn mirl::extensions::U4::to_u2(self) -> mirl::extensions::U2
pub const fn mirl::extensions::U4::value(self) -> u8
pub const fn mirl::extensions::U4::wrapping_add(self, other: Self) -> Self
pub const fn mirl::extensions::U4::wrapping_sub(self, other: Self) -> Self
impl core::clone::Clone for mirl::extensions::U4
pub fn mirl::extensions::U4::clone(&self) -> mirl::extensions::U4
impl core::cmp::Eq for mirl::extensions::U4
impl core::cmp::PartialEq for mirl::extensions::U4
pub fn mirl::extensions::U4::eq(&self, other: &mirl::extensions::U4) -> bool
impl core::cmp::PartialOrd for mirl::extensions::U4
pub fn mirl::extensions::U4::partial_cmp(&self, other: &mirl::extensions::U4) -> core::option::Option<core::cmp::Ordering>
impl core::convert::From<f32> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: f32) -> Self
impl core::convert::From<f64> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: f64) -> Self
impl core::convert::From<i128> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: i128) -> Self
impl core::convert::From<i16> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: i16) -> Self
impl core::convert::From<i32> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: i32) -> Self
impl core::convert::From<i64> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: i64) -> Self
impl core::convert::From<i8> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: i8) -> Self
impl core::convert::From<isize> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: isize) -> Self
impl core::convert::From<mirl::extensions::U1> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: mirl::extensions::U1) -> Self
impl core::convert::From<mirl::extensions::U2> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: mirl::extensions::U2) -> Self
impl core::convert::From<mirl::extensions::U4> for f32
pub fn f32::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for f64
pub fn f64::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for i128
pub fn i128::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for i16
pub fn i16::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for i32
pub fn i32::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for i64
pub fn i64::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for i8
pub fn i8::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for isize
pub fn isize::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for mirl::extensions::U1
pub fn mirl::extensions::U1::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for mirl::extensions::U2
pub fn mirl::extensions::U2::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for u128
pub fn u128::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for u16
pub fn u16::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for u32
pub fn u32::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for u64
pub fn u64::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for u8
pub fn u8::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<mirl::extensions::U4> for usize
pub fn usize::from(val: mirl::extensions::U4) -> Self
impl core::convert::From<u128> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: u128) -> Self
impl core::convert::From<u16> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: u16) -> Self
impl core::convert::From<u32> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: u32) -> Self
impl core::convert::From<u64> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: u64) -> Self
impl core::convert::From<u8> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: u8) -> Self
impl core::convert::From<usize> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(val: usize) -> Self
impl core::fmt::Debug for mirl::extensions::U4
pub fn mirl::extensions::U4::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::extensions::U4
impl core::marker::StructuralPartialEq for mirl::extensions::U4
impl core::ops::arith::Add for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::add(self, rhs: Self) -> Self
impl core::ops::arith::Div for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::div(self, rhs: Self) -> Self
impl core::ops::arith::Mul for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::mul(self, rhs: Self) -> Self
impl core::ops::arith::Rem for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::rem(self, rhs: Self) -> Self::Output
impl core::ops::arith::Sub for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::sub(self, rhs: Self) -> Self
impl core::ops::bit::BitAnd for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::bitand(self, rhs: Self) -> Self
impl core::ops::bit::BitOr for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::bitor(self, rhs: Self) -> Self
impl core::ops::bit::BitXor for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::bitxor(self, rhs: Self) -> Self
impl core::ops::bit::Not for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::not(self) -> Self
impl core::ops::bit::Shl<usize> for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::shl(self, rhs: usize) -> Self
impl core::ops::bit::Shr<usize> for mirl::extensions::U4
pub type mirl::extensions::U4::Output = mirl::extensions::U4
pub fn mirl::extensions::U4::shr(self, rhs: usize) -> Self
impl num_traits::Num for mirl::extensions::U4
pub type mirl::extensions::U4::FromStrRadixErr = core::num::error::ParseIntError
pub fn mirl::extensions::U4::from_str_radix(str: &str, radix: u32) -> core::result::Result<Self, Self::FromStrRadixErr>
impl num_traits::cast::NumCast for mirl::extensions::U4
pub fn mirl::extensions::U4::from<T: num_traits::cast::ToPrimitive>(n: T) -> core::option::Option<Self>
impl num_traits::cast::ToPrimitive for mirl::extensions::U4
pub fn mirl::extensions::U4::to_f32(&self) -> core::option::Option<f32>
pub fn mirl::extensions::U4::to_f64(&self) -> core::option::Option<f64>
pub fn mirl::extensions::U4::to_i128(&self) -> core::option::Option<i128>
pub fn mirl::extensions::U4::to_i16(&self) -> core::option::Option<i16>
pub fn mirl::extensions::U4::to_i32(&self) -> core::option::Option<i32>
pub fn mirl::extensions::U4::to_i64(&self) -> core::option::Option<i64>
pub fn mirl::extensions::U4::to_i8(&self) -> core::option::Option<i8>
pub fn mirl::extensions::U4::to_isize(&self) -> core::option::Option<isize>
pub fn mirl::extensions::U4::to_u128(&self) -> core::option::Option<u128>
pub fn mirl::extensions::U4::to_u16(&self) -> core::option::Option<u16>
pub fn mirl::extensions::U4::to_u32(&self) -> core::option::Option<u32>
pub fn mirl::extensions::U4::to_u64(&self) -> core::option::Option<u64>
pub fn mirl::extensions::U4::to_u8(&self) -> core::option::Option<u8>
impl num_traits::identities::One for mirl::extensions::U4
pub fn mirl::extensions::U4::is_one(&self) -> bool where Self: core::cmp::PartialEq
pub fn mirl::extensions::U4::one() -> Self
impl num_traits::identities::Zero for mirl::extensions::U4
pub fn mirl::extensions::U4::is_zero(&self) -> bool
pub fn mirl::extensions::U4::zero() -> Self
impl core::marker::Freeze for mirl::extensions::U4
impl core::marker::Send for mirl::extensions::U4
impl core::marker::Sync for mirl::extensions::U4
impl core::marker::Unpin for mirl::extensions::U4
impl core::panic::unwind_safe::RefUnwindSafe for mirl::extensions::U4
impl core::panic::unwind_safe::UnwindSafe for mirl::extensions::U4
impl<Q, K> equivalent::Equivalent<K> for mirl::extensions::U4 where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::extensions::U4::equivalent(&self, key: &K) -> bool
impl<T, Rhs, Output> num_traits::NumOps<Rhs, Output> for mirl::extensions::U4 where T: core::ops::arith::Sub<Rhs, Output = Output> + core::ops::arith::Mul<Rhs, Output = Output> + core::ops::arith::Div<Rhs, Output = Output> + core::ops::arith::Add<Rhs, Output = Output> + core::ops::arith::Rem<Rhs, Output = Output>
impl<T, U> core::convert::Into<U> for mirl::extensions::U4 where U: core::convert::From<T>
pub fn mirl::extensions::U4::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::extensions::U4 where U: core::convert::Into<T>
pub type mirl::extensions::U4::Error = core::convert::Infallible
pub fn mirl::extensions::U4::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::extensions::U4 where U: core::convert::TryFrom<T>
pub type mirl::extensions::U4::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::extensions::U4::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::extensions::U4 where T: core::clone::Clone
pub type mirl::extensions::U4::Owned = T
pub fn mirl::extensions::U4::clone_into(&self, target: &mut T)
pub fn mirl::extensions::U4::to_owned(&self) -> T
impl<T> core::any::Any for mirl::extensions::U4 where T: 'static + ?core::marker::Sized
pub fn mirl::extensions::U4::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::extensions::U4 where T: ?core::marker::Sized
pub fn mirl::extensions::U4::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::extensions::U4 where T: ?core::marker::Sized
pub fn mirl::extensions::U4::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::extensions::U4 where T: core::clone::Clone
pub unsafe fn mirl::extensions::U4::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::extensions::U4
pub fn mirl::extensions::U4::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::extensions::U4
pub type mirl::extensions::U4::Init = T
pub const mirl::extensions::U4::ALIGN: usize
pub unsafe fn mirl::extensions::U4::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::extensions::U4::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::extensions::U4::drop(ptr: usize)
pub unsafe fn mirl::extensions::U4::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::extensions::U4
impl<T> mirl::extensions::RepeatData for mirl::extensions::U4 where T: core::clone::Clone
pub fn mirl::extensions::U4::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
impl<T> mirl::math::NumberWithMonotoneOps for mirl::extensions::U4 where T: core::ops::arith::Sub<Output = T> + core::cmp::PartialOrd + core::ops::arith::Mul<Output = T> + num_traits::cast::NumCast + core::ops::arith::Div<Output = T> + core::ops::arith::Add<Output = T>
impl<T> mirl::math::TwoTillTen<T> for mirl::extensions::U4 where T: num_traits::identities::One + core::ops::arith::Add<Output = T>
pub trait mirl::extensions::AddSign<T>
pub fn mirl::extensions::AddSign::add_sign(&self, value: T) -> Self
pub fn mirl::extensions::AddSign::saturated_add_sign(&self, value: T) -> Self
pub fn mirl::extensions::AddSign::set_add_sign(&mut self, value: T)
pub fn mirl::extensions::AddSign::set_saturated_add_sign(&mut self, value: T)
impl<U, S> mirl::extensions::AddSign<S> for U where U: num_traits::sign::Unsigned + num_traits::bounds::Bounded + num_traits::identities::Zero + core::marker::Copy + num_traits::ops::wrapping::WrappingAdd + num_traits::ops::wrapping::WrappingSub + num_traits::ops::saturating::SaturatingAdd + num_traits::ops::saturating::SaturatingSub + core::ops::arith::Add<U, Output = U> + core::ops::arith::Sub<U, Output = U> + num_traits::cast::NumCast, S: num_traits::sign::Signed + core::marker::Copy + core::cmp::PartialOrd + num_traits::cast::NumCast
pub fn U::add_sign(&self, value: S) -> Self
pub fn U::saturated_add_sign(&self, value: S) -> Self
pub fn U::set_add_sign(&mut self, value: S)
pub fn U::set_saturated_add_sign(&mut self, value: S)
pub trait mirl::extensions::Average<T>
pub fn mirl::extensions::Average::average(&self) -> core::option::Option<T>
impl<V: num_traits::Num + num_traits::cast::NumCast + core::marker::Copy> mirl::extensions::Average<V> for alloc::vec::Vec<V>
pub fn alloc::vec::Vec<V>::average(&self) -> core::option::Option<V>
pub trait mirl::extensions::MapToSign
pub type mirl::extensions::MapToSign::Signed
pub type mirl::extensions::MapToSign::Unsigned
pub fn mirl::extensions::MapToSign::map_non_sign_to_sign(&self) -> Self::Signed
impl mirl::extensions::MapToSign for u128
pub type u128::Signed = i128
pub type u128::Unsigned = u128
pub fn u128::map_non_sign_to_sign(&self) -> Self::Signed
impl mirl::extensions::MapToSign for u16
pub type u16::Signed = i16
pub type u16::Unsigned = u16
pub fn u16::map_non_sign_to_sign(&self) -> Self::Signed
impl mirl::extensions::MapToSign for u32
pub type u32::Signed = i32
pub type u32::Unsigned = u32
pub fn u32::map_non_sign_to_sign(&self) -> Self::Signed
impl mirl::extensions::MapToSign for u64
pub type u64::Signed = i64
pub type u64::Unsigned = u64
pub fn u64::map_non_sign_to_sign(&self) -> Self::Signed
impl mirl::extensions::MapToSign for u8
pub type u8::Signed = i8
pub type u8::Unsigned = u8
pub fn u8::map_non_sign_to_sign(&self) -> Self::Signed
pub trait mirl::extensions::MapToUnSign
pub type mirl::extensions::MapToUnSign::Signed
pub type mirl::extensions::MapToUnSign::Unsigned
pub fn mirl::extensions::MapToUnSign::map_sign_to_non_sign(&self) -> Self::Unsigned
impl mirl::extensions::MapToUnSign for i128
pub type i128::Signed = i128
pub type i128::Unsigned = u128
pub fn i128::map_sign_to_non_sign(&self) -> Self::Unsigned
impl mirl::extensions::MapToUnSign for i16
pub type i16::Signed = i16
pub type i16::Unsigned = u16
pub fn i16::map_sign_to_non_sign(&self) -> Self::Unsigned
impl mirl::extensions::MapToUnSign for i32
pub type i32::Signed = i32
pub type i32::Unsigned = u32
pub fn i32::map_sign_to_non_sign(&self) -> Self::Unsigned
impl mirl::extensions::MapToUnSign for i64
pub type i64::Signed = i64
pub type i64::Unsigned = u64
pub fn i64::map_sign_to_non_sign(&self) -> Self::Unsigned
impl mirl::extensions::MapToUnSign for i8
pub type i8::Signed = i8
pub type i8::Unsigned = u8
pub fn i8::map_sign_to_non_sign(&self) -> Self::Unsigned
pub trait mirl::extensions::RemoveChar
pub fn mirl::extensions::RemoveChar::pop_char_at(&mut self, pos: usize) -> core::option::Option<char>
pub fn mirl::extensions::RemoveChar::remove_char_at(&mut self, pos: usize)
impl mirl::extensions::RemoveChar for alloc::string::String
pub fn alloc::string::String::pop_char_at(&mut self, pos: usize) -> core::option::Option<char>
pub fn alloc::string::String::remove_char_at(&mut self, pos: usize)
pub trait mirl::extensions::RepeatData where Self: core::marker::Sized + core::clone::Clone
pub fn mirl::extensions::RepeatData::repeat_value(self, times: usize) -> alloc::vec::Vec<Self>
impl<T: core::marker::Sized + core::clone::Clone> mirl::extensions::RepeatData for T
pub fn T::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub trait mirl::extensions::Sign
pub fn mirl::extensions::Sign::sign(self) -> Self
impl mirl::extensions::Sign for f32
pub fn f32::sign(self) -> Self
impl mirl::extensions::Sign for f64
pub fn f64::sign(self) -> Self
impl mirl::extensions::Sign for i128
pub fn i128::sign(self) -> Self
impl mirl::extensions::Sign for i16
pub fn i16::sign(self) -> Self
impl mirl::extensions::Sign for i32
pub fn i32::sign(self) -> Self
impl mirl::extensions::Sign for i64
pub fn i64::sign(self) -> Self
impl mirl::extensions::Sign for i8
pub fn i8::sign(self) -> Self
impl mirl::extensions::Sign for isize
pub fn isize::sign(self) -> Self
pub trait mirl::extensions::Sqrt
pub fn mirl::extensions::Sqrt::sqrt(self) -> Self
impl mirl::extensions::Sqrt for i128
pub fn i128::sqrt(self) -> Self
impl mirl::extensions::Sqrt for i16
pub fn i16::sqrt(self) -> Self
impl mirl::extensions::Sqrt for i32
pub fn i32::sqrt(self) -> Self
impl mirl::extensions::Sqrt for i64
pub fn i64::sqrt(self) -> Self
impl mirl::extensions::Sqrt for i8
pub fn i8::sqrt(self) -> Self
impl mirl::extensions::Sqrt for isize
pub fn isize::sqrt(self) -> Self
pub trait mirl::extensions::StringExtensions
pub fn mirl::extensions::StringExtensions::center(&self, length: usize, fillchar: core::option::Option<char>) -> alloc::string::String
pub fn mirl::extensions::StringExtensions::expand_tabs(&self) -> alloc::string::String
pub fn mirl::extensions::StringExtensions::is_number(&self) -> bool
pub fn mirl::extensions::StringExtensions::ljust(&self, length: usize, fillchar: core::option::Option<char>) -> alloc::string::String
pub fn mirl::extensions::StringExtensions::replace_first_occurrence(&self, target: &str, replacement: &str) -> alloc::string::String
pub fn mirl::extensions::StringExtensions::rjust(&self, length: usize, fillchar: core::option::Option<char>) -> alloc::string::String
impl mirl::extensions::StringExtensions for str
pub fn str::center(&self, length: usize, fillchar: core::option::Option<char>) -> alloc::string::String
pub fn str::expand_tabs(&self) -> alloc::string::String
pub fn str::is_number(&self) -> bool
pub fn str::ljust(&self, length: usize, fillchar: core::option::Option<char>) -> alloc::string::String
pub fn str::replace_first_occurrence(&self, target: &str, replacement: &str) -> alloc::string::String
pub fn str::rjust(&self, length: usize, fillchar: core::option::Option<char>) -> alloc::string::String
pub trait mirl::extensions::Tuple10Into
pub fn mirl::extensions::Tuple10Into::tuple_10_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10> mirl::extensions::Tuple10Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10)::tuple_10_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
pub trait mirl::extensions::Tuple11Into
pub fn mirl::extensions::Tuple11Into::tuple_11_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11> mirl::extensions::Tuple11Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11)::tuple_11_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
pub trait mirl::extensions::Tuple12Into
pub fn mirl::extensions::Tuple12Into::tuple_12_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12> mirl::extensions::Tuple12Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12)::tuple_12_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
pub trait mirl::extensions::Tuple13Into
pub fn mirl::extensions::Tuple13Into::tuple_13_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13> mirl::extensions::Tuple13Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13)::tuple_13_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
pub trait mirl::extensions::Tuple14Into
pub fn mirl::extensions::Tuple14Into::tuple_14_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14> mirl::extensions::Tuple14Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14)::tuple_14_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
pub trait mirl::extensions::Tuple15Into
pub fn mirl::extensions::Tuple15Into::tuple_15_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15> mirl::extensions::Tuple15Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15)::tuple_15_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
pub trait mirl::extensions::Tuple16Into
pub fn mirl::extensions::Tuple16Into::tuple_16_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16> mirl::extensions::Tuple16Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16)::tuple_16_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
pub trait mirl::extensions::Tuple17Into
pub fn mirl::extensions::Tuple17Into::tuple_17_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17> mirl::extensions::Tuple17Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17)::tuple_17_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
pub trait mirl::extensions::Tuple18Into
pub fn mirl::extensions::Tuple18Into::tuple_18_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18> mirl::extensions::Tuple18Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18)::tuple_18_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
pub trait mirl::extensions::Tuple19Into
pub fn mirl::extensions::Tuple19Into::tuple_19_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19> mirl::extensions::Tuple19Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19)::tuple_19_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
pub trait mirl::extensions::Tuple1Into
pub fn mirl::extensions::Tuple1Into::tuple_1_into<T: num_traits::Num + num_traits::cast::NumCast>(self) -> (T)
impl<N1> mirl::extensions::Tuple1Into for (N1) where N1: num_traits::Num + num_traits::cast::NumCast
pub fn (N1)::tuple_1_into<T: num_traits::Num + num_traits::cast::NumCast>(self) -> (T)
pub trait mirl::extensions::Tuple20Into
pub fn mirl::extensions::Tuple20Into::tuple_20_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20> mirl::extensions::Tuple20Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20)::tuple_20_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
pub trait mirl::extensions::Tuple21Into
pub fn mirl::extensions::Tuple21Into::tuple_21_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21> mirl::extensions::Tuple21Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21)::tuple_21_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
pub trait mirl::extensions::Tuple22Into
pub fn mirl::extensions::Tuple22Into::tuple_22_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22> mirl::extensions::Tuple22Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22)::tuple_22_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
pub trait mirl::extensions::Tuple23Into
pub fn mirl::extensions::Tuple23Into::tuple_23_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23> mirl::extensions::Tuple23Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23)::tuple_23_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
pub trait mirl::extensions::Tuple24Into
pub fn mirl::extensions::Tuple24Into::tuple_24_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24> mirl::extensions::Tuple24Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast, N24: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24)::tuple_24_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
pub trait mirl::extensions::Tuple25Into
pub fn mirl::extensions::Tuple25Into::tuple_25_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25> mirl::extensions::Tuple25Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast, N24: num_traits::Num + num_traits::cast::NumCast, N25: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25)::tuple_25_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
pub trait mirl::extensions::Tuple26Into
pub fn mirl::extensions::Tuple26Into::tuple_26_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26> mirl::extensions::Tuple26Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast, N24: num_traits::Num + num_traits::cast::NumCast, N25: num_traits::Num + num_traits::cast::NumCast, N26: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26)::tuple_26_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
pub trait mirl::extensions::Tuple27Into
pub fn mirl::extensions::Tuple27Into::tuple_27_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27> mirl::extensions::Tuple27Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast, N24: num_traits::Num + num_traits::cast::NumCast, N25: num_traits::Num + num_traits::cast::NumCast, N26: num_traits::Num + num_traits::cast::NumCast, N27: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27)::tuple_27_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
pub trait mirl::extensions::Tuple28Into
pub fn mirl::extensions::Tuple28Into::tuple_28_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28> mirl::extensions::Tuple28Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast, N24: num_traits::Num + num_traits::cast::NumCast, N25: num_traits::Num + num_traits::cast::NumCast, N26: num_traits::Num + num_traits::cast::NumCast, N27: num_traits::Num + num_traits::cast::NumCast, N28: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28)::tuple_28_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
pub trait mirl::extensions::Tuple29Into
pub fn mirl::extensions::Tuple29Into::tuple_29_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast, T28: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29> mirl::extensions::Tuple29Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast, N24: num_traits::Num + num_traits::cast::NumCast, N25: num_traits::Num + num_traits::cast::NumCast, N26: num_traits::Num + num_traits::cast::NumCast, N27: num_traits::Num + num_traits::cast::NumCast, N28: num_traits::Num + num_traits::cast::NumCast, N29: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29)::tuple_29_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast, T28: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
pub trait mirl::extensions::Tuple2Into
pub fn mirl::extensions::Tuple2Into::tuple_2_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1)
impl<N1, N2> mirl::extensions::Tuple2Into for (N1, N2) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2)::tuple_2_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1)
pub trait mirl::extensions::Tuple30Into
pub fn mirl::extensions::Tuple30Into::tuple_30_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast, T28: num_traits::Num + num_traits::cast::NumCast, T29: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30> mirl::extensions::Tuple30Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast, N24: num_traits::Num + num_traits::cast::NumCast, N25: num_traits::Num + num_traits::cast::NumCast, N26: num_traits::Num + num_traits::cast::NumCast, N27: num_traits::Num + num_traits::cast::NumCast, N28: num_traits::Num + num_traits::cast::NumCast, N29: num_traits::Num + num_traits::cast::NumCast, N30: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30)::tuple_30_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast, T28: num_traits::Num + num_traits::cast::NumCast, T29: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
pub trait mirl::extensions::Tuple31Into
pub fn mirl::extensions::Tuple31Into::tuple_31_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast, T28: num_traits::Num + num_traits::cast::NumCast, T29: num_traits::Num + num_traits::cast::NumCast, T30: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30, N31> mirl::extensions::Tuple31Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30, N31) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast, N24: num_traits::Num + num_traits::cast::NumCast, N25: num_traits::Num + num_traits::cast::NumCast, N26: num_traits::Num + num_traits::cast::NumCast, N27: num_traits::Num + num_traits::cast::NumCast, N28: num_traits::Num + num_traits::cast::NumCast, N29: num_traits::Num + num_traits::cast::NumCast, N30: num_traits::Num + num_traits::cast::NumCast, N31: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30, N31)::tuple_31_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast, T28: num_traits::Num + num_traits::cast::NumCast, T29: num_traits::Num + num_traits::cast::NumCast, T30: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
pub trait mirl::extensions::Tuple32Into
pub fn mirl::extensions::Tuple32Into::tuple_32_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast, T28: num_traits::Num + num_traits::cast::NumCast, T29: num_traits::Num + num_traits::cast::NumCast, T30: num_traits::Num + num_traits::cast::NumCast, T31: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30, N31, N32> mirl::extensions::Tuple32Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30, N31, N32) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast, N24: num_traits::Num + num_traits::cast::NumCast, N25: num_traits::Num + num_traits::cast::NumCast, N26: num_traits::Num + num_traits::cast::NumCast, N27: num_traits::Num + num_traits::cast::NumCast, N28: num_traits::Num + num_traits::cast::NumCast, N29: num_traits::Num + num_traits::cast::NumCast, N30: num_traits::Num + num_traits::cast::NumCast, N31: num_traits::Num + num_traits::cast::NumCast, N32: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30, N31, N32)::tuple_32_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast, T28: num_traits::Num + num_traits::cast::NumCast, T29: num_traits::Num + num_traits::cast::NumCast, T30: num_traits::Num + num_traits::cast::NumCast, T31: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
pub trait mirl::extensions::Tuple33Into
pub fn mirl::extensions::Tuple33Into::tuple_33_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast, T28: num_traits::Num + num_traits::cast::NumCast, T29: num_traits::Num + num_traits::cast::NumCast, T30: num_traits::Num + num_traits::cast::NumCast, T31: num_traits::Num + num_traits::cast::NumCast, T32: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30, N31, N32, N33> mirl::extensions::Tuple33Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30, N31, N32, N33) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast, N10: num_traits::Num + num_traits::cast::NumCast, N11: num_traits::Num + num_traits::cast::NumCast, N12: num_traits::Num + num_traits::cast::NumCast, N13: num_traits::Num + num_traits::cast::NumCast, N14: num_traits::Num + num_traits::cast::NumCast, N15: num_traits::Num + num_traits::cast::NumCast, N16: num_traits::Num + num_traits::cast::NumCast, N17: num_traits::Num + num_traits::cast::NumCast, N18: num_traits::Num + num_traits::cast::NumCast, N19: num_traits::Num + num_traits::cast::NumCast, N20: num_traits::Num + num_traits::cast::NumCast, N21: num_traits::Num + num_traits::cast::NumCast, N22: num_traits::Num + num_traits::cast::NumCast, N23: num_traits::Num + num_traits::cast::NumCast, N24: num_traits::Num + num_traits::cast::NumCast, N25: num_traits::Num + num_traits::cast::NumCast, N26: num_traits::Num + num_traits::cast::NumCast, N27: num_traits::Num + num_traits::cast::NumCast, N28: num_traits::Num + num_traits::cast::NumCast, N29: num_traits::Num + num_traits::cast::NumCast, N30: num_traits::Num + num_traits::cast::NumCast, N31: num_traits::Num + num_traits::cast::NumCast, N32: num_traits::Num + num_traits::cast::NumCast, N33: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9, N10, N11, N12, N13, N14, N15, N16, N17, N18, N19, N20, N21, N22, N23, N24, N25, N26, N27, N28, N29, N30, N31, N32, N33)::tuple_33_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast, T9: num_traits::Num + num_traits::cast::NumCast, T10: num_traits::Num + num_traits::cast::NumCast, T11: num_traits::Num + num_traits::cast::NumCast, T12: num_traits::Num + num_traits::cast::NumCast, T13: num_traits::Num + num_traits::cast::NumCast, T14: num_traits::Num + num_traits::cast::NumCast, T15: num_traits::Num + num_traits::cast::NumCast, T16: num_traits::Num + num_traits::cast::NumCast, T17: num_traits::Num + num_traits::cast::NumCast, T18: num_traits::Num + num_traits::cast::NumCast, T19: num_traits::Num + num_traits::cast::NumCast, T20: num_traits::Num + num_traits::cast::NumCast, T21: num_traits::Num + num_traits::cast::NumCast, T22: num_traits::Num + num_traits::cast::NumCast, T23: num_traits::Num + num_traits::cast::NumCast, T24: num_traits::Num + num_traits::cast::NumCast, T25: num_traits::Num + num_traits::cast::NumCast, T26: num_traits::Num + num_traits::cast::NumCast, T27: num_traits::Num + num_traits::cast::NumCast, T28: num_traits::Num + num_traits::cast::NumCast, T29: num_traits::Num + num_traits::cast::NumCast, T30: num_traits::Num + num_traits::cast::NumCast, T31: num_traits::Num + num_traits::cast::NumCast, T32: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32)
pub trait mirl::extensions::Tuple3Into
pub fn mirl::extensions::Tuple3Into::tuple_3_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2)
impl<N1, N2, N3> mirl::extensions::Tuple3Into for (N1, N2, N3) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3)::tuple_3_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2)
pub trait mirl::extensions::Tuple4Into
pub fn mirl::extensions::Tuple4Into::tuple_4_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3)
impl<N1, N2, N3, N4> mirl::extensions::Tuple4Into for (N1, N2, N3, N4) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4)::tuple_4_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3)
pub trait mirl::extensions::Tuple5Into
pub fn mirl::extensions::Tuple5Into::tuple_5_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4)
impl<N1, N2, N3, N4, N5> mirl::extensions::Tuple5Into for (N1, N2, N3, N4, N5) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5)::tuple_5_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4)
pub trait mirl::extensions::Tuple6Into
pub fn mirl::extensions::Tuple6Into::tuple_6_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5)
impl<N1, N2, N3, N4, N5, N6> mirl::extensions::Tuple6Into for (N1, N2, N3, N4, N5, N6) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6)::tuple_6_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5)
pub trait mirl::extensions::Tuple7Into
pub fn mirl::extensions::Tuple7Into::tuple_7_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6)
impl<N1, N2, N3, N4, N5, N6, N7> mirl::extensions::Tuple7Into for (N1, N2, N3, N4, N5, N6, N7) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7)::tuple_7_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6)
pub trait mirl::extensions::Tuple8Into
pub fn mirl::extensions::Tuple8Into::tuple_8_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7)
impl<N1, N2, N3, N4, N5, N6, N7, N8> mirl::extensions::Tuple8Into for (N1, N2, N3, N4, N5, N6, N7, N8) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8)::tuple_8_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7)
pub trait mirl::extensions::Tuple9Into
pub fn mirl::extensions::Tuple9Into::tuple_9_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8)
impl<N1, N2, N3, N4, N5, N6, N7, N8, N9> mirl::extensions::Tuple9Into for (N1, N2, N3, N4, N5, N6, N7, N8, N9) where N1: num_traits::Num + num_traits::cast::NumCast, N2: num_traits::Num + num_traits::cast::NumCast, N3: num_traits::Num + num_traits::cast::NumCast, N4: num_traits::Num + num_traits::cast::NumCast, N5: num_traits::Num + num_traits::cast::NumCast, N6: num_traits::Num + num_traits::cast::NumCast, N7: num_traits::Num + num_traits::cast::NumCast, N8: num_traits::Num + num_traits::cast::NumCast, N9: num_traits::Num + num_traits::cast::NumCast
pub fn (N1, N2, N3, N4, N5, N6, N7, N8, N9)::tuple_9_into<T0: num_traits::Num + num_traits::cast::NumCast, T1: num_traits::Num + num_traits::cast::NumCast, T2: num_traits::Num + num_traits::cast::NumCast, T3: num_traits::Num + num_traits::cast::NumCast, T4: num_traits::Num + num_traits::cast::NumCast, T5: num_traits::Num + num_traits::cast::NumCast, T6: num_traits::Num + num_traits::cast::NumCast, T7: num_traits::Num + num_traits::cast::NumCast, T8: num_traits::Num + num_traits::cast::NumCast>(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8)
pub trait mirl::extensions::TupleCmp<Rhs>
pub fn mirl::extensions::TupleCmp::cmp(self, rhs: Rhs) -> core::cmp::Ordering
pub fn mirl::extensions::TupleCmp::ge(self, rhs: Rhs) -> bool
pub fn mirl::extensions::TupleCmp::gt(self, rhs: Rhs) -> bool
pub fn mirl::extensions::TupleCmp::le(self, rhs: Rhs) -> bool
pub fn mirl::extensions::TupleCmp::lt(self, rhs: Rhs) -> bool
impl<T> mirl::extensions::TupleCmp for (T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T)::ge(self, rhs: Self) -> bool
pub fn (T)::gt(self, rhs: Self) -> bool
pub fn (T)::le(self, rhs: Self) -> bool
pub fn (T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T)::le(self, rhs: Self) -> bool
pub fn (T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
impl<T> mirl::extensions::TupleCmp for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::cmp::PartialOrd + core::marker::Copy
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::cmp(self, rhs: Self) -> core::cmp::Ordering
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::ge(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::gt(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::le(self, rhs: Self) -> bool
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::lt(self, rhs: Self) -> bool
pub trait mirl::extensions::TupleOps<Rhs>
pub type mirl::extensions::TupleOps::Output
pub fn mirl::extensions::TupleOps::add(self, rhs: Rhs) -> Self::Output
pub fn mirl::extensions::TupleOps::div(self, rhs: Rhs) -> Self::Output
pub fn mirl::extensions::TupleOps::mul(self, rhs: Rhs) -> Self::Output
pub fn mirl::extensions::TupleOps::sub(self, rhs: Rhs) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T)::Output = (T, T)
pub fn (T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T)::Output = (T, T, T)
pub fn (T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T)::Output = (T, T, T, T)
pub fn (T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T)::Output = (T, T, T, T, T)
pub fn (T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T)::Output = (T, T, T, T, T, T)
pub fn (T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
impl<T> mirl::extensions::TupleOps for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) where T: core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::marker::Copy
pub type (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::Output = (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::add(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::div(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::mul(self, rhs: Self) -> Self::Output
pub fn (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)::sub(self, rhs: Self) -> Self::Output
pub mod mirl::graphics
pub mod mirl::graphics::color_presets
pub const mirl::graphics::color_presets::BLACK: u32
pub const mirl::graphics::color_presets::PURE_BLUE: u32
pub const mirl::graphics::color_presets::PURE_GREEN: u32
pub const mirl::graphics::color_presets::PURE_LIGHT_BLUE: u32
pub const mirl::graphics::color_presets::PURE_MAGENTA: u32
pub const mirl::graphics::color_presets::PURE_RED: u32
pub const mirl::graphics::color_presets::PURE_YELLOW: u32
pub const mirl::graphics::color_presets::WHITE: u32
pub mod mirl::graphics::imagery
pub fn mirl::graphics::imagery::buffer_to_dynamic_image(buffer: &mirl::platform::Buffer) -> image::dynimage::DynamicImage
pub fn mirl::graphics::imagery::create_empty_image(width: u32, height: u32) -> image::dynimage::DynamicImage
pub fn mirl::graphics::imagery::draw_texture_into_image(image: &mut image::dynimage::DynamicImage, texture_width: u16, texture_height: u16, texture_x: u16, texture_y: u16, texture: &image::dynimage::DynamicImage)
pub fn mirl::graphics::imagery::dynamic_image_to_buffer(image: &image::dynimage::DynamicImage) -> mirl::platform::Buffer
pub fn mirl::graphics::imagery::image_rgba_to_u32(rgba: image::color::Rgba<u8>) -> u32
pub fn mirl::graphics::imagery::pixmap_to_dynamic_image(ras: &tiny_skia::pixmap::Pixmap) -> image::dynimage::DynamicImage
pub fn mirl::graphics::imagery::rgb_to_image_rgba(r: u8, g: u8, b: u8) -> image::color::Rgba<u8>
pub fn mirl::graphics::imagery::set_image_size(image: image::dynimage::DynamicImage, width: u32, height: u32) -> image::dynimage::DynamicImage
pub fn mirl::graphics::imagery::u32_to_image_rgba(color: u32) -> image::color::Rgba<u8>
pub enum mirl::graphics::BrightnessModel
pub mirl::graphics::BrightnessModel::HSL
pub mirl::graphics::BrightnessModel::LinearWeighted
impl core::clone::Clone for mirl::graphics::BrightnessModel
pub fn mirl::graphics::BrightnessModel::clone(&self) -> mirl::graphics::BrightnessModel
impl core::cmp::Eq for mirl::graphics::BrightnessModel
impl core::cmp::PartialEq for mirl::graphics::BrightnessModel
pub fn mirl::graphics::BrightnessModel::eq(&self, other: &mirl::graphics::BrightnessModel) -> bool
impl core::fmt::Debug for mirl::graphics::BrightnessModel
pub fn mirl::graphics::BrightnessModel::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::graphics::BrightnessModel
impl core::marker::StructuralPartialEq for mirl::graphics::BrightnessModel
impl core::marker::Freeze for mirl::graphics::BrightnessModel
impl core::marker::Send for mirl::graphics::BrightnessModel
impl core::marker::Sync for mirl::graphics::BrightnessModel
impl core::marker::Unpin for mirl::graphics::BrightnessModel
impl core::panic::unwind_safe::RefUnwindSafe for mirl::graphics::BrightnessModel
impl core::panic::unwind_safe::UnwindSafe for mirl::graphics::BrightnessModel
impl<Q, K> equivalent::Equivalent<K> for mirl::graphics::BrightnessModel where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::graphics::BrightnessModel::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::graphics::BrightnessModel where U: core::convert::From<T>
pub fn mirl::graphics::BrightnessModel::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::graphics::BrightnessModel where U: core::convert::Into<T>
pub type mirl::graphics::BrightnessModel::Error = core::convert::Infallible
pub fn mirl::graphics::BrightnessModel::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::graphics::BrightnessModel where U: core::convert::TryFrom<T>
pub type mirl::graphics::BrightnessModel::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::graphics::BrightnessModel::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::graphics::BrightnessModel where T: core::clone::Clone
pub type mirl::graphics::BrightnessModel::Owned = T
pub fn mirl::graphics::BrightnessModel::clone_into(&self, target: &mut T)
pub fn mirl::graphics::BrightnessModel::to_owned(&self) -> T
impl<T> core::any::Any for mirl::graphics::BrightnessModel where T: 'static + ?core::marker::Sized
pub fn mirl::graphics::BrightnessModel::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::graphics::BrightnessModel where T: ?core::marker::Sized
pub fn mirl::graphics::BrightnessModel::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::graphics::BrightnessModel where T: ?core::marker::Sized
pub fn mirl::graphics::BrightnessModel::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::graphics::BrightnessModel where T: core::clone::Clone
pub unsafe fn mirl::graphics::BrightnessModel::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::graphics::BrightnessModel
pub fn mirl::graphics::BrightnessModel::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::graphics::BrightnessModel
pub type mirl::graphics::BrightnessModel::Init = T
pub const mirl::graphics::BrightnessModel::ALIGN: usize
pub unsafe fn mirl::graphics::BrightnessModel::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::graphics::BrightnessModel::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::graphics::BrightnessModel::drop(ptr: usize)
pub unsafe fn mirl::graphics::BrightnessModel::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::graphics::BrightnessModel
impl<T> mirl::extensions::RepeatData for mirl::graphics::BrightnessModel where T: core::clone::Clone
pub fn mirl::graphics::BrightnessModel::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::graphics::InterpolationMode
pub mirl::graphics::InterpolationMode::Linear
pub mirl::graphics::InterpolationMode::Nearest
impl core::clone::Clone for mirl::graphics::InterpolationMode
pub fn mirl::graphics::InterpolationMode::clone(&self) -> mirl::graphics::InterpolationMode
impl core::cmp::Eq for mirl::graphics::InterpolationMode
impl core::cmp::PartialEq for mirl::graphics::InterpolationMode
pub fn mirl::graphics::InterpolationMode::eq(&self, other: &mirl::graphics::InterpolationMode) -> bool
impl core::fmt::Debug for mirl::graphics::InterpolationMode
pub fn mirl::graphics::InterpolationMode::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::graphics::InterpolationMode
impl core::marker::StructuralPartialEq for mirl::graphics::InterpolationMode
impl core::marker::Freeze for mirl::graphics::InterpolationMode
impl core::marker::Send for mirl::graphics::InterpolationMode
impl core::marker::Sync for mirl::graphics::InterpolationMode
impl core::marker::Unpin for mirl::graphics::InterpolationMode
impl core::panic::unwind_safe::RefUnwindSafe for mirl::graphics::InterpolationMode
impl core::panic::unwind_safe::UnwindSafe for mirl::graphics::InterpolationMode
impl<Q, K> equivalent::Equivalent<K> for mirl::graphics::InterpolationMode where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::graphics::InterpolationMode::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::graphics::InterpolationMode where U: core::convert::From<T>
pub fn mirl::graphics::InterpolationMode::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::graphics::InterpolationMode where U: core::convert::Into<T>
pub type mirl::graphics::InterpolationMode::Error = core::convert::Infallible
pub fn mirl::graphics::InterpolationMode::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::graphics::InterpolationMode where U: core::convert::TryFrom<T>
pub type mirl::graphics::InterpolationMode::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::graphics::InterpolationMode::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::graphics::InterpolationMode where T: core::clone::Clone
pub type mirl::graphics::InterpolationMode::Owned = T
pub fn mirl::graphics::InterpolationMode::clone_into(&self, target: &mut T)
pub fn mirl::graphics::InterpolationMode::to_owned(&self) -> T
impl<T> core::any::Any for mirl::graphics::InterpolationMode where T: 'static + ?core::marker::Sized
pub fn mirl::graphics::InterpolationMode::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::graphics::InterpolationMode where T: ?core::marker::Sized
pub fn mirl::graphics::InterpolationMode::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::graphics::InterpolationMode where T: ?core::marker::Sized
pub fn mirl::graphics::InterpolationMode::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::graphics::InterpolationMode where T: core::clone::Clone
pub unsafe fn mirl::graphics::InterpolationMode::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::graphics::InterpolationMode
pub fn mirl::graphics::InterpolationMode::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::graphics::InterpolationMode
pub type mirl::graphics::InterpolationMode::Init = T
pub const mirl::graphics::InterpolationMode::ALIGN: usize
pub unsafe fn mirl::graphics::InterpolationMode::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::graphics::InterpolationMode::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::graphics::InterpolationMode::drop(ptr: usize)
pub unsafe fn mirl::graphics::InterpolationMode::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::graphics::InterpolationMode
impl<T> mirl::extensions::RepeatData for mirl::graphics::InterpolationMode where T: core::clone::Clone
pub fn mirl::graphics::InterpolationMode::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::graphics::Pixel
pub mirl::graphics::Pixel::a: u8
pub mirl::graphics::Pixel::b: u8
pub mirl::graphics::Pixel::color: u32
pub mirl::graphics::Pixel::g: u8
pub mirl::graphics::Pixel::r: u8
impl mirl::graphics::Pixel
pub const fn mirl::graphics::Pixel::new_32(color: u32) -> Self
pub const fn mirl::graphics::Pixel::new_rgb(r: u8, g: u8, b: u8, a: u8) -> Self
impl core::clone::Clone for mirl::graphics::Pixel
pub fn mirl::graphics::Pixel::clone(&self) -> mirl::graphics::Pixel
impl core::cmp::Eq for mirl::graphics::Pixel
impl core::cmp::PartialEq for mirl::graphics::Pixel
pub fn mirl::graphics::Pixel::eq(&self, other: &mirl::graphics::Pixel) -> bool
impl core::convert::From<mirl::graphics::Pixel> for image::color::Rgba<u8>
pub fn image::color::Rgba<u8>::from(p: mirl::graphics::Pixel) -> Self
impl core::convert::From<mirl::graphics::Pixel> for u32
pub fn u32::from(p: mirl::graphics::Pixel) -> Self
impl core::fmt::Debug for mirl::graphics::Pixel
pub fn mirl::graphics::Pixel::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::graphics::Pixel
impl core::marker::StructuralPartialEq for mirl::graphics::Pixel
impl core::marker::Freeze for mirl::graphics::Pixel
impl core::marker::Send for mirl::graphics::Pixel
impl core::marker::Sync for mirl::graphics::Pixel
impl core::marker::Unpin for mirl::graphics::Pixel
impl core::panic::unwind_safe::RefUnwindSafe for mirl::graphics::Pixel
impl core::panic::unwind_safe::UnwindSafe for mirl::graphics::Pixel
impl<Q, K> equivalent::Equivalent<K> for mirl::graphics::Pixel where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::graphics::Pixel::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::graphics::Pixel where U: core::convert::From<T>
pub fn mirl::graphics::Pixel::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::graphics::Pixel where U: core::convert::Into<T>
pub type mirl::graphics::Pixel::Error = core::convert::Infallible
pub fn mirl::graphics::Pixel::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::graphics::Pixel where U: core::convert::TryFrom<T>
pub type mirl::graphics::Pixel::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::graphics::Pixel::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::graphics::Pixel where T: core::clone::Clone
pub type mirl::graphics::Pixel::Owned = T
pub fn mirl::graphics::Pixel::clone_into(&self, target: &mut T)
pub fn mirl::graphics::Pixel::to_owned(&self) -> T
impl<T> core::any::Any for mirl::graphics::Pixel where T: 'static + ?core::marker::Sized
pub fn mirl::graphics::Pixel::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::graphics::Pixel where T: ?core::marker::Sized
pub fn mirl::graphics::Pixel::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::graphics::Pixel where T: ?core::marker::Sized
pub fn mirl::graphics::Pixel::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::graphics::Pixel where T: core::clone::Clone
pub unsafe fn mirl::graphics::Pixel::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::graphics::Pixel
pub fn mirl::graphics::Pixel::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::graphics::Pixel
pub type mirl::graphics::Pixel::Init = T
pub const mirl::graphics::Pixel::ALIGN: usize
pub unsafe fn mirl::graphics::Pixel::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::graphics::Pixel::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::graphics::Pixel::drop(ptr: usize)
pub unsafe fn mirl::graphics::Pixel::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::graphics::Pixel
impl<T> mirl::extensions::RepeatData for mirl::graphics::Pixel where T: core::clone::Clone
pub fn mirl::graphics::Pixel::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::graphics::TextureManager
pub mirl::graphics::TextureManager::current_frame: u64
pub mirl::graphics::TextureManager::free_list: alloc::vec::Vec<usize>
pub mirl::graphics::TextureManager::last_used: alloc::vec::Vec<u64>
pub mirl::graphics::TextureManager::lookup: ahash::hash_map::AHashMap<alloc::string::String, usize>
pub mirl::graphics::TextureManager::texture_lookup: ahash::hash_map::AHashMap<alloc::string::String, alloc::string::String>
pub mirl::graphics::TextureManager::textures: alloc::vec::Vec<core::option::Option<mirl::platform::Buffer>>
impl mirl::graphics::TextureManager
pub fn mirl::graphics::TextureManager::cleanup_unused(&mut self, frames_unused: u64)
pub fn mirl::graphics::TextureManager::get(&mut self, name: &str, file_system: &dyn mirl::platform::file_system::FileSystem, remove_margins: bool) -> core::option::Option<&mirl::platform::Buffer>
pub fn mirl::graphics::TextureManager::insert_texture(&mut self, name: alloc::string::String, texture: mirl::platform::Buffer)
pub fn mirl::graphics::TextureManager::is_texture_registered(&self, name: &str) -> bool
pub fn mirl::graphics::TextureManager::load_texture_from_file(&self, file_path: &str, file_system: &dyn mirl::platform::file_system::FileSystem) -> core::result::Result<mirl::platform::Buffer, alloc::boxed::Box<dyn core::error::Error>>
pub fn mirl::graphics::TextureManager::new() -> Self
pub fn mirl::graphics::TextureManager::preload_texture(&mut self, name: &str, file_system: &dyn mirl::platform::file_system::FileSystem) -> core::result::Result<(), alloc::boxed::Box<dyn core::error::Error>>
pub fn mirl::graphics::TextureManager::register_texture(&mut self, name: alloc::string::String, file_path: alloc::string::String)
pub const fn mirl::graphics::TextureManager::tick(&mut self)
pub fn mirl::graphics::TextureManager::unload_texture(&mut self, name: &str)
impl core::clone::Clone for mirl::graphics::TextureManager
pub fn mirl::graphics::TextureManager::clone(&self) -> mirl::graphics::TextureManager
impl core::cmp::Eq for mirl::graphics::TextureManager
impl core::cmp::PartialEq for mirl::graphics::TextureManager
pub fn mirl::graphics::TextureManager::eq(&self, other: &mirl::graphics::TextureManager) -> bool
impl core::default::Default for mirl::graphics::TextureManager
pub fn mirl::graphics::TextureManager::default() -> Self
impl core::fmt::Debug for mirl::graphics::TextureManager
pub fn mirl::graphics::TextureManager::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::StructuralPartialEq for mirl::graphics::TextureManager
impl core::marker::Freeze for mirl::graphics::TextureManager
impl !core::marker::Send for mirl::graphics::TextureManager
impl !core::marker::Sync for mirl::graphics::TextureManager
impl core::marker::Unpin for mirl::graphics::TextureManager
impl core::panic::unwind_safe::RefUnwindSafe for mirl::graphics::TextureManager
impl core::panic::unwind_safe::UnwindSafe for mirl::graphics::TextureManager
impl<Q, K> equivalent::Equivalent<K> for mirl::graphics::TextureManager where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::graphics::TextureManager::equivalent(&self, key: &K) -> bool
impl<R, P> lebe::io::ReadPrimitive<R> for mirl::graphics::TextureManager where R: std::io::Read + lebe::io::ReadEndian<P>, P: core::default::Default
impl<T, U> core::convert::Into<U> for mirl::graphics::TextureManager where U: core::convert::From<T>
pub fn mirl::graphics::TextureManager::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::graphics::TextureManager where U: core::convert::Into<T>
pub type mirl::graphics::TextureManager::Error = core::convert::Infallible
pub fn mirl::graphics::TextureManager::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::graphics::TextureManager where U: core::convert::TryFrom<T>
pub type mirl::graphics::TextureManager::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::graphics::TextureManager::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::graphics::TextureManager where T: core::clone::Clone
pub type mirl::graphics::TextureManager::Owned = T
pub fn mirl::graphics::TextureManager::clone_into(&self, target: &mut T)
pub fn mirl::graphics::TextureManager::to_owned(&self) -> T
impl<T> core::any::Any for mirl::graphics::TextureManager where T: 'static + ?core::marker::Sized
pub fn mirl::graphics::TextureManager::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::graphics::TextureManager where T: ?core::marker::Sized
pub fn mirl::graphics::TextureManager::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::graphics::TextureManager where T: ?core::marker::Sized
pub fn mirl::graphics::TextureManager::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::graphics::TextureManager where T: core::clone::Clone
pub unsafe fn mirl::graphics::TextureManager::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::graphics::TextureManager
pub fn mirl::graphics::TextureManager::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::graphics::TextureManager
pub type mirl::graphics::TextureManager::Init = T
pub const mirl::graphics::TextureManager::ALIGN: usize
pub unsafe fn mirl::graphics::TextureManager::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::graphics::TextureManager::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::graphics::TextureManager::drop(ptr: usize)
pub unsafe fn mirl::graphics::TextureManager::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::graphics::TextureManager
impl<T> mirl::extensions::RepeatData for mirl::graphics::TextureManager where T: core::clone::Clone
pub fn mirl::graphics::TextureManager::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub fn mirl::graphics::adjust_brightness_based_on_human_eye(color: u32, x: i32, model: mirl::graphics::BrightnessModel) -> u32
pub fn mirl::graphics::adjust_brightness_fast(color: u32, x: i32) -> u32
pub fn mirl::graphics::adjust_brightness_hsl_of_rgb(color: u32, change: i32) -> u32
pub fn mirl::graphics::argb_list_to_rgba_list(input: &[u32]) -> alloc::vec::Vec<u32>
pub const fn mirl::graphics::argb_to_rgba(color: u32) -> u32
pub fn mirl::graphics::bilinear_interpolate_u32(p1: u32, p2: u32, p3: u32, p4: u32, dx: f32, dy: f32) -> u32
pub fn mirl::graphics::buffer_to_dynamic_image(buffer: &mirl::platform::Buffer) -> image::dynimage::DynamicImage
pub fn mirl::graphics::buffer_to_pixel_image(buffer: &mirl::platform::Buffer) -> glfw::PixelImage
pub fn mirl::graphics::create_empty_image(width: u32, height: u32) -> image::dynimage::DynamicImage
pub fn mirl::graphics::desaturate_fast(color: u32, amount: f32) -> u32
pub fn mirl::graphics::draw_texture_into_image(image: &mut image::dynimage::DynamicImage, texture_width: u16, texture_height: u16, texture_x: u16, texture_y: u16, texture: &image::dynimage::DynamicImage)
pub fn mirl::graphics::dynamic_image_to_buffer(image: &image::dynimage::DynamicImage) -> mirl::platform::Buffer
pub const fn mirl::graphics::get_alpha_of_u32(color: u32) -> u8
pub const fn mirl::graphics::get_blue_of_u32(color: u32) -> u8
pub const fn mirl::graphics::get_green_of_u32(color: u32) -> u8
pub fn mirl::graphics::get_hue_of_rgb(r: f32, g: f32, b: f32) -> f32
pub const fn mirl::graphics::get_red_of_u32(color: u32) -> u8
pub const fn mirl::graphics::get_u32_alpha_of_u32(color: u32) -> u32
pub const fn mirl::graphics::get_u32_blue_of_u32(color: u32) -> u32
pub const fn mirl::graphics::get_u32_green_of_u32(color: u32) -> u32
pub const fn mirl::graphics::get_u32_red_of_u32(color: u32) -> u32
pub fn mirl::graphics::get_unused_color(buffer: &[u8], current_color: (u8, u8, u8)) -> (u8, u8, u8)
pub fn mirl::graphics::get_unused_color_of_buffer(buffer: &mirl::platform::Buffer, current_color: (u8, u8, u8)) -> (u8, u8, u8)
pub fn mirl::graphics::hex_to_u32(hex: &str) -> core::result::Result<u32, core::num::error::ParseIntError>
pub fn mirl::graphics::hex_to_u32_rgb(hex: &str) -> core::result::Result<u32, core::num::error::ParseIntError>
pub fn mirl::graphics::hex_to_u32_rgba(hex: &str) -> core::result::Result<u32, core::num::error::ParseIntError>
pub const fn mirl::graphics::hsl_to_rgb_f32(hue: f32, saturation: f32, lightness: f32) -> (f32, f32, f32)
pub fn mirl::graphics::hsl_to_rgb_u32(hue: f32, saturation: f32, lightness: f32) -> (u32, u32, u32)
pub fn mirl::graphics::image_rgba_to_u32(rgba: image::color::Rgba<u8>) -> u32
pub fn mirl::graphics::interpolate_color_rgb_u32_f32(from: u32, to: u32, progress: f32) -> u32
pub fn mirl::graphics::interpolate_color_rgb_u32_f64(from: u32, to: u32, progress: f64) -> u32
pub const fn mirl::graphics::invert_color(color: u32) -> u32
pub fn mirl::graphics::pixel_image_to_buffer(pixel_image: &glfw::PixelImage) -> mirl::platform::Buffer
pub fn mirl::graphics::pixmap_to_buffer(pixmap: &tiny_skia::pixmap::Pixmap) -> mirl::platform::Buffer
pub fn mirl::graphics::pixmap_to_dynamic_image(ras: &tiny_skia::pixmap::Pixmap) -> image::dynimage::DynamicImage
pub fn mirl::graphics::rasterize_svg(svg_data: &[u8], width: u32, height: u32) -> tiny_skia::pixmap::Pixmap
pub fn mirl::graphics::resize_buffer(buffer: &[u32], src_width: usize, src_height: usize, dst_width: usize, dst_height: usize, resize_mode: mirl::graphics::InterpolationMode) -> alloc::vec::Vec<u32>
pub fn mirl::graphics::resize_buffer_linear(buffer: &[u32], src_width: usize, src_height: usize, dst_width: usize, dst_height: usize) -> alloc::vec::Vec<u32>
pub fn mirl::graphics::resize_buffer_nearest(buffer: &[u32], src_width: usize, src_height: usize, dst_width: usize, dst_height: usize) -> alloc::vec::Vec<u32>
pub fn mirl::graphics::rgb_to_hex(r: u8, g: u8, b: u8) -> alloc::string::String
pub const fn mirl::graphics::rgb_to_hsl(r: u8, g: u8, b: u8) -> (f32, f32, f32)
pub fn mirl::graphics::rgb_to_image_rgba(r: u8, g: u8, b: u8) -> image::color::Rgba<u8>
pub const fn mirl::graphics::rgb_to_u32(r: u8, g: u8, b: u8) -> u32
pub const fn mirl::graphics::rgb_u32_to_u32(r: u32, g: u32, b: u32) -> u32
pub fn mirl::graphics::rgba_list_to_argb_list(input: &[u32]) -> alloc::vec::Vec<u32>
pub const fn mirl::graphics::rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32
pub const fn mirl::graphics::rgba_u32_to_u32(r: u32, g: u32, b: u32, a: u32) -> u32
pub fn mirl::graphics::set_image_size(image: image::dynimage::DynamicImage, width: u32, height: u32) -> image::dynimage::DynamicImage
pub fn mirl::graphics::shift_color_rgb(red: u8, green: u8, blue: u8, hue_shift: f32) -> (u32, u32, u32)
pub fn mirl::graphics::shift_color_u32(color: u32, hue_shift: f32) -> u32
pub fn mirl::graphics::shift_hue_rgb(r: u8, g: u8, b: u8, hue_shift_degrees: f32) -> (u32, u32, u32)
pub fn mirl::graphics::shift_hue_u32(color: u32, hue_shift: f32) -> u32
pub const fn mirl::graphics::switch_red_and_blue(color: u32) -> u32
pub fn mirl::graphics::switch_red_and_blue_list(input: &[u32]) -> alloc::vec::Vec<u32>
pub const fn mirl::graphics::u32_to_argb(color: u32) -> (u8, u8, u8, u8)
pub const fn mirl::graphics::u32_to_argb_u32(color: u32) -> (u32, u32, u32, u32)
pub fn mirl::graphics::u32_to_hex(color: u32) -> alloc::string::String
pub fn mirl::graphics::u32_to_image_rgba(color: u32) -> image::color::Rgba<u8>
pub const fn mirl::graphics::u32_to_rgb(color: u32) -> (u8, u8, u8)
pub const fn mirl::graphics::u32_to_rgb_u32(color: u32) -> (u32, u32, u32)
pub const fn mirl::graphics::u32_to_rgba(color: u32) -> (u8, u8, u8, u8)
pub const fn mirl::graphics::u32_to_rgba_u32(color: u32) -> (u32, u32, u32, u32)
pub mod mirl::lists
pub enum mirl::lists::VariableSizeList<T> where T: core::marker::Copy
pub mirl::lists::VariableSizeList::Image128(mirl::lists::List16K<T>)
pub mirl::lists::VariableSizeList::Image256(mirl::lists::List64K<T>)
pub mirl::lists::VariableSizeList::Image32(mirl::lists::List1K<T>)
pub mirl::lists::VariableSizeList::Image64(mirl::lists::List4K<T>)
impl<T> mirl::lists::VariableSizeList<T> where T: core::marker::Copy + core::default::Default
pub fn mirl::lists::VariableSizeList<T>::get(&self, index: usize) -> core::option::Option<&T>
pub const fn mirl::lists::VariableSizeList<T>::is_empty(&self) -> bool
pub const fn mirl::lists::VariableSizeList<T>::len(&self) -> usize
pub fn mirl::lists::VariableSizeList<T>::new_128() -> Self
pub fn mirl::lists::VariableSizeList<T>::new_256() -> Self
pub fn mirl::lists::VariableSizeList<T>::new_32() -> Self
pub fn mirl::lists::VariableSizeList<T>::new_64() -> Self
pub const fn mirl::lists::VariableSizeList<T>::set(&mut self, index: usize, value: T) -> core::result::Result<(), &'static str>
impl core::convert::From<mirl::lists::VariableSizeList<u32>> for mirl::platform::Buffer
pub fn mirl::platform::Buffer::from(list: mirl::lists::VariableSizeList<u32>) -> Self
impl core::convert::TryFrom<mirl::platform::Buffer> for mirl::lists::VariableSizeList<u32>
pub type mirl::lists::VariableSizeList<u32>::Error = &'static str
pub fn mirl::lists::VariableSizeList<u32>::try_from(value: mirl::platform::Buffer) -> core::result::Result<Self, Self::Error>
impl<T> core::clone::Clone for mirl::lists::VariableSizeList<T> where T: core::marker::Copy + core::clone::Clone
pub fn mirl::lists::VariableSizeList<T>::clone(&self) -> mirl::lists::VariableSizeList<T>
impl<T> core::cmp::Eq for mirl::lists::VariableSizeList<T> where T: core::marker::Copy + core::cmp::Eq
impl<T> core::cmp::PartialEq for mirl::lists::VariableSizeList<T> where T: core::marker::Copy + core::cmp::PartialEq
pub fn mirl::lists::VariableSizeList<T>::eq(&self, other: &mirl::lists::VariableSizeList<T>) -> bool
impl<T> core::fmt::Debug for mirl::lists::VariableSizeList<T> where T: core::marker::Copy + core::fmt::Debug
pub fn mirl::lists::VariableSizeList<T>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<T> core::marker::Copy for mirl::lists::VariableSizeList<T> where T: core::marker::Copy + core::marker::Copy
impl<T> core::marker::StructuralPartialEq for mirl::lists::VariableSizeList<T> where T: core::marker::Copy
impl<T> core::marker::Freeze for mirl::lists::VariableSizeList<T> where T: core::marker::Freeze
impl<T> core::marker::Send for mirl::lists::VariableSizeList<T> where T: core::marker::Send
impl<T> core::marker::Sync for mirl::lists::VariableSizeList<T> where T: core::marker::Sync
impl<T> core::marker::Unpin for mirl::lists::VariableSizeList<T> where T: core::marker::Unpin
impl<T> core::panic::unwind_safe::RefUnwindSafe for mirl::lists::VariableSizeList<T> where T: core::panic::unwind_safe::RefUnwindSafe
impl<T> core::panic::unwind_safe::UnwindSafe for mirl::lists::VariableSizeList<T> where T: core::panic::unwind_safe::UnwindSafe
impl<Q, K> equivalent::Equivalent<K> for mirl::lists::VariableSizeList<T> where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::lists::VariableSizeList<T>::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::lists::VariableSizeList<T> where U: core::convert::From<T>
pub fn mirl::lists::VariableSizeList<T>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::lists::VariableSizeList<T> where U: core::convert::Into<T>
pub type mirl::lists::VariableSizeList<T>::Error = core::convert::Infallible
pub fn mirl::lists::VariableSizeList<T>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::lists::VariableSizeList<T> where U: core::convert::TryFrom<T>
pub type mirl::lists::VariableSizeList<T>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::lists::VariableSizeList<T>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::lists::VariableSizeList<T> where T: core::clone::Clone
pub type mirl::lists::VariableSizeList<T>::Owned = T
pub fn mirl::lists::VariableSizeList<T>::clone_into(&self, target: &mut T)
pub fn mirl::lists::VariableSizeList<T>::to_owned(&self) -> T
impl<T> core::any::Any for mirl::lists::VariableSizeList<T> where T: 'static + ?core::marker::Sized
pub fn mirl::lists::VariableSizeList<T>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::lists::VariableSizeList<T> where T: ?core::marker::Sized
pub fn mirl::lists::VariableSizeList<T>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::lists::VariableSizeList<T> where T: ?core::marker::Sized
pub fn mirl::lists::VariableSizeList<T>::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::lists::VariableSizeList<T> where T: core::clone::Clone
pub unsafe fn mirl::lists::VariableSizeList<T>::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::lists::VariableSizeList<T>
pub fn mirl::lists::VariableSizeList<T>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::lists::VariableSizeList<T>
pub type mirl::lists::VariableSizeList<T>::Init = T
pub const mirl::lists::VariableSizeList<T>::ALIGN: usize
pub unsafe fn mirl::lists::VariableSizeList<T>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::lists::VariableSizeList<T>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::lists::VariableSizeList<T>::drop(ptr: usize)
pub unsafe fn mirl::lists::VariableSizeList<T>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::lists::VariableSizeList<T>
impl<T> mirl::extensions::RepeatData for mirl::lists::VariableSizeList<T> where T: core::clone::Clone
pub fn mirl::lists::VariableSizeList<T>::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::lists::CopyableList<T, const N: usize> where T: core::marker::Copy
pub mirl::lists::CopyableList::data: [T; N]
impl<T, const N: usize> mirl::lists::CopyableList<T, N> where T: core::marker::Copy + core::default::Default
pub const fn mirl::lists::CopyableList<T, N>::as_slice(&self) -> &[T]
pub const fn mirl::lists::CopyableList<T, N>::from_array(data: [T; N]) -> Self
pub fn mirl::lists::CopyableList<T, N>::get(&self, index: usize) -> core::option::Option<&T>
pub const fn mirl::lists::CopyableList<T, N>::is_empty(&self) -> bool
pub const fn mirl::lists::CopyableList<T, N>::len(&self) -> usize
pub fn mirl::lists::CopyableList<T, N>::new() -> Self
pub const fn mirl::lists::CopyableList<T, N>::set(&mut self, index: usize, value: T) -> core::result::Result<(), &'static str>
impl<const N: usize> mirl::lists::CopyableList<u32, N>
pub const fn mirl::lists::CopyableList<u32, N>::swap_red_blue(&self) -> Self
impl<T, const N: usize> core::clone::Clone for mirl::lists::CopyableList<T, N> where T: core::marker::Copy + core::clone::Clone
pub fn mirl::lists::CopyableList<T, N>::clone(&self) -> mirl::lists::CopyableList<T, N>
impl<T, const N: usize> core::cmp::Eq for mirl::lists::CopyableList<T, N> where T: core::marker::Copy + core::cmp::Eq
impl<T, const N: usize> core::cmp::PartialEq for mirl::lists::CopyableList<T, N> where T: core::marker::Copy + core::cmp::PartialEq
pub fn mirl::lists::CopyableList<T, N>::eq(&self, other: &mirl::lists::CopyableList<T, N>) -> bool
impl<T, const N: usize> core::default::Default for mirl::lists::CopyableList<T, N> where T: core::marker::Copy + core::default::Default
pub fn mirl::lists::CopyableList<T, N>::default() -> Self
impl<T, const N: usize> core::fmt::Debug for mirl::lists::CopyableList<T, N> where T: core::marker::Copy + core::fmt::Debug
pub fn mirl::lists::CopyableList<T, N>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<T, const N: usize> core::marker::Copy for mirl::lists::CopyableList<T, N> where T: core::marker::Copy + core::marker::Copy
impl<T, const N: usize> core::marker::StructuralPartialEq for mirl::lists::CopyableList<T, N> where T: core::marker::Copy
impl<const N: usize> core::convert::From<mirl::lists::CopyableList<u32, N>> for mirl::platform::Buffer
pub fn mirl::platform::Buffer::from(list: mirl::lists::CopyableList<u32, N>) -> Self
impl<T, const N: usize> core::marker::Freeze for mirl::lists::CopyableList<T, N> where T: core::marker::Freeze
impl<T, const N: usize> core::marker::Send for mirl::lists::CopyableList<T, N> where T: core::marker::Send
impl<T, const N: usize> core::marker::Sync for mirl::lists::CopyableList<T, N> where T: core::marker::Sync
impl<T, const N: usize> core::marker::Unpin for mirl::lists::CopyableList<T, N> where T: core::marker::Unpin
impl<T, const N: usize> core::panic::unwind_safe::RefUnwindSafe for mirl::lists::CopyableList<T, N> where T: core::panic::unwind_safe::RefUnwindSafe
impl<T, const N: usize> core::panic::unwind_safe::UnwindSafe for mirl::lists::CopyableList<T, N> where T: core::panic::unwind_safe::UnwindSafe
impl<Q, K> equivalent::Equivalent<K> for mirl::lists::CopyableList<T, N> where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::lists::CopyableList<T, N>::equivalent(&self, key: &K) -> bool
impl<R, P> lebe::io::ReadPrimitive<R> for mirl::lists::CopyableList<T, N> where R: std::io::Read + lebe::io::ReadEndian<P>, P: core::default::Default
impl<T, U> core::convert::Into<U> for mirl::lists::CopyableList<T, N> where U: core::convert::From<T>
pub fn mirl::lists::CopyableList<T, N>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::lists::CopyableList<T, N> where U: core::convert::Into<T>
pub type mirl::lists::CopyableList<T, N>::Error = core::convert::Infallible
pub fn mirl::lists::CopyableList<T, N>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::lists::CopyableList<T, N> where U: core::convert::TryFrom<T>
pub type mirl::lists::CopyableList<T, N>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::lists::CopyableList<T, N>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::lists::CopyableList<T, N> where T: core::clone::Clone
pub type mirl::lists::CopyableList<T, N>::Owned = T
pub fn mirl::lists::CopyableList<T, N>::clone_into(&self, target: &mut T)
pub fn mirl::lists::CopyableList<T, N>::to_owned(&self) -> T
impl<T> core::any::Any for mirl::lists::CopyableList<T, N> where T: 'static + ?core::marker::Sized
pub fn mirl::lists::CopyableList<T, N>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::lists::CopyableList<T, N> where T: ?core::marker::Sized
pub fn mirl::lists::CopyableList<T, N>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::lists::CopyableList<T, N> where T: ?core::marker::Sized
pub fn mirl::lists::CopyableList<T, N>::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::lists::CopyableList<T, N> where T: core::clone::Clone
pub unsafe fn mirl::lists::CopyableList<T, N>::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::lists::CopyableList<T, N>
pub fn mirl::lists::CopyableList<T, N>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::lists::CopyableList<T, N>
pub type mirl::lists::CopyableList<T, N>::Init = T
pub const mirl::lists::CopyableList<T, N>::ALIGN: usize
pub unsafe fn mirl::lists::CopyableList<T, N>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::lists::CopyableList<T, N>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::lists::CopyableList<T, N>::drop(ptr: usize)
pub unsafe fn mirl::lists::CopyableList<T, N>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::lists::CopyableList<T, N>
impl<T> mirl::extensions::RepeatData for mirl::lists::CopyableList<T, N> where T: core::clone::Clone
pub fn mirl::lists::CopyableList<T, N>::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub fn mirl::lists::add_item_to_max_sized_list<T>(list: &mut alloc::vec::Vec<T>, max_size: usize, item: T)
pub fn mirl::lists::average<T: num_traits::Num + num_traits::cast::NumCast + core::marker::Copy>(vec: &[T]) -> core::option::Option<T>
pub fn mirl::lists::buffer_to_copy_list(buffer: &mirl::platform::Buffer) -> core::result::Result<alloc::boxed::Box<mirl::lists::VariableSizeList<u32>>, &'static str>
pub fn mirl::lists::combined<T: core::clone::Clone + core::marker::Sized>(vec: &[T], other: T) -> alloc::vec::Vec<T>
pub fn mirl::lists::fill(from: &mirl::platform::Buffer, to: alloc::boxed::Box<mirl::lists::VariableSizeList<u32>>) -> core::result::Result<alloc::boxed::Box<mirl::lists::VariableSizeList<u32>>, &'static str>
pub fn mirl::lists::get_difference_new<'a, T: core::cmp::PartialEq>(old: &'a [T], new: &'a alloc::vec::Vec<T>) -> alloc::vec::Vec<&'a T>
pub fn mirl::lists::get_sub_vec_of_vec<T: core::marker::Copy>(vec: &[T], width: u32, cutout_x: u32, cutout_y: u32, cutout_width: u32, cutout_height: u32) -> alloc::vec::Vec<T>
pub fn mirl::lists::u2_to_size_list<T: core::marker::Copy + core::default::Default>(size: mirl::extensions::U2) -> mirl::lists::VariableSizeList<T>
pub type mirl::lists::List16K<T> = mirl::lists::CopyableList<T, 16384>
pub type mirl::lists::List1K<T> = mirl::lists::CopyableList<T, 1024>
pub type mirl::lists::List4K<T> = mirl::lists::CopyableList<T, 4096>
pub type mirl::lists::List64K<T> = mirl::lists::CopyableList<T, 65536>
pub mod mirl::math
pub mod mirl::math::collision
pub mod mirl::math::collision::circle
pub struct mirl::math::collision::circle::Circle<T, const CS: bool>
pub mirl::math::collision::circle::Circle::half_radius: T
pub mirl::math::collision::circle::Circle::radius: T
pub mirl::math::collision::circle::Circle::x: T
pub mirl::math::collision::circle::Circle::y: T
impl<const CS: bool, T: mirl::math::NumberWithMonotoneOps + core::marker::Copy + mirl::math::ConvenientOps> mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::new(x: T, y: T, radius: T) -> Self
impl<const CS: bool, T: mirl::math::NumberWithMonotoneOps + core::marker::Copy> mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::does_area_contain_point(&self, point: (T, T)) -> bool
impl<T, const CS: bool> core::marker::StructuralPartialEq for mirl::math::collision::circle::Circle<T, CS>
impl<T: core::clone::Clone, const CS: bool> core::clone::Clone for mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::clone(&self) -> mirl::math::collision::circle::Circle<T, CS>
impl<T: core::cmp::Eq, const CS: bool> core::cmp::Eq for mirl::math::collision::circle::Circle<T, CS>
impl<T: core::cmp::PartialEq, const CS: bool> core::cmp::PartialEq for mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::eq(&self, other: &mirl::math::collision::circle::Circle<T, CS>) -> bool
impl<T: core::fmt::Debug, const CS: bool> core::fmt::Debug for mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<T: core::marker::Copy, const CS: bool> core::marker::Copy for mirl::math::collision::circle::Circle<T, CS>
impl<T, const CS: bool> core::marker::Freeze for mirl::math::collision::circle::Circle<T, CS> where T: core::marker::Freeze
impl<T, const CS: bool> core::marker::Send for mirl::math::collision::circle::Circle<T, CS> where T: core::marker::Send
impl<T, const CS: bool> core::marker::Sync for mirl::math::collision::circle::Circle<T, CS> where T: core::marker::Sync
impl<T, const CS: bool> core::marker::Unpin for mirl::math::collision::circle::Circle<T, CS> where T: core::marker::Unpin
impl<T, const CS: bool> core::panic::unwind_safe::RefUnwindSafe for mirl::math::collision::circle::Circle<T, CS> where T: core::panic::unwind_safe::RefUnwindSafe
impl<T, const CS: bool> core::panic::unwind_safe::UnwindSafe for mirl::math::collision::circle::Circle<T, CS> where T: core::panic::unwind_safe::UnwindSafe
impl<Q, K> equivalent::Equivalent<K> for mirl::math::collision::circle::Circle<T, CS> where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::math::collision::circle::Circle<T, CS>::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::math::collision::circle::Circle<T, CS> where U: core::convert::From<T>
pub fn mirl::math::collision::circle::Circle<T, CS>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::math::collision::circle::Circle<T, CS> where U: core::convert::Into<T>
pub type mirl::math::collision::circle::Circle<T, CS>::Error = core::convert::Infallible
pub fn mirl::math::collision::circle::Circle<T, CS>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::math::collision::circle::Circle<T, CS> where U: core::convert::TryFrom<T>
pub type mirl::math::collision::circle::Circle<T, CS>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::math::collision::circle::Circle<T, CS>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::math::collision::circle::Circle<T, CS> where T: core::clone::Clone
pub type mirl::math::collision::circle::Circle<T, CS>::Owned = T
pub fn mirl::math::collision::circle::Circle<T, CS>::clone_into(&self, target: &mut T)
pub fn mirl::math::collision::circle::Circle<T, CS>::to_owned(&self) -> T
impl<T> core::any::Any for mirl::math::collision::circle::Circle<T, CS> where T: 'static + ?core::marker::Sized
pub fn mirl::math::collision::circle::Circle<T, CS>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::math::collision::circle::Circle<T, CS> where T: ?core::marker::Sized
pub fn mirl::math::collision::circle::Circle<T, CS>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::math::collision::circle::Circle<T, CS> where T: ?core::marker::Sized
pub fn mirl::math::collision::circle::Circle<T, CS>::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::math::collision::circle::Circle<T, CS> where T: core::clone::Clone
pub unsafe fn mirl::math::collision::circle::Circle<T, CS>::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::math::collision::circle::Circle<T, CS>
pub type mirl::math::collision::circle::Circle<T, CS>::Init = T
pub const mirl::math::collision::circle::Circle<T, CS>::ALIGN: usize
pub unsafe fn mirl::math::collision::circle::Circle<T, CS>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::math::collision::circle::Circle<T, CS>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::math::collision::circle::Circle<T, CS>::drop(ptr: usize)
pub unsafe fn mirl::math::collision::circle::Circle<T, CS>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::math::collision::circle::Circle<T, CS>
impl<T> mirl::extensions::RepeatData for mirl::math::collision::circle::Circle<T, CS> where T: core::clone::Clone
pub fn mirl::math::collision::circle::Circle<T, CS>::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub mod mirl::math::collision::rectangle
pub struct mirl::math::collision::rectangle::Rectangle<T, const CS: bool>
pub mirl::math::collision::rectangle::Rectangle::height: T
pub mirl::math::collision::rectangle::Rectangle::width: T
pub mirl::math::collision::rectangle::Rectangle::x: T
pub mirl::math::collision::rectangle::Rectangle::y: T
impl<T, const BOTTOM_HIGHER: bool> mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER> where T: core::ops::arith::Add<Output = T> + core::cmp::PartialOrd + core::marker::Copy
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::bottom(&self) -> T where T: core::ops::arith::Add<Output = T> + core::marker::Copy
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::get_height(&self) -> T
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::get_width(&self) -> T
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::get_x(&self) -> T
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::get_y(&self) -> T
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::left(&self) -> T
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::new(x: T, y: T, width: T, height: T) -> Self
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::right(&self) -> T where T: core::ops::arith::Add<Output = T> + core::marker::Copy
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::top(&self) -> T where T: core::ops::arith::Add<Output = T> + core::marker::Copy
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<f32, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<f32, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<f32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f32, CS>::does_area_contain_point(&self, point: (f32, f32)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f32, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<f32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f32, CS>::get_edge_position(&self, point: (f32, f32), margin: f32) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<f32, CS>::is_point_at_edge(&self, point: (f32, f32), margin: f32) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<f64, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<f64, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<f64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f64, CS>::does_area_contain_point(&self, point: (f64, f64)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f64, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<f64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f64, CS>::get_edge_position(&self, point: (f64, f64), margin: f64) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<f64, CS>::is_point_at_edge(&self, point: (f64, f64), margin: f64) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<i128, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<i128, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<i128, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i128, CS>::does_area_contain_point(&self, point: (i128, i128)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i128, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<i128, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i128, CS>::get_edge_position(&self, point: (i128, i128), margin: i128) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<i128, CS>::is_point_at_edge(&self, point: (i128, i128), margin: i128) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<i16, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<i16, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<i16, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i16, CS>::does_area_contain_point(&self, point: (i16, i16)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i16, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<i16, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i16, CS>::get_edge_position(&self, point: (i16, i16), margin: i16) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<i16, CS>::is_point_at_edge(&self, point: (i16, i16), margin: i16) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<i32, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<i32, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<i32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i32, CS>::does_area_contain_point(&self, point: (i32, i32)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i32, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<i32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i32, CS>::get_edge_position(&self, point: (i32, i32), margin: i32) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<i32, CS>::is_point_at_edge(&self, point: (i32, i32), margin: i32) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<i64, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<i64, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<i64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i64, CS>::does_area_contain_point(&self, point: (i64, i64)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i64, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<i64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i64, CS>::get_edge_position(&self, point: (i64, i64), margin: i64) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<i64, CS>::is_point_at_edge(&self, point: (i64, i64), margin: i64) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<i8, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<i8, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<i8, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i8, CS>::does_area_contain_point(&self, point: (i8, i8)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i8, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<i8, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i8, CS>::get_edge_position(&self, point: (i8, i8), margin: i8) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<i8, CS>::is_point_at_edge(&self, point: (i8, i8), margin: i8) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<isize, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<isize, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<isize, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<isize, CS>::does_area_contain_point(&self, point: (isize, isize)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<isize, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<isize, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<isize, CS>::get_edge_position(&self, point: (isize, isize), margin: isize) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<isize, CS>::is_point_at_edge(&self, point: (isize, isize), margin: isize) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<u128, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<u128, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<u128, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u128, CS>::does_area_contain_point(&self, point: (u128, u128)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u128, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<u128, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u128, CS>::get_edge_position(&self, point: (u128, u128), margin: u128) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<u128, CS>::is_point_at_edge(&self, point: (u128, u128), margin: u128) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<u16, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<u16, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<u16, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u16, CS>::does_area_contain_point(&self, point: (u16, u16)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u16, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<u16, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u16, CS>::get_edge_position(&self, point: (u16, u16), margin: u16) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<u16, CS>::is_point_at_edge(&self, point: (u16, u16), margin: u16) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<u32, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<u32, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<u32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u32, CS>::does_area_contain_point(&self, point: (u32, u32)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u32, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<u32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u32, CS>::get_edge_position(&self, point: (u32, u32), margin: u32) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<u32, CS>::is_point_at_edge(&self, point: (u32, u32), margin: u32) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<u64, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<u64, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<u64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u64, CS>::does_area_contain_point(&self, point: (u64, u64)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u64, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<u64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u64, CS>::get_edge_position(&self, point: (u64, u64), margin: u64) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<u64, CS>::is_point_at_edge(&self, point: (u64, u64), margin: u64) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<u8, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<u8, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<u8, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u8, CS>::does_area_contain_point(&self, point: (u8, u8)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u8, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<u8, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u8, CS>::get_edge_position(&self, point: (u8, u8), margin: u8) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<u8, CS>::is_point_at_edge(&self, point: (u8, u8), margin: u8) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<usize, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<usize, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<usize, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<usize, CS>::does_area_contain_point(&self, point: (usize, usize)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<usize, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<usize, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<usize, CS>::get_edge_position(&self, point: (usize, usize), margin: usize) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<usize, CS>::is_point_at_edge(&self, point: (usize, usize), margin: usize) -> bool
impl<T, const CS: bool> core::marker::StructuralPartialEq for mirl::math::collision::rectangle::Rectangle<T, CS>
impl<T: core::clone::Clone, const CS: bool> core::clone::Clone for mirl::math::collision::rectangle::Rectangle<T, CS>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::clone(&self) -> mirl::math::collision::rectangle::Rectangle<T, CS>
impl<T: core::cmp::Eq, const CS: bool> core::cmp::Eq for mirl::math::collision::rectangle::Rectangle<T, CS>
impl<T: core::cmp::PartialEq, const CS: bool> core::cmp::PartialEq for mirl::math::collision::rectangle::Rectangle<T, CS>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::eq(&self, other: &mirl::math::collision::rectangle::Rectangle<T, CS>) -> bool
impl<T: core::fmt::Debug, const CS: bool> core::fmt::Debug for mirl::math::collision::rectangle::Rectangle<T, CS>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<T: core::marker::Copy, const CS: bool> core::marker::Copy for mirl::math::collision::rectangle::Rectangle<T, CS>
impl<T: core::ops::arith::Add<Output = T> + core::marker::Copy + core::cmp::PartialOrd, const CS: bool> core::convert::From<(T, T, T, T)> for mirl::math::collision::rectangle::Rectangle<T, CS>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::from(bush: (T, T, T, T)) -> Self
impl<T, const CS: bool> core::marker::Freeze for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::marker::Freeze
impl<T, const CS: bool> core::marker::Send for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::marker::Send
impl<T, const CS: bool> core::marker::Sync for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::marker::Sync
impl<T, const CS: bool> core::marker::Unpin for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::marker::Unpin
impl<T, const CS: bool> core::panic::unwind_safe::RefUnwindSafe for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::panic::unwind_safe::RefUnwindSafe
impl<T, const CS: bool> core::panic::unwind_safe::UnwindSafe for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::panic::unwind_safe::UnwindSafe
impl<Q, K> equivalent::Equivalent<K> for mirl::math::collision::rectangle::Rectangle<T, CS> where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::math::collision::rectangle::Rectangle<T, CS> where U: core::convert::From<T>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::math::collision::rectangle::Rectangle<T, CS> where U: core::convert::Into<T>
pub type mirl::math::collision::rectangle::Rectangle<T, CS>::Error = core::convert::Infallible
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::math::collision::rectangle::Rectangle<T, CS> where U: core::convert::TryFrom<T>
pub type mirl::math::collision::rectangle::Rectangle<T, CS>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::clone::Clone
pub type mirl::math::collision::rectangle::Rectangle<T, CS>::Owned = T
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::clone_into(&self, target: &mut T)
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::to_owned(&self) -> T
impl<T> core::any::Any for mirl::math::collision::rectangle::Rectangle<T, CS> where T: 'static + ?core::marker::Sized
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::math::collision::rectangle::Rectangle<T, CS> where T: ?core::marker::Sized
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::math::collision::rectangle::Rectangle<T, CS> where T: ?core::marker::Sized
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::clone::Clone
pub unsafe fn mirl::math::collision::rectangle::Rectangle<T, CS>::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::math::collision::rectangle::Rectangle<T, CS>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::math::collision::rectangle::Rectangle<T, CS>
pub type mirl::math::collision::rectangle::Rectangle<T, CS>::Init = T
pub const mirl::math::collision::rectangle::Rectangle<T, CS>::ALIGN: usize
pub unsafe fn mirl::math::collision::rectangle::Rectangle<T, CS>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::math::collision::rectangle::Rectangle<T, CS>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::math::collision::rectangle::Rectangle<T, CS>::drop(ptr: usize)
pub unsafe fn mirl::math::collision::rectangle::Rectangle<T, CS>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::math::collision::rectangle::Rectangle<T, CS>
impl<T> mirl::extensions::RepeatData for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::clone::Clone
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::math::collision::Circle<T, const CS: bool>
pub mirl::math::collision::Circle::half_radius: T
pub mirl::math::collision::Circle::radius: T
pub mirl::math::collision::Circle::x: T
pub mirl::math::collision::Circle::y: T
impl<const CS: bool, T: mirl::math::NumberWithMonotoneOps + core::marker::Copy + mirl::math::ConvenientOps> mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::new(x: T, y: T, radius: T) -> Self
impl<const CS: bool, T: mirl::math::NumberWithMonotoneOps + core::marker::Copy> mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::does_area_contain_point(&self, point: (T, T)) -> bool
impl<T, const CS: bool> core::marker::StructuralPartialEq for mirl::math::collision::circle::Circle<T, CS>
impl<T: core::clone::Clone, const CS: bool> core::clone::Clone for mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::clone(&self) -> mirl::math::collision::circle::Circle<T, CS>
impl<T: core::cmp::Eq, const CS: bool> core::cmp::Eq for mirl::math::collision::circle::Circle<T, CS>
impl<T: core::cmp::PartialEq, const CS: bool> core::cmp::PartialEq for mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::eq(&self, other: &mirl::math::collision::circle::Circle<T, CS>) -> bool
impl<T: core::fmt::Debug, const CS: bool> core::fmt::Debug for mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<T: core::marker::Copy, const CS: bool> core::marker::Copy for mirl::math::collision::circle::Circle<T, CS>
impl<T, const CS: bool> core::marker::Freeze for mirl::math::collision::circle::Circle<T, CS> where T: core::marker::Freeze
impl<T, const CS: bool> core::marker::Send for mirl::math::collision::circle::Circle<T, CS> where T: core::marker::Send
impl<T, const CS: bool> core::marker::Sync for mirl::math::collision::circle::Circle<T, CS> where T: core::marker::Sync
impl<T, const CS: bool> core::marker::Unpin for mirl::math::collision::circle::Circle<T, CS> where T: core::marker::Unpin
impl<T, const CS: bool> core::panic::unwind_safe::RefUnwindSafe for mirl::math::collision::circle::Circle<T, CS> where T: core::panic::unwind_safe::RefUnwindSafe
impl<T, const CS: bool> core::panic::unwind_safe::UnwindSafe for mirl::math::collision::circle::Circle<T, CS> where T: core::panic::unwind_safe::UnwindSafe
impl<Q, K> equivalent::Equivalent<K> for mirl::math::collision::circle::Circle<T, CS> where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::math::collision::circle::Circle<T, CS>::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::math::collision::circle::Circle<T, CS> where U: core::convert::From<T>
pub fn mirl::math::collision::circle::Circle<T, CS>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::math::collision::circle::Circle<T, CS> where U: core::convert::Into<T>
pub type mirl::math::collision::circle::Circle<T, CS>::Error = core::convert::Infallible
pub fn mirl::math::collision::circle::Circle<T, CS>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::math::collision::circle::Circle<T, CS> where U: core::convert::TryFrom<T>
pub type mirl::math::collision::circle::Circle<T, CS>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::math::collision::circle::Circle<T, CS>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::math::collision::circle::Circle<T, CS> where T: core::clone::Clone
pub type mirl::math::collision::circle::Circle<T, CS>::Owned = T
pub fn mirl::math::collision::circle::Circle<T, CS>::clone_into(&self, target: &mut T)
pub fn mirl::math::collision::circle::Circle<T, CS>::to_owned(&self) -> T
impl<T> core::any::Any for mirl::math::collision::circle::Circle<T, CS> where T: 'static + ?core::marker::Sized
pub fn mirl::math::collision::circle::Circle<T, CS>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::math::collision::circle::Circle<T, CS> where T: ?core::marker::Sized
pub fn mirl::math::collision::circle::Circle<T, CS>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::math::collision::circle::Circle<T, CS> where T: ?core::marker::Sized
pub fn mirl::math::collision::circle::Circle<T, CS>::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::math::collision::circle::Circle<T, CS> where T: core::clone::Clone
pub unsafe fn mirl::math::collision::circle::Circle<T, CS>::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::math::collision::circle::Circle<T, CS>
pub fn mirl::math::collision::circle::Circle<T, CS>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::math::collision::circle::Circle<T, CS>
pub type mirl::math::collision::circle::Circle<T, CS>::Init = T
pub const mirl::math::collision::circle::Circle<T, CS>::ALIGN: usize
pub unsafe fn mirl::math::collision::circle::Circle<T, CS>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::math::collision::circle::Circle<T, CS>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::math::collision::circle::Circle<T, CS>::drop(ptr: usize)
pub unsafe fn mirl::math::collision::circle::Circle<T, CS>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::math::collision::circle::Circle<T, CS>
impl<T> mirl::extensions::RepeatData for mirl::math::collision::circle::Circle<T, CS> where T: core::clone::Clone
pub fn mirl::math::collision::circle::Circle<T, CS>::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::math::collision::Rectangle<T, const CS: bool>
pub mirl::math::collision::Rectangle::height: T
pub mirl::math::collision::Rectangle::width: T
pub mirl::math::collision::Rectangle::x: T
pub mirl::math::collision::Rectangle::y: T
impl<T, const BOTTOM_HIGHER: bool> mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER> where T: core::ops::arith::Add<Output = T> + core::cmp::PartialOrd + core::marker::Copy
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::bottom(&self) -> T where T: core::ops::arith::Add<Output = T> + core::marker::Copy
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::get_height(&self) -> T
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::get_width(&self) -> T
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::get_x(&self) -> T
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::get_y(&self) -> T
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::left(&self) -> T
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::new(x: T, y: T, width: T, height: T) -> Self
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::right(&self) -> T where T: core::ops::arith::Add<Output = T> + core::marker::Copy
pub const fn mirl::math::collision::rectangle::Rectangle<T, BOTTOM_HIGHER>::top(&self) -> T where T: core::ops::arith::Add<Output = T> + core::marker::Copy
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<f32, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<f32, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<f32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f32, CS>::does_area_contain_point(&self, point: (f32, f32)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f32, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<f32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f32, CS>::get_edge_position(&self, point: (f32, f32), margin: f32) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<f32, CS>::is_point_at_edge(&self, point: (f32, f32), margin: f32) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<f64, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<f64, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<f64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f64, CS>::does_area_contain_point(&self, point: (f64, f64)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f64, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<f64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<f64, CS>::get_edge_position(&self, point: (f64, f64), margin: f64) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<f64, CS>::is_point_at_edge(&self, point: (f64, f64), margin: f64) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<i128, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<i128, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<i128, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i128, CS>::does_area_contain_point(&self, point: (i128, i128)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i128, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<i128, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i128, CS>::get_edge_position(&self, point: (i128, i128), margin: i128) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<i128, CS>::is_point_at_edge(&self, point: (i128, i128), margin: i128) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<i16, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<i16, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<i16, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i16, CS>::does_area_contain_point(&self, point: (i16, i16)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i16, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<i16, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i16, CS>::get_edge_position(&self, point: (i16, i16), margin: i16) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<i16, CS>::is_point_at_edge(&self, point: (i16, i16), margin: i16) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<i32, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<i32, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<i32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i32, CS>::does_area_contain_point(&self, point: (i32, i32)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i32, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<i32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i32, CS>::get_edge_position(&self, point: (i32, i32), margin: i32) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<i32, CS>::is_point_at_edge(&self, point: (i32, i32), margin: i32) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<i64, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<i64, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<i64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i64, CS>::does_area_contain_point(&self, point: (i64, i64)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i64, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<i64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i64, CS>::get_edge_position(&self, point: (i64, i64), margin: i64) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<i64, CS>::is_point_at_edge(&self, point: (i64, i64), margin: i64) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<i8, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<i8, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<i8, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i8, CS>::does_area_contain_point(&self, point: (i8, i8)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i8, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<i8, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<i8, CS>::get_edge_position(&self, point: (i8, i8), margin: i8) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<i8, CS>::is_point_at_edge(&self, point: (i8, i8), margin: i8) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<isize, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<isize, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<isize, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<isize, CS>::does_area_contain_point(&self, point: (isize, isize)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<isize, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<isize, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<isize, CS>::get_edge_position(&self, point: (isize, isize), margin: isize) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<isize, CS>::is_point_at_edge(&self, point: (isize, isize), margin: isize) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<u128, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<u128, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<u128, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u128, CS>::does_area_contain_point(&self, point: (u128, u128)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u128, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<u128, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u128, CS>::get_edge_position(&self, point: (u128, u128), margin: u128) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<u128, CS>::is_point_at_edge(&self, point: (u128, u128), margin: u128) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<u16, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<u16, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<u16, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u16, CS>::does_area_contain_point(&self, point: (u16, u16)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u16, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<u16, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u16, CS>::get_edge_position(&self, point: (u16, u16), margin: u16) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<u16, CS>::is_point_at_edge(&self, point: (u16, u16), margin: u16) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<u32, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<u32, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<u32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u32, CS>::does_area_contain_point(&self, point: (u32, u32)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u32, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<u32, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u32, CS>::get_edge_position(&self, point: (u32, u32), margin: u32) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<u32, CS>::is_point_at_edge(&self, point: (u32, u32), margin: u32) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<u64, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<u64, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<u64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u64, CS>::does_area_contain_point(&self, point: (u64, u64)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u64, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<u64, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u64, CS>::get_edge_position(&self, point: (u64, u64), margin: u64) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<u64, CS>::is_point_at_edge(&self, point: (u64, u64), margin: u64) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<u8, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<u8, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<u8, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u8, CS>::does_area_contain_point(&self, point: (u8, u8)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u8, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<u8, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<u8, CS>::get_edge_position(&self, point: (u8, u8), margin: u8) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<u8, CS>::is_point_at_edge(&self, point: (u8, u8), margin: u8) -> bool
impl<const CS: bool> mirl::math::collision::rectangle::Rectangle<usize, CS>
pub const fn mirl::math::collision::rectangle::Rectangle<usize, CS>::do_areas_intersect(&self, smaller: &mirl::math::collision::rectangle::Rectangle<usize, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<usize, CS>::does_area_contain_point(&self, point: (usize, usize)) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<usize, CS>::does_area_fully_include_other_area(&self, other: &mirl::math::collision::rectangle::Rectangle<usize, CS>) -> bool
pub const fn mirl::math::collision::rectangle::Rectangle<usize, CS>::get_edge_position(&self, point: (usize, usize), margin: usize) -> u8
pub const fn mirl::math::collision::rectangle::Rectangle<usize, CS>::is_point_at_edge(&self, point: (usize, usize), margin: usize) -> bool
impl<T, const CS: bool> core::marker::StructuralPartialEq for mirl::math::collision::rectangle::Rectangle<T, CS>
impl<T: core::clone::Clone, const CS: bool> core::clone::Clone for mirl::math::collision::rectangle::Rectangle<T, CS>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::clone(&self) -> mirl::math::collision::rectangle::Rectangle<T, CS>
impl<T: core::cmp::Eq, const CS: bool> core::cmp::Eq for mirl::math::collision::rectangle::Rectangle<T, CS>
impl<T: core::cmp::PartialEq, const CS: bool> core::cmp::PartialEq for mirl::math::collision::rectangle::Rectangle<T, CS>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::eq(&self, other: &mirl::math::collision::rectangle::Rectangle<T, CS>) -> bool
impl<T: core::fmt::Debug, const CS: bool> core::fmt::Debug for mirl::math::collision::rectangle::Rectangle<T, CS>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<T: core::marker::Copy, const CS: bool> core::marker::Copy for mirl::math::collision::rectangle::Rectangle<T, CS>
impl<T: core::ops::arith::Add<Output = T> + core::marker::Copy + core::cmp::PartialOrd, const CS: bool> core::convert::From<(T, T, T, T)> for mirl::math::collision::rectangle::Rectangle<T, CS>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::from(bush: (T, T, T, T)) -> Self
impl<T, const CS: bool> core::marker::Freeze for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::marker::Freeze
impl<T, const CS: bool> core::marker::Send for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::marker::Send
impl<T, const CS: bool> core::marker::Sync for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::marker::Sync
impl<T, const CS: bool> core::marker::Unpin for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::marker::Unpin
impl<T, const CS: bool> core::panic::unwind_safe::RefUnwindSafe for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::panic::unwind_safe::RefUnwindSafe
impl<T, const CS: bool> core::panic::unwind_safe::UnwindSafe for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::panic::unwind_safe::UnwindSafe
impl<Q, K> equivalent::Equivalent<K> for mirl::math::collision::rectangle::Rectangle<T, CS> where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::math::collision::rectangle::Rectangle<T, CS> where U: core::convert::From<T>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::math::collision::rectangle::Rectangle<T, CS> where U: core::convert::Into<T>
pub type mirl::math::collision::rectangle::Rectangle<T, CS>::Error = core::convert::Infallible
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::math::collision::rectangle::Rectangle<T, CS> where U: core::convert::TryFrom<T>
pub type mirl::math::collision::rectangle::Rectangle<T, CS>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::clone::Clone
pub type mirl::math::collision::rectangle::Rectangle<T, CS>::Owned = T
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::clone_into(&self, target: &mut T)
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::to_owned(&self) -> T
impl<T> core::any::Any for mirl::math::collision::rectangle::Rectangle<T, CS> where T: 'static + ?core::marker::Sized
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::math::collision::rectangle::Rectangle<T, CS> where T: ?core::marker::Sized
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::math::collision::rectangle::Rectangle<T, CS> where T: ?core::marker::Sized
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::clone::Clone
pub unsafe fn mirl::math::collision::rectangle::Rectangle<T, CS>::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::math::collision::rectangle::Rectangle<T, CS>
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::math::collision::rectangle::Rectangle<T, CS>
pub type mirl::math::collision::rectangle::Rectangle<T, CS>::Init = T
pub const mirl::math::collision::rectangle::Rectangle<T, CS>::ALIGN: usize
pub unsafe fn mirl::math::collision::rectangle::Rectangle<T, CS>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::math::collision::rectangle::Rectangle<T, CS>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::math::collision::rectangle::Rectangle<T, CS>::drop(ptr: usize)
pub unsafe fn mirl::math::collision::rectangle::Rectangle<T, CS>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::math::collision::rectangle::Rectangle<T, CS>
impl<T> mirl::extensions::RepeatData for mirl::math::collision::rectangle::Rectangle<T, CS> where T: core::clone::Clone
pub fn mirl::math::collision::rectangle::Rectangle<T, CS>::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub const mirl::math::collision::BOTTOM_HIGHER: bool
pub const mirl::math::collision::BOTTOM_LOWER: bool
pub struct mirl::math::UniformRange<T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy>
impl<T> mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T>
pub fn mirl::math::UniformRange<T>::from_f32(f: f32) -> Self
pub fn mirl::math::UniformRange<T>::from_f64(f: f64) -> Self
pub const fn mirl::math::UniformRange<T>::from_raw(value: T) -> Self
pub fn mirl::math::UniformRange<T>::one() -> Self
pub const fn mirl::math::UniformRange<T>::raw(&self) -> T
pub fn mirl::math::UniformRange<T>::saturating_add(self, other: Self) -> Self
pub fn mirl::math::UniformRange<T>::saturating_sub(self, other: Self) -> Self
pub fn mirl::math::UniformRange<T>::to_f32(&self) -> f32
pub fn mirl::math::UniformRange<T>::to_f64(&self) -> f64
pub fn mirl::math::UniformRange<T>::zero() -> Self
impl<T: core::clone::Clone + num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy> core::clone::Clone for mirl::math::UniformRange<T>
pub fn mirl::math::UniformRange<T>::clone(&self) -> mirl::math::UniformRange<T>
impl<T: core::cmp::Eq + num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy> core::cmp::Eq for mirl::math::UniformRange<T>
impl<T: core::cmp::Ord + num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy> core::cmp::Ord for mirl::math::UniformRange<T>
pub fn mirl::math::UniformRange<T>::cmp(&self, other: &mirl::math::UniformRange<T>) -> core::cmp::Ordering
impl<T: core::cmp::PartialEq + num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy> core::cmp::PartialEq for mirl::math::UniformRange<T>
pub fn mirl::math::UniformRange<T>::eq(&self, other: &mirl::math::UniformRange<T>) -> bool
impl<T: core::cmp::PartialOrd + num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy> core::cmp::PartialOrd for mirl::math::UniformRange<T>
pub fn mirl::math::UniformRange<T>::partial_cmp(&self, other: &mirl::math::UniformRange<T>) -> core::option::Option<core::cmp::Ordering>
impl<T: core::hash::Hash + num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy> core::hash::Hash for mirl::math::UniformRange<T>
pub fn mirl::math::UniformRange<T>::hash<__H: core::hash::Hasher>(&self, state: &mut __H)
impl<T: core::marker::Copy + num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy> core::marker::Copy for mirl::math::UniformRange<T>
impl<T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy> core::marker::StructuralPartialEq for mirl::math::UniformRange<T>
impl<T> core::fmt::Debug for mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T> + core::fmt::Debug
pub fn mirl::math::UniformRange<T>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<T> core::fmt::Display for mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T>
pub fn mirl::math::UniformRange<T>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<T> core::ops::arith::Add for mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T>
pub type mirl::math::UniformRange<T>::Output = mirl::math::UniformRange<T>
pub fn mirl::math::UniformRange<T>::add(self, rhs: Self) -> Self::Output
impl<T> core::ops::arith::AddAssign for mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T>
pub fn mirl::math::UniformRange<T>::add_assign(&mut self, rhs: Self)
impl<T> core::ops::arith::Div for mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T>
pub type mirl::math::UniformRange<T>::Output = mirl::math::UniformRange<T>
pub fn mirl::math::UniformRange<T>::div(self, rhs: Self) -> Self::Output
impl<T> core::ops::arith::DivAssign for mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T>
pub fn mirl::math::UniformRange<T>::div_assign(&mut self, rhs: Self)
impl<T> core::ops::arith::Mul for mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T>
pub type mirl::math::UniformRange<T>::Output = mirl::math::UniformRange<T>
pub fn mirl::math::UniformRange<T>::mul(self, rhs: Self) -> Self::Output
impl<T> core::ops::arith::MulAssign for mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T>
pub fn mirl::math::UniformRange<T>::mul_assign(&mut self, rhs: Self)
impl<T> core::ops::arith::Sub for mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T>
pub type mirl::math::UniformRange<T>::Output = mirl::math::UniformRange<T>
pub fn mirl::math::UniformRange<T>::sub(self, rhs: Self) -> Self::Output
impl<T> core::ops::arith::SubAssign for mirl::math::UniformRange<T> where T: num_traits::sign::Unsigned + num_traits::cast::NumCast + core::marker::Copy + num_traits::identities::Zero + num_traits::identities::One + core::cmp::PartialOrd + core::ops::arith::Add<Output = T> + core::ops::arith::Sub<Output = T> + core::ops::arith::Mul<Output = T> + core::ops::arith::Div<Output = T>
pub fn mirl::math::UniformRange<T>::sub_assign(&mut self, rhs: Self)
impl<T> core::marker::Freeze for mirl::math::UniformRange<T> where T: core::marker::Freeze
impl<T> core::marker::Send for mirl::math::UniformRange<T> where T: core::marker::Send
impl<T> core::marker::Sync for mirl::math::UniformRange<T> where T: core::marker::Sync
impl<T> core::marker::Unpin for mirl::math::UniformRange<T> where T: core::marker::Unpin
impl<T> core::panic::unwind_safe::RefUnwindSafe for mirl::math::UniformRange<T> where T: core::panic::unwind_safe::RefUnwindSafe
impl<T> core::panic::unwind_safe::UnwindSafe for mirl::math::UniformRange<T> where T: core::panic::unwind_safe::UnwindSafe
impl<Q, K> equivalent::Comparable<K> for mirl::math::UniformRange<T> where Q: core::cmp::Ord + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::math::UniformRange<T>::compare(&self, key: &K) -> core::cmp::Ordering
impl<Q, K> equivalent::Equivalent<K> for mirl::math::UniformRange<T> where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::math::UniformRange<T>::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::math::UniformRange<T> where U: core::convert::From<T>
pub fn mirl::math::UniformRange<T>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::math::UniformRange<T> where U: core::convert::Into<T>
pub type mirl::math::UniformRange<T>::Error = core::convert::Infallible
pub fn mirl::math::UniformRange<T>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::math::UniformRange<T> where U: core::convert::TryFrom<T>
pub type mirl::math::UniformRange<T>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::math::UniformRange<T>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::math::UniformRange<T> where T: core::clone::Clone
pub type mirl::math::UniformRange<T>::Owned = T
pub fn mirl::math::UniformRange<T>::clone_into(&self, target: &mut T)
pub fn mirl::math::UniformRange<T>::to_owned(&self) -> T
impl<T> alloc::string::ToString for mirl::math::UniformRange<T> where T: core::fmt::Display + ?core::marker::Sized
pub fn mirl::math::UniformRange<T>::to_string(&self) -> alloc::string::String
impl<T> core::any::Any for mirl::math::UniformRange<T> where T: 'static + ?core::marker::Sized
pub fn mirl::math::UniformRange<T>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::math::UniformRange<T> where T: ?core::marker::Sized
pub fn mirl::math::UniformRange<T>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::math::UniformRange<T> where T: ?core::marker::Sized
pub fn mirl::math::UniformRange<T>::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::math::UniformRange<T> where T: core::clone::Clone
pub unsafe fn mirl::math::UniformRange<T>::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::math::UniformRange<T>
pub fn mirl::math::UniformRange<T>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::math::UniformRange<T>
pub type mirl::math::UniformRange<T>::Init = T
pub const mirl::math::UniformRange<T>::ALIGN: usize
pub unsafe fn mirl::math::UniformRange<T>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::math::UniformRange<T>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::math::UniformRange<T>::drop(ptr: usize)
pub unsafe fn mirl::math::UniformRange<T>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::math::UniformRange<T>
impl<T> mirl::extensions::RepeatData for mirl::math::UniformRange<T> where T: core::clone::Clone
pub fn mirl::math::UniformRange<T>::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub trait mirl::math::ConstTwoTillTen
pub const mirl::math::ConstTwoTillTen::EIGHT: Self
pub const mirl::math::ConstTwoTillTen::FIVE: Self
pub const mirl::math::ConstTwoTillTen::FOUR: Self
pub const mirl::math::ConstTwoTillTen::NINE: Self
pub const mirl::math::ConstTwoTillTen::SEVEN: Self
pub const mirl::math::ConstTwoTillTen::SIX: Self
pub const mirl::math::ConstTwoTillTen::TEN: Self
pub const mirl::math::ConstTwoTillTen::THREE: Self
pub const mirl::math::ConstTwoTillTen::TWO: Self
impl<T: num_traits::identities::ConstOne + core::ops::arith::Add<Output = T>> mirl::math::ConstTwoTillTen for T
pub const T::EIGHT: Self
pub const T::FIVE: Self
pub const T::FOUR: Self
pub const T::NINE: Self
pub const T::SEVEN: Self
pub const T::SIX: Self
pub const T::TEN: Self
pub const T::THREE: Self
pub const T::TWO: Self
pub trait mirl::math::ConvenientOps: num_traits::bounds::UpperBounded + core::marker::Copy + core::cmp::PartialOrd
pub fn mirl::math::ConvenientOps::half(&self) -> Self
pub fn mirl::math::ConvenientOps::more_than_half(&self) -> bool
impl<T: mirl::math::TwoTillTen<T> + mirl::math::NumberWithMonotoneOps + core::marker::Copy + num_traits::bounds::UpperBounded + core::cmp::PartialOrd + num_traits::identities::One> mirl::math::ConvenientOps for T
pub fn T::half(&self) -> Self
pub trait mirl::math::NumberWithMonotoneOps: core::cmp::PartialOrd + num_traits::cast::NumCast + core::ops::arith::Add<Output = Self> + core::ops::arith::Sub<Output = Self> + core::ops::arith::Mul<Output = Self> + core::ops::arith::Div<Output = Self>
impl<T: core::cmp::PartialOrd + num_traits::cast::NumCast + core::ops::arith::Add<Output = Self> + core::ops::arith::Sub<Output = Self> + core::ops::arith::Mul<Output = Self> + core::ops::arith::Div<Output = Self>> mirl::math::NumberWithMonotoneOps for T
pub trait mirl::math::TwoTillTen<T: num_traits::identities::One + core::ops::arith::Add<Output = T>>
pub fn mirl::math::TwoTillTen::eight() -> T
pub fn mirl::math::TwoTillTen::five() -> T
pub fn mirl::math::TwoTillTen::four() -> T
pub fn mirl::math::TwoTillTen::nine() -> T
pub fn mirl::math::TwoTillTen::seven() -> T
pub fn mirl::math::TwoTillTen::six() -> T
pub fn mirl::math::TwoTillTen::ten() -> T
pub fn mirl::math::TwoTillTen::three() -> T
pub fn mirl::math::TwoTillTen::two() -> T
impl<T: num_traits::identities::One + core::ops::arith::Add<Output = T>> mirl::math::TwoTillTen<T> for T
pub trait mirl::math::UniformPreviousNext
pub fn mirl::math::UniformPreviousNext::biggest_smaller_than_one(&self) -> Self
pub fn mirl::math::UniformPreviousNext::smallest_bigger_than_zero(&self) -> Self
impl mirl::math::UniformPreviousNext for f32
pub fn f32::biggest_smaller_than_one(&self) -> Self
pub fn f32::smallest_bigger_than_zero(&self) -> Self
impl mirl::math::UniformPreviousNext for f64
pub fn f64::biggest_smaller_than_one(&self) -> Self
pub fn f64::smallest_bigger_than_zero(&self) -> Self
pub fn mirl::math::degrees<T: num_traits::float::Float>(angle_radians: T) -> T
pub fn mirl::math::get_center_position_of_object_for_object<T: core::ops::arith::Div<Output = T> + core::ops::arith::Sub<Output = T> + mirl::math::ConstTwoTillTen>(inner_width: T, inner_height: T, outer_width: T, outer_height: T) -> (T, T)
pub fn mirl::math::interpolate<T: mirl::math::NumberWithMonotoneOps + core::marker::Copy + num_traits::identities::One>(start: T, end: T, progress: T) -> T
pub fn mirl::math::normalize_vector<T: num_traits::float::Float>(x: T, y: T, z: T) -> (T, T, T)
pub fn mirl::math::radians<T: num_traits::float::Float>(angle_degrees: T) -> T
pub type mirl::math::UnitRangeU128 = mirl::math::UniformRange<u128>
pub type mirl::math::UnitRangeU16 = mirl::math::UniformRange<u16>
pub type mirl::math::UnitRangeU32 = mirl::math::UniformRange<u32>
pub type mirl::math::UnitRangeU64 = mirl::math::UniformRange<u64>
pub type mirl::math::UnitRangeU8 = mirl::math::UniformRange<u8>
pub type mirl::math::UnitRangeUsize = mirl::math::UniformRange<usize>
pub mod mirl::misc
pub mod mirl::misc::windows
pub fn mirl::misc::windows::get_actual_stack_info() -> core::result::Result<(), alloc::boxed::Box<dyn core::error::Error>>
pub fn mirl::misc::concatenate<A: core::convert::AsRef<str>, B: core::convert::AsRef<str>>(a: A, b: B) -> alloc::string::String
pub const fn mirl::misc::corner_type_and_delta_to_metric_change(corner: u8, mouse_pos_delta: (isize, isize)) -> (isize, isize, isize, isize)
pub const fn mirl::misc::corner_type_to_cursor_style(corner: u8) -> core::option::Option<mirl::platform::CursorStyle>
pub fn mirl::misc::hash_value<T: core::hash::Hash>(value: &T) -> u64
pub mod mirl::platform
pub mod mirl::platform::const_buffer
pub struct mirl::platform::const_buffer::ConstBuffer<const WIDTH: usize, const HEIGHT: usize>
pub mirl::platform::const_buffer::ConstBuffer::data: alloc::boxed::Box<[u32]>
pub mirl::platform::const_buffer::ConstBuffer::pointer: *mut u32
impl<const WIDTH: usize, const HEIGHT: usize> mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
pub const mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::TOTAL_SIZE: usize
impl<const WIDTH: usize, const HEIGHT: usize> core::cmp::Eq for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
impl<const WIDTH: usize, const HEIGHT: usize> core::cmp::PartialEq for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
pub fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::eq(&self, other: &mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>) -> bool
impl<const WIDTH: usize, const HEIGHT: usize> core::fmt::Debug for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
pub fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<const WIDTH: usize, const HEIGHT: usize> core::marker::StructuralPartialEq for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
impl<const WIDTH: usize, const HEIGHT: usize> core::marker::Freeze for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
impl<const WIDTH: usize, const HEIGHT: usize> !core::marker::Send for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
impl<const WIDTH: usize, const HEIGHT: usize> !core::marker::Sync for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
impl<const WIDTH: usize, const HEIGHT: usize> core::marker::Unpin for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
impl<const WIDTH: usize, const HEIGHT: usize> core::panic::unwind_safe::RefUnwindSafe for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
impl<const WIDTH: usize, const HEIGHT: usize> core::panic::unwind_safe::UnwindSafe for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT> where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT> where U: core::convert::From<T>
pub fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT> where U: core::convert::Into<T>
pub type mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::Error = core::convert::Infallible
pub fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT> where U: core::convert::TryFrom<T>
pub type mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> core::any::Any for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT> where T: 'static + ?core::marker::Sized
pub fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT> where T: ?core::marker::Sized
pub fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT> where T: ?core::marker::Sized
pub fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::borrow_mut(&mut self) -> &mut T
impl<T> core::convert::From<T> for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
pub fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
pub type mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::Init = T
pub const mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::ALIGN: usize
pub unsafe fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::drop(ptr: usize)
pub unsafe fn mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::const_buffer::ConstBuffer<WIDTH, HEIGHT>
pub mod mirl::platform::file_system
pub struct mirl::platform::file_system::FileData
pub mirl::platform::file_system::FileData::expected_data_type: mirl::platform::file_system::file_data::DataType
pub mirl::platform::file_system::FileData::raw_data: alloc::vec::Vec<u8>
impl mirl::platform::file_system::FileData
pub const fn mirl::platform::file_system::FileData::as_bytes(&self) -> &alloc::vec::Vec<u8>
pub fn mirl::platform::file_system::FileData::as_font(&self) -> core::result::Result<fontdue::font::Font, alloc::boxed::Box<dyn core::error::Error>>
pub fn mirl::platform::file_system::FileData::as_image(&self) -> core::result::Result<image::dynimage::DynamicImage, alloc::boxed::Box<dyn core::error::Error>>
pub fn mirl::platform::file_system::FileData::as_string(&self) -> core::result::Result<alloc::string::String, alloc::string::FromUtf8Error>
pub const fn mirl::platform::file_system::FileData::from_bytes(data: alloc::vec::Vec<u8>, expected_data_type: mirl::platform::file_system::file_data::DataType) -> Self
impl core::clone::Clone for mirl::platform::file_system::FileData
pub fn mirl::platform::file_system::FileData::clone(&self) -> mirl::platform::file_system::FileData
impl core::cmp::Eq for mirl::platform::file_system::FileData
impl core::cmp::PartialEq for mirl::platform::file_system::FileData
pub fn mirl::platform::file_system::FileData::eq(&self, other: &mirl::platform::file_system::FileData) -> bool
impl core::fmt::Debug for mirl::platform::file_system::FileData
pub fn mirl::platform::file_system::FileData::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::StructuralPartialEq for mirl::platform::file_system::FileData
impl core::marker::Freeze for mirl::platform::file_system::FileData
impl core::marker::Send for mirl::platform::file_system::FileData
impl core::marker::Sync for mirl::platform::file_system::FileData
impl core::marker::Unpin for mirl::platform::file_system::FileData
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::file_system::FileData
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::file_system::FileData
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::file_system::FileData where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::file_system::FileData::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::file_system::FileData where U: core::convert::From<T>
pub fn mirl::platform::file_system::FileData::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::file_system::FileData where U: core::convert::Into<T>
pub type mirl::platform::file_system::FileData::Error = core::convert::Infallible
pub fn mirl::platform::file_system::FileData::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::file_system::FileData where U: core::convert::TryFrom<T>
pub type mirl::platform::file_system::FileData::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::file_system::FileData::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::file_system::FileData where T: core::clone::Clone
pub type mirl::platform::file_system::FileData::Owned = T
pub fn mirl::platform::file_system::FileData::clone_into(&self, target: &mut T)
pub fn mirl::platform::file_system::FileData::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::file_system::FileData where T: 'static + ?core::marker::Sized
pub fn mirl::platform::file_system::FileData::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::file_system::FileData where T: ?core::marker::Sized
pub fn mirl::platform::file_system::FileData::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::file_system::FileData where T: ?core::marker::Sized
pub fn mirl::platform::file_system::FileData::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::file_system::FileData where T: core::clone::Clone
pub unsafe fn mirl::platform::file_system::FileData::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::file_system::FileData
pub fn mirl::platform::file_system::FileData::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::file_system::FileData
pub type mirl::platform::file_system::FileData::Init = T
pub const mirl::platform::file_system::FileData::ALIGN: usize
pub unsafe fn mirl::platform::file_system::FileData::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::file_system::FileData::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::file_system::FileData::drop(ptr: usize)
pub unsafe fn mirl::platform::file_system::FileData::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::file_system::FileData
impl<T> mirl::extensions::RepeatData for mirl::platform::file_system::FileData where T: core::clone::Clone
pub fn mirl::platform::file_system::FileData::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::platform::file_system::NativeFileSystem
impl core::clone::Clone for mirl::platform::file_system::NativeFileSystem
pub fn mirl::platform::file_system::NativeFileSystem::clone(&self) -> mirl::platform::file_system::NativeFileSystem
impl core::cmp::Eq for mirl::platform::file_system::NativeFileSystem
impl core::cmp::PartialEq for mirl::platform::file_system::NativeFileSystem
pub fn mirl::platform::file_system::NativeFileSystem::eq(&self, other: &mirl::platform::file_system::NativeFileSystem) -> bool
impl core::fmt::Debug for mirl::platform::file_system::NativeFileSystem
pub fn mirl::platform::file_system::NativeFileSystem::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::StructuralPartialEq for mirl::platform::file_system::NativeFileSystem
impl mirl::platform::file_system::FileSystem for mirl::platform::file_system::NativeFileSystem
pub fn mirl::platform::file_system::NativeFileSystem::does_file_exist(&self, path: &str) -> bool
pub fn mirl::platform::file_system::NativeFileSystem::get_file_contents(&self, path: &str) -> core::result::Result<mirl::platform::file_system::FileData, alloc::boxed::Box<dyn core::error::Error>>
pub fn mirl::platform::file_system::NativeFileSystem::get_files_in_folder(&self, path: &str) -> alloc::vec::Vec<alloc::string::String>
pub fn mirl::platform::file_system::NativeFileSystem::get_folders_in_folder(&self, path: &str) -> alloc::vec::Vec<alloc::string::String>
pub fn mirl::platform::file_system::NativeFileSystem::get_searched_folders(&self) -> alloc::vec::Vec<alloc::string::String>
pub fn mirl::platform::file_system::NativeFileSystem::join(&self, path1: &str, path2: &str) -> alloc::string::String
pub fn mirl::platform::file_system::NativeFileSystem::new(required_files: alloc::vec::Vec<&'static str>) -> core::result::Result<Self, alloc::boxed::Box<dyn core::error::Error>>
pub fn mirl::platform::file_system::NativeFileSystem::write_to_file(&self, path: &str, contents: &[u8]) -> std::io::error::Result<()>
impl core::marker::Freeze for mirl::platform::file_system::NativeFileSystem
impl core::marker::Send for mirl::platform::file_system::NativeFileSystem
impl core::marker::Sync for mirl::platform::file_system::NativeFileSystem
impl core::marker::Unpin for mirl::platform::file_system::NativeFileSystem
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::file_system::NativeFileSystem
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::file_system::NativeFileSystem
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::file_system::NativeFileSystem where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::file_system::NativeFileSystem::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::file_system::NativeFileSystem where U: core::convert::From<T>
pub fn mirl::platform::file_system::NativeFileSystem::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::file_system::NativeFileSystem where U: core::convert::Into<T>
pub type mirl::platform::file_system::NativeFileSystem::Error = core::convert::Infallible
pub fn mirl::platform::file_system::NativeFileSystem::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::file_system::NativeFileSystem where U: core::convert::TryFrom<T>
pub type mirl::platform::file_system::NativeFileSystem::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::file_system::NativeFileSystem::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::file_system::NativeFileSystem where T: core::clone::Clone
pub type mirl::platform::file_system::NativeFileSystem::Owned = T
pub fn mirl::platform::file_system::NativeFileSystem::clone_into(&self, target: &mut T)
pub fn mirl::platform::file_system::NativeFileSystem::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::file_system::NativeFileSystem where T: 'static + ?core::marker::Sized
pub fn mirl::platform::file_system::NativeFileSystem::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::file_system::NativeFileSystem where T: ?core::marker::Sized
pub fn mirl::platform::file_system::NativeFileSystem::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::file_system::NativeFileSystem where T: ?core::marker::Sized
pub fn mirl::platform::file_system::NativeFileSystem::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::file_system::NativeFileSystem where T: core::clone::Clone
pub unsafe fn mirl::platform::file_system::NativeFileSystem::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::file_system::NativeFileSystem
pub fn mirl::platform::file_system::NativeFileSystem::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::file_system::NativeFileSystem
pub type mirl::platform::file_system::NativeFileSystem::Init = T
pub const mirl::platform::file_system::NativeFileSystem::ALIGN: usize
pub unsafe fn mirl::platform::file_system::NativeFileSystem::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::file_system::NativeFileSystem::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::file_system::NativeFileSystem::drop(ptr: usize)
pub unsafe fn mirl::platform::file_system::NativeFileSystem::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::file_system::NativeFileSystem
impl<T> mirl::extensions::RepeatData for mirl::platform::file_system::NativeFileSystem where T: core::clone::Clone
pub fn mirl::platform::file_system::NativeFileSystem::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub trait mirl::platform::file_system::FileSystem
pub fn mirl::platform::file_system::FileSystem::does_file_exist(&self, path: &str) -> bool
pub fn mirl::platform::file_system::FileSystem::get_file_contents(&self, path: &str) -> core::result::Result<mirl::platform::file_system::FileData, alloc::boxed::Box<dyn core::error::Error>>
pub fn mirl::platform::file_system::FileSystem::get_files_in_folder(&self, path: &str) -> alloc::vec::Vec<alloc::string::String>
pub fn mirl::platform::file_system::FileSystem::get_folders_in_folder(&self, path: &str) -> alloc::vec::Vec<alloc::string::String>
pub fn mirl::platform::file_system::FileSystem::get_searched_folders(&self) -> alloc::vec::Vec<alloc::string::String>
pub fn mirl::platform::file_system::FileSystem::join(&self, path1: &str, path2: &str) -> alloc::string::String
pub fn mirl::platform::file_system::FileSystem::new(required_files: alloc::vec::Vec<&'static str>) -> core::result::Result<Self, alloc::boxed::Box<dyn core::error::Error>> where Self: core::marker::Sized
pub fn mirl::platform::file_system::FileSystem::write_to_file(&self, path: &str, contents: &[u8]) -> std::io::error::Result<()>
impl mirl::platform::file_system::FileSystem for mirl::platform::file_system::NativeFileSystem
pub fn mirl::platform::file_system::NativeFileSystem::does_file_exist(&self, path: &str) -> bool
pub fn mirl::platform::file_system::NativeFileSystem::get_file_contents(&self, path: &str) -> core::result::Result<mirl::platform::file_system::FileData, alloc::boxed::Box<dyn core::error::Error>>
pub fn mirl::platform::file_system::NativeFileSystem::get_files_in_folder(&self, path: &str) -> alloc::vec::Vec<alloc::string::String>
pub fn mirl::platform::file_system::NativeFileSystem::get_folders_in_folder(&self, path: &str) -> alloc::vec::Vec<alloc::string::String>
pub fn mirl::platform::file_system::NativeFileSystem::get_searched_folders(&self) -> alloc::vec::Vec<alloc::string::String>
pub fn mirl::platform::file_system::NativeFileSystem::join(&self, path1: &str, path2: &str) -> alloc::string::String
pub fn mirl::platform::file_system::NativeFileSystem::new(required_files: alloc::vec::Vec<&'static str>) -> core::result::Result<Self, alloc::boxed::Box<dyn core::error::Error>>
pub fn mirl::platform::file_system::NativeFileSystem::write_to_file(&self, path: &str, contents: &[u8]) -> std::io::error::Result<()>
pub fn mirl::platform::file_system::get_default_font(file_system: &dyn mirl::platform::file_system::FileSystem) -> core::result::Result<mirl::platform::file_system::FileData, alloc::boxed::Box<dyn core::error::Error>>
pub mod mirl::platform::framework_traits
pub enum mirl::platform::framework_traits::Errors
pub mirl::platform::framework_traits::Errors::BufferTooSmall((usize, usize))
impl core::clone::Clone for mirl::platform::framework_traits::Errors
pub fn mirl::platform::framework_traits::Errors::clone(&self) -> mirl::platform::framework_traits::Errors
impl core::cmp::PartialEq for mirl::platform::framework_traits::Errors
pub fn mirl::platform::framework_traits::Errors::eq(&self, other: &mirl::platform::framework_traits::Errors) -> bool
impl core::fmt::Debug for mirl::platform::framework_traits::Errors
pub fn mirl::platform::framework_traits::Errors::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::framework_traits::Errors
impl core::marker::StructuralPartialEq for mirl::platform::framework_traits::Errors
impl core::marker::Freeze for mirl::platform::framework_traits::Errors
impl core::marker::Send for mirl::platform::framework_traits::Errors
impl core::marker::Sync for mirl::platform::framework_traits::Errors
impl core::marker::Unpin for mirl::platform::framework_traits::Errors
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::framework_traits::Errors
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::framework_traits::Errors
impl<T, U> core::convert::Into<U> for mirl::platform::framework_traits::Errors where U: core::convert::From<T>
pub fn mirl::platform::framework_traits::Errors::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::framework_traits::Errors where U: core::convert::Into<T>
pub type mirl::platform::framework_traits::Errors::Error = core::convert::Infallible
pub fn mirl::platform::framework_traits::Errors::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::framework_traits::Errors where U: core::convert::TryFrom<T>
pub type mirl::platform::framework_traits::Errors::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::framework_traits::Errors::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::framework_traits::Errors where T: core::clone::Clone
pub type mirl::platform::framework_traits::Errors::Owned = T
pub fn mirl::platform::framework_traits::Errors::clone_into(&self, target: &mut T)
pub fn mirl::platform::framework_traits::Errors::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::framework_traits::Errors where T: 'static + ?core::marker::Sized
pub fn mirl::platform::framework_traits::Errors::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::framework_traits::Errors where T: ?core::marker::Sized
pub fn mirl::platform::framework_traits::Errors::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::framework_traits::Errors where T: ?core::marker::Sized
pub fn mirl::platform::framework_traits::Errors::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::framework_traits::Errors where T: core::clone::Clone
pub unsafe fn mirl::platform::framework_traits::Errors::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::framework_traits::Errors
pub fn mirl::platform::framework_traits::Errors::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::framework_traits::Errors
pub type mirl::platform::framework_traits::Errors::Init = T
pub const mirl::platform::framework_traits::Errors::ALIGN: usize
pub unsafe fn mirl::platform::framework_traits::Errors::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::framework_traits::Errors::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::framework_traits::Errors::drop(ptr: usize)
pub unsafe fn mirl::platform::framework_traits::Errors::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::framework_traits::Errors
impl<T> mirl::extensions::RepeatData for mirl::platform::framework_traits::Errors where T: core::clone::Clone
pub fn mirl::platform::framework_traits::Errors::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub trait mirl::platform::framework_traits::Control
pub fn mirl::platform::framework_traits::Control::get_position(&self) -> (isize, isize)
pub fn mirl::platform::framework_traits::Control::get_size(&self) -> (isize, isize)
pub fn mirl::platform::framework_traits::Control::move_window(&mut self, xy: (isize, isize))
pub fn mirl::platform::framework_traits::Control::set_position(&mut self, xy: (isize, isize))
pub fn mirl::platform::framework_traits::Control::set_size(&mut self, buffer: &mirl::platform::Buffer)
impl mirl::platform::framework_traits::Control for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::get_position(&self) -> (isize, isize)
pub fn mirl::platform::minifb::Framework::get_size(&self) -> (isize, isize)
pub fn mirl::platform::minifb::Framework::set_position(&mut self, xy: (isize, isize))
pub fn mirl::platform::minifb::Framework::set_size(&mut self, buffer: &mirl::platform::Buffer)
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::Control for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_position(&self) -> (isize, isize)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_size(&self) -> (isize, isize)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_position(&mut self, xy: (isize, isize))
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_size(&mut self, buffer: &mirl::platform::Buffer)
pub trait mirl::platform::framework_traits::ExtendedControl
pub fn mirl::platform::framework_traits::ExtendedControl::is_maximized(&self) -> bool
pub fn mirl::platform::framework_traits::ExtendedControl::is_minimized(&self) -> bool
pub fn mirl::platform::framework_traits::ExtendedControl::maximize(&mut self)
pub fn mirl::platform::framework_traits::ExtendedControl::minimize(&mut self)
pub fn mirl::platform::framework_traits::ExtendedControl::restore(&mut self)
pub fn mirl::platform::framework_traits::ExtendedControl::set_render_layer(&mut self, render_layer: mirl::platform::WindowLevel)
impl mirl::platform::framework_traits::ExtendedControl for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::is_maximized(&self) -> bool
pub fn mirl::platform::minifb::Framework::is_minimized(&self) -> bool
pub fn mirl::platform::minifb::Framework::maximize(&mut self)
pub fn mirl::platform::minifb::Framework::minimize(&mut self)
pub fn mirl::platform::minifb::Framework::restore(&mut self)
pub fn mirl::platform::minifb::Framework::set_render_layer(&mut self, level: mirl::platform::WindowLevel)
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::ExtendedControl for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::is_maximized(&self) -> bool
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::is_minimized(&self) -> bool
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::maximize(&mut self)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::minimize(&mut self)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::restore(&mut self)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_render_layer(&mut self, level: mirl::platform::WindowLevel)
pub trait mirl::platform::framework_traits::ExtendedFramework<MouseManagerScrollAccuracy: num_traits::float::Float>: mirl::platform::framework_traits::Framework + mirl::platform::framework_traits::ExtendedInput<MouseManagerScrollAccuracy> + mirl::platform::framework_traits::ExtendedWindow + mirl::platform::framework_traits::Control + mirl::platform::framework_traits::ExtendedControl
impl<T, MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::ExtendedFramework<MouseManagerScrollAccuracy> for T where T: mirl::platform::framework_traits::Framework + mirl::platform::framework_traits::ExtendedInput<MouseManagerScrollAccuracy> + mirl::platform::framework_traits::ExtendedWindow + mirl::platform::framework_traits::Control + mirl::platform::framework_traits::ExtendedControl
pub trait mirl::platform::framework_traits::ExtendedInput<MouseManagerScrollAccuracy: num_traits::float::Float>
pub fn mirl::platform::framework_traits::ExtendedInput::get_all_keys_down(&self) -> alloc::vec::Vec<mirl::platform::KeyCode>
pub fn mirl::platform::framework_traits::ExtendedInput::get_mouse_scroll(&self) -> core::option::Option<(MouseManagerScrollAccuracy, MouseManagerScrollAccuracy)>
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::ExtendedInput<MouseManagerScrollAccuracy> for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_all_keys_down(&self) -> alloc::vec::Vec<mirl::platform::KeyCode>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_mouse_scroll(&self) -> core::option::Option<(MouseManagerScrollAccuracy, MouseManagerScrollAccuracy)>
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::ExtendedInput<MouseManagerScrollAccuracy> for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::get_all_keys_down(&self) -> alloc::vec::Vec<mirl::platform::KeyCode>
pub fn mirl::platform::minifb::Framework::get_mouse_scroll(&self) -> core::option::Option<(MouseManagerScrollAccuracy, MouseManagerScrollAccuracy)>
pub trait mirl::platform::framework_traits::ExtendedTiming
pub fn mirl::platform::framework_traits::ExtendedTiming::set_target_fps(&mut self, fps: usize)
impl mirl::platform::framework_traits::ExtendedTiming for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::set_target_fps(&mut self, fps: usize)
pub trait mirl::platform::framework_traits::ExtendedWindow
pub fn mirl::platform::framework_traits::ExtendedWindow::get_window_handle(&self) -> raw_window_handle::RawWindowHandle
pub fn mirl::platform::framework_traits::ExtendedWindow::load_custom_cursor(&mut self, size: mirl::extensions::U2, main_color: u32, secondary_color: u32) -> mirl::platform::mouse::Cursors
pub fn mirl::platform::framework_traits::ExtendedWindow::set_cursor_style(&mut self, style: &mirl::platform::mouse::Cursor)
pub fn mirl::platform::framework_traits::ExtendedWindow::set_icon(&mut self, buffer: &[u32], width: u32, height: u32)
pub fn mirl::platform::framework_traits::ExtendedWindow::set_title(&mut self, title: &str)
impl mirl::platform::framework_traits::ExtendedWindow for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::get_window_handle(&self) -> raw_window_handle::RawWindowHandle
pub fn mirl::platform::minifb::Framework::load_custom_cursor(&mut self, size: mirl::extensions::U2, main_color: u32, secondary_color: u32) -> mirl::platform::mouse::Cursors
pub fn mirl::platform::minifb::Framework::set_cursor_style(&mut self, style: &mirl::platform::mouse::Cursor)
pub fn mirl::platform::minifb::Framework::set_icon(&mut self, buffer: &[u32], width: u32, height: u32)
pub fn mirl::platform::minifb::Framework::set_title(&mut self, title: &str)
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::ExtendedWindow for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_window_handle(&self) -> raw_window_handle::RawWindowHandle
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::load_custom_cursor(&mut self, size: mirl::extensions::U2, main_color: u32, secondary_color: u32) -> mirl::platform::mouse::Cursors
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_cursor_style(&mut self, style: &mirl::platform::mouse::Cursor)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_icon(&mut self, _buffer: &[u32], _width: u32, _height: u32)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_title(&mut self, title: &str)
pub trait mirl::platform::framework_traits::Framework: mirl::platform::framework_traits::Window + mirl::platform::framework_traits::Input + mirl::platform::framework_traits::Output + mirl::platform::framework_traits::Timing
impl<T: mirl::platform::framework_traits::Window + mirl::platform::framework_traits::Input + mirl::platform::framework_traits::Output + mirl::platform::framework_traits::Timing> mirl::platform::framework_traits::Framework for T
pub trait mirl::platform::framework_traits::Input
pub fn mirl::platform::framework_traits::Input::get_mouse_position(&self) -> core::option::Option<(isize, isize)>
pub fn mirl::platform::framework_traits::Input::is_key_down(&self, key: mirl::platform::KeyCode) -> bool
pub fn mirl::platform::framework_traits::Input::is_mouse_down(&self, button: mirl::platform::MouseButton) -> bool
impl mirl::platform::framework_traits::Input for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::get_mouse_position(&self) -> core::option::Option<(isize, isize)>
pub fn mirl::platform::minifb::Framework::is_key_down(&self, key: mirl::platform::KeyCode) -> bool
pub fn mirl::platform::minifb::Framework::is_mouse_down(&self, button: mirl::platform::MouseButton) -> bool
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::Input for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_mouse_position(&self) -> core::option::Option<(isize, isize)>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::is_key_down(&self, keycode: mirl::platform::KeyCode) -> bool
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::is_mouse_down(&self, button: mirl::platform::MouseButton) -> bool
pub trait mirl::platform::framework_traits::Output
pub fn mirl::platform::framework_traits::Output::log(&self, t: &str)
impl mirl::platform::framework_traits::Output for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::log(&self, t: &str)
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::Output for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::log(&self, t: &str)
pub trait mirl::platform::framework_traits::RelativeMousePos
pub fn mirl::platform::framework_traits::RelativeMousePos::get_mouse_position_relative(&self) -> core::option::Option<(isize, isize)>
impl<T: mirl::platform::framework_traits::Input + mirl::platform::framework_traits::Control> mirl::platform::framework_traits::RelativeMousePos for T
pub fn T::get_mouse_position_relative(&self) -> core::option::Option<(isize, isize)>
pub trait mirl::platform::framework_traits::Timing
pub fn mirl::platform::framework_traits::Timing::get_delta_time(&mut self) -> f64
pub fn mirl::platform::framework_traits::Timing::get_time(&self) -> alloc::boxed::Box<dyn mirl::platform::Time>
pub fn mirl::platform::framework_traits::Timing::sleep(&self, time: core::time::Duration)
impl mirl::platform::framework_traits::Timing for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::get_delta_time(&mut self) -> f64
pub fn mirl::platform::minifb::Framework::get_time(&self) -> alloc::boxed::Box<dyn mirl::platform::Time>
pub fn mirl::platform::minifb::Framework::sleep(&self, time: core::time::Duration)
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::Timing for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_delta_time(&mut self) -> f64
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_time(&self) -> alloc::boxed::Box<dyn mirl::platform::Time>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::sleep(&self, time: core::time::Duration)
pub trait mirl::platform::framework_traits::Window
pub fn mirl::platform::framework_traits::Window::clean_up(&self)
pub fn mirl::platform::framework_traits::Window::is_open(&self) -> bool
pub fn mirl::platform::framework_traits::Window::new(title: &str, settings: mirl::platform::WindowSettings) -> Self where Self: core::marker::Sized
pub fn mirl::platform::framework_traits::Window::update(&mut self, buffer: &[u32])
impl mirl::platform::framework_traits::Window for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::is_open(&self) -> bool
pub fn mirl::platform::minifb::Framework::new(title: &str, settings: mirl::platform::WindowSettings) -> Self
pub fn mirl::platform::minifb::Framework::update(&mut self, buffer: &[u32])
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::Window for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::clean_up(&self)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::is_open(&self) -> bool
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::new(title: &str, settings: mirl::platform::WindowSettings) -> Self
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::update(&mut self, buffer: &[u32])
pub mod mirl::platform::glfw
pub struct mirl::platform::glfw::Framework<MouseManagerScrollAccuracy: num_traits::float::Float>
impl<MouseManagerScrollAccuracy: core::fmt::Debug + num_traits::float::Float> core::fmt::Debug for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::Control for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_position(&self) -> (isize, isize)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_size(&self) -> (isize, isize)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_position(&mut self, xy: (isize, isize))
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_size(&mut self, buffer: &mirl::platform::Buffer)
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::ExtendedControl for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::is_maximized(&self) -> bool
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::is_minimized(&self) -> bool
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::maximize(&mut self)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::minimize(&mut self)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::restore(&mut self)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_render_layer(&mut self, level: mirl::platform::WindowLevel)
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::ExtendedInput<MouseManagerScrollAccuracy> for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_all_keys_down(&self) -> alloc::vec::Vec<mirl::platform::KeyCode>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_mouse_scroll(&self) -> core::option::Option<(MouseManagerScrollAccuracy, MouseManagerScrollAccuracy)>
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::ExtendedWindow for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_window_handle(&self) -> raw_window_handle::RawWindowHandle
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::load_custom_cursor(&mut self, size: mirl::extensions::U2, main_color: u32, secondary_color: u32) -> mirl::platform::mouse::Cursors
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_cursor_style(&mut self, style: &mirl::platform::mouse::Cursor)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_icon(&mut self, _buffer: &[u32], _width: u32, _height: u32)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::set_title(&mut self, title: &str)
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::Input for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_mouse_position(&self) -> core::option::Option<(isize, isize)>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::is_key_down(&self, keycode: mirl::platform::KeyCode) -> bool
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::is_mouse_down(&self, button: mirl::platform::MouseButton) -> bool
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::Output for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::log(&self, t: &str)
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::Timing for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_delta_time(&mut self) -> f64
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_time(&self) -> alloc::boxed::Box<dyn mirl::platform::Time>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::sleep(&self, time: core::time::Duration)
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::Window for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::clean_up(&self)
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::is_open(&self) -> bool
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::new(title: &str, settings: mirl::platform::WindowSettings) -> Self
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::update(&mut self, buffer: &[u32])
impl<MouseManagerScrollAccuracy> core::marker::Freeze for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where MouseManagerScrollAccuracy: core::marker::Freeze
impl<MouseManagerScrollAccuracy> !core::marker::Send for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
impl<MouseManagerScrollAccuracy> !core::marker::Sync for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
impl<MouseManagerScrollAccuracy> core::marker::Unpin for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where MouseManagerScrollAccuracy: core::marker::Unpin
impl<MouseManagerScrollAccuracy> core::panic::unwind_safe::RefUnwindSafe for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where MouseManagerScrollAccuracy: core::panic::unwind_safe::RefUnwindSafe
impl<MouseManagerScrollAccuracy> core::panic::unwind_safe::UnwindSafe for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where MouseManagerScrollAccuracy: core::panic::unwind_safe::UnwindSafe
impl<T, MouseManagerScrollAccuracy> mirl::platform::framework_traits::ExtendedFramework<MouseManagerScrollAccuracy> for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where MouseManagerScrollAccuracy: num_traits::float::Float, T: mirl::platform::framework_traits::Framework + mirl::platform::framework_traits::ExtendedInput<MouseManagerScrollAccuracy> + mirl::platform::framework_traits::ExtendedWindow + mirl::platform::framework_traits::Control + mirl::platform::framework_traits::ExtendedControl
impl<T, U> core::convert::Into<U> for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where U: core::convert::From<T>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where U: core::convert::Into<T>
pub type mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::Error = core::convert::Infallible
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where U: core::convert::TryFrom<T>
pub type mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> core::any::Any for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where T: 'static + ?core::marker::Sized
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where T: ?core::marker::Sized
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where T: ?core::marker::Sized
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::borrow_mut(&mut self) -> &mut T
impl<T> core::convert::From<T> for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
pub type mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::Init = T
pub const mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::ALIGN: usize
pub unsafe fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::drop(ptr: usize)
pub unsafe fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>
impl<T> mirl::platform::framework_traits::Framework for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where T: mirl::platform::framework_traits::Window + mirl::platform::framework_traits::Input + mirl::platform::framework_traits::Output + mirl::platform::framework_traits::Timing
impl<T> mirl::platform::framework_traits::RelativeMousePos for mirl::platform::glfw::Framework<MouseManagerScrollAccuracy> where T: mirl::platform::framework_traits::Input + mirl::platform::framework_traits::Control
pub fn mirl::platform::glfw::Framework<MouseManagerScrollAccuracy>::get_mouse_position_relative(&self) -> core::option::Option<(isize, isize)>
pub fn mirl::platform::glfw::get_native_window_handle_from_glfw(window: &glfw::Window) -> raw_window_handle::RawWindowHandle
pub mod mirl::platform::keyboard
pub const fn mirl::platform::keyboard::device_query_keycodes_to_mirls_keycode(dq_keycode: device_query::keymap::Keycode) -> mirl::platform::KeyCode
pub fn mirl::platform::keyboard::get_all_pressed_keys() -> alloc::vec::Vec<mirl::platform::KeyCode>
pub fn mirl::platform::keyboard::is_key_down(vk_code: mirl::platform::KeyCode) -> bool
pub fn mirl::platform::keyboard::is_key_down_raw(vk_code: u32) -> bool
pub fn mirl::platform::keyboard::is_key_pressed(keycode: mirl::platform::KeyCode) -> bool
pub fn mirl::platform::keyboard::keycode_to_vk_code(keycode: mirl::platform::KeyCode) -> u32
pub const fn mirl::platform::keyboard::mirl_keycode_to_device_query_keycode(keycode: mirl::platform::KeyCode) -> core::option::Option<device_query::keymap::Keycode>
pub const fn mirl::platform::keyboard::vk_code_to_keycode(vk_code: u32) -> mirl::platform::KeyCode
pub mod mirl::platform::minifb
pub struct mirl::platform::minifb::Framework
impl core::fmt::Debug for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl mirl::platform::framework_traits::Control for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::get_position(&self) -> (isize, isize)
pub fn mirl::platform::minifb::Framework::get_size(&self) -> (isize, isize)
pub fn mirl::platform::minifb::Framework::set_position(&mut self, xy: (isize, isize))
pub fn mirl::platform::minifb::Framework::set_size(&mut self, buffer: &mirl::platform::Buffer)
impl mirl::platform::framework_traits::ExtendedControl for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::is_maximized(&self) -> bool
pub fn mirl::platform::minifb::Framework::is_minimized(&self) -> bool
pub fn mirl::platform::minifb::Framework::maximize(&mut self)
pub fn mirl::platform::minifb::Framework::minimize(&mut self)
pub fn mirl::platform::minifb::Framework::restore(&mut self)
pub fn mirl::platform::minifb::Framework::set_render_layer(&mut self, level: mirl::platform::WindowLevel)
impl mirl::platform::framework_traits::ExtendedTiming for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::set_target_fps(&mut self, fps: usize)
impl mirl::platform::framework_traits::ExtendedWindow for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::get_window_handle(&self) -> raw_window_handle::RawWindowHandle
pub fn mirl::platform::minifb::Framework::load_custom_cursor(&mut self, size: mirl::extensions::U2, main_color: u32, secondary_color: u32) -> mirl::platform::mouse::Cursors
pub fn mirl::platform::minifb::Framework::set_cursor_style(&mut self, style: &mirl::platform::mouse::Cursor)
pub fn mirl::platform::minifb::Framework::set_icon(&mut self, buffer: &[u32], width: u32, height: u32)
pub fn mirl::platform::minifb::Framework::set_title(&mut self, title: &str)
impl mirl::platform::framework_traits::Input for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::get_mouse_position(&self) -> core::option::Option<(isize, isize)>
pub fn mirl::platform::minifb::Framework::is_key_down(&self, key: mirl::platform::KeyCode) -> bool
pub fn mirl::platform::minifb::Framework::is_mouse_down(&self, button: mirl::platform::MouseButton) -> bool
impl mirl::platform::framework_traits::Output for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::log(&self, t: &str)
impl mirl::platform::framework_traits::Timing for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::get_delta_time(&mut self) -> f64
pub fn mirl::platform::minifb::Framework::get_time(&self) -> alloc::boxed::Box<dyn mirl::platform::Time>
pub fn mirl::platform::minifb::Framework::sleep(&self, time: core::time::Duration)
impl mirl::platform::framework_traits::Window for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::is_open(&self) -> bool
pub fn mirl::platform::minifb::Framework::new(title: &str, settings: mirl::platform::WindowSettings) -> Self
pub fn mirl::platform::minifb::Framework::update(&mut self, buffer: &[u32])
impl<MouseManagerScrollAccuracy: num_traits::float::Float> mirl::platform::framework_traits::ExtendedInput<MouseManagerScrollAccuracy> for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::get_all_keys_down(&self) -> alloc::vec::Vec<mirl::platform::KeyCode>
pub fn mirl::platform::minifb::Framework::get_mouse_scroll(&self) -> core::option::Option<(MouseManagerScrollAccuracy, MouseManagerScrollAccuracy)>
impl core::marker::Freeze for mirl::platform::minifb::Framework
impl !core::marker::Send for mirl::platform::minifb::Framework
impl !core::marker::Sync for mirl::platform::minifb::Framework
impl core::marker::Unpin for mirl::platform::minifb::Framework
impl !core::panic::unwind_safe::RefUnwindSafe for mirl::platform::minifb::Framework
impl !core::panic::unwind_safe::UnwindSafe for mirl::platform::minifb::Framework
impl<T, MouseManagerScrollAccuracy> mirl::platform::framework_traits::ExtendedFramework<MouseManagerScrollAccuracy> for mirl::platform::minifb::Framework where MouseManagerScrollAccuracy: num_traits::float::Float, T: mirl::platform::framework_traits::Framework + mirl::platform::framework_traits::ExtendedInput<MouseManagerScrollAccuracy> + mirl::platform::framework_traits::ExtendedWindow + mirl::platform::framework_traits::Control + mirl::platform::framework_traits::ExtendedControl
impl<T, U> core::convert::Into<U> for mirl::platform::minifb::Framework where U: core::convert::From<T>
pub fn mirl::platform::minifb::Framework::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::minifb::Framework where U: core::convert::Into<T>
pub type mirl::platform::minifb::Framework::Error = core::convert::Infallible
pub fn mirl::platform::minifb::Framework::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::minifb::Framework where U: core::convert::TryFrom<T>
pub type mirl::platform::minifb::Framework::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::minifb::Framework::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> core::any::Any for mirl::platform::minifb::Framework where T: 'static + ?core::marker::Sized
pub fn mirl::platform::minifb::Framework::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::minifb::Framework where T: ?core::marker::Sized
pub fn mirl::platform::minifb::Framework::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::minifb::Framework where T: ?core::marker::Sized
pub fn mirl::platform::minifb::Framework::borrow_mut(&mut self) -> &mut T
impl<T> core::convert::From<T> for mirl::platform::minifb::Framework
pub fn mirl::platform::minifb::Framework::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::minifb::Framework
pub type mirl::platform::minifb::Framework::Init = T
pub const mirl::platform::minifb::Framework::ALIGN: usize
pub unsafe fn mirl::platform::minifb::Framework::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::minifb::Framework::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::minifb::Framework::drop(ptr: usize)
pub unsafe fn mirl::platform::minifb::Framework::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::minifb::Framework
impl<T> mirl::platform::framework_traits::Framework for mirl::platform::minifb::Framework where T: mirl::platform::framework_traits::Window + mirl::platform::framework_traits::Input + mirl::platform::framework_traits::Output + mirl::platform::framework_traits::Timing
impl<T> mirl::platform::framework_traits::RelativeMousePos for mirl::platform::minifb::Framework where T: mirl::platform::framework_traits::Input + mirl::platform::framework_traits::Control
pub fn mirl::platform::minifb::Framework::get_mouse_position_relative(&self) -> core::option::Option<(isize, isize)>
pub const fn mirl::platform::minifb::map_keycode_to_minifb(key: mirl::platform::KeyCode) -> minifb::key::Key
pub const fn mirl::platform::minifb::map_minifb_to_keycode(key: minifb::key::Key) -> mirl::platform::KeyCode
pub mod mirl::platform::mouse
pub mod mirl::platform::mouse::cursor_glfw
pub fn mirl::platform::mouse::cursor_glfw::load_base_cursor_with_file(cursor: mirl::platform::mouse::BaseCursor, size: mirl::extensions::U2, main_color: u32, secondary_color: u32, svg_data: alloc::string::String) -> mirl::platform::mouse::Cursor
pub mod mirl::platform::mouse::cursors_windows
pub fn mirl::platform::mouse::cursors_windows::load_base_cursor_with_file(cursor: mirl::platform::mouse::BaseCursor, size: mirl::extensions::U2, main_color: u32, secondary_color: u32, svg_data: alloc::string::String) -> mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::cursors_windows::set_cursor(cursor: &windows::Windows::Win32::UI::WindowsAndMessaging::HCURSOR)
pub unsafe fn mirl::platform::mouse::cursors_windows::subclass_window(handle: raw_window_handle::RawWindowHandle, cursor: mirl::platform::mouse::Cursor)
pub unsafe fn mirl::platform::mouse::cursors_windows::update_cursor(cursor: &windows::Windows::Win32::UI::WindowsAndMessaging::HCURSOR)
pub unsafe system fn mirl::platform::mouse::cursors_windows::wndproc(hwnd: windows::Windows::Win32::Foundation::HWND, msg: u32, wparam: windows::Windows::Win32::Foundation::WPARAM, lparam: windows::Windows::Win32::Foundation::LPARAM) -> windows::Windows::Win32::Foundation::LRESULT
pub mod mirl::platform::mouse::position
pub struct mirl::platform::mouse::position::MouseDelta
pub mirl::platform::mouse::position::MouseDelta::dx: i32
pub mirl::platform::mouse::position::MouseDelta::dy: i32
impl core::clone::Clone for mirl::platform::mouse::position::MouseDelta
pub fn mirl::platform::mouse::position::MouseDelta::clone(&self) -> mirl::platform::mouse::position::MouseDelta
impl core::default::Default for mirl::platform::mouse::position::MouseDelta
pub fn mirl::platform::mouse::position::MouseDelta::default() -> mirl::platform::mouse::position::MouseDelta
impl core::fmt::Debug for mirl::platform::mouse::position::MouseDelta
pub fn mirl::platform::mouse::position::MouseDelta::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::mouse::position::MouseDelta
impl core::marker::Freeze for mirl::platform::mouse::position::MouseDelta
impl core::marker::Send for mirl::platform::mouse::position::MouseDelta
impl core::marker::Sync for mirl::platform::mouse::position::MouseDelta
impl core::marker::Unpin for mirl::platform::mouse::position::MouseDelta
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::mouse::position::MouseDelta
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::mouse::position::MouseDelta
impl<R, P> lebe::io::ReadPrimitive<R> for mirl::platform::mouse::position::MouseDelta where R: std::io::Read + lebe::io::ReadEndian<P>, P: core::default::Default
impl<T, U> core::convert::Into<U> for mirl::platform::mouse::position::MouseDelta where U: core::convert::From<T>
pub fn mirl::platform::mouse::position::MouseDelta::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::mouse::position::MouseDelta where U: core::convert::Into<T>
pub type mirl::platform::mouse::position::MouseDelta::Error = core::convert::Infallible
pub fn mirl::platform::mouse::position::MouseDelta::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::mouse::position::MouseDelta where U: core::convert::TryFrom<T>
pub type mirl::platform::mouse::position::MouseDelta::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::mouse::position::MouseDelta::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::mouse::position::MouseDelta where T: core::clone::Clone
pub type mirl::platform::mouse::position::MouseDelta::Owned = T
pub fn mirl::platform::mouse::position::MouseDelta::clone_into(&self, target: &mut T)
pub fn mirl::platform::mouse::position::MouseDelta::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::mouse::position::MouseDelta where T: 'static + ?core::marker::Sized
pub fn mirl::platform::mouse::position::MouseDelta::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::mouse::position::MouseDelta where T: ?core::marker::Sized
pub fn mirl::platform::mouse::position::MouseDelta::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::mouse::position::MouseDelta where T: ?core::marker::Sized
pub fn mirl::platform::mouse::position::MouseDelta::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::mouse::position::MouseDelta where T: core::clone::Clone
pub unsafe fn mirl::platform::mouse::position::MouseDelta::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::mouse::position::MouseDelta
pub fn mirl::platform::mouse::position::MouseDelta::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::mouse::position::MouseDelta
pub type mirl::platform::mouse::position::MouseDelta::Init = T
pub const mirl::platform::mouse::position::MouseDelta::ALIGN: usize
pub unsafe fn mirl::platform::mouse::position::MouseDelta::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::mouse::position::MouseDelta::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::mouse::position::MouseDelta::drop(ptr: usize)
pub unsafe fn mirl::platform::mouse::position::MouseDelta::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::mouse::position::MouseDelta
impl<T> mirl::extensions::RepeatData for mirl::platform::mouse::position::MouseDelta where T: core::clone::Clone
pub fn mirl::platform::mouse::position::MouseDelta::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::platform::mouse::position::RawMouseInput
impl core::clone::Clone for mirl::platform::mouse::position::RawMouseInput
pub fn mirl::platform::mouse::position::RawMouseInput::clone(&self) -> mirl::platform::mouse::position::RawMouseInput
impl core::fmt::Debug for mirl::platform::mouse::position::RawMouseInput
pub fn mirl::platform::mouse::position::RawMouseInput::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::ops::drop::Drop for mirl::platform::mouse::position::RawMouseInput
pub fn mirl::platform::mouse::position::RawMouseInput::drop(&mut self)
impl mirl::platform::mouse::position::RawMouseInputTrait for mirl::platform::mouse::position::RawMouseInput
pub fn mirl::platform::mouse::position::RawMouseInput::get_delta(&self) -> (i32, i32)
pub fn mirl::platform::mouse::position::RawMouseInput::new(handle: raw_window_handle::RawWindowHandle) -> core::result::Result<Self, &'static str>
impl core::marker::Freeze for mirl::platform::mouse::position::RawMouseInput
impl !core::marker::Send for mirl::platform::mouse::position::RawMouseInput
impl !core::marker::Sync for mirl::platform::mouse::position::RawMouseInput
impl core::marker::Unpin for mirl::platform::mouse::position::RawMouseInput
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::mouse::position::RawMouseInput
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::mouse::position::RawMouseInput
impl<T, U> core::convert::Into<U> for mirl::platform::mouse::position::RawMouseInput where U: core::convert::From<T>
pub fn mirl::platform::mouse::position::RawMouseInput::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::mouse::position::RawMouseInput where U: core::convert::Into<T>
pub type mirl::platform::mouse::position::RawMouseInput::Error = core::convert::Infallible
pub fn mirl::platform::mouse::position::RawMouseInput::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::mouse::position::RawMouseInput where U: core::convert::TryFrom<T>
pub type mirl::platform::mouse::position::RawMouseInput::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::mouse::position::RawMouseInput::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::mouse::position::RawMouseInput where T: core::clone::Clone
pub type mirl::platform::mouse::position::RawMouseInput::Owned = T
pub fn mirl::platform::mouse::position::RawMouseInput::clone_into(&self, target: &mut T)
pub fn mirl::platform::mouse::position::RawMouseInput::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::mouse::position::RawMouseInput where T: 'static + ?core::marker::Sized
pub fn mirl::platform::mouse::position::RawMouseInput::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::mouse::position::RawMouseInput where T: ?core::marker::Sized
pub fn mirl::platform::mouse::position::RawMouseInput::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::mouse::position::RawMouseInput where T: ?core::marker::Sized
pub fn mirl::platform::mouse::position::RawMouseInput::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::mouse::position::RawMouseInput where T: core::clone::Clone
pub unsafe fn mirl::platform::mouse::position::RawMouseInput::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::mouse::position::RawMouseInput
pub fn mirl::platform::mouse::position::RawMouseInput::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::mouse::position::RawMouseInput
pub type mirl::platform::mouse::position::RawMouseInput::Init = T
pub const mirl::platform::mouse::position::RawMouseInput::ALIGN: usize
pub unsafe fn mirl::platform::mouse::position::RawMouseInput::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::mouse::position::RawMouseInput::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::mouse::position::RawMouseInput::drop(ptr: usize)
pub unsafe fn mirl::platform::mouse::position::RawMouseInput::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::mouse::position::RawMouseInput
impl<T> mirl::extensions::RepeatData for mirl::platform::mouse::position::RawMouseInput where T: core::clone::Clone
pub fn mirl::platform::mouse::position::RawMouseInput::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub trait mirl::platform::mouse::position::RawMouseInputTrait
pub fn mirl::platform::mouse::position::RawMouseInputTrait::get_delta(&self) -> (i32, i32)
pub fn mirl::platform::mouse::position::RawMouseInputTrait::new(handle: raw_window_handle::RawWindowHandle) -> core::result::Result<Self, &'static str> where Self: core::marker::Sized
impl mirl::platform::mouse::position::RawMouseInputTrait for mirl::platform::mouse::position::RawMouseInput
pub fn mirl::platform::mouse::position::RawMouseInput::get_delta(&self) -> (i32, i32)
pub fn mirl::platform::mouse::position::RawMouseInput::new(handle: raw_window_handle::RawWindowHandle) -> core::result::Result<Self, &'static str>
pub enum mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursor::Glfw((mirl::lists::VariableSizeList<u32>, u32, u32))
pub mirl::platform::mouse::Cursor::Win(windows::Windows::Win32::UI::WindowsAndMessaging::HCURSOR)
impl core::clone::Clone for mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::Cursor::clone(&self) -> mirl::platform::mouse::Cursor
impl core::cmp::Eq for mirl::platform::mouse::Cursor
impl core::cmp::PartialEq for mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::Cursor::eq(&self, other: &mirl::platform::mouse::Cursor) -> bool
impl core::fmt::Debug for mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::Cursor::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::mouse::Cursor
impl core::marker::StructuralPartialEq for mirl::platform::mouse::Cursor
impl core::marker::Freeze for mirl::platform::mouse::Cursor
impl core::marker::Send for mirl::platform::mouse::Cursor
impl core::marker::Sync for mirl::platform::mouse::Cursor
impl core::marker::Unpin for mirl::platform::mouse::Cursor
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::mouse::Cursor
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::mouse::Cursor
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::mouse::Cursor where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::mouse::Cursor::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::mouse::Cursor where U: core::convert::From<T>
pub fn mirl::platform::mouse::Cursor::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::mouse::Cursor where U: core::convert::Into<T>
pub type mirl::platform::mouse::Cursor::Error = core::convert::Infallible
pub fn mirl::platform::mouse::Cursor::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::mouse::Cursor where U: core::convert::TryFrom<T>
pub type mirl::platform::mouse::Cursor::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::mouse::Cursor::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::mouse::Cursor where T: core::clone::Clone
pub type mirl::platform::mouse::Cursor::Owned = T
pub fn mirl::platform::mouse::Cursor::clone_into(&self, target: &mut T)
pub fn mirl::platform::mouse::Cursor::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::mouse::Cursor where T: 'static + ?core::marker::Sized
pub fn mirl::platform::mouse::Cursor::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::mouse::Cursor where T: ?core::marker::Sized
pub fn mirl::platform::mouse::Cursor::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::mouse::Cursor where T: ?core::marker::Sized
pub fn mirl::platform::mouse::Cursor::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::mouse::Cursor where T: core::clone::Clone
pub unsafe fn mirl::platform::mouse::Cursor::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::Cursor::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::mouse::Cursor
pub type mirl::platform::mouse::Cursor::Init = T
pub const mirl::platform::mouse::Cursor::ALIGN: usize
pub unsafe fn mirl::platform::mouse::Cursor::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::mouse::Cursor::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::mouse::Cursor::drop(ptr: usize)
pub unsafe fn mirl::platform::mouse::Cursor::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::mouse::Cursor
impl<T> mirl::extensions::RepeatData for mirl::platform::mouse::Cursor where T: core::clone::Clone
pub fn mirl::platform::mouse::Cursor::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::platform::mouse::BaseCursor
impl core::clone::Clone for mirl::platform::mouse::BaseCursor
pub fn mirl::platform::mouse::BaseCursor::clone(&self) -> mirl::platform::mouse::BaseCursor
impl core::cmp::Eq for mirl::platform::mouse::BaseCursor
impl core::cmp::PartialEq for mirl::platform::mouse::BaseCursor
pub fn mirl::platform::mouse::BaseCursor::eq(&self, other: &mirl::platform::mouse::BaseCursor) -> bool
impl core::fmt::Debug for mirl::platform::mouse::BaseCursor
pub fn mirl::platform::mouse::BaseCursor::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::mouse::BaseCursor
impl core::marker::StructuralPartialEq for mirl::platform::mouse::BaseCursor
impl core::marker::Freeze for mirl::platform::mouse::BaseCursor
impl core::marker::Send for mirl::platform::mouse::BaseCursor
impl core::marker::Sync for mirl::platform::mouse::BaseCursor
impl core::marker::Unpin for mirl::platform::mouse::BaseCursor
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::mouse::BaseCursor
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::mouse::BaseCursor
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::mouse::BaseCursor where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::mouse::BaseCursor::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::mouse::BaseCursor where U: core::convert::From<T>
pub fn mirl::platform::mouse::BaseCursor::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::mouse::BaseCursor where U: core::convert::Into<T>
pub type mirl::platform::mouse::BaseCursor::Error = core::convert::Infallible
pub fn mirl::platform::mouse::BaseCursor::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::mouse::BaseCursor where U: core::convert::TryFrom<T>
pub type mirl::platform::mouse::BaseCursor::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::mouse::BaseCursor::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::mouse::BaseCursor where T: core::clone::Clone
pub type mirl::platform::mouse::BaseCursor::Owned = T
pub fn mirl::platform::mouse::BaseCursor::clone_into(&self, target: &mut T)
pub fn mirl::platform::mouse::BaseCursor::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::mouse::BaseCursor where T: 'static + ?core::marker::Sized
pub fn mirl::platform::mouse::BaseCursor::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::mouse::BaseCursor where T: ?core::marker::Sized
pub fn mirl::platform::mouse::BaseCursor::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::mouse::BaseCursor where T: ?core::marker::Sized
pub fn mirl::platform::mouse::BaseCursor::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::mouse::BaseCursor where T: core::clone::Clone
pub unsafe fn mirl::platform::mouse::BaseCursor::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::mouse::BaseCursor
pub fn mirl::platform::mouse::BaseCursor::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::mouse::BaseCursor
pub type mirl::platform::mouse::BaseCursor::Init = T
pub const mirl::platform::mouse::BaseCursor::ALIGN: usize
pub unsafe fn mirl::platform::mouse::BaseCursor::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::mouse::BaseCursor::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::mouse::BaseCursor::drop(ptr: usize)
pub unsafe fn mirl::platform::mouse::BaseCursor::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::mouse::BaseCursor
impl<T> mirl::extensions::RepeatData for mirl::platform::mouse::BaseCursor where T: core::clone::Clone
pub fn mirl::platform::mouse::BaseCursor::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::platform::mouse::Cursors
pub mirl::platform::mouse::Cursors::alias: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::all_scroll: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::arrow_bottom_right: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::arrow_down: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::arrow_left: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::arrow_right: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::arrow_top_left: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::arrow_top_right: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::arrow_up: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::bottom_left_corner: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::cell: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::centered_pointer: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::closed_hand: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::closed_hand_no_drop: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::col_resize: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::color_picker: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::context_menu: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::copy: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::crosshair: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::default: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::draft: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::fleur: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::help: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::mirrored_pointer: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::no_drop: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::not_allowed: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::open_hand: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::pencil: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::pirate: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::pointer: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::side_bottom: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::side_left: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::side_right: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::side_top: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::size_hor: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::size_nesw: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::size_nwse: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::size_ver: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::text: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::vertical_text: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::zoom_in: mirl::platform::mouse::Cursor
pub mirl::platform::mouse::Cursors::zoom_out: mirl::platform::mouse::Cursor
impl mirl::platform::mouse::Cursors
pub const fn mirl::platform::mouse::Cursors::from_cursor_style(&self, style: mirl::platform::CursorStyle) -> mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::Cursors::load<F>(size: mirl::extensions::U2, main_color: u32, secondary_color: u32, load_base_cursor_with_file: F) -> Self where F: core::ops::function::Fn(mirl::platform::mouse::BaseCursor, mirl::extensions::U2, u32, u32, alloc::string::String) -> mirl::platform::mouse::Cursor
impl core::clone::Clone for mirl::platform::mouse::Cursors
pub fn mirl::platform::mouse::Cursors::clone(&self) -> mirl::platform::mouse::Cursors
impl core::cmp::Eq for mirl::platform::mouse::Cursors
impl core::cmp::PartialEq for mirl::platform::mouse::Cursors
pub fn mirl::platform::mouse::Cursors::eq(&self, other: &mirl::platform::mouse::Cursors) -> bool
impl core::fmt::Debug for mirl::platform::mouse::Cursors
pub fn mirl::platform::mouse::Cursors::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::mouse::Cursors
impl core::marker::StructuralPartialEq for mirl::platform::mouse::Cursors
impl core::marker::Freeze for mirl::platform::mouse::Cursors
impl core::marker::Send for mirl::platform::mouse::Cursors
impl core::marker::Sync for mirl::platform::mouse::Cursors
impl core::marker::Unpin for mirl::platform::mouse::Cursors
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::mouse::Cursors
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::mouse::Cursors
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::mouse::Cursors where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::mouse::Cursors::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::mouse::Cursors where U: core::convert::From<T>
pub fn mirl::platform::mouse::Cursors::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::mouse::Cursors where U: core::convert::Into<T>
pub type mirl::platform::mouse::Cursors::Error = core::convert::Infallible
pub fn mirl::platform::mouse::Cursors::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::mouse::Cursors where U: core::convert::TryFrom<T>
pub type mirl::platform::mouse::Cursors::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::mouse::Cursors::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::mouse::Cursors where T: core::clone::Clone
pub type mirl::platform::mouse::Cursors::Owned = T
pub fn mirl::platform::mouse::Cursors::clone_into(&self, target: &mut T)
pub fn mirl::platform::mouse::Cursors::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::mouse::Cursors where T: 'static + ?core::marker::Sized
pub fn mirl::platform::mouse::Cursors::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::mouse::Cursors where T: ?core::marker::Sized
pub fn mirl::platform::mouse::Cursors::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::mouse::Cursors where T: ?core::marker::Sized
pub fn mirl::platform::mouse::Cursors::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::mouse::Cursors where T: core::clone::Clone
pub unsafe fn mirl::platform::mouse::Cursors::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::mouse::Cursors
pub fn mirl::platform::mouse::Cursors::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::mouse::Cursors
pub type mirl::platform::mouse::Cursors::Init = T
pub const mirl::platform::mouse::Cursors::ALIGN: usize
pub unsafe fn mirl::platform::mouse::Cursors::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::mouse::Cursors::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::mouse::Cursors::drop(ptr: usize)
pub unsafe fn mirl::platform::mouse::Cursors::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::mouse::Cursors
impl<T> mirl::extensions::RepeatData for mirl::platform::mouse::Cursors where T: core::clone::Clone
pub fn mirl::platform::mouse::Cursors::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub trait mirl::platform::mouse::CursorManager
pub fn mirl::platform::mouse::CursorManager::get_cursor(&self, name: &str) -> mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::CursorManager::load_builtin_cursors(&mut self, size: mirl::extensions::U2, main_color: u32, secondary_color: u32)
pub fn mirl::platform::mouse::CursorManager::load_cursor(&mut self, name: &str, size: mirl::extensions::U2, image_data: mirl::platform::Buffer, hotspot_x: u16, hotspot_y: u16)
pub fn mirl::platform::mouse::cursor_resolution(quality: mirl::extensions::U2) -> u8
pub fn mirl::platform::mouse::load_base_cursor_with_file(cursor: mirl::platform::mouse::BaseCursor, size: mirl::extensions::U2, main_color: u32, secondary_color: u32, svg_data: alloc::string::String) -> mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::resolution_to_quality(resolution: u8) -> core::result::Result<mirl::extensions::U2, &'static str>
pub fn mirl::platform::mouse::use_cursor(cursor: &mirl::platform::mouse::Cursor, glfw_window: core::option::Option<&mut glfw::Window>)
pub mod mirl::platform::shared
pub struct mirl::platform::shared::KeyManager
impl mirl::platform::shared::KeyManager
pub fn mirl::platform::shared::KeyManager::get_all_pressed_keys(&self) -> alloc::vec::Vec<mirl::platform::KeyCode>
pub fn mirl::platform::shared::KeyManager::is_key_pressed(&self, keycode: mirl::platform::KeyCode) -> bool
pub const fn mirl::platform::shared::KeyManager::new() -> Self
pub fn mirl::platform::shared::KeyManager::set_key_state(&mut self, keycode: mirl::platform::KeyCode, value: bool)
impl core::clone::Clone for mirl::platform::shared::KeyManager
pub fn mirl::platform::shared::KeyManager::clone(&self) -> mirl::platform::shared::KeyManager
impl core::cmp::PartialEq for mirl::platform::shared::KeyManager
pub fn mirl::platform::shared::KeyManager::eq(&self, other: &mirl::platform::shared::KeyManager) -> bool
impl core::fmt::Debug for mirl::platform::shared::KeyManager
pub fn mirl::platform::shared::KeyManager::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::shared::KeyManager
impl core::marker::StructuralPartialEq for mirl::platform::shared::KeyManager
impl core::marker::Freeze for mirl::platform::shared::KeyManager
impl core::marker::Send for mirl::platform::shared::KeyManager
impl core::marker::Sync for mirl::platform::shared::KeyManager
impl core::marker::Unpin for mirl::platform::shared::KeyManager
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::shared::KeyManager
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::shared::KeyManager
impl<T, U> core::convert::Into<U> for mirl::platform::shared::KeyManager where U: core::convert::From<T>
pub fn mirl::platform::shared::KeyManager::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::shared::KeyManager where U: core::convert::Into<T>
pub type mirl::platform::shared::KeyManager::Error = core::convert::Infallible
pub fn mirl::platform::shared::KeyManager::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::shared::KeyManager where U: core::convert::TryFrom<T>
pub type mirl::platform::shared::KeyManager::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::shared::KeyManager::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::shared::KeyManager where T: core::clone::Clone
pub type mirl::platform::shared::KeyManager::Owned = T
pub fn mirl::platform::shared::KeyManager::clone_into(&self, target: &mut T)
pub fn mirl::platform::shared::KeyManager::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::shared::KeyManager where T: 'static + ?core::marker::Sized
pub fn mirl::platform::shared::KeyManager::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::shared::KeyManager where T: ?core::marker::Sized
pub fn mirl::platform::shared::KeyManager::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::shared::KeyManager where T: ?core::marker::Sized
pub fn mirl::platform::shared::KeyManager::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::shared::KeyManager where T: core::clone::Clone
pub unsafe fn mirl::platform::shared::KeyManager::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::shared::KeyManager
pub fn mirl::platform::shared::KeyManager::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::shared::KeyManager
pub type mirl::platform::shared::KeyManager::Init = T
pub const mirl::platform::shared::KeyManager::ALIGN: usize
pub unsafe fn mirl::platform::shared::KeyManager::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::shared::KeyManager::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::shared::KeyManager::drop(ptr: usize)
pub unsafe fn mirl::platform::shared::KeyManager::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::shared::KeyManager
impl<T> mirl::extensions::RepeatData for mirl::platform::shared::KeyManager where T: core::clone::Clone
pub fn mirl::platform::shared::KeyManager::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::platform::shared::MouseManager<T: num_traits::float::Float>
impl<T: num_traits::float::Float> mirl::platform::shared::MouseManager<T>
pub fn mirl::platform::shared::MouseManager<T>::add_scroll(&mut self, xy: (T, T))
pub fn mirl::platform::shared::MouseManager<T>::get_scroll(&self) -> (T, T)
pub fn mirl::platform::shared::MouseManager<T>::is_mouse_button_pressed(&self, button: mirl::platform::MouseButton) -> bool
pub fn mirl::platform::shared::MouseManager<T>::new() -> Self
pub fn mirl::platform::shared::MouseManager<T>::reset_scroll(&mut self)
pub fn mirl::platform::shared::MouseManager<T>::set_mouse_button_state(&mut self, button: mirl::platform::MouseButton, value: bool)
pub fn mirl::platform::shared::MouseManager<T>::set_scroll(&mut self, xy: (T, T))
impl<T: core::clone::Clone + num_traits::float::Float> core::clone::Clone for mirl::platform::shared::MouseManager<T>
pub fn mirl::platform::shared::MouseManager<T>::clone(&self) -> mirl::platform::shared::MouseManager<T>
impl<T: core::cmp::Eq + num_traits::float::Float> core::cmp::Eq for mirl::platform::shared::MouseManager<T>
impl<T: core::cmp::PartialEq + num_traits::float::Float> core::cmp::PartialEq for mirl::platform::shared::MouseManager<T>
pub fn mirl::platform::shared::MouseManager<T>::eq(&self, other: &mirl::platform::shared::MouseManager<T>) -> bool
impl<T: core::fmt::Debug + num_traits::float::Float> core::fmt::Debug for mirl::platform::shared::MouseManager<T>
pub fn mirl::platform::shared::MouseManager<T>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<T: core::marker::Copy + num_traits::float::Float> core::marker::Copy for mirl::platform::shared::MouseManager<T>
impl<T: num_traits::float::Float> core::marker::StructuralPartialEq for mirl::platform::shared::MouseManager<T>
impl<T> core::marker::Freeze for mirl::platform::shared::MouseManager<T> where T: core::marker::Freeze
impl<T> core::marker::Send for mirl::platform::shared::MouseManager<T> where T: core::marker::Send
impl<T> core::marker::Sync for mirl::platform::shared::MouseManager<T> where T: core::marker::Sync
impl<T> core::marker::Unpin for mirl::platform::shared::MouseManager<T> where T: core::marker::Unpin
impl<T> core::panic::unwind_safe::RefUnwindSafe for mirl::platform::shared::MouseManager<T> where T: core::panic::unwind_safe::RefUnwindSafe
impl<T> core::panic::unwind_safe::UnwindSafe for mirl::platform::shared::MouseManager<T> where T: core::panic::unwind_safe::UnwindSafe
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::shared::MouseManager<T> where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::shared::MouseManager<T>::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::shared::MouseManager<T> where U: core::convert::From<T>
pub fn mirl::platform::shared::MouseManager<T>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::shared::MouseManager<T> where U: core::convert::Into<T>
pub type mirl::platform::shared::MouseManager<T>::Error = core::convert::Infallible
pub fn mirl::platform::shared::MouseManager<T>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::shared::MouseManager<T> where U: core::convert::TryFrom<T>
pub type mirl::platform::shared::MouseManager<T>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::shared::MouseManager<T>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::shared::MouseManager<T> where T: core::clone::Clone
pub type mirl::platform::shared::MouseManager<T>::Owned = T
pub fn mirl::platform::shared::MouseManager<T>::clone_into(&self, target: &mut T)
pub fn mirl::platform::shared::MouseManager<T>::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::shared::MouseManager<T> where T: 'static + ?core::marker::Sized
pub fn mirl::platform::shared::MouseManager<T>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::shared::MouseManager<T> where T: ?core::marker::Sized
pub fn mirl::platform::shared::MouseManager<T>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::shared::MouseManager<T> where T: ?core::marker::Sized
pub fn mirl::platform::shared::MouseManager<T>::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::shared::MouseManager<T> where T: core::clone::Clone
pub unsafe fn mirl::platform::shared::MouseManager<T>::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::shared::MouseManager<T>
pub fn mirl::platform::shared::MouseManager<T>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::shared::MouseManager<T>
pub type mirl::platform::shared::MouseManager<T>::Init = T
pub const mirl::platform::shared::MouseManager<T>::ALIGN: usize
pub unsafe fn mirl::platform::shared::MouseManager<T>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::shared::MouseManager<T>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::shared::MouseManager<T>::drop(ptr: usize)
pub unsafe fn mirl::platform::shared::MouseManager<T>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::shared::MouseManager<T>
impl<T> mirl::extensions::RepeatData for mirl::platform::shared::MouseManager<T> where T: core::clone::Clone
pub fn mirl::platform::shared::MouseManager<T>::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub fn mirl::platform::shared::get_time() -> alloc::boxed::Box<dyn mirl::platform::Time>
pub fn mirl::platform::shared::get_window_handle(window: &*mut std::os::raw::c_void) -> winapi::shared::windef::HWND
pub fn mirl::platform::shared::is_window_maximized(window: &*mut std::os::raw::c_void) -> bool
pub fn mirl::platform::shared::is_window_minimized(window: &*mut std::os::raw::c_void) -> bool
pub fn mirl::platform::shared::log(t: &str)
pub const fn mirl::platform::shared::map_button<MouseManagerScrollAccuracy: num_traits::float::Float>(button: mirl::platform::MouseButton, mouse_manager: &mirl::platform::shared::MouseManager<MouseManagerScrollAccuracy>) -> bool
pub const fn mirl::platform::shared::map_keycode(keycode: mirl::platform::KeyCode, key_manager: &mirl::platform::shared::KeyManager) -> bool
pub fn mirl::platform::shared::maximize(window: &*mut std::os::raw::c_void)
pub fn mirl::platform::shared::minimize(window: &*mut std::os::raw::c_void)
pub fn mirl::platform::shared::resize<T: num_traits::cast::ToPrimitive>(window: &*mut std::os::raw::c_void, size: (T, T))
pub fn mirl::platform::shared::restore(window: &*mut std::os::raw::c_void)
pub fn mirl::platform::shared::sample_fps<T: mirl::platform::Time>(since: &T) -> (mirl::platform::time::NativeTime, f64)
pub fn mirl::platform::shared::set_keycode(keycode: mirl::platform::KeyCode, key_manager: &mut mirl::platform::shared::KeyManager, value: bool)
pub const fn mirl::platform::shared::set_mouse_button<MouseManagerScrollAccuracy: num_traits::float::Float>(button: mirl::platform::MouseButton, mouse_manager: &mut mirl::platform::shared::MouseManager<MouseManagerScrollAccuracy>, value: bool)
pub fn mirl::platform::shared::sleep(time: core::time::Duration)
pub mod mirl::platform::time
pub struct mirl::platform::time::NativeTime
pub mirl::platform::time::NativeTime::time: std::time::Instant
impl mirl::platform::time::NativeTime
pub fn mirl::platform::time::NativeTime::new() -> Self
impl core::clone::Clone for mirl::platform::time::NativeTime
pub fn mirl::platform::time::NativeTime::clone(&self) -> mirl::platform::time::NativeTime
impl core::cmp::Eq for mirl::platform::time::NativeTime
impl core::cmp::PartialEq for mirl::platform::time::NativeTime
pub fn mirl::platform::time::NativeTime::eq(&self, other: &mirl::platform::time::NativeTime) -> bool
impl core::default::Default for mirl::platform::time::NativeTime
pub fn mirl::platform::time::NativeTime::default() -> Self
impl core::fmt::Debug for mirl::platform::time::NativeTime
pub fn mirl::platform::time::NativeTime::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::time::NativeTime
impl core::marker::StructuralPartialEq for mirl::platform::time::NativeTime
impl mirl::platform::Time for mirl::platform::time::NativeTime
pub fn mirl::platform::time::NativeTime::get_elapsed_time(&self) -> f64
impl core::marker::Freeze for mirl::platform::time::NativeTime
impl core::marker::Send for mirl::platform::time::NativeTime
impl core::marker::Sync for mirl::platform::time::NativeTime
impl core::marker::Unpin for mirl::platform::time::NativeTime
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::time::NativeTime
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::time::NativeTime
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::time::NativeTime where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::time::NativeTime::equivalent(&self, key: &K) -> bool
impl<R, P> lebe::io::ReadPrimitive<R> for mirl::platform::time::NativeTime where R: std::io::Read + lebe::io::ReadEndian<P>, P: core::default::Default
impl<T, U> core::convert::Into<U> for mirl::platform::time::NativeTime where U: core::convert::From<T>
pub fn mirl::platform::time::NativeTime::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::time::NativeTime where U: core::convert::Into<T>
pub type mirl::platform::time::NativeTime::Error = core::convert::Infallible
pub fn mirl::platform::time::NativeTime::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::time::NativeTime where U: core::convert::TryFrom<T>
pub type mirl::platform::time::NativeTime::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::time::NativeTime::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::time::NativeTime where T: core::clone::Clone
pub type mirl::platform::time::NativeTime::Owned = T
pub fn mirl::platform::time::NativeTime::clone_into(&self, target: &mut T)
pub fn mirl::platform::time::NativeTime::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::time::NativeTime where T: 'static + ?core::marker::Sized
pub fn mirl::platform::time::NativeTime::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::time::NativeTime where T: ?core::marker::Sized
pub fn mirl::platform::time::NativeTime::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::time::NativeTime where T: ?core::marker::Sized
pub fn mirl::platform::time::NativeTime::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::time::NativeTime where T: core::clone::Clone
pub unsafe fn mirl::platform::time::NativeTime::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::time::NativeTime
pub fn mirl::platform::time::NativeTime::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::time::NativeTime
pub type mirl::platform::time::NativeTime::Init = T
pub const mirl::platform::time::NativeTime::ALIGN: usize
pub unsafe fn mirl::platform::time::NativeTime::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::time::NativeTime::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::time::NativeTime::drop(ptr: usize)
pub unsafe fn mirl::platform::time::NativeTime::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::time::NativeTime
impl<T> mirl::extensions::RepeatData for mirl::platform::time::NativeTime where T: core::clone::Clone
pub fn mirl::platform::time::NativeTime::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::platform::Cursor
pub mirl::platform::Cursor::Glfw((mirl::lists::VariableSizeList<u32>, u32, u32))
pub mirl::platform::Cursor::Win(windows::Windows::Win32::UI::WindowsAndMessaging::HCURSOR)
impl core::clone::Clone for mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::Cursor::clone(&self) -> mirl::platform::mouse::Cursor
impl core::cmp::Eq for mirl::platform::mouse::Cursor
impl core::cmp::PartialEq for mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::Cursor::eq(&self, other: &mirl::platform::mouse::Cursor) -> bool
impl core::fmt::Debug for mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::Cursor::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::mouse::Cursor
impl core::marker::StructuralPartialEq for mirl::platform::mouse::Cursor
impl core::marker::Freeze for mirl::platform::mouse::Cursor
impl core::marker::Send for mirl::platform::mouse::Cursor
impl core::marker::Sync for mirl::platform::mouse::Cursor
impl core::marker::Unpin for mirl::platform::mouse::Cursor
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::mouse::Cursor
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::mouse::Cursor
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::mouse::Cursor where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::mouse::Cursor::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::mouse::Cursor where U: core::convert::From<T>
pub fn mirl::platform::mouse::Cursor::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::mouse::Cursor where U: core::convert::Into<T>
pub type mirl::platform::mouse::Cursor::Error = core::convert::Infallible
pub fn mirl::platform::mouse::Cursor::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::mouse::Cursor where U: core::convert::TryFrom<T>
pub type mirl::platform::mouse::Cursor::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::mouse::Cursor::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::mouse::Cursor where T: core::clone::Clone
pub type mirl::platform::mouse::Cursor::Owned = T
pub fn mirl::platform::mouse::Cursor::clone_into(&self, target: &mut T)
pub fn mirl::platform::mouse::Cursor::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::mouse::Cursor where T: 'static + ?core::marker::Sized
pub fn mirl::platform::mouse::Cursor::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::mouse::Cursor where T: ?core::marker::Sized
pub fn mirl::platform::mouse::Cursor::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::mouse::Cursor where T: ?core::marker::Sized
pub fn mirl::platform::mouse::Cursor::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::mouse::Cursor where T: core::clone::Clone
pub unsafe fn mirl::platform::mouse::Cursor::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::mouse::Cursor
pub fn mirl::platform::mouse::Cursor::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::mouse::Cursor
pub type mirl::platform::mouse::Cursor::Init = T
pub const mirl::platform::mouse::Cursor::ALIGN: usize
pub unsafe fn mirl::platform::mouse::Cursor::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::mouse::Cursor::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::mouse::Cursor::drop(ptr: usize)
pub unsafe fn mirl::platform::mouse::Cursor::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::mouse::Cursor
impl<T> mirl::extensions::RepeatData for mirl::platform::mouse::Cursor where T: core::clone::Clone
pub fn mirl::platform::mouse::Cursor::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::platform::CursorStyle
pub mirl::platform::CursorStyle::Alias
pub mirl::platform::CursorStyle::AllScroll
pub mirl::platform::CursorStyle::ArrowBottomLeft
pub mirl::platform::CursorStyle::ArrowBottomRight
pub mirl::platform::CursorStyle::ArrowDown
pub mirl::platform::CursorStyle::ArrowLeft
pub mirl::platform::CursorStyle::ArrowRight
pub mirl::platform::CursorStyle::ArrowTopLeft
pub mirl::platform::CursorStyle::ArrowTopRight
pub mirl::platform::CursorStyle::ArrowUp
pub mirl::platform::CursorStyle::Cell
pub mirl::platform::CursorStyle::CenteredPointer
pub mirl::platform::CursorStyle::ClosedHand
pub mirl::platform::CursorStyle::ClosedHandNoDrop
pub mirl::platform::CursorStyle::ColorPicker
pub mirl::platform::CursorStyle::ContextMenu
pub mirl::platform::CursorStyle::Copy
pub mirl::platform::CursorStyle::Crosshair
pub mirl::platform::CursorStyle::Default
pub mirl::platform::CursorStyle::Draft
pub mirl::platform::CursorStyle::Fleur
pub mirl::platform::CursorStyle::Help
pub mirl::platform::CursorStyle::MirroredPointer
pub mirl::platform::CursorStyle::NoDrop
pub mirl::platform::CursorStyle::NotAllowed
pub mirl::platform::CursorStyle::OpenHand
pub mirl::platform::CursorStyle::Pencil
pub mirl::platform::CursorStyle::Pirate
pub mirl::platform::CursorStyle::Pointer
pub mirl::platform::CursorStyle::ResizeHorizontally
pub mirl::platform::CursorStyle::ResizeNESW
pub mirl::platform::CursorStyle::ResizeNWSE
pub mirl::platform::CursorStyle::ResizeVertically
pub mirl::platform::CursorStyle::SideBottom
pub mirl::platform::CursorStyle::SideLeft
pub mirl::platform::CursorStyle::SideRight
pub mirl::platform::CursorStyle::SideTop
pub mirl::platform::CursorStyle::SizeHor
pub mirl::platform::CursorStyle::Text
pub mirl::platform::CursorStyle::VerticalText
pub mirl::platform::CursorStyle::ZoomIn
pub mirl::platform::CursorStyle::ZoomOut
impl core::clone::Clone for mirl::platform::CursorStyle
pub fn mirl::platform::CursorStyle::clone(&self) -> mirl::platform::CursorStyle
impl core::cmp::Eq for mirl::platform::CursorStyle
impl core::cmp::PartialEq for mirl::platform::CursorStyle
pub fn mirl::platform::CursorStyle::eq(&self, other: &mirl::platform::CursorStyle) -> bool
impl core::fmt::Debug for mirl::platform::CursorStyle
pub fn mirl::platform::CursorStyle::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::CursorStyle
impl core::marker::StructuralPartialEq for mirl::platform::CursorStyle
impl core::marker::Freeze for mirl::platform::CursorStyle
impl core::marker::Send for mirl::platform::CursorStyle
impl core::marker::Sync for mirl::platform::CursorStyle
impl core::marker::Unpin for mirl::platform::CursorStyle
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::CursorStyle
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::CursorStyle
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::CursorStyle where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::CursorStyle::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::CursorStyle where U: core::convert::From<T>
pub fn mirl::platform::CursorStyle::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::CursorStyle where U: core::convert::Into<T>
pub type mirl::platform::CursorStyle::Error = core::convert::Infallible
pub fn mirl::platform::CursorStyle::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::CursorStyle where U: core::convert::TryFrom<T>
pub type mirl::platform::CursorStyle::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::CursorStyle::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::CursorStyle where T: core::clone::Clone
pub type mirl::platform::CursorStyle::Owned = T
pub fn mirl::platform::CursorStyle::clone_into(&self, target: &mut T)
pub fn mirl::platform::CursorStyle::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::CursorStyle where T: 'static + ?core::marker::Sized
pub fn mirl::platform::CursorStyle::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::CursorStyle where T: ?core::marker::Sized
pub fn mirl::platform::CursorStyle::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::CursorStyle where T: ?core::marker::Sized
pub fn mirl::platform::CursorStyle::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::CursorStyle where T: core::clone::Clone
pub unsafe fn mirl::platform::CursorStyle::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::CursorStyle
pub fn mirl::platform::CursorStyle::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::CursorStyle
pub type mirl::platform::CursorStyle::Init = T
pub const mirl::platform::CursorStyle::ALIGN: usize
pub unsafe fn mirl::platform::CursorStyle::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::CursorStyle::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::CursorStyle::drop(ptr: usize)
pub unsafe fn mirl::platform::CursorStyle::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::CursorStyle
impl<T> mirl::extensions::RepeatData for mirl::platform::CursorStyle where T: core::clone::Clone
pub fn mirl::platform::CursorStyle::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::platform::KeyCode
pub mirl::platform::KeyCode::A
pub mirl::platform::KeyCode::AAcute├ü
pub mirl::platform::KeyCode::ACircumflex├é
pub mirl::platform::KeyCode::AELigature├å
pub mirl::platform::KeyCode::ARing├à
pub mirl::platform::KeyCode::AUmlaut├ä
pub mirl::platform::KeyCode::AnyAlt
pub mirl::platform::KeyCode::AnyControl
pub mirl::platform::KeyCode::AnyShift
pub mirl::platform::KeyCode::AnySuper
pub mirl::platform::KeyCode::Apostrophe
pub mirl::platform::KeyCode::Application
pub mirl::platform::KeyCode::B
pub mirl::platform::KeyCode::Backslash
pub mirl::platform::KeyCode::Backspace
pub mirl::platform::KeyCode::BrowserBack
pub mirl::platform::KeyCode::BrowserForward
pub mirl::platform::KeyCode::BrowserHome
pub mirl::platform::KeyCode::BrowserRefresh
pub mirl::platform::KeyCode::C
pub mirl::platform::KeyCode::CapsLock
pub mirl::platform::KeyCode::Comma
pub mirl::platform::KeyCode::D
pub mirl::platform::KeyCode::Delete
pub mirl::platform::KeyCode::Down
pub mirl::platform::KeyCode::E
pub mirl::platform::KeyCode::ECircumflex├è
pub mirl::platform::KeyCode::End
pub mirl::platform::KeyCode::Enter
pub mirl::platform::KeyCode::Equal
pub mirl::platform::KeyCode::Escape
pub mirl::platform::KeyCode::Eth├É
pub mirl::platform::KeyCode::F
pub mirl::platform::KeyCode::F1
pub mirl::platform::KeyCode::F10
pub mirl::platform::KeyCode::F11
pub mirl::platform::KeyCode::F12
pub mirl::platform::KeyCode::F13
pub mirl::platform::KeyCode::F14
pub mirl::platform::KeyCode::F15
pub mirl::platform::KeyCode::F16
pub mirl::platform::KeyCode::F17
pub mirl::platform::KeyCode::F18
pub mirl::platform::KeyCode::F19
pub mirl::platform::KeyCode::F2
pub mirl::platform::KeyCode::F20
pub mirl::platform::KeyCode::F21
pub mirl::platform::KeyCode::F22
pub mirl::platform::KeyCode::F23
pub mirl::platform::KeyCode::F24
pub mirl::platform::KeyCode::F25
pub mirl::platform::KeyCode::F3
pub mirl::platform::KeyCode::F4
pub mirl::platform::KeyCode::F5
pub mirl::platform::KeyCode::F6
pub mirl::platform::KeyCode::F7
pub mirl::platform::KeyCode::F8
pub mirl::platform::KeyCode::F9
pub mirl::platform::KeyCode::G
pub mirl::platform::KeyCode::Grave
pub mirl::platform::KeyCode::H
pub mirl::platform::KeyCode::Home
pub mirl::platform::KeyCode::I
pub mirl::platform::KeyCode::ICircumflex├Ä
pub mirl::platform::KeyCode::IGrave├î
pub mirl::platform::KeyCode::IUmlaut├Å
pub mirl::platform::KeyCode::Insert
pub mirl::platform::KeyCode::J
pub mirl::platform::KeyCode::K
pub mirl::platform::KeyCode::KeyPad0
pub mirl::platform::KeyCode::KeyPad1
pub mirl::platform::KeyCode::KeyPad2
pub mirl::platform::KeyCode::KeyPad3
pub mirl::platform::KeyCode::KeyPad4
pub mirl::platform::KeyCode::KeyPad5
pub mirl::platform::KeyCode::KeyPad6
pub mirl::platform::KeyCode::KeyPad7
pub mirl::platform::KeyCode::KeyPad8
pub mirl::platform::KeyCode::KeyPad9
pub mirl::platform::KeyCode::KeyPadAdd
pub mirl::platform::KeyCode::KeyPadDecimal
pub mirl::platform::KeyCode::KeyPadDivide
pub mirl::platform::KeyCode::KeyPadEnter
pub mirl::platform::KeyCode::KeyPadEqual
pub mirl::platform::KeyCode::KeyPadMultiply
pub mirl::platform::KeyCode::KeyPadSubtract
pub mirl::platform::KeyCode::L
pub mirl::platform::KeyCode::LaunchApp1
pub mirl::platform::KeyCode::LaunchApp2
pub mirl::platform::KeyCode::LaunchMail
pub mirl::platform::KeyCode::Left
pub mirl::platform::KeyCode::LeftAlt
pub mirl::platform::KeyCode::LeftBracket
pub mirl::platform::KeyCode::LeftControl
pub mirl::platform::KeyCode::LeftShift
pub mirl::platform::KeyCode::LeftSuper
pub mirl::platform::KeyCode::M
pub mirl::platform::KeyCode::MediaNext
pub mirl::platform::KeyCode::MediaPlayPause
pub mirl::platform::KeyCode::MediaPrev
pub mirl::platform::KeyCode::MediaStop
pub mirl::platform::KeyCode::Menu
pub mirl::platform::KeyCode::Minus
pub mirl::platform::KeyCode::Mute
pub mirl::platform::KeyCode::N
pub mirl::platform::KeyCode::NTilde├æ
pub mirl::platform::KeyCode::Num0
pub mirl::platform::KeyCode::Num1
pub mirl::platform::KeyCode::Num2
pub mirl::platform::KeyCode::Num3
pub mirl::platform::KeyCode::Num4
pub mirl::platform::KeyCode::Num5
pub mirl::platform::KeyCode::Num6
pub mirl::platform::KeyCode::Num7
pub mirl::platform::KeyCode::Num8
pub mirl::platform::KeyCode::Num9
pub mirl::platform::KeyCode::NumLock
pub mirl::platform::KeyCode::O
pub mirl::platform::KeyCode::OCircumflex├ö
pub mirl::platform::KeyCode::OELigature┼Æ
pub mirl::platform::KeyCode::OGrave├Æ
pub mirl::platform::KeyCode::OSlash├ÿ
pub mirl::platform::KeyCode::OUmlaut├û
pub mirl::platform::KeyCode::P
pub mirl::platform::KeyCode::PageDown
pub mirl::platform::KeyCode::PageUp
pub mirl::platform::KeyCode::Pause
pub mirl::platform::KeyCode::Period
pub mirl::platform::KeyCode::PrintScreen
pub mirl::platform::KeyCode::Q
pub mirl::platform::KeyCode::Quote
pub mirl::platform::KeyCode::R
pub mirl::platform::KeyCode::Right
pub mirl::platform::KeyCode::RightAlt
pub mirl::platform::KeyCode::RightBracket
pub mirl::platform::KeyCode::RightControl
pub mirl::platform::KeyCode::RightShift
pub mirl::platform::KeyCode::RightSuper
pub mirl::platform::KeyCode::S
pub mirl::platform::KeyCode::SS
pub mirl::platform::KeyCode::ScrollLock
pub mirl::platform::KeyCode::Semicolon
pub mirl::platform::KeyCode::Slash
pub mirl::platform::KeyCode::Space
pub mirl::platform::KeyCode::T
pub mirl::platform::KeyCode::Tab
pub mirl::platform::KeyCode::Thorn├×
pub mirl::platform::KeyCode::Tilde
pub mirl::platform::KeyCode::U
pub mirl::platform::KeyCode::UAcute├Ü
pub mirl::platform::KeyCode::UGrave├Ö
pub mirl::platform::KeyCode::UUmlaut├£
pub mirl::platform::KeyCode::Unknown
pub mirl::platform::KeyCode::Up
pub mirl::platform::KeyCode::V
pub mirl::platform::KeyCode::VolumeDown
pub mirl::platform::KeyCode::VolumeUp
pub mirl::platform::KeyCode::W
pub mirl::platform::KeyCode::World1
pub mirl::platform::KeyCode::World2
pub mirl::platform::KeyCode::X
pub mirl::platform::KeyCode::Y
pub mirl::platform::KeyCode::YAcute├Ø
pub mirl::platform::KeyCode::Z
impl mirl::platform::KeyCode
pub const fn mirl::platform::KeyCode::to_string(&self) -> &'static str
impl mirl::platform::KeyCode
pub fn mirl::platform::KeyCode::to_user_friendly_string(&self) -> core::option::Option<alloc::string::String>
impl core::clone::Clone for mirl::platform::KeyCode
pub fn mirl::platform::KeyCode::clone(&self) -> mirl::platform::KeyCode
impl core::cmp::Eq for mirl::platform::KeyCode
impl core::cmp::PartialEq for mirl::platform::KeyCode
pub fn mirl::platform::KeyCode::eq(&self, other: &mirl::platform::KeyCode) -> bool
impl core::fmt::Debug for mirl::platform::KeyCode
pub fn mirl::platform::KeyCode::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::fmt::Display for mirl::platform::KeyCode
pub fn mirl::platform::KeyCode::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::KeyCode
impl core::marker::StructuralPartialEq for mirl::platform::KeyCode
impl strum::IntoEnumIterator for mirl::platform::KeyCode
pub type mirl::platform::KeyCode::Iterator = mirl::platform::KeyCodeIter
pub fn mirl::platform::KeyCode::iter() -> mirl::platform::KeyCodeIter
impl core::marker::Freeze for mirl::platform::KeyCode
impl core::marker::Send for mirl::platform::KeyCode
impl core::marker::Sync for mirl::platform::KeyCode
impl core::marker::Unpin for mirl::platform::KeyCode
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::KeyCode
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::KeyCode
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::KeyCode where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::KeyCode::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::KeyCode where U: core::convert::From<T>
pub fn mirl::platform::KeyCode::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::KeyCode where U: core::convert::Into<T>
pub type mirl::platform::KeyCode::Error = core::convert::Infallible
pub fn mirl::platform::KeyCode::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::KeyCode where U: core::convert::TryFrom<T>
pub type mirl::platform::KeyCode::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::KeyCode::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::KeyCode where T: core::clone::Clone
pub type mirl::platform::KeyCode::Owned = T
pub fn mirl::platform::KeyCode::clone_into(&self, target: &mut T)
pub fn mirl::platform::KeyCode::to_owned(&self) -> T
impl<T> alloc::string::ToString for mirl::platform::KeyCode where T: core::fmt::Display + ?core::marker::Sized
pub fn mirl::platform::KeyCode::to_string(&self) -> alloc::string::String
impl<T> core::any::Any for mirl::platform::KeyCode where T: 'static + ?core::marker::Sized
pub fn mirl::platform::KeyCode::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::KeyCode where T: ?core::marker::Sized
pub fn mirl::platform::KeyCode::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::KeyCode where T: ?core::marker::Sized
pub fn mirl::platform::KeyCode::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::KeyCode where T: core::clone::Clone
pub unsafe fn mirl::platform::KeyCode::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::KeyCode
pub fn mirl::platform::KeyCode::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::KeyCode
pub type mirl::platform::KeyCode::Init = T
pub const mirl::platform::KeyCode::ALIGN: usize
pub unsafe fn mirl::platform::KeyCode::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::KeyCode::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::KeyCode::drop(ptr: usize)
pub unsafe fn mirl::platform::KeyCode::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::KeyCode
impl<T> mirl::extensions::RepeatData for mirl::platform::KeyCode where T: core::clone::Clone
pub fn mirl::platform::KeyCode::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::platform::MouseButton
pub mirl::platform::MouseButton::Extra1
pub mirl::platform::MouseButton::Extra2
pub mirl::platform::MouseButton::Extra3
pub mirl::platform::MouseButton::Extra4
pub mirl::platform::MouseButton::Left
pub mirl::platform::MouseButton::Middle
pub mirl::platform::MouseButton::Right
pub mirl::platform::MouseButton::Unsupported
impl core::clone::Clone for mirl::platform::MouseButton
pub fn mirl::platform::MouseButton::clone(&self) -> mirl::platform::MouseButton
impl core::cmp::Eq for mirl::platform::MouseButton
impl core::cmp::PartialEq for mirl::platform::MouseButton
pub fn mirl::platform::MouseButton::eq(&self, other: &mirl::platform::MouseButton) -> bool
impl core::fmt::Debug for mirl::platform::MouseButton
pub fn mirl::platform::MouseButton::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::MouseButton
impl core::marker::StructuralPartialEq for mirl::platform::MouseButton
impl core::marker::Freeze for mirl::platform::MouseButton
impl core::marker::Send for mirl::platform::MouseButton
impl core::marker::Sync for mirl::platform::MouseButton
impl core::marker::Unpin for mirl::platform::MouseButton
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::MouseButton
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::MouseButton
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::MouseButton where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::MouseButton::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::MouseButton where U: core::convert::From<T>
pub fn mirl::platform::MouseButton::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::MouseButton where U: core::convert::Into<T>
pub type mirl::platform::MouseButton::Error = core::convert::Infallible
pub fn mirl::platform::MouseButton::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::MouseButton where U: core::convert::TryFrom<T>
pub type mirl::platform::MouseButton::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::MouseButton::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::MouseButton where T: core::clone::Clone
pub type mirl::platform::MouseButton::Owned = T
pub fn mirl::platform::MouseButton::clone_into(&self, target: &mut T)
pub fn mirl::platform::MouseButton::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::MouseButton where T: 'static + ?core::marker::Sized
pub fn mirl::platform::MouseButton::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::MouseButton where T: ?core::marker::Sized
pub fn mirl::platform::MouseButton::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::MouseButton where T: ?core::marker::Sized
pub fn mirl::platform::MouseButton::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::MouseButton where T: core::clone::Clone
pub unsafe fn mirl::platform::MouseButton::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::MouseButton
pub fn mirl::platform::MouseButton::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::MouseButton
pub type mirl::platform::MouseButton::Init = T
pub const mirl::platform::MouseButton::ALIGN: usize
pub unsafe fn mirl::platform::MouseButton::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::MouseButton::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::MouseButton::drop(ptr: usize)
pub unsafe fn mirl::platform::MouseButton::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::MouseButton
impl<T> mirl::extensions::RepeatData for mirl::platform::MouseButton where T: core::clone::Clone
pub fn mirl::platform::MouseButton::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub enum mirl::platform::WindowLevel
pub mirl::platform::WindowLevel::AlwaysOnBottom
pub mirl::platform::WindowLevel::AlwaysOnTop
pub mirl::platform::WindowLevel::Normal
impl core::clone::Clone for mirl::platform::WindowLevel
pub fn mirl::platform::WindowLevel::clone(&self) -> mirl::platform::WindowLevel
impl core::cmp::Eq for mirl::platform::WindowLevel
impl core::cmp::PartialEq for mirl::platform::WindowLevel
pub fn mirl::platform::WindowLevel::eq(&self, other: &mirl::platform::WindowLevel) -> bool
impl core::fmt::Debug for mirl::platform::WindowLevel
pub fn mirl::platform::WindowLevel::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::WindowLevel
impl core::marker::StructuralPartialEq for mirl::platform::WindowLevel
impl core::marker::Freeze for mirl::platform::WindowLevel
impl core::marker::Send for mirl::platform::WindowLevel
impl core::marker::Sync for mirl::platform::WindowLevel
impl core::marker::Unpin for mirl::platform::WindowLevel
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::WindowLevel
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::WindowLevel
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::WindowLevel where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::WindowLevel::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::WindowLevel where U: core::convert::From<T>
pub fn mirl::platform::WindowLevel::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::WindowLevel where U: core::convert::Into<T>
pub type mirl::platform::WindowLevel::Error = core::convert::Infallible
pub fn mirl::platform::WindowLevel::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::WindowLevel where U: core::convert::TryFrom<T>
pub type mirl::platform::WindowLevel::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::WindowLevel::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::WindowLevel where T: core::clone::Clone
pub type mirl::platform::WindowLevel::Owned = T
pub fn mirl::platform::WindowLevel::clone_into(&self, target: &mut T)
pub fn mirl::platform::WindowLevel::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::WindowLevel where T: 'static + ?core::marker::Sized
pub fn mirl::platform::WindowLevel::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::WindowLevel where T: ?core::marker::Sized
pub fn mirl::platform::WindowLevel::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::WindowLevel where T: ?core::marker::Sized
pub fn mirl::platform::WindowLevel::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::WindowLevel where T: core::clone::Clone
pub unsafe fn mirl::platform::WindowLevel::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::WindowLevel
pub fn mirl::platform::WindowLevel::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::WindowLevel
pub type mirl::platform::WindowLevel::Init = T
pub const mirl::platform::WindowLevel::ALIGN: usize
pub unsafe fn mirl::platform::WindowLevel::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::WindowLevel::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::WindowLevel::drop(ptr: usize)
pub unsafe fn mirl::platform::WindowLevel::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::WindowLevel
impl<T> mirl::extensions::RepeatData for mirl::platform::WindowLevel where T: core::clone::Clone
pub fn mirl::platform::WindowLevel::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::platform::Buffer
pub mirl::platform::Buffer::data: alloc::boxed::Box<[u32]>
pub mirl::platform::Buffer::height: usize
pub mirl::platform::Buffer::pointer: *mut u32
pub mirl::platform::Buffer::total_size: usize
pub mirl::platform::Buffer::width: usize
impl mirl::platform::Buffer
pub fn mirl::platform::Buffer::apply_trim(&mut self, top: usize, bottom: usize, left: usize, right: usize)
pub fn mirl::platform::Buffer::calculate_trims(&self) -> (usize, usize, usize, usize)
pub fn mirl::platform::Buffer::is_col_transparent(&self, col: usize) -> bool
pub fn mirl::platform::Buffer::is_row_transparent(&self, row: usize) -> bool
pub fn mirl::platform::Buffer::remove_margins(&mut self)
impl mirl::platform::Buffer
pub const fn mirl::platform::Buffer::clear(&self)
pub fn mirl::platform::Buffer::clear_buffer_with_color(&self, color: u32)
pub fn mirl::platform::Buffer::clear_buffer_with_color_sliced(&self, color: u32)
pub fn mirl::platform::Buffer::replace_transparent_with_color(&self, color: u32)
pub fn mirl::platform::Buffer::replace_transparent_with_color_chunked(&mut self, color: u32)
impl mirl::platform::Buffer
pub const fn mirl::platform::Buffer::create_collision<const CS: bool>(&self, x: usize, y: usize) -> mirl::math::collision::rectangle::Rectangle<usize, CS>
pub const fn mirl::platform::Buffer::create_collision_isize<const CS: bool>(&self, x: isize, y: isize) -> mirl::math::collision::rectangle::Rectangle<isize, CS>
pub const fn mirl::platform::Buffer::is_pixel_position_in_buffer(&self, x: usize, y: usize) -> bool
pub const fn mirl::platform::Buffer::is_pixel_position_in_buffer_isize(&self, x: isize, y: isize) -> bool
impl mirl::platform::Buffer
pub fn mirl::platform::Buffer::flip_horizontally(&self) -> Self
pub fn mirl::platform::Buffer::flip_vertically(&self) -> Self
pub fn mirl::platform::Buffer::rotate_180(&self) -> Self
pub fn mirl::platform::Buffer::rotate_270(&self) -> Self
pub fn mirl::platform::Buffer::rotate_90(&self) -> Self
impl mirl::platform::Buffer
pub fn mirl::platform::Buffer::from_u8_rgb(rgba: &[u8], width: usize, height: usize) -> core::result::Result<Self, alloc::string::String>
pub fn mirl::platform::Buffer::from_u8_rgba(rgba: &[u8], width: usize, height: usize) -> core::result::Result<Self, alloc::string::String>
pub fn mirl::platform::Buffer::generate_fallback(width: usize, height: usize, squares: usize) -> Self
pub fn mirl::platform::Buffer::new(data: alloc::vec::Vec<u32>, width: usize, height: usize) -> core::result::Result<Self, alloc::string::String>
pub fn mirl::platform::Buffer::new_empty(width: usize, height: usize) -> Self
pub fn mirl::platform::Buffer::new_empty_with_color(width: usize, height: usize, color: u32) -> Self
impl mirl::platform::Buffer
pub fn mirl::platform::Buffer::get_pixel(&self, xy: (usize, usize)) -> u32
pub fn mirl::platform::Buffer::get_pixel_fallback(&self, xy: (usize, usize), fallback: u32) -> u32
pub fn mirl::platform::Buffer::get_pixel_isize(&self, xy: (isize, isize)) -> core::option::Option<u32>
pub fn mirl::platform::Buffer::get_pixel_isize_fallback(&self, xy: (isize, isize), fallback: u32) -> u32
pub fn mirl::platform::Buffer::get_pixel_unsafe(&self, xy: (usize, usize)) -> u32
impl mirl::platform::Buffer
pub fn mirl::platform::Buffer::resize_content(&self, width: usize, height: usize, resize_mode: mirl::graphics::InterpolationMode) -> Self
pub fn mirl::platform::Buffer::to_u8_argb(&self) -> alloc::vec::Vec<u8>
pub fn mirl::platform::Buffer::to_u8_rgba(&self) -> alloc::vec::Vec<u8>
impl mirl::platform::Buffer
pub fn mirl::platform::Buffer::set_pixel_safe(&self, x: usize, y: usize, color: u32)
pub fn mirl::platform::Buffer::set_pixel_unsafe(&self, x: usize, y: usize, color: u32)
impl mirl::platform::Buffer
pub fn mirl::platform::Buffer::update_pointer(&mut self)
pub const fn mirl::platform::Buffer::update_total_size(&mut self)
impl core::clone::Clone for mirl::platform::Buffer
pub fn mirl::platform::Buffer::clone(&self) -> Self
impl core::cmp::Eq for mirl::platform::Buffer
impl core::cmp::PartialEq for mirl::platform::Buffer
pub fn mirl::platform::Buffer::eq(&self, other: &mirl::platform::Buffer) -> bool
impl core::convert::From<glfw::PixelImage> for mirl::platform::Buffer
pub fn mirl::platform::Buffer::from(pixel_image: glfw::PixelImage) -> Self
impl core::convert::From<image::dynimage::DynamicImage> for mirl::platform::Buffer
pub fn mirl::platform::Buffer::from(bush: image::dynimage::DynamicImage) -> Self
impl core::convert::From<mirl::lists::VariableSizeList<u32>> for mirl::platform::Buffer
pub fn mirl::platform::Buffer::from(list: mirl::lists::VariableSizeList<u32>) -> Self
impl core::convert::From<mirl::platform::Buffer> for glfw::PixelImage
pub fn glfw::PixelImage::from(buffer: mirl::platform::Buffer) -> Self
impl core::convert::From<mirl::platform::Buffer> for image::dynimage::DynamicImage
pub fn image::dynimage::DynamicImage::from(bush: mirl::platform::Buffer) -> Self
impl core::convert::TryFrom<mirl::platform::Buffer> for mirl::lists::VariableSizeList<u32>
pub type mirl::lists::VariableSizeList<u32>::Error = &'static str
pub fn mirl::lists::VariableSizeList<u32>::try_from(value: mirl::platform::Buffer) -> core::result::Result<Self, Self::Error>
impl core::fmt::Debug for mirl::platform::Buffer
pub fn mirl::platform::Buffer::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::StructuralPartialEq for mirl::platform::Buffer
impl core::ops::deref::Deref for mirl::platform::Buffer
pub type mirl::platform::Buffer::Target = [u32]
pub fn mirl::platform::Buffer::deref(&self) -> &Self::Target
impl<const N: usize> core::convert::From<mirl::lists::CopyableList<u32, N>> for mirl::platform::Buffer
pub fn mirl::platform::Buffer::from(list: mirl::lists::CopyableList<u32, N>) -> Self
impl core::marker::Freeze for mirl::platform::Buffer
impl !core::marker::Send for mirl::platform::Buffer
impl !core::marker::Sync for mirl::platform::Buffer
impl core::marker::Unpin for mirl::platform::Buffer
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::Buffer
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::Buffer
impl<P, T> core::ops::deref::Receiver for mirl::platform::Buffer where P: core::ops::deref::Deref<Target = T> + ?core::marker::Sized, T: ?core::marker::Sized
pub type mirl::platform::Buffer::Target = T
impl<Q, K> equivalent::Equivalent<K> for mirl::platform::Buffer where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::platform::Buffer::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::platform::Buffer where U: core::convert::From<T>
pub fn mirl::platform::Buffer::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::Buffer where U: core::convert::Into<T>
pub type mirl::platform::Buffer::Error = core::convert::Infallible
pub fn mirl::platform::Buffer::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::Buffer where U: core::convert::TryFrom<T>
pub type mirl::platform::Buffer::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::Buffer::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::Buffer where T: core::clone::Clone
pub type mirl::platform::Buffer::Owned = T
pub fn mirl::platform::Buffer::clone_into(&self, target: &mut T)
pub fn mirl::platform::Buffer::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::Buffer where T: 'static + ?core::marker::Sized
pub fn mirl::platform::Buffer::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::Buffer where T: ?core::marker::Sized
pub fn mirl::platform::Buffer::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::Buffer where T: ?core::marker::Sized
pub fn mirl::platform::Buffer::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::Buffer where T: core::clone::Clone
pub unsafe fn mirl::platform::Buffer::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::Buffer
pub fn mirl::platform::Buffer::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::Buffer
pub type mirl::platform::Buffer::Init = T
pub const mirl::platform::Buffer::ALIGN: usize
pub unsafe fn mirl::platform::Buffer::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::Buffer::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::Buffer::drop(ptr: usize)
pub unsafe fn mirl::platform::Buffer::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::Buffer
impl<T> mirl::extensions::RepeatData for mirl::platform::Buffer where T: core::clone::Clone
pub fn mirl::platform::Buffer::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::platform::DoubleBuffer
impl mirl::platform::DoubleBuffer
pub fn mirl::platform::DoubleBuffer::new(width: usize, height: usize) -> Self
pub fn mirl::platform::DoubleBuffer::read(&self) -> &mirl::platform::Buffer
pub fn mirl::platform::DoubleBuffer::write(&mut self, new_data: mirl::platform::Buffer)
impl core::fmt::Debug for mirl::platform::DoubleBuffer
pub fn mirl::platform::DoubleBuffer::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl !core::marker::Freeze for mirl::platform::DoubleBuffer
impl !core::marker::Send for mirl::platform::DoubleBuffer
impl !core::marker::Sync for mirl::platform::DoubleBuffer
impl core::marker::Unpin for mirl::platform::DoubleBuffer
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::DoubleBuffer
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::DoubleBuffer
impl<T, U> core::convert::Into<U> for mirl::platform::DoubleBuffer where U: core::convert::From<T>
pub fn mirl::platform::DoubleBuffer::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::DoubleBuffer where U: core::convert::Into<T>
pub type mirl::platform::DoubleBuffer::Error = core::convert::Infallible
pub fn mirl::platform::DoubleBuffer::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::DoubleBuffer where U: core::convert::TryFrom<T>
pub type mirl::platform::DoubleBuffer::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::DoubleBuffer::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> core::any::Any for mirl::platform::DoubleBuffer where T: 'static + ?core::marker::Sized
pub fn mirl::platform::DoubleBuffer::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::DoubleBuffer where T: ?core::marker::Sized
pub fn mirl::platform::DoubleBuffer::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::DoubleBuffer where T: ?core::marker::Sized
pub fn mirl::platform::DoubleBuffer::borrow_mut(&mut self) -> &mut T
impl<T> core::convert::From<T> for mirl::platform::DoubleBuffer
pub fn mirl::platform::DoubleBuffer::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::DoubleBuffer
pub type mirl::platform::DoubleBuffer::Init = T
pub const mirl::platform::DoubleBuffer::ALIGN: usize
pub unsafe fn mirl::platform::DoubleBuffer::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::DoubleBuffer::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::DoubleBuffer::drop(ptr: usize)
pub unsafe fn mirl::platform::DoubleBuffer::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::DoubleBuffer
pub struct mirl::platform::KeyCodeIter
impl core::clone::Clone for mirl::platform::KeyCodeIter
pub fn mirl::platform::KeyCodeIter::clone(&self) -> mirl::platform::KeyCodeIter
impl core::fmt::Debug for mirl::platform::KeyCodeIter
pub fn mirl::platform::KeyCodeIter::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::iter::traits::double_ended::DoubleEndedIterator for mirl::platform::KeyCodeIter
pub fn mirl::platform::KeyCodeIter::next_back(&mut self) -> core::option::Option<<Self as core::iter::traits::iterator::Iterator>::Item>
impl core::iter::traits::exact_size::ExactSizeIterator for mirl::platform::KeyCodeIter
pub fn mirl::platform::KeyCodeIter::len(&self) -> usize
impl core::iter::traits::iterator::Iterator for mirl::platform::KeyCodeIter
pub type mirl::platform::KeyCodeIter::Item = mirl::platform::KeyCode
pub fn mirl::platform::KeyCodeIter::next(&mut self) -> core::option::Option<<Self as core::iter::traits::iterator::Iterator>::Item>
pub fn mirl::platform::KeyCodeIter::nth(&mut self, n: usize) -> core::option::Option<<Self as core::iter::traits::iterator::Iterator>::Item>
pub fn mirl::platform::KeyCodeIter::size_hint(&self) -> (usize, core::option::Option<usize>)
impl core::iter::traits::marker::FusedIterator for mirl::platform::KeyCodeIter
impl core::marker::Freeze for mirl::platform::KeyCodeIter
impl core::marker::Send for mirl::platform::KeyCodeIter
impl core::marker::Sync for mirl::platform::KeyCodeIter
impl core::marker::Unpin for mirl::platform::KeyCodeIter
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::KeyCodeIter
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::KeyCodeIter
impl<I> core::iter::traits::collect::IntoIterator for mirl::platform::KeyCodeIter where I: core::iter::traits::iterator::Iterator
pub type mirl::platform::KeyCodeIter::IntoIter = I
pub type mirl::platform::KeyCodeIter::Item = <I as core::iter::traits::iterator::Iterator>::Item
pub fn mirl::platform::KeyCodeIter::into_iter(self) -> I
impl<T, U> core::convert::Into<U> for mirl::platform::KeyCodeIter where U: core::convert::From<T>
pub fn mirl::platform::KeyCodeIter::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::KeyCodeIter where U: core::convert::Into<T>
pub type mirl::platform::KeyCodeIter::Error = core::convert::Infallible
pub fn mirl::platform::KeyCodeIter::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::KeyCodeIter where U: core::convert::TryFrom<T>
pub type mirl::platform::KeyCodeIter::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::KeyCodeIter::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::KeyCodeIter where T: core::clone::Clone
pub type mirl::platform::KeyCodeIter::Owned = T
pub fn mirl::platform::KeyCodeIter::clone_into(&self, target: &mut T)
pub fn mirl::platform::KeyCodeIter::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::KeyCodeIter where T: 'static + ?core::marker::Sized
pub fn mirl::platform::KeyCodeIter::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::KeyCodeIter where T: ?core::marker::Sized
pub fn mirl::platform::KeyCodeIter::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::KeyCodeIter where T: ?core::marker::Sized
pub fn mirl::platform::KeyCodeIter::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::KeyCodeIter where T: core::clone::Clone
pub unsafe fn mirl::platform::KeyCodeIter::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::KeyCodeIter
pub fn mirl::platform::KeyCodeIter::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::KeyCodeIter
pub type mirl::platform::KeyCodeIter::Init = T
pub const mirl::platform::KeyCodeIter::ALIGN: usize
pub unsafe fn mirl::platform::KeyCodeIter::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::KeyCodeIter::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::KeyCodeIter::drop(ptr: usize)
pub unsafe fn mirl::platform::KeyCodeIter::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::KeyCodeIter
impl<T> itertools::Itertools for mirl::platform::KeyCodeIter where T: core::iter::traits::iterator::Iterator + ?core::marker::Sized
impl<T> mirl::extensions::RepeatData for mirl::platform::KeyCodeIter where T: core::clone::Clone
pub fn mirl::platform::KeyCodeIter::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
impl<T> rayon::iter::par_bridge::ParallelBridge for mirl::platform::KeyCodeIter where T: core::iter::traits::iterator::Iterator + core::marker::Send, <T as core::iter::traits::iterator::Iterator>::Item: core::marker::Send
pub fn mirl::platform::KeyCodeIter::par_bridge(self) -> rayon::iter::par_bridge::IterBridge<T>
pub struct mirl::platform::ScreenNormalizer<S: num_traits::float::Float>
impl<S: num_traits::float::Float> mirl::platform::ScreenNormalizer<S>
pub const fn mirl::platform::ScreenNormalizer<S>::new<A: num_traits::cast::ToPrimitive>(screen_size: (S, S)) -> Self
pub fn mirl::platform::ScreenNormalizer<S>::percentile_to_x<T: num_traits::Num + num_traits::cast::NumCast>(&self, p: S) -> core::option::Option<T>
pub fn mirl::platform::ScreenNormalizer<S>::percentile_to_y<T: num_traits::Num + num_traits::cast::NumCast>(&self, p: S) -> core::option::Option<T>
pub fn mirl::platform::ScreenNormalizer<S>::x_to_percentile<T: num_traits::Num + num_traits::cast::NumCast>(&self, x: T) -> core::option::Option<S>
pub fn mirl::platform::ScreenNormalizer<S>::y_to_percentile<T: num_traits::Num + num_traits::cast::NumCast>(&self, y: T) -> core::option::Option<S>
impl<S: core::clone::Clone + num_traits::float::Float> core::clone::Clone for mirl::platform::ScreenNormalizer<S>
pub fn mirl::platform::ScreenNormalizer<S>::clone(&self) -> mirl::platform::ScreenNormalizer<S>
impl<S: core::fmt::Debug + num_traits::float::Float> core::fmt::Debug for mirl::platform::ScreenNormalizer<S>
pub fn mirl::platform::ScreenNormalizer<S>::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl<S: core::marker::Copy + num_traits::float::Float> core::marker::Copy for mirl::platform::ScreenNormalizer<S>
impl<S> core::marker::Freeze for mirl::platform::ScreenNormalizer<S> where S: core::marker::Freeze
impl<S> core::marker::Send for mirl::platform::ScreenNormalizer<S> where S: core::marker::Send
impl<S> core::marker::Sync for mirl::platform::ScreenNormalizer<S> where S: core::marker::Sync
impl<S> core::marker::Unpin for mirl::platform::ScreenNormalizer<S> where S: core::marker::Unpin
impl<S> core::panic::unwind_safe::RefUnwindSafe for mirl::platform::ScreenNormalizer<S> where S: core::panic::unwind_safe::RefUnwindSafe
impl<S> core::panic::unwind_safe::UnwindSafe for mirl::platform::ScreenNormalizer<S> where S: core::panic::unwind_safe::UnwindSafe
impl<T, U> core::convert::Into<U> for mirl::platform::ScreenNormalizer<S> where U: core::convert::From<T>
pub fn mirl::platform::ScreenNormalizer<S>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::ScreenNormalizer<S> where U: core::convert::Into<T>
pub type mirl::platform::ScreenNormalizer<S>::Error = core::convert::Infallible
pub fn mirl::platform::ScreenNormalizer<S>::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::ScreenNormalizer<S> where U: core::convert::TryFrom<T>
pub type mirl::platform::ScreenNormalizer<S>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::ScreenNormalizer<S>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::ScreenNormalizer<S> where T: core::clone::Clone
pub type mirl::platform::ScreenNormalizer<S>::Owned = T
pub fn mirl::platform::ScreenNormalizer<S>::clone_into(&self, target: &mut T)
pub fn mirl::platform::ScreenNormalizer<S>::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::ScreenNormalizer<S> where T: 'static + ?core::marker::Sized
pub fn mirl::platform::ScreenNormalizer<S>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::ScreenNormalizer<S> where T: ?core::marker::Sized
pub fn mirl::platform::ScreenNormalizer<S>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::ScreenNormalizer<S> where T: ?core::marker::Sized
pub fn mirl::platform::ScreenNormalizer<S>::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::ScreenNormalizer<S> where T: core::clone::Clone
pub unsafe fn mirl::platform::ScreenNormalizer<S>::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::ScreenNormalizer<S>
pub fn mirl::platform::ScreenNormalizer<S>::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::ScreenNormalizer<S>
pub type mirl::platform::ScreenNormalizer<S>::Init = T
pub const mirl::platform::ScreenNormalizer<S>::ALIGN: usize
pub unsafe fn mirl::platform::ScreenNormalizer<S>::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::ScreenNormalizer<S>::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::ScreenNormalizer<S>::drop(ptr: usize)
pub unsafe fn mirl::platform::ScreenNormalizer<S>::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::ScreenNormalizer<S>
impl<T> mirl::extensions::RepeatData for mirl::platform::ScreenNormalizer<S> where T: core::clone::Clone
pub fn mirl::platform::ScreenNormalizer<S>::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::platform::WindowSettings
pub mirl::platform::WindowSettings::borderless: bool
pub mirl::platform::WindowSettings::os_menu: bool
pub mirl::platform::WindowSettings::position: (isize, isize)
pub mirl::platform::WindowSettings::resizable: bool
pub mirl::platform::WindowSettings::size: (isize, isize)
pub mirl::platform::WindowSettings::title_visible: bool
pub mirl::platform::WindowSettings::visible: bool
pub mirl::platform::WindowSettings::window_level: mirl::platform::WindowLevel
impl mirl::platform::WindowSettings
pub fn mirl::platform::WindowSettings::default(buffer: &mirl::platform::Buffer) -> Self
pub fn mirl::platform::WindowSettings::set_borderless(&mut self, borderless: bool) -> &mut Self
pub fn mirl::platform::WindowSettings::set_os_menu(&mut self, os_menu: bool) -> &mut Self
pub fn mirl::platform::WindowSettings::set_position(&mut self, position: (isize, isize)) -> &mut Self
pub fn mirl::platform::WindowSettings::set_position_to_middle_of_screen(&mut self) -> &mut Self
pub fn mirl::platform::WindowSettings::set_resizable(&mut self, resizable: bool) -> &mut Self
pub fn mirl::platform::WindowSettings::set_size(&mut self, size: (isize, isize)) -> &mut Self
pub fn mirl::platform::WindowSettings::set_size_to_buffer(&mut self, buffer: &mirl::platform::Buffer) -> &mut Self
pub fn mirl::platform::WindowSettings::set_title_visible(&mut self, title: bool) -> &mut Self
pub fn mirl::platform::WindowSettings::set_visible(&mut self, visible: bool) -> &mut Self
pub fn mirl::platform::WindowSettings::set_window_level(&mut self, window_level: mirl::platform::WindowLevel) -> &mut Self
impl core::clone::Clone for mirl::platform::WindowSettings
pub fn mirl::platform::WindowSettings::clone(&self) -> mirl::platform::WindowSettings
impl core::cmp::PartialEq for mirl::platform::WindowSettings
pub fn mirl::platform::WindowSettings::eq(&self, other: &mirl::platform::WindowSettings) -> bool
impl core::fmt::Debug for mirl::platform::WindowSettings
pub fn mirl::platform::WindowSettings::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::platform::WindowSettings
impl core::marker::StructuralPartialEq for mirl::platform::WindowSettings
impl core::marker::Freeze for mirl::platform::WindowSettings
impl core::marker::Send for mirl::platform::WindowSettings
impl core::marker::Sync for mirl::platform::WindowSettings
impl core::marker::Unpin for mirl::platform::WindowSettings
impl core::panic::unwind_safe::RefUnwindSafe for mirl::platform::WindowSettings
impl core::panic::unwind_safe::UnwindSafe for mirl::platform::WindowSettings
impl<T, U> core::convert::Into<U> for mirl::platform::WindowSettings where U: core::convert::From<T>
pub fn mirl::platform::WindowSettings::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::platform::WindowSettings where U: core::convert::Into<T>
pub type mirl::platform::WindowSettings::Error = core::convert::Infallible
pub fn mirl::platform::WindowSettings::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::platform::WindowSettings where U: core::convert::TryFrom<T>
pub type mirl::platform::WindowSettings::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::platform::WindowSettings::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::platform::WindowSettings where T: core::clone::Clone
pub type mirl::platform::WindowSettings::Owned = T
pub fn mirl::platform::WindowSettings::clone_into(&self, target: &mut T)
pub fn mirl::platform::WindowSettings::to_owned(&self) -> T
impl<T> core::any::Any for mirl::platform::WindowSettings where T: 'static + ?core::marker::Sized
pub fn mirl::platform::WindowSettings::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::platform::WindowSettings where T: ?core::marker::Sized
pub fn mirl::platform::WindowSettings::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::platform::WindowSettings where T: ?core::marker::Sized
pub fn mirl::platform::WindowSettings::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::platform::WindowSettings where T: core::clone::Clone
pub unsafe fn mirl::platform::WindowSettings::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::platform::WindowSettings
pub fn mirl::platform::WindowSettings::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::platform::WindowSettings
pub type mirl::platform::WindowSettings::Init = T
pub const mirl::platform::WindowSettings::ALIGN: usize
pub unsafe fn mirl::platform::WindowSettings::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::platform::WindowSettings::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::platform::WindowSettings::drop(ptr: usize)
pub unsafe fn mirl::platform::WindowSettings::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::platform::WindowSettings
impl<T> mirl::extensions::RepeatData for mirl::platform::WindowSettings where T: core::clone::Clone
pub fn mirl::platform::WindowSettings::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub const mirl::platform::AVAILABLE_KEYS: &[mirl::platform::KeyCode]
pub const mirl::platform::AVAILABLE_KEY_NAMES: &[&str]
pub trait mirl::platform::Time
pub fn mirl::platform::Time::get_elapsed_time(&self) -> f64
impl mirl::platform::Time for mirl::platform::time::NativeTime
pub fn mirl::platform::time::NativeTime::get_elapsed_time(&self) -> f64
pub const fn mirl::platform::get_available_key_names() -> &'static [&'static str]
pub const fn mirl::platform::get_available_keys() -> &'static [mirl::platform::KeyCode]
pub fn mirl::platform::keycodes_to_str(keycodes: &alloc::vec::Vec<mirl::platform::KeyCode>) -> (alloc::string::String, alloc::vec::Vec<mirl::platform::KeyCode>)
pub mod mirl::render
pub mod mirl::render::extra
pub struct mirl::render::extra::Point2D
pub mirl::render::extra::Point2D::x: f64
pub mirl::render::extra::Point2D::y: f64
impl core::clone::Clone for mirl::render::extra::Point2D
pub fn mirl::render::extra::Point2D::clone(&self) -> mirl::render::extra::Point2D
impl core::cmp::PartialEq for mirl::render::extra::Point2D
pub fn mirl::render::extra::Point2D::eq(&self, other: &mirl::render::extra::Point2D) -> bool
impl core::fmt::Debug for mirl::render::extra::Point2D
pub fn mirl::render::extra::Point2D::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::render::extra::Point2D
impl core::marker::StructuralPartialEq for mirl::render::extra::Point2D
impl core::marker::Freeze for mirl::render::extra::Point2D
impl core::marker::Send for mirl::render::extra::Point2D
impl core::marker::Sync for mirl::render::extra::Point2D
impl core::marker::Unpin for mirl::render::extra::Point2D
impl core::panic::unwind_safe::RefUnwindSafe for mirl::render::extra::Point2D
impl core::panic::unwind_safe::UnwindSafe for mirl::render::extra::Point2D
impl<T, U> core::convert::Into<U> for mirl::render::extra::Point2D where U: core::convert::From<T>
pub fn mirl::render::extra::Point2D::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::render::extra::Point2D where U: core::convert::Into<T>
pub type mirl::render::extra::Point2D::Error = core::convert::Infallible
pub fn mirl::render::extra::Point2D::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::render::extra::Point2D where U: core::convert::TryFrom<T>
pub type mirl::render::extra::Point2D::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::render::extra::Point2D::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::render::extra::Point2D where T: core::clone::Clone
pub type mirl::render::extra::Point2D::Owned = T
pub fn mirl::render::extra::Point2D::clone_into(&self, target: &mut T)
pub fn mirl::render::extra::Point2D::to_owned(&self) -> T
impl<T> core::any::Any for mirl::render::extra::Point2D where T: 'static + ?core::marker::Sized
pub fn mirl::render::extra::Point2D::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::render::extra::Point2D where T: ?core::marker::Sized
pub fn mirl::render::extra::Point2D::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::render::extra::Point2D where T: ?core::marker::Sized
pub fn mirl::render::extra::Point2D::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::render::extra::Point2D where T: core::clone::Clone
pub unsafe fn mirl::render::extra::Point2D::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::render::extra::Point2D
pub fn mirl::render::extra::Point2D::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::render::extra::Point2D
pub type mirl::render::extra::Point2D::Init = T
pub const mirl::render::extra::Point2D::ALIGN: usize
pub unsafe fn mirl::render::extra::Point2D::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::render::extra::Point2D::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::render::extra::Point2D::drop(ptr: usize)
pub unsafe fn mirl::render::extra::Point2D::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::render::extra::Point2D
impl<T> mirl::extensions::RepeatData for mirl::render::extra::Point2D where T: core::clone::Clone
pub fn mirl::render::extra::Point2D::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::render::extra::Point3D
pub mirl::render::extra::Point3D::x: f64
pub mirl::render::extra::Point3D::y: f64
pub mirl::render::extra::Point3D::z: f64
impl core::clone::Clone for mirl::render::extra::Point3D
pub fn mirl::render::extra::Point3D::clone(&self) -> mirl::render::extra::Point3D
impl core::cmp::PartialEq for mirl::render::extra::Point3D
pub fn mirl::render::extra::Point3D::eq(&self, other: &mirl::render::extra::Point3D) -> bool
impl core::fmt::Debug for mirl::render::extra::Point3D
pub fn mirl::render::extra::Point3D::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::render::extra::Point3D
impl core::marker::StructuralPartialEq for mirl::render::extra::Point3D
impl core::marker::Freeze for mirl::render::extra::Point3D
impl core::marker::Send for mirl::render::extra::Point3D
impl core::marker::Sync for mirl::render::extra::Point3D
impl core::marker::Unpin for mirl::render::extra::Point3D
impl core::panic::unwind_safe::RefUnwindSafe for mirl::render::extra::Point3D
impl core::panic::unwind_safe::UnwindSafe for mirl::render::extra::Point3D
impl<T, U> core::convert::Into<U> for mirl::render::extra::Point3D where U: core::convert::From<T>
pub fn mirl::render::extra::Point3D::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::render::extra::Point3D where U: core::convert::Into<T>
pub type mirl::render::extra::Point3D::Error = core::convert::Infallible
pub fn mirl::render::extra::Point3D::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::render::extra::Point3D where U: core::convert::TryFrom<T>
pub type mirl::render::extra::Point3D::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::render::extra::Point3D::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::render::extra::Point3D where T: core::clone::Clone
pub type mirl::render::extra::Point3D::Owned = T
pub fn mirl::render::extra::Point3D::clone_into(&self, target: &mut T)
pub fn mirl::render::extra::Point3D::to_owned(&self) -> T
impl<T> core::any::Any for mirl::render::extra::Point3D where T: 'static + ?core::marker::Sized
pub fn mirl::render::extra::Point3D::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::render::extra::Point3D where T: ?core::marker::Sized
pub fn mirl::render::extra::Point3D::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::render::extra::Point3D where T: ?core::marker::Sized
pub fn mirl::render::extra::Point3D::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::render::extra::Point3D where T: core::clone::Clone
pub unsafe fn mirl::render::extra::Point3D::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::render::extra::Point3D
pub fn mirl::render::extra::Point3D::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::render::extra::Point3D
pub type mirl::render::extra::Point3D::Init = T
pub const mirl::render::extra::Point3D::ALIGN: usize
pub unsafe fn mirl::render::extra::Point3D::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::render::extra::Point3D::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::render::extra::Point3D::drop(ptr: usize)
pub unsafe fn mirl::render::extra::Point3D::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::render::extra::Point3D
impl<T> mirl::extensions::RepeatData for mirl::render::extra::Point3D where T: core::clone::Clone
pub fn mirl::render::extra::Point3D::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::render::extra::Polygon
pub mirl::render::extra::Polygon::point1: mirl::render::extra::Vertex3D
pub mirl::render::extra::Polygon::point2: mirl::render::extra::Vertex3D
pub mirl::render::extra::Polygon::point3: mirl::render::extra::Vertex3D
impl core::clone::Clone for mirl::render::extra::Polygon
pub fn mirl::render::extra::Polygon::clone(&self) -> mirl::render::extra::Polygon
impl core::cmp::PartialEq for mirl::render::extra::Polygon
pub fn mirl::render::extra::Polygon::eq(&self, other: &mirl::render::extra::Polygon) -> bool
impl core::fmt::Debug for mirl::render::extra::Polygon
pub fn mirl::render::extra::Polygon::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::render::extra::Polygon
impl core::marker::StructuralPartialEq for mirl::render::extra::Polygon
impl core::marker::Freeze for mirl::render::extra::Polygon
impl core::marker::Send for mirl::render::extra::Polygon
impl core::marker::Sync for mirl::render::extra::Polygon
impl core::marker::Unpin for mirl::render::extra::Polygon
impl core::panic::unwind_safe::RefUnwindSafe for mirl::render::extra::Polygon
impl core::panic::unwind_safe::UnwindSafe for mirl::render::extra::Polygon
impl<T, U> core::convert::Into<U> for mirl::render::extra::Polygon where U: core::convert::From<T>
pub fn mirl::render::extra::Polygon::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::render::extra::Polygon where U: core::convert::Into<T>
pub type mirl::render::extra::Polygon::Error = core::convert::Infallible
pub fn mirl::render::extra::Polygon::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::render::extra::Polygon where U: core::convert::TryFrom<T>
pub type mirl::render::extra::Polygon::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::render::extra::Polygon::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::render::extra::Polygon where T: core::clone::Clone
pub type mirl::render::extra::Polygon::Owned = T
pub fn mirl::render::extra::Polygon::clone_into(&self, target: &mut T)
pub fn mirl::render::extra::Polygon::to_owned(&self) -> T
impl<T> core::any::Any for mirl::render::extra::Polygon where T: 'static + ?core::marker::Sized
pub fn mirl::render::extra::Polygon::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::render::extra::Polygon where T: ?core::marker::Sized
pub fn mirl::render::extra::Polygon::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::render::extra::Polygon where T: ?core::marker::Sized
pub fn mirl::render::extra::Polygon::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::render::extra::Polygon where T: core::clone::Clone
pub unsafe fn mirl::render::extra::Polygon::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::render::extra::Polygon
pub fn mirl::render::extra::Polygon::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::render::extra::Polygon
pub type mirl::render::extra::Polygon::Init = T
pub const mirl::render::extra::Polygon::ALIGN: usize
pub unsafe fn mirl::render::extra::Polygon::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::render::extra::Polygon::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::render::extra::Polygon::drop(ptr: usize)
pub unsafe fn mirl::render::extra::Polygon::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::render::extra::Polygon
impl<T> mirl::extensions::RepeatData for mirl::render::extra::Polygon where T: core::clone::Clone
pub fn mirl::render::extra::Polygon::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::render::extra::Vertex3D
pub mirl::render::extra::Vertex3D::u: f32
pub mirl::render::extra::Vertex3D::v: f32
pub mirl::render::extra::Vertex3D::x: f64
pub mirl::render::extra::Vertex3D::y: f64
pub mirl::render::extra::Vertex3D::z: f64
impl core::clone::Clone for mirl::render::extra::Vertex3D
pub fn mirl::render::extra::Vertex3D::clone(&self) -> mirl::render::extra::Vertex3D
impl core::cmp::PartialEq for mirl::render::extra::Vertex3D
pub fn mirl::render::extra::Vertex3D::eq(&self, other: &mirl::render::extra::Vertex3D) -> bool
impl core::fmt::Debug for mirl::render::extra::Vertex3D
pub fn mirl::render::extra::Vertex3D::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::render::extra::Vertex3D
impl core::marker::StructuralPartialEq for mirl::render::extra::Vertex3D
impl core::marker::Freeze for mirl::render::extra::Vertex3D
impl core::marker::Send for mirl::render::extra::Vertex3D
impl core::marker::Sync for mirl::render::extra::Vertex3D
impl core::marker::Unpin for mirl::render::extra::Vertex3D
impl core::panic::unwind_safe::RefUnwindSafe for mirl::render::extra::Vertex3D
impl core::panic::unwind_safe::UnwindSafe for mirl::render::extra::Vertex3D
impl<T, U> core::convert::Into<U> for mirl::render::extra::Vertex3D where U: core::convert::From<T>
pub fn mirl::render::extra::Vertex3D::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::render::extra::Vertex3D where U: core::convert::Into<T>
pub type mirl::render::extra::Vertex3D::Error = core::convert::Infallible
pub fn mirl::render::extra::Vertex3D::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::render::extra::Vertex3D where U: core::convert::TryFrom<T>
pub type mirl::render::extra::Vertex3D::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::render::extra::Vertex3D::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::render::extra::Vertex3D where T: core::clone::Clone
pub type mirl::render::extra::Vertex3D::Owned = T
pub fn mirl::render::extra::Vertex3D::clone_into(&self, target: &mut T)
pub fn mirl::render::extra::Vertex3D::to_owned(&self) -> T
impl<T> core::any::Any for mirl::render::extra::Vertex3D where T: 'static + ?core::marker::Sized
pub fn mirl::render::extra::Vertex3D::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::render::extra::Vertex3D where T: ?core::marker::Sized
pub fn mirl::render::extra::Vertex3D::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::render::extra::Vertex3D where T: ?core::marker::Sized
pub fn mirl::render::extra::Vertex3D::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::render::extra::Vertex3D where T: core::clone::Clone
pub unsafe fn mirl::render::extra::Vertex3D::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::render::extra::Vertex3D
pub fn mirl::render::extra::Vertex3D::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::render::extra::Vertex3D
pub type mirl::render::extra::Vertex3D::Init = T
pub const mirl::render::extra::Vertex3D::ALIGN: usize
pub unsafe fn mirl::render::extra::Vertex3D::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::render::extra::Vertex3D::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::render::extra::Vertex3D::drop(ptr: usize)
pub unsafe fn mirl::render::extra::Vertex3D::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::render::extra::Vertex3D
impl<T> mirl::extensions::RepeatData for mirl::render::extra::Vertex3D where T: core::clone::Clone
pub fn mirl::render::extra::Vertex3D::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub fn mirl::render::extra::rotate_x_polygon_3d(angle_degrees: f64, rotation_center: mirl::render::extra::Point3D, polygon: &mut mirl::render::extra::Polygon)
pub fn mirl::render::extra::rotate_x_vertex_3d(angle_degrees: f64, rotation_center: mirl::render::extra::Point3D, vertex: &mut mirl::render::extra::Vertex3D)
pub fn mirl::render::extra::rotate_y_polygon_3d(angle_degrees: f64, rotation_center: mirl::render::extra::Point3D, polygon: &mut mirl::render::extra::Polygon)
pub fn mirl::render::extra::rotate_y_vertex_3d(angle_degrees: f64, rotation_center: mirl::render::extra::Point3D, vertex: &mut mirl::render::extra::Vertex3D)
pub fn mirl::render::extra::rotate_z_polygon_3d(angle_degrees: f64, rotation_center: mirl::render::extra::Point3D, polygon: &mut mirl::render::extra::Polygon)
pub fn mirl::render::extra::rotate_z_vertex_3d(angle_degrees: f64, rotation_center: mirl::render::extra::Point3D, vertex: &mut mirl::render::extra::Vertex3D)
pub fn mirl::render::extra::set_pixel_safe(buffer: *mut u32, width: usize, height: usize, x: usize, y: usize, color: u32)
pub fn mirl::render::extra::set_pixel_unsafe(buffer: *mut u32, width: usize, x: usize, y: usize, color: u32)
pub fn mirl::render::extra::uv_interpolate(target_y: f32, start_y: f32, start_val: f32, end_y: f32, end_val: f32) -> f32
pub fn mirl::render::extra::vertex_3d_to_2d(vertex: &mirl::render::extra::Vertex3D, width: usize, height: usize) -> (isize, isize)
pub fn mirl::render::_add_to_glyph_cache(key: (char, (i32, i32), usize), data: (fontdue::font::Metrics, alloc::vec::Vec<u8>))
pub fn mirl::render::_get_glyph_cache() -> &'static std::sync::poison::rwlock::RwLock<std::collections::hash::map::HashMap<(char, (i32, i32), usize), (fontdue::font::Metrics, alloc::vec::Vec<u8>)>>
pub fn mirl::render::_remove_glyph_from_glyph_cache(glyph: &(char, (i32, i32), usize))
pub fn mirl::render::_reset_glyph_cache()
pub fn mirl::render::draw_buffer_on_buffer<const SAFE: bool, const TRANSPARENCY: bool, const TRANSPARENCY_CHECK: bool>(canvas: &mirl::platform::Buffer, texture: &mirl::platform::Buffer, position: (isize, isize), result_dimensions: (usize, usize))
pub fn mirl::render::draw_buffer_on_buffer_1_to_1<const SAFE: bool, const TRANSPARENCY: bool, const TRANSPARENCY_CHECK: bool>(canvas: &mirl::platform::Buffer, texture: &mirl::platform::Buffer, position: (isize, isize))
pub fn mirl::render::draw_circle<const FIX_STRAY_PIXEL: bool>(buffer: &mirl::platform::Buffer, pos_x: usize, pos_y: usize, radius: isize, color: u32, safe: bool)
pub fn mirl::render::draw_circle_outline(buffer: &mirl::platform::Buffer, pos_x: usize, pos_y: usize, radius: isize, color: u32, safe: bool)
pub fn mirl::render::draw_line(buffer: &mirl::platform::Buffer, start: (usize, usize), end: (usize, usize), color: u32, thickness: isize, safe: bool)
pub fn mirl::render::draw_line_straight(buffer: &mirl::platform::Buffer, start: (usize, usize), length: usize, vertical: bool, color: u32, thickness: isize, safe: bool)
pub fn mirl::render::draw_pixel_safe(buffer: &mirl::platform::Buffer, x: usize, y: usize, color: u32)
pub fn mirl::render::draw_pixel_unsafe(buffer: &mirl::platform::Buffer, x: usize, y: usize, color: u32)
pub fn mirl::render::draw_rectangle(buffer: &mirl::platform::Buffer, pos_x: isize, pos_y: isize, width: isize, height: isize, color: u32, safe: bool)
pub fn mirl::render::draw_rectangle_angled(buffer: &mirl::platform::Buffer, pos: (usize, usize), size: (isize, isize), color: u32, anchor_pos: (usize, usize), rotation: f32, safe: bool)
pub fn mirl::render::draw_rectangle_impl_simd(buffer: &mirl::platform::Buffer, pos_x: isize, pos_y: isize, width: isize, height: isize, color: u32, safe: bool)
pub fn mirl::render::draw_text(buffer: &mirl::platform::Buffer, text: &str, x: usize, y: usize, color: u32, size: f32, font: &fontdue::font::Font, safe: bool)
pub fn mirl::render::draw_text_antialiased(buffer: &mirl::platform::Buffer, text: &str, x: usize, y: usize, color: u32, size: f32, font: &fontdue::font::Font, safe: bool)
pub fn mirl::render::draw_text_antialiased_execute_at(buffer: &mirl::platform::Buffer, text: &str, x: usize, y: usize, color: u32, size: f32, font: &fontdue::font::Font, safe: bool, function: fn(original_color: u32, color_under_pixel: u32) -> u32)
pub fn mirl::render::draw_text_antialiased_isize(buffer: &mirl::platform::Buffer, text: &str, x: isize, y: isize, color: u32, size: f32, font: &fontdue::font::Font, safe: bool)
pub fn mirl::render::draw_text_antialiased_stretched<F: num_traits::float::Float>(buffer: &mirl::platform::Buffer, text: &str, x: usize, y: usize, color: u32, size: f32, font: &fontdue::font::Font, stretch_x: F, stretch_y: F, safe: bool)
pub fn mirl::render::draw_text_antialiased_stretched_isize<F: num_traits::float::Float>(buffer: &mirl::platform::Buffer, text: &str, x: isize, y: isize, color: u32, size: f32, font: &fontdue::font::Font, stretch_x: F, stretch_y: F, safe: bool)
pub fn mirl::render::draw_text_isize(buffer: &mirl::platform::Buffer, text: &str, x: isize, y: isize, color: u32, size: f32, font: &fontdue::font::Font, safe: bool)
pub fn mirl::render::draw_text_stretch_isize<F: num_traits::float::Float>(buffer: &mirl::platform::Buffer, text: &str, x: isize, y: isize, color: u32, size: f32, font: &fontdue::font::Font, draw_pixel: fn(&mirl::platform::Buffer, usize, usize, u32), stretch_x: F, stretch_y: F)
pub fn mirl::render::draw_text_stretched<F: num_traits::float::Float>(buffer: &mirl::platform::Buffer, text: &str, x: usize, y: usize, color: u32, size: f32, font: &fontdue::font::Font, draw_pixel: fn(&mirl::platform::Buffer, usize, usize, u32), stretch_x: F, stretch_y: F)
pub fn mirl::render::draw_triangle(buffer: &mirl::platform::Buffer, width: usize, height: usize, point1: (isize, isize, f32, f32), point2: (isize, isize, f32, f32), point3: (isize, isize, f32, f32), texture: &mirl::platform::Buffer)
pub fn mirl::render::execute_at_rectangle(buffer: &mirl::platform::Buffer, pos: (isize, isize), size: (isize, isize), color: u32, safe: bool, function: fn(&mirl::platform::Buffer, usize, usize, u32))
pub fn mirl::render::get_character(ch: char, size: f32, font: &fontdue::font::Font) -> (fontdue::font::Metrics, alloc::vec::Vec<u8>)
pub fn mirl::render::get_text_height(string: &str, size: f32, font: &fontdue::font::Font) -> f32
pub fn mirl::render::get_text_width(string: &str, size: f32, font: &fontdue::font::Font) -> f32
pub mod mirl::system
pub mod mirl::system::action
pub mod mirl::system::action::windows_actions
pub struct mirl::system::action::windows_actions::WindowsActions
impl core::clone::Clone for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::clone(&self) -> mirl::system::action::windows_actions::WindowsActions
impl core::cmp::Eq for mirl::system::action::windows_actions::WindowsActions
impl core::cmp::PartialEq for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::eq(&self, other: &mirl::system::action::windows_actions::WindowsActions) -> bool
impl core::fmt::Debug for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::system::action::windows_actions::WindowsActions
impl core::marker::StructuralPartialEq for mirl::system::action::windows_actions::WindowsActions
impl mirl::system::action::Decoration for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_borderless(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool
impl mirl::system::action::Default for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_position(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32)
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_size(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32)
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_level(handle: &raw_window_handle::RawWindowHandle, level: mirl::platform::WindowLevel) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_position(handle: &raw_window_handle::RawWindowHandle, x: i32, y: i32) -> bool
impl mirl::system::action::Misc for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::capture_desktop_background() -> core::option::Option<mirl::platform::Buffer>
pub fn mirl::system::action::windows_actions::WindowsActions::capture_screen() -> core::option::Option<mirl::platform::Buffer>
pub fn mirl::system::action::windows_actions::WindowsActions::get_all_windows() -> alloc::vec::Vec<raw_window_handle::RawWindowHandle>
pub fn mirl::system::action::windows_actions::WindowsActions::get_id_using_title(title: &str, exact_match: bool, case_sensitive: bool, include_hidden: bool, just_one: bool) -> core::option::Option<alloc::vec::Vec<raw_window_handle::RawWindowHandle>>
pub fn mirl::system::action::windows_actions::WindowsActions::get_title_using_id(handle: &raw_window_handle::RawWindowHandle) -> alloc::string::String
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_z(handle: &raw_window_handle::RawWindowHandle) -> u32
pub fn mirl::system::action::windows_actions::WindowsActions::set_click_ability_of_window(handle: &raw_window_handle::RawWindowHandle, click_through: bool)
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_hidden_from_taskbar_and_alt_tab(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_z(handle: &raw_window_handle::RawWindowHandle, z: u32) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_z_after(handle: &raw_window_handle::RawWindowHandle, after: &raw_window_handle::RawWindowHandle) -> bool
impl mirl::system::action::TaskBar for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::set_icon_progress(handle: &raw_window_handle::RawWindowHandle, current: u64, total: u64)
pub fn mirl::system::action::windows_actions::WindowsActions::set_icon_state(handle: &raw_window_handle::RawWindowHandle, state: &mirl::system::action::ProgressionState)
impl mirl::system::action::Transparency for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::make_color_transparent(handle: &raw_window_handle::RawWindowHandle, color: (u8, u8, u8)) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_opacity(handle: &raw_window_handle::RawWindowHandle, opacity: u8) -> bool
impl core::marker::Freeze for mirl::system::action::windows_actions::WindowsActions
impl core::marker::Send for mirl::system::action::windows_actions::WindowsActions
impl core::marker::Sync for mirl::system::action::windows_actions::WindowsActions
impl core::marker::Unpin for mirl::system::action::windows_actions::WindowsActions
impl core::panic::unwind_safe::RefUnwindSafe for mirl::system::action::windows_actions::WindowsActions
impl core::panic::unwind_safe::UnwindSafe for mirl::system::action::windows_actions::WindowsActions
impl<Q, K> equivalent::Equivalent<K> for mirl::system::action::windows_actions::WindowsActions where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::system::action::windows_actions::WindowsActions where U: core::convert::From<T>
pub fn mirl::system::action::windows_actions::WindowsActions::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::system::action::windows_actions::WindowsActions where U: core::convert::Into<T>
pub type mirl::system::action::windows_actions::WindowsActions::Error = core::convert::Infallible
pub fn mirl::system::action::windows_actions::WindowsActions::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::system::action::windows_actions::WindowsActions where U: core::convert::TryFrom<T>
pub type mirl::system::action::windows_actions::WindowsActions::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::system::action::windows_actions::WindowsActions::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::system::action::windows_actions::WindowsActions where T: core::clone::Clone
pub type mirl::system::action::windows_actions::WindowsActions::Owned = T
pub fn mirl::system::action::windows_actions::WindowsActions::clone_into(&self, target: &mut T)
pub fn mirl::system::action::windows_actions::WindowsActions::to_owned(&self) -> T
impl<T> core::any::Any for mirl::system::action::windows_actions::WindowsActions where T: 'static + ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::system::action::windows_actions::WindowsActions where T: ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::system::action::windows_actions::WindowsActions where T: ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::system::action::windows_actions::WindowsActions where T: core::clone::Clone
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::system::action::windows_actions::WindowsActions
pub type mirl::system::action::windows_actions::WindowsActions::Init = T
pub const mirl::system::action::windows_actions::WindowsActions::ALIGN: usize
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::drop(ptr: usize)
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::system::action::windows_actions::WindowsActions
impl<T> mirl::extensions::RepeatData for mirl::system::action::windows_actions::WindowsActions where T: core::clone::Clone
pub fn mirl::system::action::windows_actions::WindowsActions::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub const fn mirl::system::action::windows_actions::map_state_to_windows_state(state: &mirl::system::action::ProgressionState) -> windows::Windows::Win32::UI::Shell::TBPFLAG
pub fn mirl::system::action::windows_actions::set_taskbar_progress_state(hwnd: windows::Windows::Win32::Foundation::HWND, state: &mirl::system::action::ProgressionState) -> windows::core::Result<()>
pub fn mirl::system::action::windows_actions::set_taskbar_progress_value(hwnd: windows::Windows::Win32::Foundation::HWND, completed: u64, total: u64) -> windows::core::Result<()>
pub enum mirl::system::action::ProgressionState
pub mirl::system::action::ProgressionState::Error
pub mirl::system::action::ProgressionState::Loading
pub mirl::system::action::ProgressionState::NoBar
pub mirl::system::action::ProgressionState::Normal
pub mirl::system::action::ProgressionState::Paused
impl core::clone::Clone for mirl::system::action::ProgressionState
pub fn mirl::system::action::ProgressionState::clone(&self) -> mirl::system::action::ProgressionState
impl core::cmp::Eq for mirl::system::action::ProgressionState
impl core::cmp::PartialEq for mirl::system::action::ProgressionState
pub fn mirl::system::action::ProgressionState::eq(&self, other: &mirl::system::action::ProgressionState) -> bool
impl core::fmt::Debug for mirl::system::action::ProgressionState
pub fn mirl::system::action::ProgressionState::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::system::action::ProgressionState
impl core::marker::StructuralPartialEq for mirl::system::action::ProgressionState
impl core::marker::Freeze for mirl::system::action::ProgressionState
impl core::marker::Send for mirl::system::action::ProgressionState
impl core::marker::Sync for mirl::system::action::ProgressionState
impl core::marker::Unpin for mirl::system::action::ProgressionState
impl core::panic::unwind_safe::RefUnwindSafe for mirl::system::action::ProgressionState
impl core::panic::unwind_safe::UnwindSafe for mirl::system::action::ProgressionState
impl<Q, K> equivalent::Equivalent<K> for mirl::system::action::ProgressionState where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::system::action::ProgressionState::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::system::action::ProgressionState where U: core::convert::From<T>
pub fn mirl::system::action::ProgressionState::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::system::action::ProgressionState where U: core::convert::Into<T>
pub type mirl::system::action::ProgressionState::Error = core::convert::Infallible
pub fn mirl::system::action::ProgressionState::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::system::action::ProgressionState where U: core::convert::TryFrom<T>
pub type mirl::system::action::ProgressionState::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::system::action::ProgressionState::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::system::action::ProgressionState where T: core::clone::Clone
pub type mirl::system::action::ProgressionState::Owned = T
pub fn mirl::system::action::ProgressionState::clone_into(&self, target: &mut T)
pub fn mirl::system::action::ProgressionState::to_owned(&self) -> T
impl<T> core::any::Any for mirl::system::action::ProgressionState where T: 'static + ?core::marker::Sized
pub fn mirl::system::action::ProgressionState::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::system::action::ProgressionState where T: ?core::marker::Sized
pub fn mirl::system::action::ProgressionState::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::system::action::ProgressionState where T: ?core::marker::Sized
pub fn mirl::system::action::ProgressionState::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::system::action::ProgressionState where T: core::clone::Clone
pub unsafe fn mirl::system::action::ProgressionState::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::system::action::ProgressionState
pub fn mirl::system::action::ProgressionState::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::system::action::ProgressionState
pub type mirl::system::action::ProgressionState::Init = T
pub const mirl::system::action::ProgressionState::ALIGN: usize
pub unsafe fn mirl::system::action::ProgressionState::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::system::action::ProgressionState::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::system::action::ProgressionState::drop(ptr: usize)
pub unsafe fn mirl::system::action::ProgressionState::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::system::action::ProgressionState
impl<T> mirl::extensions::RepeatData for mirl::system::action::ProgressionState where T: core::clone::Clone
pub fn mirl::system::action::ProgressionState::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::system::action::OsActions
impl core::clone::Clone for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::clone(&self) -> mirl::system::action::windows_actions::WindowsActions
impl core::cmp::Eq for mirl::system::action::windows_actions::WindowsActions
impl core::cmp::PartialEq for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::eq(&self, other: &mirl::system::action::windows_actions::WindowsActions) -> bool
impl core::fmt::Debug for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::system::action::windows_actions::WindowsActions
impl core::marker::StructuralPartialEq for mirl::system::action::windows_actions::WindowsActions
impl mirl::system::action::Decoration for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_borderless(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool
impl mirl::system::action::Default for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_position(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32)
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_size(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32)
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_level(handle: &raw_window_handle::RawWindowHandle, level: mirl::platform::WindowLevel) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_position(handle: &raw_window_handle::RawWindowHandle, x: i32, y: i32) -> bool
impl mirl::system::action::Misc for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::capture_desktop_background() -> core::option::Option<mirl::platform::Buffer>
pub fn mirl::system::action::windows_actions::WindowsActions::capture_screen() -> core::option::Option<mirl::platform::Buffer>
pub fn mirl::system::action::windows_actions::WindowsActions::get_all_windows() -> alloc::vec::Vec<raw_window_handle::RawWindowHandle>
pub fn mirl::system::action::windows_actions::WindowsActions::get_id_using_title(title: &str, exact_match: bool, case_sensitive: bool, include_hidden: bool, just_one: bool) -> core::option::Option<alloc::vec::Vec<raw_window_handle::RawWindowHandle>>
pub fn mirl::system::action::windows_actions::WindowsActions::get_title_using_id(handle: &raw_window_handle::RawWindowHandle) -> alloc::string::String
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_z(handle: &raw_window_handle::RawWindowHandle) -> u32
pub fn mirl::system::action::windows_actions::WindowsActions::set_click_ability_of_window(handle: &raw_window_handle::RawWindowHandle, click_through: bool)
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_hidden_from_taskbar_and_alt_tab(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_z(handle: &raw_window_handle::RawWindowHandle, z: u32) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_z_after(handle: &raw_window_handle::RawWindowHandle, after: &raw_window_handle::RawWindowHandle) -> bool
impl mirl::system::action::TaskBar for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::set_icon_progress(handle: &raw_window_handle::RawWindowHandle, current: u64, total: u64)
pub fn mirl::system::action::windows_actions::WindowsActions::set_icon_state(handle: &raw_window_handle::RawWindowHandle, state: &mirl::system::action::ProgressionState)
impl mirl::system::action::Transparency for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::make_color_transparent(handle: &raw_window_handle::RawWindowHandle, color: (u8, u8, u8)) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_opacity(handle: &raw_window_handle::RawWindowHandle, opacity: u8) -> bool
impl core::marker::Freeze for mirl::system::action::windows_actions::WindowsActions
impl core::marker::Send for mirl::system::action::windows_actions::WindowsActions
impl core::marker::Sync for mirl::system::action::windows_actions::WindowsActions
impl core::marker::Unpin for mirl::system::action::windows_actions::WindowsActions
impl core::panic::unwind_safe::RefUnwindSafe for mirl::system::action::windows_actions::WindowsActions
impl core::panic::unwind_safe::UnwindSafe for mirl::system::action::windows_actions::WindowsActions
impl<Q, K> equivalent::Equivalent<K> for mirl::system::action::windows_actions::WindowsActions where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::system::action::windows_actions::WindowsActions where U: core::convert::From<T>
pub fn mirl::system::action::windows_actions::WindowsActions::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::system::action::windows_actions::WindowsActions where U: core::convert::Into<T>
pub type mirl::system::action::windows_actions::WindowsActions::Error = core::convert::Infallible
pub fn mirl::system::action::windows_actions::WindowsActions::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::system::action::windows_actions::WindowsActions where U: core::convert::TryFrom<T>
pub type mirl::system::action::windows_actions::WindowsActions::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::system::action::windows_actions::WindowsActions::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::system::action::windows_actions::WindowsActions where T: core::clone::Clone
pub type mirl::system::action::windows_actions::WindowsActions::Owned = T
pub fn mirl::system::action::windows_actions::WindowsActions::clone_into(&self, target: &mut T)
pub fn mirl::system::action::windows_actions::WindowsActions::to_owned(&self) -> T
impl<T> core::any::Any for mirl::system::action::windows_actions::WindowsActions where T: 'static + ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::system::action::windows_actions::WindowsActions where T: ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::system::action::windows_actions::WindowsActions where T: ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::system::action::windows_actions::WindowsActions where T: core::clone::Clone
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::system::action::windows_actions::WindowsActions
pub type mirl::system::action::windows_actions::WindowsActions::Init = T
pub const mirl::system::action::windows_actions::WindowsActions::ALIGN: usize
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::drop(ptr: usize)
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::system::action::windows_actions::WindowsActions
impl<T> mirl::extensions::RepeatData for mirl::system::action::windows_actions::WindowsActions where T: core::clone::Clone
pub fn mirl::system::action::windows_actions::WindowsActions::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::system::action::ToolbarTool
impl core::clone::Clone for mirl::system::action::ToolbarTool
pub fn mirl::system::action::ToolbarTool::clone(&self) -> mirl::system::action::ToolbarTool
impl core::cmp::Eq for mirl::system::action::ToolbarTool
impl core::cmp::PartialEq for mirl::system::action::ToolbarTool
pub fn mirl::system::action::ToolbarTool::eq(&self, other: &mirl::system::action::ToolbarTool) -> bool
impl core::fmt::Debug for mirl::system::action::ToolbarTool
pub fn mirl::system::action::ToolbarTool::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::system::action::ToolbarTool
impl core::marker::StructuralPartialEq for mirl::system::action::ToolbarTool
impl core::marker::Freeze for mirl::system::action::ToolbarTool
impl core::marker::Send for mirl::system::action::ToolbarTool
impl core::marker::Sync for mirl::system::action::ToolbarTool
impl core::marker::Unpin for mirl::system::action::ToolbarTool
impl core::panic::unwind_safe::RefUnwindSafe for mirl::system::action::ToolbarTool
impl core::panic::unwind_safe::UnwindSafe for mirl::system::action::ToolbarTool
impl<Q, K> equivalent::Equivalent<K> for mirl::system::action::ToolbarTool where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::system::action::ToolbarTool::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::system::action::ToolbarTool where U: core::convert::From<T>
pub fn mirl::system::action::ToolbarTool::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::system::action::ToolbarTool where U: core::convert::Into<T>
pub type mirl::system::action::ToolbarTool::Error = core::convert::Infallible
pub fn mirl::system::action::ToolbarTool::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::system::action::ToolbarTool where U: core::convert::TryFrom<T>
pub type mirl::system::action::ToolbarTool::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::system::action::ToolbarTool::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::system::action::ToolbarTool where T: core::clone::Clone
pub type mirl::system::action::ToolbarTool::Owned = T
pub fn mirl::system::action::ToolbarTool::clone_into(&self, target: &mut T)
pub fn mirl::system::action::ToolbarTool::to_owned(&self) -> T
impl<T> core::any::Any for mirl::system::action::ToolbarTool where T: 'static + ?core::marker::Sized
pub fn mirl::system::action::ToolbarTool::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::system::action::ToolbarTool where T: ?core::marker::Sized
pub fn mirl::system::action::ToolbarTool::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::system::action::ToolbarTool where T: ?core::marker::Sized
pub fn mirl::system::action::ToolbarTool::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::system::action::ToolbarTool where T: core::clone::Clone
pub unsafe fn mirl::system::action::ToolbarTool::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::system::action::ToolbarTool
pub fn mirl::system::action::ToolbarTool::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::system::action::ToolbarTool
pub type mirl::system::action::ToolbarTool::Init = T
pub const mirl::system::action::ToolbarTool::ALIGN: usize
pub unsafe fn mirl::system::action::ToolbarTool::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::system::action::ToolbarTool::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::system::action::ToolbarTool::drop(ptr: usize)
pub unsafe fn mirl::system::action::ToolbarTool::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::system::action::ToolbarTool
impl<T> mirl::extensions::RepeatData for mirl::system::action::ToolbarTool where T: core::clone::Clone
pub fn mirl::system::action::ToolbarTool::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub trait mirl::system::action::Decoration
pub fn mirl::system::action::Decoration::set_window_borderless(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool
impl mirl::system::action::Decoration for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_borderless(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool
pub trait mirl::system::action::Default
pub fn mirl::system::action::Default::get_window_position(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32)
pub fn mirl::system::action::Default::get_window_size(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32)
pub fn mirl::system::action::Default::set_window_level(handle: &raw_window_handle::RawWindowHandle, level: mirl::platform::WindowLevel) -> bool
pub fn mirl::system::action::Default::set_window_position(handle: &raw_window_handle::RawWindowHandle, x: i32, y: i32) -> bool
impl mirl::system::action::Default for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_position(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32)
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_size(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32)
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_level(handle: &raw_window_handle::RawWindowHandle, level: mirl::platform::WindowLevel) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_position(handle: &raw_window_handle::RawWindowHandle, x: i32, y: i32) -> bool
pub trait mirl::system::action::Misc
pub fn mirl::system::action::Misc::capture_desktop_background() -> core::option::Option<mirl::platform::Buffer>
pub fn mirl::system::action::Misc::capture_screen() -> core::option::Option<mirl::platform::Buffer>
pub fn mirl::system::action::Misc::get_all_windows() -> alloc::vec::Vec<raw_window_handle::RawWindowHandle>
pub fn mirl::system::action::Misc::get_id_using_title(title: &str, exact_match: bool, case_sensitive: bool, include_hidden: bool, just_one: bool) -> core::option::Option<alloc::vec::Vec<raw_window_handle::RawWindowHandle>>
pub fn mirl::system::action::Misc::get_title_using_id(handle: &raw_window_handle::RawWindowHandle) -> alloc::string::String
pub fn mirl::system::action::Misc::get_window_z(handle: &raw_window_handle::RawWindowHandle) -> u32
pub fn mirl::system::action::Misc::set_click_ability_of_window(handle: &raw_window_handle::RawWindowHandle, click_through: bool)
pub fn mirl::system::action::Misc::set_window_hidden_from_taskbar_and_alt_tab(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool
pub fn mirl::system::action::Misc::set_window_z(handle: &raw_window_handle::RawWindowHandle, z: u32) -> bool
pub fn mirl::system::action::Misc::set_window_z_after(handle: &raw_window_handle::RawWindowHandle, after: &raw_window_handle::RawWindowHandle) -> bool
impl mirl::system::action::Misc for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::capture_desktop_background() -> core::option::Option<mirl::platform::Buffer>
pub fn mirl::system::action::windows_actions::WindowsActions::capture_screen() -> core::option::Option<mirl::platform::Buffer>
pub fn mirl::system::action::windows_actions::WindowsActions::get_all_windows() -> alloc::vec::Vec<raw_window_handle::RawWindowHandle>
pub fn mirl::system::action::windows_actions::WindowsActions::get_id_using_title(title: &str, exact_match: bool, case_sensitive: bool, include_hidden: bool, just_one: bool) -> core::option::Option<alloc::vec::Vec<raw_window_handle::RawWindowHandle>>
pub fn mirl::system::action::windows_actions::WindowsActions::get_title_using_id(handle: &raw_window_handle::RawWindowHandle) -> alloc::string::String
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_z(handle: &raw_window_handle::RawWindowHandle) -> u32
pub fn mirl::system::action::windows_actions::WindowsActions::set_click_ability_of_window(handle: &raw_window_handle::RawWindowHandle, click_through: bool)
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_hidden_from_taskbar_and_alt_tab(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_z(handle: &raw_window_handle::RawWindowHandle, z: u32) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_z_after(handle: &raw_window_handle::RawWindowHandle, after: &raw_window_handle::RawWindowHandle) -> bool
pub trait mirl::system::action::TaskBar
pub fn mirl::system::action::TaskBar::set_icon_progress(handle: &raw_window_handle::RawWindowHandle, current: u64, total: u64)
pub fn mirl::system::action::TaskBar::set_icon_state(handle: &raw_window_handle::RawWindowHandle, state: &mirl::system::action::ProgressionState)
impl mirl::system::action::TaskBar for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::set_icon_progress(handle: &raw_window_handle::RawWindowHandle, current: u64, total: u64)
pub fn mirl::system::action::windows_actions::WindowsActions::set_icon_state(handle: &raw_window_handle::RawWindowHandle, state: &mirl::system::action::ProgressionState)
pub trait mirl::system::action::Transparency
pub fn mirl::system::action::Transparency::make_color_transparent(handle: &raw_window_handle::RawWindowHandle, color: (u8, u8, u8)) -> bool
pub fn mirl::system::action::Transparency::set_window_opacity(handle: &raw_window_handle::RawWindowHandle, opacity: u8) -> bool
impl mirl::system::action::Transparency for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::make_color_transparent(handle: &raw_window_handle::RawWindowHandle, color: (u8, u8, u8)) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_opacity(handle: &raw_window_handle::RawWindowHandle, opacity: u8) -> bool
pub mod mirl::system::info
pub struct mirl::system::info::OsInfo
impl mirl::system::OsInfo
pub fn mirl::system::OsInfo::new() -> Self
impl core::clone::Clone for mirl::system::OsInfo
pub fn mirl::system::OsInfo::clone(&self) -> mirl::system::OsInfo
impl core::cmp::Eq for mirl::system::OsInfo
impl core::cmp::PartialEq for mirl::system::OsInfo
pub fn mirl::system::OsInfo::eq(&self, other: &mirl::system::OsInfo) -> bool
impl core::fmt::Debug for mirl::system::OsInfo
pub fn mirl::system::OsInfo::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::system::OsInfo
impl core::marker::StructuralPartialEq for mirl::system::OsInfo
impl mirl::system::info::Battery for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_battery_percentage(&self) -> core::option::Option<u8>
pub fn mirl::system::OsInfo::is_battery_charging() -> bool
pub fn mirl::system::OsInfo::is_in_low_power_mode() -> bool
impl mirl::system::info::Info for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_os_name() -> alloc::string::String
impl mirl::system::info::Memory for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_free_memory(&self) -> u64
pub fn mirl::system::OsInfo::get_total_memory(&self) -> u64
impl mirl::system::info::Screen for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_os_menu_height() -> i32
pub fn mirl::system::OsInfo::get_screen_resolution() -> (i32, i32)
pub fn mirl::system::OsInfo::get_taskbar_height() -> i32
impl core::marker::Freeze for mirl::system::OsInfo
impl core::marker::Send for mirl::system::OsInfo
impl core::marker::Sync for mirl::system::OsInfo
impl core::marker::Unpin for mirl::system::OsInfo
impl core::panic::unwind_safe::RefUnwindSafe for mirl::system::OsInfo
impl core::panic::unwind_safe::UnwindSafe for mirl::system::OsInfo
impl<Q, K> equivalent::Equivalent<K> for mirl::system::OsInfo where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::system::OsInfo::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::system::OsInfo where U: core::convert::From<T>
pub fn mirl::system::OsInfo::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::system::OsInfo where U: core::convert::Into<T>
pub type mirl::system::OsInfo::Error = core::convert::Infallible
pub fn mirl::system::OsInfo::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::system::OsInfo where U: core::convert::TryFrom<T>
pub type mirl::system::OsInfo::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::system::OsInfo::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::system::OsInfo where T: core::clone::Clone
pub type mirl::system::OsInfo::Owned = T
pub fn mirl::system::OsInfo::clone_into(&self, target: &mut T)
pub fn mirl::system::OsInfo::to_owned(&self) -> T
impl<T> core::any::Any for mirl::system::OsInfo where T: 'static + ?core::marker::Sized
pub fn mirl::system::OsInfo::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::system::OsInfo where T: ?core::marker::Sized
pub fn mirl::system::OsInfo::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::system::OsInfo where T: ?core::marker::Sized
pub fn mirl::system::OsInfo::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::system::OsInfo where T: core::clone::Clone
pub unsafe fn mirl::system::OsInfo::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::system::OsInfo
pub fn mirl::system::OsInfo::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::system::OsInfo
pub type mirl::system::OsInfo::Init = T
pub const mirl::system::OsInfo::ALIGN: usize
pub unsafe fn mirl::system::OsInfo::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::system::OsInfo::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::system::OsInfo::drop(ptr: usize)
pub unsafe fn mirl::system::OsInfo::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::system::OsInfo
impl<T> mirl::extensions::RepeatData for mirl::system::OsInfo where T: core::clone::Clone
pub fn mirl::system::OsInfo::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub trait mirl::system::info::Battery
pub fn mirl::system::info::Battery::get_battery_percentage(&self) -> core::option::Option<u8>
pub fn mirl::system::info::Battery::is_battery_charging() -> bool
pub fn mirl::system::info::Battery::is_in_low_power_mode() -> bool
impl mirl::system::info::Battery for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_battery_percentage(&self) -> core::option::Option<u8>
pub fn mirl::system::OsInfo::is_battery_charging() -> bool
pub fn mirl::system::OsInfo::is_in_low_power_mode() -> bool
pub trait mirl::system::info::Info
pub fn mirl::system::info::Info::get_os_name() -> alloc::string::String
impl mirl::system::info::Info for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_os_name() -> alloc::string::String
pub trait mirl::system::info::Memory
pub fn mirl::system::info::Memory::get_free_memory(&self) -> u64
pub fn mirl::system::info::Memory::get_total_memory(&self) -> u64
impl mirl::system::info::Memory for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_free_memory(&self) -> u64
pub fn mirl::system::OsInfo::get_total_memory(&self) -> u64
pub trait mirl::system::info::Network
pub fn mirl::system::info::Network::is_connected_to_internet(website_connection: core::option::Option<alloc::string::String>) -> bool
pub trait mirl::system::info::Screen
pub fn mirl::system::info::Screen::get_os_menu_height() -> i32
pub fn mirl::system::info::Screen::get_screen_resolution() -> (i32, i32)
pub fn mirl::system::info::Screen::get_taskbar_height() -> i32
impl mirl::system::info::Screen for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_os_menu_height() -> i32
pub fn mirl::system::OsInfo::get_screen_resolution() -> (i32, i32)
pub fn mirl::system::OsInfo::get_taskbar_height() -> i32
pub trait mirl::system::info::Time
pub fn mirl::system::info::Time::get_timezone_offset() -> i8
pub fn mirl::system::info::get_center_of_screen_for_object(width: i32, height: i32) -> (i32, i32)
pub fn mirl::system::info::get_center_of_screen_of_buffer(buffer: &mirl::platform::Buffer) -> (i32, i32)
pub struct mirl::system::OsActions
impl core::clone::Clone for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::clone(&self) -> mirl::system::action::windows_actions::WindowsActions
impl core::cmp::Eq for mirl::system::action::windows_actions::WindowsActions
impl core::cmp::PartialEq for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::eq(&self, other: &mirl::system::action::windows_actions::WindowsActions) -> bool
impl core::fmt::Debug for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::system::action::windows_actions::WindowsActions
impl core::marker::StructuralPartialEq for mirl::system::action::windows_actions::WindowsActions
impl mirl::system::action::Decoration for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_borderless(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool
impl mirl::system::action::Default for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_position(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32)
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_size(handle: &raw_window_handle::RawWindowHandle) -> (i32, i32)
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_level(handle: &raw_window_handle::RawWindowHandle, level: mirl::platform::WindowLevel) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_position(handle: &raw_window_handle::RawWindowHandle, x: i32, y: i32) -> bool
impl mirl::system::action::Misc for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::capture_desktop_background() -> core::option::Option<mirl::platform::Buffer>
pub fn mirl::system::action::windows_actions::WindowsActions::capture_screen() -> core::option::Option<mirl::platform::Buffer>
pub fn mirl::system::action::windows_actions::WindowsActions::get_all_windows() -> alloc::vec::Vec<raw_window_handle::RawWindowHandle>
pub fn mirl::system::action::windows_actions::WindowsActions::get_id_using_title(title: &str, exact_match: bool, case_sensitive: bool, include_hidden: bool, just_one: bool) -> core::option::Option<alloc::vec::Vec<raw_window_handle::RawWindowHandle>>
pub fn mirl::system::action::windows_actions::WindowsActions::get_title_using_id(handle: &raw_window_handle::RawWindowHandle) -> alloc::string::String
pub fn mirl::system::action::windows_actions::WindowsActions::get_window_z(handle: &raw_window_handle::RawWindowHandle) -> u32
pub fn mirl::system::action::windows_actions::WindowsActions::set_click_ability_of_window(handle: &raw_window_handle::RawWindowHandle, click_through: bool)
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_hidden_from_taskbar_and_alt_tab(handle: &raw_window_handle::RawWindowHandle, boolean: bool) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_z(handle: &raw_window_handle::RawWindowHandle, z: u32) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_z_after(handle: &raw_window_handle::RawWindowHandle, after: &raw_window_handle::RawWindowHandle) -> bool
impl mirl::system::action::TaskBar for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::set_icon_progress(handle: &raw_window_handle::RawWindowHandle, current: u64, total: u64)
pub fn mirl::system::action::windows_actions::WindowsActions::set_icon_state(handle: &raw_window_handle::RawWindowHandle, state: &mirl::system::action::ProgressionState)
impl mirl::system::action::Transparency for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::make_color_transparent(handle: &raw_window_handle::RawWindowHandle, color: (u8, u8, u8)) -> bool
pub fn mirl::system::action::windows_actions::WindowsActions::set_window_opacity(handle: &raw_window_handle::RawWindowHandle, opacity: u8) -> bool
impl core::marker::Freeze for mirl::system::action::windows_actions::WindowsActions
impl core::marker::Send for mirl::system::action::windows_actions::WindowsActions
impl core::marker::Sync for mirl::system::action::windows_actions::WindowsActions
impl core::marker::Unpin for mirl::system::action::windows_actions::WindowsActions
impl core::panic::unwind_safe::RefUnwindSafe for mirl::system::action::windows_actions::WindowsActions
impl core::panic::unwind_safe::UnwindSafe for mirl::system::action::windows_actions::WindowsActions
impl<Q, K> equivalent::Equivalent<K> for mirl::system::action::windows_actions::WindowsActions where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::system::action::windows_actions::WindowsActions where U: core::convert::From<T>
pub fn mirl::system::action::windows_actions::WindowsActions::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::system::action::windows_actions::WindowsActions where U: core::convert::Into<T>
pub type mirl::system::action::windows_actions::WindowsActions::Error = core::convert::Infallible
pub fn mirl::system::action::windows_actions::WindowsActions::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::system::action::windows_actions::WindowsActions where U: core::convert::TryFrom<T>
pub type mirl::system::action::windows_actions::WindowsActions::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::system::action::windows_actions::WindowsActions::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::system::action::windows_actions::WindowsActions where T: core::clone::Clone
pub type mirl::system::action::windows_actions::WindowsActions::Owned = T
pub fn mirl::system::action::windows_actions::WindowsActions::clone_into(&self, target: &mut T)
pub fn mirl::system::action::windows_actions::WindowsActions::to_owned(&self) -> T
impl<T> core::any::Any for mirl::system::action::windows_actions::WindowsActions where T: 'static + ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::system::action::windows_actions::WindowsActions where T: ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::system::action::windows_actions::WindowsActions where T: ?core::marker::Sized
pub fn mirl::system::action::windows_actions::WindowsActions::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::system::action::windows_actions::WindowsActions where T: core::clone::Clone
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::system::action::windows_actions::WindowsActions
pub fn mirl::system::action::windows_actions::WindowsActions::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::system::action::windows_actions::WindowsActions
pub type mirl::system::action::windows_actions::WindowsActions::Init = T
pub const mirl::system::action::windows_actions::WindowsActions::ALIGN: usize
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::drop(ptr: usize)
pub unsafe fn mirl::system::action::windows_actions::WindowsActions::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::system::action::windows_actions::WindowsActions
impl<T> mirl::extensions::RepeatData for mirl::system::action::windows_actions::WindowsActions where T: core::clone::Clone
pub fn mirl::system::action::windows_actions::WindowsActions::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub struct mirl::system::OsInfo
impl mirl::system::OsInfo
pub fn mirl::system::OsInfo::new() -> Self
impl core::clone::Clone for mirl::system::OsInfo
pub fn mirl::system::OsInfo::clone(&self) -> mirl::system::OsInfo
impl core::cmp::Eq for mirl::system::OsInfo
impl core::cmp::PartialEq for mirl::system::OsInfo
pub fn mirl::system::OsInfo::eq(&self, other: &mirl::system::OsInfo) -> bool
impl core::fmt::Debug for mirl::system::OsInfo
pub fn mirl::system::OsInfo::fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
impl core::marker::Copy for mirl::system::OsInfo
impl core::marker::StructuralPartialEq for mirl::system::OsInfo
impl mirl::system::info::Battery for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_battery_percentage(&self) -> core::option::Option<u8>
pub fn mirl::system::OsInfo::is_battery_charging() -> bool
pub fn mirl::system::OsInfo::is_in_low_power_mode() -> bool
impl mirl::system::info::Info for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_os_name() -> alloc::string::String
impl mirl::system::info::Memory for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_free_memory(&self) -> u64
pub fn mirl::system::OsInfo::get_total_memory(&self) -> u64
impl mirl::system::info::Screen for mirl::system::OsInfo
pub fn mirl::system::OsInfo::get_os_menu_height() -> i32
pub fn mirl::system::OsInfo::get_screen_resolution() -> (i32, i32)
pub fn mirl::system::OsInfo::get_taskbar_height() -> i32
impl core::marker::Freeze for mirl::system::OsInfo
impl core::marker::Send for mirl::system::OsInfo
impl core::marker::Sync for mirl::system::OsInfo
impl core::marker::Unpin for mirl::system::OsInfo
impl core::panic::unwind_safe::RefUnwindSafe for mirl::system::OsInfo
impl core::panic::unwind_safe::UnwindSafe for mirl::system::OsInfo
impl<Q, K> equivalent::Equivalent<K> for mirl::system::OsInfo where Q: core::cmp::Eq + ?core::marker::Sized, K: core::borrow::Borrow<Q> + ?core::marker::Sized
pub fn mirl::system::OsInfo::equivalent(&self, key: &K) -> bool
impl<T, U> core::convert::Into<U> for mirl::system::OsInfo where U: core::convert::From<T>
pub fn mirl::system::OsInfo::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for mirl::system::OsInfo where U: core::convert::Into<T>
pub type mirl::system::OsInfo::Error = core::convert::Infallible
pub fn mirl::system::OsInfo::try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for mirl::system::OsInfo where U: core::convert::TryFrom<T>
pub type mirl::system::OsInfo::Error = <U as core::convert::TryFrom<T>>::Error
pub fn mirl::system::OsInfo::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for mirl::system::OsInfo where T: core::clone::Clone
pub type mirl::system::OsInfo::Owned = T
pub fn mirl::system::OsInfo::clone_into(&self, target: &mut T)
pub fn mirl::system::OsInfo::to_owned(&self) -> T
impl<T> core::any::Any for mirl::system::OsInfo where T: 'static + ?core::marker::Sized
pub fn mirl::system::OsInfo::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for mirl::system::OsInfo where T: ?core::marker::Sized
pub fn mirl::system::OsInfo::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for mirl::system::OsInfo where T: ?core::marker::Sized
pub fn mirl::system::OsInfo::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for mirl::system::OsInfo where T: core::clone::Clone
pub unsafe fn mirl::system::OsInfo::clone_to_uninit(&self, dest: *mut u8)
impl<T> core::convert::From<T> for mirl::system::OsInfo
pub fn mirl::system::OsInfo::from(t: T) -> T
impl<T> crossbeam_epoch::atomic::Pointable for mirl::system::OsInfo
pub type mirl::system::OsInfo::Init = T
pub const mirl::system::OsInfo::ALIGN: usize
pub unsafe fn mirl::system::OsInfo::deref<'a>(ptr: usize) -> &'a T
pub unsafe fn mirl::system::OsInfo::deref_mut<'a>(ptr: usize) -> &'a mut T
pub unsafe fn mirl::system::OsInfo::drop(ptr: usize)
pub unsafe fn mirl::system::OsInfo::init(init: <T as crossbeam_epoch::atomic::Pointable>::Init) -> usize
impl<T> either::into_either::IntoEither for mirl::system::OsInfo
impl<T> mirl::extensions::RepeatData for mirl::system::OsInfo where T: core::clone::Clone
pub fn mirl::system::OsInfo::repeat_value(self, amount: usize) -> alloc::vec::Vec<T>
pub mod mirl::time
pub const mirl::time::DAYS_PER_WEEK: usize
pub const mirl::time::HOURS_PER_DAY: usize
pub const mirl::time::MICROS_PER_SEC: usize
pub const mirl::time::MILLIS_PER_SEC: usize
pub const mirl::time::MINS_PER_HOUR: usize
pub const mirl::time::NANOS_PER_MICRO: usize
pub const mirl::time::NANOS_PER_MILLI: usize
pub const mirl::time::NANOS_PER_SEC: usize
pub const mirl::time::SECS_PER_MINUTE: usize
pub const fn mirl::time::from_micros_u128(micros: u128) -> core::time::Duration
pub const fn mirl::time::from_millis_u128(millis: u128) -> core::time::Duration
pub const fn mirl::time::from_nanos_u128(nanos: u128) -> core::time::Duration
pub macro mirl::signed_to_unsigned!
pub macro mirl::u1!
pub macro mirl::u2!
pub macro mirl::u4!
pub macro mirl::unsigned_to_signed!
pub fn mirl::disable_traceback()
pub fn mirl::enable_traceback()
pub fn mirl::enable_traceback_detailed()
"""


data = file.split("\n")

stuff = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ|_!1234567890"

paths: list[str] = []
for line in data:
    if not line.startswith("pub"):
        continue

    cleaned_line = line
    while "<" in cleaned_line and ">" in cleaned_line:
        start = cleaned_line.find("<")
        end = cleaned_line.find(">", start)
        if end != -1:
            cleaned_line = cleaned_line[:start] + cleaned_line[end + 1 :]
        else:
            break

    name = ""
    potentials = cleaned_line.split(" ")
    for i in potentials:
        if not i.__contains__("::"):
            continue
        name = i.replace("::", "|")
        break

    refined = ""
    for i in name:
        if not i in stuff:
            break
        refined += i

    if refined:
        paths.append(refined.replace("|", "::"))

paths = list(set(paths))
sort_by_depth = True

depth = [x.count("::") + 1 for x in paths]
length = [len(x) - x.count("::") * 2 for x in paths]
path_data = list(zip(paths, depth, length))

if sort_by_depth:
    # Primary sort by depth, secondary sort by length
    path_data.sort(key=lambda x: (x[1], x[2], x[0]))
    sort_criteria = "depth (then length)"
else:
    # Primary sort by length, secondary sort by depth
    path_data.sort(key=lambda x: (x[2], x[1], x[0]))
    sort_criteria = "length (then depth)"

if False:
    max_path_width = max(len(path) for path, _, _ in path_data)
    max_path_width = max(max_path_width, len("Path"))

    print(f"Paths sorted by {sort_criteria}:")
    print("-" * (max_path_width + 20))
    print(f"{'Path'.ljust(max_path_width)} {'Depth'.rjust(8)} {'Length'.rjust(8)}")
    print("-" * (max_path_width + 20))

    for path, d, l in path_data:
        print(f"{path.ljust(max_path_width)} {str(d).rjust(8)} {str(l).rjust(8)}")
    quit()

from typing import Any, Optional


if False:
    import collections
    import json

    def create_dict_from_paths(paths: list[str]):
        result: collections.OrderedDict[Any, Any] = collections.OrderedDict()

        for path in paths:
            parts = path.split("::")

            current = result
            for part in parts[:-1]:
                if part not in current:
                    current[part] = {}
                current = current[part]

            current[parts[-1]] = {}

        return result

    value = create_dict_from_paths(paths)

    print(json.dumps(value, indent=4))
    quit()

from collections import defaultdict


# type: ignore, my beloved
def visualize_paths(
    paths: list[str], prefix: str = "", lines: Optional[list[str]] = None
) -> list[str]:
    if lines is None:
        lines = []

    # Build a nested dict/tree
    tree = lambda: defaultdict(tree)  # type: ignore
    root = tree()  # type: ignore
    for path in paths:
        parts = path.split("::")
        node = root  # type: ignore
        for part in parts:
            node = node[part]  # type: ignore

    def _walk(node, root, prefix=""):  # type: ignore
        keys = list(node.keys())  # type: ignore
        for i, key in enumerate(keys):  # type: ignore
            is_last = i == len(keys) - 1  # type: ignore
            if root:
                # Root node: print without └── / ├──
                lines.append(f"{key}")
                new_prefix = ""
            else:
                marker = "└──" if is_last else "├──"
                lines.append(f"{prefix}{marker} {key}")
                new_prefix = prefix + ("    " if is_last else "│   ")
            _walk(node[key], False, new_prefix)  # type: ignore

    _walk(root, True)
    return lines


full = "\n".join(visualize_paths([p for p in paths if p.startswith("mirl")]))

with open("paths.txt", "w", encoding="utf-8") as f:
    f.write(full)
