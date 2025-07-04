use crate::{
    graphics::{
        get_alpha_of_u32, resize_buffer, u32_to_rgba, InterpolationMode,
    },
    platform::file_data::FileData,
};
#[cfg(feature = "system")]
use crate::{
    render::{Tuple2Into, TupleOps},
    system::info::Screen,
};

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
    fn new(required_files: Vec<String>) -> Self
    where
        Self: Sized;
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

#[derive(Debug, Clone, Copy, PartialEq)]
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
pub struct WindowSettings {
    pub borderless: bool,
    pub title: bool,
    pub window_level: WindowLevel,
    pub position: (isize, isize),
    pub size: (isize, isize),
    pub resizeable: bool,
    pub os_menu: bool,
    pub visible: bool,
}
#[cfg(feature = "system")]
impl WindowSettings {
    pub fn default() -> Self {
        let (screen_width, screen_height) =
            crate::system::info::OsInfo::get_screen_resolution();

        let size = (screen_width as isize, screen_height as isize).div((2, 2));
        Self {
            borderless: false,
            title: true,
            window_level: WindowLevel::Normal,
            position: crate::system::info::get_center_of_screen_for_object(
                size.0 as i32,
                size.1 as i32,
            )
            .tuple_2_into(),

            resizeable: false,
            os_menu: true,
            size: size,
            visible: true,
        }
    }
    pub fn set_visible(&mut self, visible: bool) -> &mut Self {
        self.visible = visible;
        self
    }
    pub fn set_size(&mut self, size: (isize, isize)) -> &mut Self {
        self.size = size;
        self
    }
    pub fn set_position(&mut self, position: (isize, isize)) -> &mut Self {
        self.position = position;
        self
    }
    pub fn set_title(&mut self, title: bool) -> &mut Self {
        self.title = title;
        self
    }
    pub fn set_borderless(&mut self, borderless: bool) -> &mut Self {
        self.borderless = borderless;
        self
    }
    pub fn set_window_level(&mut self, window_level: WindowLevel) -> &mut Self {
        self.window_level = window_level;
        self
    }
    pub fn set_resizeable(&mut self, resizeable: bool) -> &mut Self {
        self.resizeable = resizeable;
        self
    }
    pub fn set_os_menu(&mut self, os_menu: bool) -> &mut Self {
        self.os_menu = os_menu;
        self
    }
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
pub enum WindowLevel {
    AlwaysOnBottom,
    Normal,
    AlwaysOnTop,
}

#[cfg(feature = "resvg")]
pub use cursors::Cursor;

#[derive(PartialEq, Clone, Debug)]
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
    pub fn clear_buffer_with_color(&self, color: u32) {
        for idx in 0..self.total_size {
            unsafe {
                *self.pointer.add(idx) = color;
            }
        }
    }

    // Wait that doesn't make any sense:

    /// ~10% faster
    pub fn clear_buffer_with_color_sliced(&self, color: u32) {
        let slice = unsafe {
            std::slice::from_raw_parts_mut(self.pointer, self.total_size)
        };
        slice.fill(color);
    }
    /// ~0-60% faster
    pub fn clear_buffer_with_color_chunked(&self, color: u32) {
        const CHUNK_SIZE: usize = 1024; // Tune this value

        let slice = unsafe {
            std::slice::from_raw_parts_mut(self.pointer, self.total_size)
        };

        for chunk in slice.chunks_mut(CHUNK_SIZE) {
            chunk.fill(color);
        }
    }
    pub fn replace_transparent_with_color(&self, color: u32) {
        for idx in 0..self.total_size {
            unsafe {
                if get_alpha_of_u32(*self.pointer.add(idx)) == 0 {
                    *self.pointer.add(idx) = color;
                }
            }
        }
    }
    /// This function is 0-33% faster than replace_transparent_with_color
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
    // This function is 0-55% faster than replace_transparent_with_color
    pub fn replace_transparent_with_color_lut(&mut self, color: u32) {
        // Pre-compute lookup table for all possible alpha values
        let mut lut = [false; 256];
        lut[0] = true; // Only alpha 0 is transparent

        unsafe {
            for idx in 0..self.total_size {
                let pixel = self.pointer.add(idx);
                let alpha = (*pixel >> 24) as u8;
                if lut[alpha as usize] {
                    *pixel = color;
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

pub fn load_font(path: &str) -> fontdue::Font {
    let font_data = std::fs::read(path).expect("Failed to read font file");
    fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default())
        .expect("Failed to parse font")
}

// Windows
#[cfg(feature = "platform")]
#[cfg(not(target_arch = "wasm32"))]
pub mod minifb;

#[cfg(feature = "platform")]
#[cfg(not(target_arch = "wasm32"))]
pub mod glfw;

#[cfg(feature = "resvg")]
// Window associates
pub mod cursors;
#[cfg(feature = "system")]
pub mod framework_traits;

pub mod file_data;
pub mod file_system;

#[cfg(feature = "system")]
pub mod shared;
pub mod time;

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

// A thread safe double buffer
pub struct DoubleBuffer {
    front: Buffer,
    back: Buffer,
    front_is_back: std::sync::atomic::AtomicBool, // true if front buffer is the "back" buffer
}

impl DoubleBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            front: Buffer::new(width, height),
            back: Buffer::new(width, height),
            front_is_back: std::sync::atomic::AtomicBool::new(false),
        }
    }

    // Renderer reads from the front buffer
    pub fn read(&self) -> &Buffer {
        if self.front_is_back.load(std::sync::atomic::Ordering::Acquire) {
            &self.back
        } else {
            &self.front
        }
    }

    // Sim writes to the back buffer, then swaps
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
//std::thread::yield_now()
#[derive(Clone, Copy, Debug)]
pub struct ScreenNormalizer {
    screen_width: f64,
    screen_height: f64,
}

impl ScreenNormalizer {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        ScreenNormalizer {
            screen_width: screen_width as f64,
            screen_height: screen_height as f64,
        }
    }
    pub fn percentile_to_x<T: num_traits::Num + num_traits::NumCast>(
        &self,
        p: f64,
    ) -> T {
        num_traits::NumCast::from(p * self.screen_width).unwrap()
    }
    pub fn percentile_to_y<T: num_traits::Num + num_traits::NumCast>(
        &self,
        p: f64,
    ) -> T {
        num_traits::NumCast::from(p * self.screen_height).unwrap()
    }
    pub fn x_to_pertentile<T: num_traits::Num + num_traits::NumCast>(
        &self,
        x: T,
    ) -> f64 {
        x.to_f64().unwrap() / self.screen_width
    }
    pub fn y_to_pertentile<T: num_traits::Num + num_traits::NumCast>(
        &self,
        y: T,
    ) -> f64 {
        y.to_f64().unwrap() / self.screen_height
    }
}
