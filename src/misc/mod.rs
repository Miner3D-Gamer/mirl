#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A enum that will return the value it beholds when a ? is used - The opposite of [Option]
pub enum FoundReturn<V> {
    #[default]
    /// The item was not found, continue execution
    NotFound,
    /// The item was found, exit the current function!
    Found(V),
}

impl<V> core::ops::Try for FoundReturn<V> {
    type Output = ();
    type Residual = V;

    fn from_output((): Self::Output) -> Self {
        Self::NotFound
    }

    fn branch(self) -> core::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::NotFound => core::ops::ControlFlow::Continue(()),
            Self::Found(v) => core::ops::ControlFlow::Break(v),
        }
    }
}

impl<V> core::ops::FromResidual<V> for FoundReturn<V> {
    fn from_residual(residual: V) -> Self {
        Self::Found(residual)
    }
}
impl<T> From<Option<T>> for FoundReturn<T> {
    fn from(val: Option<T>) -> Self {
        val.map_or_else(|| Self::NotFound, |val| Self::Found(val))
    }
}
impl<T> FromPatch<Option<T>> for FoundReturn<T> {
    fn from_value(val: Option<T>) -> Self {
        val.map_or_else(|| Self::NotFound, |val| Self::Found(val))
    }
}
impl<T> From<T> for FoundReturn<T> {
    fn from(val: T) -> Self {
        Self::Found(val)
    }
}
impl<T> FromPatch<T> for FoundReturn<T> {
    fn from_value(val: T) -> Self {
        Self::Found(val)
    }
}

/// Lists but copyable
#[cfg(feature = "std")]
pub mod copyable_list;

#[cfg(feature = "std")]
mod codec;
#[cfg(feature = "std")]
pub use codec::*;

/// Horizontal Arrow + Control behavior
#[cfg(feature = "std")]
pub mod skipping_text_type;

#[cfg(feature = "keycodes")]
#[cfg(feature = "std")]
/// A few lines of helper code for easier keybind handling time
pub mod keybinds;

#[cfg(feature = "std")]
use core::hash::Hasher;

#[cfg(feature = "std")]
/// Combine 2 strings
pub fn concatenate<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> String {
    let mut result = String::from(a.as_ref());
    result.push_str(b.as_ref());
    result
}
#[cfg(feature = "std")]
/// Hash a value
pub fn hash_value<T: core::hash::Hash>(value: &T) -> u64 {
    let mut s = std::hash::DefaultHasher::new();
    value.hash(&mut s);
    s.finish()
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy)]
/// Convert from one ratio to another
pub struct RatioConverter2D<T: FromPatch<F>, F: FromPatch<T> + Copy> {
    ratio: (F, F),
    inverse_ratio: (F, F),
    phantom: core::marker::PhantomData<T>,
}
use crate::prelude::FromPatch;
#[allow(clippy::missing_const_for_fn)]
const impl<
        T: [const] core::ops::Add<Output = T>
            + [const] core::ops::Sub<Output = T>
            + [const] core::ops::Mul<Output = T>
            + [const] core::ops::Div<Output = T>
            + Copy
            + [const] FromPatch<F>,
        F: [const] FromPatch<T>
            + [const] core::ops::Add<Output = F>
            + [const] core::ops::Sub<Output = F>
            + [const] core::ops::Mul<Output = F>
            + [const] core::ops::Div<Output = F>
            + Copy,
    > RatioConverter2D<T, F>
{
    /// Create a new ratio
    #[must_use]
    pub fn new(smaller: (T, T), bigger: (T, T)) -> Self {
        Self {
            ratio: (
                F::from_value(smaller.0) / F::from_value(bigger.0),
                F::from_value(smaller.1) / F::from_value(bigger.1),
            ),
            inverse_ratio: (
                F::from_value(bigger.0) / F::from_value(smaller.0),
                F::from_value(bigger.1) / F::from_value(smaller.1),
            ),
            phantom: core::marker::PhantomData,
        }
    }
    /// Convert a value from the smaller ratio to the bigger ratio
    pub fn smaller_to_bigger(&self, value: (T, T)) -> (T, T) {
        (
            T::from_value(F::from_value(value.0) * self.inverse_ratio.0),
            T::from_value(F::from_value(value.1) * self.inverse_ratio.1),
        )
    }
    /// Convert a value from the bigger ratio to the smaller ratio
    pub fn bigger_to_smaller(&self, value: (T, T)) -> (T, T) {
        (
            T::from_value(F::from_value(value.0) * self.ratio.0),
            T::from_value(F::from_value(value.1) * self.ratio.1),
        )
    }
}

/// A windows only section for misc function
#[cfg(target_os = "windows")]
#[cfg(feature = "system")]
#[cfg(feature = "std")]
pub mod windows {
    // use windows::Win32::System::Diagnostics::Debug::GetThreadContext;
    // use windows::Win32::System::Memory::{
    //     VirtualQuery, MEMORY_BASIC_INFORMATION,
    // };

    use windows::Win32::System::Memory::{
        VirtualQuery, MEMORY_BASIC_INFORMATION,
    };
    #[allow(trivial_casts)]
    #[allow(clippy::ref_as_ptr)]
    /// Check the stack use
    ///
    /// # Errors
    pub fn get_actual_stack_info() {
        unsafe {
            let current_sp = (&0 as *const i32).cast::<std::ffi::c_void>(); // &0 as *const i32 as *const std::ffi::c_void
            let mut mbi = MEMORY_BASIC_INFORMATION::default();

            // Query the memory region containing current stack pointer
            VirtualQuery(
                Some(current_sp),
                &raw mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );

            let region_base = mbi.BaseAddress as usize;
            let region_size = mbi.RegionSize;
            let current_addr = current_sp as usize;

            println!("Memory region base:   0x{region_base:x}");
            println!("Memory region size:   {} MB", region_size / 1024 / 1024);
            println!("Current SP:           0x{current_addr:x}");
            println!(
                "Offset in region:     {} KB",
                (current_addr - region_base) / 1024
            );

            // Stack grows downward, so distance from top of region
            let used_from_top = (region_base + region_size) - current_addr;
            println!("Used from region top: {} KB", used_from_top / 1024);
        }
    }
}

#[macro_export]
/// Create a compile time warning using deprecation
macro_rules! compile_warn {
    ($msg:expr) => {
        #[allow(dead_code)]
        fn deprecated_container() {
            #[deprecated(note = $msg)]
            const fn deprecated_trigger() {}
            let _ = deprecated_trigger;
        }
    };
}
#[must_use]
#[cfg(feature = "std")]
/// Get the name of the type of the inputted variable
pub fn type_name_of_val<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

// /// Check if 2 objects are the same
// #[cfg(feature = "std")]
// pub const trait Comparable {
//     /// Convert self to `&dyn std::any::Any`
//     fn compare_as_any(&self) -> &dyn std::any::Any;
//     /// Check if this and another object are the same
//     fn is_same(&self, other: &dyn Comparable) -> bool;
// }
// #[cfg(feature = "std")]
// impl<T: 'static + PartialEq> Comparable for T {
//     fn compare_as_any(&self) -> &dyn std::any::Any {
//         self
//     }

//     fn is_same(&self, other: &dyn Comparable) -> bool {
//         // Try to downcast `other` to Foo
//         other.compare_as_any().downcast_ref::<Self>() == Some(self)
//     }
// }
// impl<T: PartialEq> Comparable for T {
//     fn is_same(&self, other: &Self) -> bool {
//         self == other
//     }
// }

// impl<T: core::hash::Hash> Comparable for T {
//     fn is_same(&self, other: &Self) -> bool {
//         let mut own_hasher = std::hash::DefaultHasher::new();
//         let mut other_hasher = std::hash::DefaultHasher::new();
//         self.hash(&mut own_hasher);
//         other.hash(&mut other_hasher);
//         own_hasher == other_hasher
//     }
// }
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Choose between 2 values
pub enum TwoOptions<O1, O2> {
    /// Select the first of 2 options
    Option1(O1),
    /// Select the second of 2 options
    Option2(O2),
}

/// I am already aware it is unsafe, just let me unwrap it!
pub const trait EasyUnwrapUnchecked<T> {
    /// I am already aware it is unsafe, just let me unwrap it!
    fn easy_unwrap_unchecked(self) -> T;
}

impl<T> EasyUnwrapUnchecked<T> for Option<T> {
    fn easy_unwrap_unchecked(self) -> T {
        unsafe { self.unwrap_unchecked() }
    }
}
impl<T, E> EasyUnwrapUnchecked<T> for Result<T, E> {
    fn easy_unwrap_unchecked(self) -> T {
        unsafe { self.unwrap_unchecked() }
    }
}

#[cfg(feature = "std")]
/// A standardized map format
mod map_extension;
#[cfg(feature = "std")]
pub use map_extension::*;

mod scrollable_container;
pub use scrollable_container::*;

#[cfg(feature = "std")]
mod clone_any;
#[cfg(feature = "std")]
pub use clone_any::*;

#[macro_export]
/// Usage: `impl_empty_trait!(std::sync::Send for Struct1 Struct2 Struct3)`
macro_rules! impl_empty_trait {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {}
    )*)
}

#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A ticker, regulate the timing of an application
pub struct Ticker {
    /// 1/fps
    pub target_delta_time: std::time::Duration,
    /// The last time this struct got ticked
    pub last_frame: std::time::Instant,
    /// When the next frame should start
    pub next_frame: std::time::Instant,
    /// The current delta time -> Time between frame starts/ends
    pub delta_time: std::time::Duration,
}
#[cfg(feature = "std")]
impl Ticker {
    #[must_use]
    /// Create a new ticker, holds invalid data until ticked once
    ///
    /// # Errors
    /// When fps it too high/negative to fit into [`std::time::Duration`]
    pub fn new(fps: f64) -> Option<Self> {
        Some(Self {
            target_delta_time: std::time::Duration::try_from_secs_f64(
                1.0 / fps,
            )
            .ok()?,
            last_frame: std::time::Instant::now(),
            next_frame: std::time::Instant::now(),
            delta_time: std::time::Duration::new(0, 0),
        })
    }
    /// Tick the Ticker
    ///
    /// If the frame took too long, the next frame will be skipped
    /// If there is still time left, it will sleep until the desired frame time
    pub fn tick(&mut self) {
        let now = std::time::Instant::now();
        self.delta_time = now - self.last_frame;
        self.last_frame = now;
        if now > self.next_frame {
            self.next_frame = now + self.target_delta_time;
        } else {
            std::thread::sleep(self.next_frame - now);
            self.next_frame += self.target_delta_time;
        }
    }
    #[must_use]
    /// Get the delta time, use [`as_secs_f32`](std::time::Duration::as_secs_f32) or [`as_secs_64`](std::time::Duration::as_secs_f64) on that duration
    pub const fn get_delta_time(&self) -> std::time::Duration {
        self.delta_time
    }
    /// Get the delta time and the fps of the last tick
    #[must_use]
    pub const fn get_delta_time_and_fps_f32(&self) -> (f32, f32) {
        let delta = self.delta_time.as_secs_f32();
        (delta, 1.0 / delta)
    }
    /// Get the delta time and the fps of the last tick
    #[must_use]
    pub const fn get_delta_time_and_fps_f64(&self) -> (f64, f64) {
        let delta = self.delta_time.as_secs_f64();
        (delta, 1.0 / delta)
    }
}
#[cfg(feature = "std")]
/// A scene - A collection of shapes and positions
pub mod scene;
