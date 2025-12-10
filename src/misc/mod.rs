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

/// A 2d point specialize for lines and columns
pub mod text_position;

#[cfg(feature = "std")]
use std::hash::Hasher;

#[cfg(feature = "std")]
/// Combine 2 strings
pub fn concatenate<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> String {
    let mut result = String::from(a.as_ref());
    result.push_str(b.as_ref());
    result
}
#[cfg(feature = "std")]
/// Hash a value
pub fn hash_value<T: std::hash::Hash>(value: &T) -> u64 {
    let mut s = std::hash::DefaultHasher::new();
    value.hash(&mut s);
    s.finish()
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
// pub trait Comparable {
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

// impl<T: std::hash::Hash> Comparable for T {
//     fn is_same(&self, other: &Self) -> bool {
//         let mut own_hasher = std::hash::DefaultHasher::new();
//         let mut other_hasher = std::hash::DefaultHasher::new();
//         self.hash(&mut own_hasher);
//         other.hash(&mut other_hasher);
//         own_hasher == other_hasher
//     }
// }
#[derive(Debug)]
/// Choose between 2 values
pub enum TwoOptions<O1, O2> {
    /// Select the first of 2 options
    Option1(O1),
    /// Select the second of 2 options
    Option2(O2),
}

/// I am already aware it is unsafe, just let me unwrap it!
pub trait EasyUnwrapUnchecked<T> {
    /// I am already aware it is unsafe, just let me unwrap it!
    fn easy_unwrap_unchecked(self) -> T;
}

impl<T> EasyUnwrapUnchecked<T> for Option<T> {
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
