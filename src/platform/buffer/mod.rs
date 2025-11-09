// Rewrite to use copyable list instead of Vec<[u32]>?
/// A raw color buffer to be modified and read quickly
#[derive(PartialEq, Debug, Eq, Clone, Hash)]
pub struct Buffer {
    /// Actual color data
    pub data: Vec<u32>,
    // /// Pointer to the color data
    // pub pointer: *mut u32,
    /// Width of the buffer
    pub width: usize,
    /// Height of the buffer
    pub height: usize,
    /// The total size -> width*height
    pub total_size: usize,
}
unsafe impl std::marker::Send for Buffer {}
unsafe impl std::marker::Sync for Buffer {}

mod collision_support;
mod get_converted;
mod get_pixel;
mod manipulate;
mod misc;
mod new;
mod set_color;
mod set_pixel;
mod trim;

/// A const buffer making money on more compile time optimizations
pub mod const_buffer;

use std::ops::Deref;
// Automatically convert the usage of Buffer to Buffer.data
impl Deref for Buffer {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Buffer {
    #[must_use]
    /// Get the pointer to self.data
    pub const fn pointer(&self) -> *const u32 {
        self.data.as_ptr()
    }
    #[must_use]
    /// Get the pointer to self.data mutably
    pub const fn mut_pointer(&mut self) -> *mut u32 {
        self.data.as_mut_ptr()
    }
    // /// Update internal pointer
    // pub const fn update_pointer(&mut self) {
    //     self.pointer = self.data.as_mut_ptr();
    // }
    /// Update the total size of the buffer
    pub const fn update_total_size(&mut self) {
        self.total_size = self.width * self.height;
    }
}

// impl Clone for Buffer {
//     #[allow(clippy::as_ptr_cast_mut)]
//     fn clone(&self) -> Self {
//         let data = self.data.clone();

//         Self {
//             data,
//             width: self.width,
//             height: self.height,
//             total_size: self.total_size,
//         }
//     }
// }
