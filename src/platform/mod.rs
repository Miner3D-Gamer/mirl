mod file_data;

use crate::platform::file_data::FileData;

pub trait Framework: FrameworkCore + FrameworkExtended {}
impl<T: FrameworkCore + FrameworkExtended> Framework for T {}

pub trait FrameworkCore {
    fn update(&mut self, buffer: &[u32]);
    fn is_open(&self) -> bool;
    fn get_mouse_position(&self) -> Option<(f32, f32)>;
    fn get_size(&self) -> (usize, usize);
    fn is_key_down(&self, key: KeyCode) -> bool;
    fn is_mouse_down(&self, button: MouseButton) -> bool;
    fn log<T: std::fmt::Debug>(&self, t: T);
    fn get_time(&self) -> Box<dyn Time>;
    fn sleep(&self, time: u64);
}
pub trait FrameworkExtended {
    fn set_title(&mut self, title: &str);
    fn set_target_fps(&mut self, fps: usize);
    fn set_always_ontop(&mut self, always_ontop: bool);
    fn set_position(&mut self, x: isize, y: isize);
    fn get_position(&self) -> (isize, isize);
    fn set_icon(&mut self, buffer: &[u32], width: u32, height: u32);
    fn set_cursor_style(&mut self, style: CursorStyle);
    fn get_mouse_scroll(&self) -> Option<(f32, f32)>;
}
impl dyn FrameworkExtended {
    fn move_window(&mut self, x: isize, y: isize) {
        let current = self.get_position();
        self.set_position(current.0 + x, current.1 + y);
    }
}

pub trait Time {
    /// Get time in milliseconds
    fn get_elapsed_time(&self) -> u64;
}

pub enum CursorStyle {
    Default,   // Pointer
    Insertion, // Ibeam
    Crosshair, // Cross
    OpenHand,
    ClosedHand,
    ResizeHorizontal,
    ResizeVertical,
    ResizeAll,
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

// Import everything from the correct module
#[cfg(not(target_arch = "wasm32"))]
pub use minifb::*;
