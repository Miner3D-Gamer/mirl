mod file_data;

use crate::platform::file_data::FileData;

// Stitch together the traits for basic framework
pub trait BasicFramework: FrameworkCore + FrameworkExtended {}
impl<T: FrameworkCore + FrameworkExtended> BasicFramework for T {}
// Stitch together the traits for extended framework
pub trait Framework: BasicFramework + FrameworkControl {}
impl<T: BasicFramework + FrameworkControl> Framework for T {}

pub trait FrameworkCore {
    fn new(buffer: &Buffer, title: &str) -> Self;
    fn update(&mut self, buffer: &[u32]);
    fn is_open(&self) -> bool;
    fn get_mouse_position(&self) -> Option<(f32, f32)>;
    fn get_size(&self) -> (usize, usize);
    fn is_key_down(&self, key: KeyCode) -> bool;
    fn is_mouse_down(&self, button: MouseButton) -> bool;
    fn log<T: std::fmt::Debug>(&self, t: T);
    fn get_time(&self) -> Box<dyn Time>;
    fn sleep(&self, time: u64);
    fn sample_fps(&mut self) -> u64;
}
pub trait FrameworkExtended {
    fn set_title(&mut self, title: &str);
    fn set_target_fps(&mut self, fps: usize);
    fn get_position(&self) -> (isize, isize);
    /// Width/Height should be something like 32x32 or 48x48
    fn set_icon(&mut self, buffer: &[u32], width: u32, height: u32);
    fn set_cursor_style(&mut self, style: &Cursor);
    fn get_mouse_scroll(&self) -> Option<(f32, f32)>;
}
pub trait FrameworkControl: FrameworkExtended {
    fn move_window(&mut self, x: isize, y: isize) {
        let current = self.get_position();
        self.set_position(current.0 + x, current.1 + y);
    }
    fn set_always_ontop(&mut self, always_ontop: bool);
    fn set_position(&mut self, x: isize, y: isize);
    fn set_size(&mut self, buffer: &Buffer);
    fn minimize(&mut self);
    fn maximize(&mut self);
    fn restore(&mut self);
}

pub trait Time {
    /// Get time in milliseconds
    fn get_elapsed_time(&self) -> u64;
}

pub enum CursorStyle {
    Default,   // Pointer
    Crosshair, // Cross
    OpenHand,
    ClosedHand,

    Alias,
    AllScroll,
    BottomLeftCorner,
    BottomRightCorner,
    BottomSide,
    Cell,
    CenterPtr,
    ColResize,
    ColorPicker,
    ContextMenu,
    Copy,
    ClosedHandNoDrop,
    DownArrow,
    Draft,
    Fleur,
    Help,
    LeftArrow,
    LeftSide,
    NoDrop,
    NotAllowed,
    Pencil,
    Pirate,
    Pointer,
    RightArrow,
    RightPtr,
    RightSide,
    RowResize,
    SizeBDiag,
    SizeFDiag,
    SizeHor,
    SizeVer,
    Text,
    TopLeftCorner,
    TopRightCorner,
    TopSide,
    UpArrow,
    VerticalText,
    ZoomIn,
    ZoomOut,
}

pub trait FileSystem {
    fn get_file_contents(
        &self,
        path: &str,
    ) -> Result<FileData, Box<dyn std::error::Error>>;
    fn write_to_file(&self, path: &str, contents: &str);
    fn get_files_in_folder(&self, path: &str) -> Vec<String>;
    fn get_folders_in_folder(&self, path: &str) -> Vec<String>;
    fn join(&self, path1: &str, path2: &str) -> String;
    fn does_file_exist(&self, path: &str) -> bool;
}
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

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
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Quote,
    Tilde,
    Slash,
    Grave,

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
    Ä,
    Ü,
    Ö,
    SS,
    Â,
    Ú,
    Ô,
    Î,
    Ê,
    Ð,
    Œ,
    Á,
    Ý,
    Ï,
    Ñ,
    Ò,

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
}

// mod file_data;
#[cfg(not(target_arch = "wasm32"))]
pub mod minifb;

use cursors::Cursor;
// Import everything from the correct module
#[cfg(not(target_arch = "wasm32"))]
pub use minifb::*;

pub struct Buffer {
    pub buffer: Box<[u32]>,
    pub pointer: *mut u32,
    pub width: usize,
    pub height: usize,
    pub total_size: usize,
}

impl Buffer {
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
    #[inline(always)]
    pub fn clear(&self) {
        unsafe {
            std::ptr::write_bytes(self.pointer, 0, self.total_size);
        }
    }
    pub fn color_buffer(&self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                unsafe {
                    *self.pointer.offset((y * self.width + x) as isize) = color;
                }
            }
        }
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

pub mod cursors;

pub mod file_system;

#[cfg(target_os = "windows")]
mod other;
