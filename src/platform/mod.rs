mod file_data;

use crate::platform::file_data::FileData;
pub mod framework_traits;

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
    Unsupported,
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
    Ù,
    Å,
    Æ,
    Ø,
    Ì,
    Þ,

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

// mod file_data;
#[cfg(not(target_arch = "wasm32"))]
pub mod minifb;

#[cfg(not(target_arch = "wasm32"))]
pub mod glfw;

pub use cursors::Cursor;

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

mod cursors;

pub mod file_system;

mod shared;
mod time;

pub fn load_font(path: &str) -> fontdue::Font {
    let font_data = std::fs::read(path).expect("Failed to read font file");
    fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default())
        .expect("Failed to parse font")
}
