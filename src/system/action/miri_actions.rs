use crate::{
    platform::WindowLevel,
    system::action::{
        Decoration, Default, Host, Iconized, Screen, Transparency,
    },
};
#[derive(Debug, Clone, Copy, Default, Default, Hash)]
/// A stub for miri to "emulate" os actions, or in other words: a list of hardcoded values because miri doesn't support os specific stuff
pub struct MiriActions {}

impl MiriActions {
    #[must_use]
    /// Create a new instance
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for MiriActions {
    fn set_window_position(
        _handle: &raw_window_handle::RawWindowHandle,
        _x: i32,
        _y: i32,
    ) -> bool {
        false
    }

    fn set_window_level(
        _handle: &raw_window_handle::RawWindowHandle,
        _level: WindowLevel,
    ) -> bool {
        false
    }

    fn get_window_position(
        _handle: &raw_window_handle::RawWindowHandle,
    ) -> (i32, i32) {
        (0, 0)
    }

    fn get_window_size(
        _handle: &raw_window_handle::RawWindowHandle,
    ) -> (i32, i32) {
        (0, 0)
    }

    fn get_window_hitbox_size(
        _handle: &raw_window_handle::RawWindowHandle,
    ) -> (i32, i32) {
        (0, 0)
    }

    fn set_window_size(
        _handle: &raw_window_handle::RawWindowHandle,
        _size: (i32, i32),
    ) -> bool {
        false
    }
}

impl Transparency for MiriActions {
    fn make_color_transparent(
        _handle: &raw_window_handle::RawWindowHandle,
        _color: (u8, u8, u8),
    ) -> bool {
        false
    }

    fn set_window_opacity(
        _handle: &raw_window_handle::RawWindowHandle,
        _opacity: u8,
    ) -> bool {
        false
    }
}
impl Decoration for MiriActions {
    fn set_window_borderless(
        _handle: &raw_window_handle::RawWindowHandle,
        _boolean: bool,
    ) -> bool {
        false
    }
}
impl Iconized for MiriActions {
    fn is_minimized(_handle: &raw_window_handle::RawWindowHandle) -> bool {
        false
    }

    fn is_maximized(_handle: &raw_window_handle::RawWindowHandle) -> bool {
        false
    }

    fn restore(_handle: &raw_window_handle::RawWindowHandle) -> bool {
        false
    }

    fn minimize(_handle: &raw_window_handle::RawWindowHandle) -> bool {
        false
    }

    fn maximize(_handle: &raw_window_handle::RawWindowHandle) -> bool {
        false
    }
}

impl Host for MiriActions {
    fn get_os_name() -> String {
        "miri".to_string()
    }
}

impl Screen for MiriActions {
    fn get_screen_resolution() -> (i32, i32) {
        (800, 600)
    }
}
