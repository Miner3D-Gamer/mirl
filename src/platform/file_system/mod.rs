mod native;
pub use native::*;

/// Why bother reading files if you can't process them? Let [`file_data::FileData`] fix that.
mod file_data;
pub use file_data::DataType;
pub use file_data::FileData;

#[cfg(feature = "font_support")]
mod get_os_font;
#[cfg(feature = "font_support")]
pub use get_os_font::get_default_font;

/// The trait used by the file system implementations
pub mod file_system_traits;
#[cfg(not(target_arch = "wasm32"))]
pub use native::NativeFileSystem as FileSystem;
