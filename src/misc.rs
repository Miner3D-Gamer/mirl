use std::hash::Hasher;
/// Combine 2 strings
pub fn concatenate<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> String {
    let mut result = String::from(a.as_ref());
    result.push_str(b.as_ref());
    result
}
/// Hash a value
pub fn hash_value<T: std::hash::Hash>(value: &T) -> u64 {
    let mut s = std::hash::DefaultHasher::new();
    value.hash(&mut s);
    s.finish()
}
/// Repeat the given data X times
pub fn repeat_data<T: Clone>(data: T, amount: usize) -> Vec<T> {
    vec![data; amount]
}
/// A windows only section for misc function
#[cfg(target_os = "windows")]
#[cfg(feature = "system")]
pub mod windows {
    // use windows::Win32::System::Diagnostics::Debug::GetThreadContext;
    // use windows::Win32::System::Memory::{
    //     VirtualQuery, MEMORY_BASIC_INFORMATION,
    // };

    use windows::Win32::System::Memory::{
        VirtualQuery, MEMORY_BASIC_INFORMATION,
    };
    #[allow(trivial_casts)]
    /// Check the stack use
    pub fn get_actual_stack_info() -> Result<(), Box<dyn std::error::Error>> {
        unsafe {
            let current_sp = &0 as *const i32 as *const std::ffi::c_void;
            let mut mbi = MEMORY_BASIC_INFORMATION::default();

            // Query the memory region containing current stack pointer
            VirtualQuery(
                Some(current_sp),
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );

            let region_base = mbi.BaseAddress as usize;
            let region_size = mbi.RegionSize;
            let current_addr = current_sp as usize;

            println!("Memory region base:   0x{:x}", region_base);
            println!("Memory region size:   {} MB", region_size / 1024 / 1024);
            println!("Current SP:           0x{:x}", current_addr);
            println!(
                "Offset in region:     {} KB",
                (current_addr - region_base) / 1024
            );

            // Stack grows downward, so distance from top of region
            let used_from_top = (region_base + region_size) - current_addr;
            println!("Used from region top: {} KB", used_from_top / 1024);

            Ok(())
        }
    }
}

#[macro_export]
/// Converts unsigned types to their signed versions
macro_rules! unsigned_to_signed {
    (u8) => {
        i8
    };
    (u16) => {
        i16
    };
    (u32) => {
        i32
    };
    (u64) => {
        i64
    };
    (u128) => {
        i128
    };
    (usize) => {
        isize
    };
}
#[macro_export]
/// Converts signed types to their unsigned versions
macro_rules! signed_to_unsigned {
    (i8) => {
        u8
    };
    (i16) => {
        u16
    };
    (i32) => {
        u32
    };
    (i64) => {
        u64
    };
    (i128) => {
        u128
    };
    (isize) => {
        usize
    };
}
