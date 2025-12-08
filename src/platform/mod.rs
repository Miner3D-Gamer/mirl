#[cfg(feature = "system")]
#[cfg(feature = "std")]
use crate::extensions::*;

#[const_trait]
/// Time trait, IDK
pub trait Time {
    /// Get time in seconds
    fn get_elapsed_time(&self) -> f64;
}
/// A cursor style, what else to say?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorStyle {
    /// Default Pointer
    Default,
    /// Open hand
    HandOpen,
    /// Closed hand
    HandClosed,
    /// Default cursor with an extra arrow (e.g. clickable text)
    Alias,
    /// Resize vertically + Resize horizontally
    AllScroll,
    /// Arrow pointing to the bottom left ⬋
    ArrowBottomLeft,
    /// Arrow pointing to the bottom right ⬊
    ArrowBottomRight,
    /// Arrow down with a _ at the end
    SideBottom,
    /// A plus shape
    Cell,
    /// Default cursor rotated to be vertical
    CenteredPointer,
    /// Horizontal resizing
    ResizeHorizontally,
    /// Eyedropper
    ColorPicker,
    /// Default cursor with ≡ attached
    ContextMenu,
    /// Default cursor with a plus
    Copy,
    /// Cross
    Crosshair,
    /// Closed hand with an 🚫 attached
    HandClosedNoDrop,
    /// Arrow pointing down
    ArrowDown,
    /// Tip of an ink pen
    Draft,
    /// Small pointers in all directions like this: ◄ ►
    Fleur,
    /// Question mark
    Help,
    /// Arrow pointing left
    ArrowLeft,
    /// Arrow left with a stopper |←
    SideLeft,
    /// Default cursor with a 🚫 attached
    NoDrop,
    /// "🚫"
    NotAllowed,
    /// A Pencil
    Pencil,
    /// Skull
    Pirate,
    /// Hand with pointing index finger
    Pointer,
    /// Arrow pointing right
    ArrowRight,
    /// Mirrored version of normal cursor
    MirroredPointer,
    /// Arrow pointing right with a stopper →|
    SideRight,
    /// Resize top right to bottom left
    ResizeNESW,
    /// Resize top left to bottom right
    ResizeNWSE,
    /// Resize horizontally
    SizeHor,
    /// Resize vertically
    ResizeVertically,
    /// I Beam
    Text,
    /// Arrow pointing up top left
    ArrowTopLeft,
    /// Arrow pointing up top right
    ArrowTopRight,
    /// Arrow pointing up with an _ on top
    SideTop,
    /// Arrow pointing up
    ArrowUp,
    /// I Beam rotated 90°
    VerticalText,
    /// Magnifying glass with plus
    ZoomIn,
    /// Magnifying glass with minus
    ZoomOut,
}
/// Supported (and unsupported) mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// ✨ The left mouse button ✨
    Left,
    /// ✨ The right mouse button ✨
    Right,
    /// ✨ The button between the left and right mouse buttons ✨
    Middle,
    /// An extra niche button some mice have
    Extra1,
    /// Another extra niche button some mice have
    Extra2,
    /// A freakish amalgamation of human invention
    Extra3,
    /// No one should be allowed this much power.
    Extra4,
    /// You can't expect to be able to expect everything ¯\_(ツ)_/¯
    Unsupported,
}
#[cfg(feature = "keycodes")]
/// Represents digital keys using `KeyCodes` of which there should be plenty enough to pretty all libraries that use their own `KeyCodes`
pub mod keycodes;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Settings for spawning in a window
pub struct WindowSettings {
    /// Remove the border of the window
    pub borderless: bool,
    /// If the title should be displayed
    pub title_visible: bool,
    /// Window render level -> Topmost, normal, bottommost
    pub window_level: WindowLevel,
    /// Position on screen
    pub position: (i32, i32),
    /// Size of the spawning window
    pub size: (i32, i32),
    /// If the window should be resizable
    pub resizable: bool,
    /// If the window should have the default os menu around it (fullscreen, minimize, close). The border however will still retain
    pub os_menu: bool,
    /// If the window should be considered visible to the user or not
    pub visible: bool,
}

#[cfg(feature = "system")]
#[cfg(feature = "std")]
impl WindowSettings {
    /// Get the settings on default settings
    /// For size a buffer is required
    /// For position, it will be centered on the screen
    #[must_use]
    pub fn default(buffer: &Buffer) -> Self {
        let size =
            (buffer.width, buffer.height).try_tuple_into().unwrap_or_default();
        Self {
            borderless: false,
            title_visible: true,
            window_level: WindowLevel::Normal,
            position: crate::system::get_center_of_screen_for_object(
                size.0, size.1,
            ),

            resizable: false,
            os_menu: true,
            size,
            visible: true,
        }
    }
    /// Set the visibility of the window
    #[must_use]
    pub const fn set_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }
    /// Set the size of the window
    #[must_use]
    pub const fn set_size(mut self, size: (i32, i32)) -> Self {
        self.size = size;
        self
    }
    #[must_use]
    /// Set the size of the window to the size of the given buffer
    pub fn set_size_to_buffer(mut self, buffer: &Buffer) -> Self {
        self.size =
            (buffer.width, buffer.height).try_tuple_into().unwrap_or_default();
        self
    }
    #[must_use]
    /// Set the position of the window
    pub const fn set_position(mut self, position: (i32, i32)) -> Self {
        self.position = position;
        self
    }
    #[must_use]
    /// Set if the title should be visible
    pub const fn set_title_visible(mut self, title: bool) -> Self {
        self.title_visible = title;
        self
    }
    #[must_use]
    /// Sets if the border should be hidden
    pub const fn set_borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }
    #[must_use]
    /// Set the render level of the window (topmost, normal, bottommost)
    pub const fn set_window_level(mut self, window_level: WindowLevel) -> Self {
        self.window_level = window_level;
        self
    }
    #[must_use]
    /// Set if the window should be resizable by the user
    pub const fn set_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
    #[must_use]
    /// Set if the default os decoration should be visible (fullscreen, minimize, close)
    pub const fn set_os_menu(mut self, os_menu: bool) -> Self {
        self.os_menu = os_menu;
        self
    }
    #[must_use]
    /// Sets the position of the window to be centered on the screen
    pub fn set_position_to_middle_of_screen(mut self) -> Self {
        self.position = crate::system::get_center_of_screen_for_object(
            self.size.0,
            self.size.1,
        );
        self
    }
}
#[derive(PartialEq, Copy, Clone, Debug, Eq)]
/// The render level of the window the os should use
/// Any window on the same render level will move in front of every other window on that same level if the user clicks them
pub enum WindowLevel {
    /// Render layer on the bottom -> Always under '[`WindowLevel::Normal`]'
    AlwaysOnBottom,
    /// Render layer sandwiched in the middle of '[`WindowLevel::AlwaysOnBottom`]' and '[`WindowLevel::AlwaysOnTop`]'
    Normal,
    /// Render layer on the top -> Always on top of '[`WindowLevel::Normal`]'
    AlwaysOnTop,
}

#[cfg(feature = "keycodes")]
#[cfg(all(feature = "svg", feature = "system"))]
#[cfg(feature = "std")]
pub use mouse::Cursor;

#[cfg(feature = "std")]
mod buffer;
#[cfg(feature = "std")]
pub use buffer::*;

// Windows
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "minifb")]
#[cfg(feature = "keycodes")]
#[cfg(feature = "std")]
/// The minifb version of the backend
pub mod minifb;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "glfw")]
#[cfg(feature = "keycodes")]
#[cfg(feature = "std")]
/// The glfw version of the backend
pub mod glfw;

#[cfg(feature = "system")]
#[cfg(feature = "std")]
/// Traits used by the backends
pub mod framework_traits;
#[cfg(all(
    //feature = "svg",
    feature = "system",
    //feature = "keycodes",
    feature = "std"
))]
// Window associates
/// Everything do to with cursors
pub mod mouse;

/// A modular system of accessing files/folders
#[cfg(feature = "std")]
pub mod file_system;

#[cfg(feature = "system")]
#[cfg(feature = "keycodes")]
/// There is a lot of duplicate functionality between backends, why not share them?
pub mod shared;
/// Time related stuff, it's not a lot I Reckon
pub mod time;

/// The standart key-board/code detection
#[cfg(feature = "system")]
#[cfg(feature = "keycodes")]
#[cfg(feature = "keyboard_query")]
pub mod keyboard;

// struct Camera {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64,
//     pub width: isize,
//     pub height: isize,
//     half_width: isize,
//     half_height: isize,
// }
// impl Camera {
//     fn new(buffer: &Buffer) -> Self {
//         Self {
//             x: 0.0,
//             y: 0.0,
//             z: 1.0,
//             width: buffer.width as isize,
//             height: buffer.height as isize,
//             half_width: buffer.width as isize / 2,
//             half_height: buffer.height as isize / 2,
//         }
//     }
//     fn get_screen_x(&self, x: isize) -> isize {
//         if self.z == 0.0 {
//             return self.half_width as isize;
//         }
//         return ((x as f64 + self.x) / self.z) as isize;
//     }
//     fn get_screen_y(&self, y: isize) -> isize {
//         if self.z == 0.0 {
//             return self.half_height;
//         }
//         return ((y as f64 + self.y) / self.z) as isize;
//     }
//     fn is_point_visible(&self, xy: (isize,isize)) -> bool {
//         return self.get_screen_x(x) < self.width
//             && self.get_screen_y(y) < self.height;
//     }
// }

// pub mod keyboard;

/// A thread safe double buffer when fast isn't fast enough
#[derive(Debug)]
#[cfg(feature = "std")]
pub struct DoubleBuffer {
    front: Buffer,
    back: Buffer,
    front_is_back: std::sync::atomic::AtomicBool, // true if front buffer is the "back" buffer
}

#[cfg(feature = "std")]
impl DoubleBuffer {
    /// Creates a new instance of the double buffer starting out empty
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            front: Buffer::new_empty((width, height)),
            back: Buffer::new_empty((width, height)),
            front_is_back: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Renderer reads from the front buffer
    pub fn read(&self) -> &Buffer {
        if self.front_is_back.load(std::sync::atomic::Ordering::Acquire) {
            &self.back
        } else {
            &self.front
        }
    }

    /// Sim writes to the back buffer, then swaps
    pub fn write(&mut self, new_data: Buffer) {
        if self.front_is_back.load(std::sync::atomic::Ordering::Acquire) {
            self.front = new_data;
            self.front_is_back
                .store(false, std::sync::atomic::Ordering::Release);
        } else {
            self.back = new_data;
            self.front_is_back
                .store(true, std::sync::atomic::Ordering::Release);
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg(feature = "num_traits")]
/// A struct to convert between 0.0-1.0 and the metrics of the screen
pub struct ScreenNormalizer<S: num_traits::Float> {
    screen_width: S,
    screen_height: S,
}
#[cfg(feature = "num_traits")]
impl<S: num_traits::Float> ScreenNormalizer<S> {
    /// Recommended is using [`crate::math::UniformRange`] in conjunction with this struct
    ///
    /// # Panics
    /// If the dimensions are invalid
    pub const fn new(screen_size: (S, S)) -> Self {
        Self {
            screen_width: screen_size.0,
            screen_height: screen_size.1,
        }
    }
    /// Convert a percentage into screen a coordinate horizontally
    pub fn percentile_to_x<T: num_traits::Num + num_traits::NumCast>(
        &self,
        p: S,
    ) -> Option<T> {
        num_traits::NumCast::from(p * self.screen_width)
    }
    /// Convert a percentage into screen a coordinate vertically
    pub fn percentile_to_y<T: num_traits::Num + num_traits::NumCast>(
        &self,
        p: S,
    ) -> Option<T> {
        num_traits::NumCast::from(p * self.screen_height)
    }
    /// Convert a percentage into screen a coordinate
    pub fn percentile_to_xy<T: num_traits::Num + num_traits::NumCast>(
        &self,
        pxy: (S, S),
    ) -> Option<(T, T)> {
        let x = num_traits::NumCast::from(pxy.0 * self.screen_width);
        let y = num_traits::NumCast::from(pxy.1 * self.screen_height);
        if x.is_none() || y.is_none() {
            None
        } else {
            unsafe { Some((x.unwrap_unchecked(), y.unwrap_unchecked())) }
        }
    }
    /// Convert a horizontal screen coordinate into a percentage
    pub fn x_to_percentile<T: num_traits::Num + num_traits::NumCast>(
        &self,
        x: T,
    ) -> Option<S> {
        if let Some(value) = S::from(x) {
            return Some(value / self.screen_width);
        }
        None
    }
    /// Convert a vertical screen coordinate into a percentage
    pub fn y_to_percentile<T: num_traits::Num + num_traits::NumCast>(
        &self,
        y: T,
    ) -> Option<S> {
        if let Some(value) = S::from(y) {
            return Some(value / self.screen_height);
        }
        None
    }
}
