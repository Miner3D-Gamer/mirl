/// Wrappers for active actions to take that would usually be OS specific
pub mod action;
pub use action::OsActions;
/// Information about the OS
pub mod info;
pub use info::OsInfo;