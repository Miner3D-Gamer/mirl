#[cfg(feature = "system")]
use crate::render::Tuple2Into;
use crate::{
    graphics::{
        get_alpha_of_u32, resize_buffer, u32_to_rgba, InterpolationMode,
    },
    platform::file_data::FileData,
};
/// Time trait, IDK
pub trait Time {
    /// Get time in seconds
    fn get_elapsed_time(&self) -> f64;
}
/// A cursor style, what else to say?
pub enum CursorStyle {
    /// Default Pointer
    Default,
    /// Open hand
    OpenHand,
    /// Closed hand
    ClosedHand,
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
    ColResize,
    /// Eyedropper
    ColorPicker,
    /// Default cursor with ≡ attached
    ContextMenu,
    /// Default cursor with a plus
    Copy,
    /// Cross
    Crosshair,
    /// Closed hand with an 🚫 attached
    ClosedHandNoDrop,
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
    SizeNESW,
    /// Resize top left to bottom right
    SizeNWSE,
    /// Resize horizontally
    SizeHor,
    /// Resize vertically
    SizeVer,
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
/// A trait for a simple file system for possible portability
pub trait FileSystem {
    /// Create a new file system access-er, files that are not defined in required_files are not guaranteed to exist
    fn new(required_files: Vec<String>) -> Self
    where
        Self: Sized;
    /// Get the contents of a file
    fn get_file_contents(
        &self,
        path: &str,
    ) -> Result<FileData, Box<dyn std::error::Error>>;
    /// Write the desired data into the specified file in byte format
    fn write_to_file(&self, path: &str, contents: &[u8]);
    /// Get all file paths in the requested folder
    fn get_files_in_folder(&self, path: &str) -> Vec<String>;
    /// Get all sub folder paths in the requested folder
    fn get_folders_in_folder(&self, path: &str) -> Vec<String>;
    /// Join 2 paths together
    fn join(&self, path1: &str, path2: &str) -> String;
    /// Checks if a file exists
    fn does_file_exist(&self, path: &str) -> bool;
}
/// Supported (and unsupported) mouse buttons
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

#[derive(Debug, Clone, Copy, PartialEq)]
/// Key code to be interpreted anywhere
#[allow(missing_docs)]
pub enum KeyCode {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Numbers
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    KeyPad0,
    KeyPad1,
    KeyPad2,
    KeyPad3,
    KeyPad4,
    KeyPad5,
    KeyPad6,
    KeyPad7,
    KeyPad8,
    KeyPad9,

    // Function Keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    // Modifiers
    LeftShift,
    RightShift,
    LeftControl,
    RightControl,
    LeftAlt,
    RightAlt,
    LeftSuper,
    RightSuper,

    // Symbols / Punctuation
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    Comma,
    Period,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Quote,
    Tilde,
    Slash,
    Grave,
    Apostrophe,

    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Editing keys
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    // Lock keys
    CapsLock,
    NumLock,
    ScrollLock,

    // Keypad operations
    KeyPadDivide,
    KeyPadMultiply,
    KeyPadSubtract,
    KeyPadAdd,
    KeyPadDecimal,
    KeyPadEnter,

    // International & special characters
    AUmlautÄ,
    UUmlautÜ,
    OUmlautÖ,
    SS,
    ACircumflexÂ,
    UAcuteÚ,
    OCircumflexÔ,
    ICircumflexÎ,
    ECircumflexÊ,
    EthÐ,
    OELigatureŒ,
    AAcuteÁ,
    YAcuteÝ,
    IUmlautÏ,
    NTildeÑ,
    OGraveÒ,
    UGraveÙ,
    ARingÅ,
    AELigatureÆ,
    OSlashØ,
    IGraveÌ,
    ThornÞ,

    // Multimedia keys
    MediaPlayPause,
    MediaStop,
    MediaNext,
    MediaPrev,
    VolumeUp,
    VolumeDown,
    Mute,

    // Browser/OS keys
    BrowserBack,
    BrowserForward,
    BrowserRefresh,
    BrowserHome,
    LaunchMail,
    LaunchApp1,
    LaunchApp2,

    // Platform-specific
    Menu,
    PrintScreen,
    Pause,
    Application,

    // what
    F25,
    KeyPadEqual,
    World1,
    World2,
    Unknown,
}
#[cfg(feature = "system")]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Settings for spawning in a window
pub struct WindowSettings {
    /// Remove the border of the window
    pub borderless: bool,
    /// If the title should be displayed
    pub title_visible: bool,
    /// Window render level -> Topmost, normal, bottommost
    pub window_level: WindowLevel,
    /// Position on screen (pixels)
    pub position: (isize, isize),
    /// Size of the spawning window
    pub size: (isize, isize),
    /// If the window should be resizable
    pub resizable: bool,
    /// If the window should have the default os menu around it (fullscreen, minimize, close). The border however will still retain
    pub os_menu: bool,
    /// If the window should be considered visible to the user or not
    pub visible: bool,
}
#[cfg(feature = "system")]
impl WindowSettings {
    /// Get the settings on default settings
    /// For size a buffer is required
    /// For position, it will be centered on the screen
    pub fn default(buffer: &Buffer) -> Self {
        let size = (buffer.width, buffer.height).tuple_2_into();
        Self {
            borderless: false,
            title_visible: true,
            window_level: WindowLevel::Normal,
            position: crate::system::info::get_center_of_screen_for_object(
                size.0 as i32,
                size.1 as i32,
            )
            .tuple_2_into(),

            resizable: false,
            os_menu: true,
            size: size,
            visible: true,
        }
    }
    /// Set the visibility of the window
    pub fn set_visible(&mut self, visible: bool) -> &mut Self {
        self.visible = visible;
        self
    }
    /// Set the size of the window
    pub fn set_size(&mut self, size: (isize, isize)) -> &mut Self {
        self.size = size;
        self
    }
    /// Set the size of the window to the size of the given buffer
    pub fn set_size_to_buffer(&mut self, buffer: &Buffer) -> &mut Self {
        self.size = (buffer.width, buffer.height).tuple_2_into();
        self
    }
    /// Set the position of the window
    pub fn set_position(&mut self, position: (isize, isize)) -> &mut Self {
        self.position = position;
        self
    }
    /// Set if the title should be visible
    pub fn set_title_visible(&mut self, title: bool) -> &mut Self {
        self.title_visible = title;
        self
    }
    /// Sets if the border should be hidden
    pub fn set_borderless(&mut self, borderless: bool) -> &mut Self {
        self.borderless = borderless;
        self
    }
    /// Set the render level of the window (topmost, normal, bottommost)
    pub fn set_window_level(&mut self, window_level: WindowLevel) -> &mut Self {
        self.window_level = window_level;
        self
    }
    /// Set if the window should be resizable by the user
    pub fn set_resizable(&mut self, resizable: bool) -> &mut Self {
        self.resizable = resizable;
        self
    }
    /// Set if the default os decoration should be visible (fullscreen, minimize, close)
    pub fn set_os_menu(&mut self, os_menu: bool) -> &mut Self {
        self.os_menu = os_menu;
        self
    }
    /// Sets the position of the window to be centered on the screen
    pub fn set_position_to_middle_of_screen(&mut self) -> &mut Self {
        self.position = crate::system::info::get_center_of_screen_for_object(
            self.size.0 as i32,
            self.size.1 as i32,
        )
        .tuple_2_into();
        self
    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
/// The render level of the window the os should use
/// Any window on the same render level will move in front of every other window on that same level if the user clicks them
pub enum WindowLevel {
    /// Render layer on the bottom -> Always under '[WindowLevel::Normal]'
    AlwaysOnBottom,
    /// Render layer sandwiched in the middle of '[WindowLevel::AlwaysOnBottom]' and '[WindowLevel::AlwaysOnTop]'
    Normal,
    /// Render layer on the top -> Always on top of '[WindowLevel::Normal]'
    AlwaysOnTop,
}

#[cfg(feature = "resvg")]
pub use cursors::Cursor;

/// A raw color buffer to be modified and read quickly
#[derive(PartialEq, Clone, Debug)]
pub struct Buffer {
    /// Actual color data
    pub buffer: Box<[u32]>,
    /// Pointer to the color data
    pub pointer: *mut u32,
    /// Width of the buffer
    pub width: usize,
    /// Height of the buffer
    pub height: usize,
    /// The total size -> width*height
    pub total_size: usize,
}

impl Buffer {
    /// Create a new color
    pub fn new(width: usize, height: usize) -> Self {
        let total_size = width * height;
        let mut buffer = vec![0u32; total_size].into_boxed_slice();
        let buffer_pointer = buffer.as_mut_ptr();
        Self {
            buffer,
            pointer: buffer_pointer,
            width,
            height,
            total_size,
        }
    }
    /// Replaces all data with zeros very fast
    #[inline(always)]
    pub fn clear(&self) {
        unsafe {
            std::ptr::write_bytes(self.pointer, 0, self.total_size);
        }
    }
    /// Replaces all data with a flat color
    /// An alternative function would be [Self::clear_buffer_with_color_sliced] with an approximate, yet not guaranteed, 10% speed increase
    pub fn clear_buffer_with_color(&self, color: u32) {
        for idx in 0..self.total_size {
            unsafe {
                *self.pointer.add(idx) = color;
            }
        }
    }

    // Wait that doesn't make any sense:

    /// Replaces all data with a flat color
    /// Not yet been properly tested yet roughly 10% faster than [Self::clear_buffer_with_color]
    pub fn clear_buffer_with_color_sliced(&self, color: u32) {
        let slice = unsafe {
            std::slice::from_raw_parts_mut(self.pointer, self.total_size)
        };
        slice.fill(color);
    }
    /// Replaces all color with alpha 0 to the given color
    /// An alternative function would be [Self::replace_transparent_with_color_chunked] with an approximate, yet not guaranteed, 30% speed increase
    pub fn replace_transparent_with_color(&self, color: u32) {
        for idx in 0..self.total_size {
            unsafe {
                if get_alpha_of_u32(*self.pointer.add(idx)) == 0 {
                    *self.pointer.add(idx) = color;
                }
            }
        }
    }
    /// Replaces all color with alpha 0 to the given color
    /// Not yet been properly tested yet roughly 0-33% faster than [Self::replace_transparent_with_color]
    pub fn replace_transparent_with_color_chunked(&mut self, color: u32) {
        const CHUNK_SIZE: usize = 1024; // Tune based on cache size

        for chunk in self.buffer.chunks_mut(CHUNK_SIZE) {
            for pixel in chunk {
                if (*pixel & 0xFF000000) == 0 {
                    *pixel = color;
                }
            }
        }
    }
    /// Converts the Vec<u32> to Vec<8> by unpacking the u32 into argb style
    pub fn to_u8_argb(&self) -> Vec<u8> {
        let mut return_list = Vec::new();
        for i in &self.buffer {
            let temp = u32_to_rgba(*i);
            return_list.push(temp.0);
            return_list.push(temp.1);
            return_list.push(temp.2);
            return_list.push(temp.3);
        }
        return return_list;
    }
    /// Converts the Vec<u32> to Vec<8> by unpacking the u32 into rgba style
    pub fn to_u8_rgba(&self) -> Vec<u8> {
        let mut return_list = Vec::new();
        for i in &self.buffer {
            let temp = u32_to_rgba(*i);
            return_list.push(temp.3);
            return_list.push(temp.1);
            return_list.push(temp.2);
            return_list.push(temp.0);
        }
        return return_list;
    }
    /// Creates a new buffer and copies the contents of the current buffer
    pub fn resize_content(
        &mut self,
        width: usize,
        height: usize,
        resize_mode: InterpolationMode,
    ) -> Buffer {
        let mut new = Buffer::new(width, height);
        let b = resize_buffer(
            &self,
            self.width,
            self.height,
            width,
            height,
            resize_mode,
        );
        new.buffer.copy_from_slice(&b);
        return new;
    }
}

use std::ops::Deref;

// Automatically convert the usage of Buffer to Buffer.buffer
impl Deref for Buffer {
    type Target = [u32];

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

// Windows
#[cfg(feature = "platform")]
#[cfg(not(target_arch = "wasm32"))]
/// The minifb version of the backend
pub mod minifb;

#[cfg(feature = "platform")]
#[cfg(not(target_arch = "wasm32"))]
/// The glfw version of the backend
pub mod glfw;

#[cfg(feature = "resvg")]
// Window associates
/// Everything do to with cursors
pub mod cursors;
#[cfg(feature = "system")]
/// Traits used by the backends
pub mod framework_traits;

/// Why bother reading files if you can't store/process them? Let [file_data::FileData] fix that.
pub mod file_data;
/// A modular system of accessing files/folders
pub mod file_system;

#[cfg(feature = "system")]
/// There is a lot of duplicate functionality between backends, why not share them?
pub mod shared;
/// Time related stuff, it's not a lot I Reckon
pub mod time;

/// The standart key-board/code detection
#[cfg(feature = "system")]
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
//     fn is_point_visible(&self, x: isize, y: isize) -> bool {
//         return self.get_screen_x(x) < self.width
//             && self.get_screen_y(y) < self.height;
//     }
// }

// pub mod keyboard;

/// A thread safe double buffer when fast isn't fast enough
pub struct DoubleBuffer {
    front: Buffer,
    back: Buffer,
    front_is_back: std::sync::atomic::AtomicBool, // true if front buffer is the "back" buffer
}

impl DoubleBuffer {
    /// Creates a new instance of the double buffer starting out empty
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            front: Buffer::new(width, height),
            back: Buffer::new(width, height),
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
/// A struct to convert between 0.0-1.0 and the metrics of the screen
pub struct ScreenNormalizer<S: num_traits::Float> {
    screen_width: S,
    screen_height: S,
}

impl<S: num_traits::Float> ScreenNormalizer<S> {
    /// Recommended is using [crate::math::UniformRange] in conjunction with this struct
    pub fn new<A: num_traits::ToPrimitive>(screen_size: (A, A)) -> Self {
        ScreenNormalizer {
            screen_width: num_traits::NumCast::from(screen_size.0).unwrap(),
            screen_height: num_traits::NumCast::from(screen_size.1).unwrap(),
        }
    }
    /// Convert a percentage into screen a coordinate horizontally
    pub fn percentile_to_x<T: num_traits::Num + num_traits::NumCast>(
        &self,
        p: S,
    ) -> T {
        num_traits::NumCast::from(p * self.screen_width).unwrap()
    }
    /// Convert a percentage into screen a coordinate vertically
    pub fn percentile_to_y<T: num_traits::Num + num_traits::NumCast>(
        &self,
        p: S,
    ) -> T {
        num_traits::NumCast::from(p * self.screen_height).unwrap()
    }
    /// Convert a horizontal screen coordinate into a percentage
    pub fn x_to_percentile<T: num_traits::Num + num_traits::NumCast>(
        &self,
        x: T,
    ) -> S {
        S::from(x).unwrap() / self.screen_width
    }
    /// Convert a vertical screen coordinate into a percentage
    pub fn y_to_percentile<T: num_traits::Num + num_traits::NumCast>(
        &self,
        y: T,
    ) -> S {
        S::from(y).unwrap() / self.screen_height
    }
}
