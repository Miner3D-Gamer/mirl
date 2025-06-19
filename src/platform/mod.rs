

use crate::{graphics::u32_to_rgba, platform::file_data::FileData};

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
    fn write_to_file(&self, path: &str, contents: &[u8]);
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

#[derive(Debug, Clone,Copy,PartialEq)]
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
    pub fn to_u8_rgba(&self) -> Vec<u8> {
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
    pub fn to_u8_argb(&self) -> Vec<u8> {
        let mut return_list = Vec::new();
        for i in &self.buffer {
            let temp = u32_to_rgba(*i);
            return_list.push(temp.3);
            return_list.push(temp.0);
            return_list.push(temp.1);
            return_list.push(temp.2);
        }
        return return_list;
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


pub fn load_font(path: &str) -> fontdue::Font {
    let font_data = std::fs::read(path).expect("Failed to read font file");
    fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default())
        .expect("Failed to parse font")
}

// Windows
#[cfg(not(target_arch = "wasm32"))]
pub mod minifb;

#[cfg(not(target_arch = "wasm32"))]
pub mod glfw;

// Window associates 
pub mod framework_traits;
pub mod cursors;

pub mod file_system;
pub mod file_data;

pub mod shared;
pub mod time;


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