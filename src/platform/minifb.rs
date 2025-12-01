// #[cfg(feature = "ico")]
// use ico::{IconDir, IconDirEntry, IconImage, ResourceType};

use super::framework_traits::{
    Control, ExtendedControl, ExtendedTiming, ExtendedWindow, Input, Output,
    Timing, Window,
};
#[cfg(feature = "svg")]
use super::mouse::load_base_cursor_with_file;
#[cfg(feature = "svg")]
use super::mouse::Cursor;
use super::Time;
use super::{time::NativeTime, Buffer};
use crate::extensions::*;
// #[cfg(feature = "ico")]
// use crate::graphics::u32_to_rgba_u8;
use crate::platform::framework_traits::CursorStyleControl;
use crate::platform::framework_traits::Errors;
use crate::platform::keycodes::KeyCode;
#[cfg(feature = "svg")]
use crate::platform::mouse::CursorResolution;
#[cfg(feature = "svg")]
use crate::platform::mouse::LoadCursorError;
use crate::platform::{MouseButton, WindowLevel};
#[cfg(feature = "keyboard_query")]
use crate::prelude::ExtendedInput;
use crate::system::action::Decoration;
use crate::system::action::Default;
use crate::system::Os;
/// Backend implementation using `MiniFB`
#[derive(Debug)]
pub struct Framework {
    window: minifb::Window,
    time: NativeTime,
    #[cfg(feature = "svg")]
    cursor_subclassed: bool,
}

fn minifb_window_options_from_options(
    window_options: &super::WindowSettings,
) -> minifb::WindowOptions {
    minifb::WindowOptions {
        borderless: window_options.borderless,
        title: window_options.title_visible,
        resize: window_options.resizable,
        scale: minifb::Scale::X1,
        scale_mode: minifb::ScaleMode::Stretch,
        topmost: window_options.window_level == WindowLevel::AlwaysOnTop,
        transparency: false,
        none: false,
    }
}
#[must_use]
const fn minifb_error_to_error(error: &minifb::Error) -> Errors {
    match error {
        minifb::Error::MenuExists(_) => Errors::DuplicateWindow,
        minifb::Error::MenusNotSupported => Errors::OsNotSupported,
        minifb::Error::UpdateFailed(_) | minifb::Error::WindowCreate(_) => {
            Errors::Unknown
        }
    }
}

impl Window for Framework {
    /// Settings not accounted for:
    ///
    /// visible
    fn new(
        title: &str,
        settings: super::WindowSettings,
    ) -> Result<Self, Errors> {
        let width = settings.size.0;
        let height = settings.size.1;
        let mut window = match minifb::Window::new(
            title,
            width as usize,
            height as usize,
            minifb_window_options_from_options(&settings),
        ) {
            Ok(w) => w,
            Err(er) => {
                return Err(match er {
                    minifb::Error::MenuExists(_) => Errors::DuplicateWindow,
                    minifb::Error::MenusNotSupported => Errors::OsNotSupported,
                    minifb::Error::UpdateFailed(_)
                    | minifb::Error::WindowCreate(_) => Errors::Unknown,
                });
            }
        };

        window.set_position(
            settings.position.0 as isize,
            settings.position.1 as isize,
        );
        crate::system::Os::set_window_borderless(
            &get_native_window_handle_from_minifb(&window),
            settings.borderless,
        );
        crate::system::Os::set_window_level(
            &get_native_window_handle_from_minifb(&window),
            settings.window_level,
        );
        Ok(Self {
            window,
            time: NativeTime::new(),
            #[cfg(feature = "svg")]
            cursor_subclassed: false,
        })
    }
    #[inline]
    fn update(&mut self, buffer: &[u32]) -> Errors {
        let s = self.window.get_size();
        match self.window.update_with_buffer(buffer, s.0, s.1) {
            Ok(()) => Errors::AllGood,
            Err(e) => minifb_error_to_error(&e),
        }
    }

    #[inline]
    fn is_open(&self) -> bool {
        self.window.is_open()
    }
}

impl Input for Framework {
    #[inline]
    fn get_mouse_position(&self) -> Option<(i32, i32)> {
        let value =
            self.window.get_unscaled_mouse_pos(minifb::MouseMode::Pass)?;

        value.try_tuple_into()
    }
    #[inline]
    fn is_key_down(&self, key: KeyCode) -> bool {
        self.window.is_key_down(map_keycode_to_minifb(key))
    }
    #[inline]
    fn is_mouse_down(&self, button: MouseButton) -> bool {
        if let Some(key) = map_mouse_button_to_minifb(button) {
            return self.window.get_mouse_down(key);
        }
        false
    }
}

impl Output for Framework {
    #[inline]
    fn log(&self, t: &str) {
        super::shared::log(t);
    }
}

impl Timing for Framework {
    #[inline]
    fn get_time(&self) -> Box<dyn Time> {
        super::shared::get_time()
    }
    #[inline]
    fn get_delta_time(&mut self) -> f64 {
        let (time, r) = super::shared::sample_fps(&self.time);
        self.time = time;
        r
    }
    #[inline]
    fn sleep(&self, time: std::time::Duration) {
        super::shared::sleep(time);
    }
}
use crate::system::action::Iconized;

impl ExtendedControl for Framework {
    #[inline]
    fn set_render_layer(&mut self, level: WindowLevel) {
        crate::system::Os::set_window_level(&self.get_window_handle(), level);
    }
    #[inline]
    fn maximize(&mut self) {
        Os::maximize(&self.get_window_handle());
    }
    #[inline]
    fn minimize(&mut self) {
        Os::minimize(&self.get_window_handle());
    }
    #[inline]
    fn restore(&mut self) {
        Os::restore(&self.get_window_handle());
    }
    fn is_maximized(&self) -> bool {
        Os::is_maximized(&self.get_window_handle())
    }
    fn is_minimized(&self) -> bool {
        Os::is_minimized(&self.get_window_handle())
    }
}

#[cfg(feature = "keyboard_query")]
impl ExtendedInput for Framework {
    #[inline]
    fn get_mouse_scroll(&self) -> Option<(f32, f32)> {
        let t = self.window.get_scroll_wheel();

        let (x, y) = t?;
        (x, y).try_tuple_into()
    }
    fn get_all_keys_down(&self) -> Vec<KeyCode> {
        super::keyboard::get_all_pressed_keys()
    }
}

#[cfg(feature = "svg")]
impl CursorStyleControl for Framework {
    #[inline]
    fn set_cursor_style(&mut self, style: &Cursor) -> Errors {
        #[cfg(target_os = "windows")]
        {
            if !self.cursor_subclassed {
                unsafe {
                    super::mouse::cursors_windows::subclass_window(
                        self.get_window_handle(),
                        style,
                    );
                }
                self.cursor_subclassed = true;
            }
        }
        super::mouse::use_cursor(style, None)
    }
    fn load_custom_cursors(
        &mut self,
        size: CursorResolution,
        main_color: u32,
        secondary_color: u32,
    ) -> Result<super::mouse::Cursors, LoadCursorError> {
        super::mouse::Cursors::load(
            size,
            main_color,
            secondary_color,
            load_base_cursor_with_file,
        )
        .map_err(|x| {
            LoadCursorError::InvalidImageData(format!("Unable to access {x}"))
        })
    }
    fn load_custom_cursor(
        &mut self,
        image: super::Buffer,
        hotspot: (u8, u8),
    ) -> Result<super::mouse::Cursor, LoadCursorError> {
        #[cfg(feature = "cursor_show_hotspot")]
        let mut image = image;
        super::mouse::cursors_windows::load_cursor(
            #[cfg(feature = "cursor_show_hotspot")]
            &mut image,
            #[cfg(not(feature = "cursor_show_hotspot"))]
            &image,
            u16::from(hotspot.0),
            u16::from(hotspot.1),
        )
        .result_error_into()
        .map(Cursor::Win)
    }
}

impl ExtendedTiming for Framework {
    #[inline]
    fn set_target_fps(&mut self, fps: usize) {
        self.window.set_target_fps(fps);
    }
}

impl ExtendedWindow for Framework {
    #[inline]
    fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }
    // #[inline]
    // fn wait(&self, time: u64) {
    //     std::thread::sleep(Duration::from_millis(time));
    // }
    #[inline]
    fn set_icon(&mut self, buffer: &Buffer) -> Errors {
        // assert_eq!(
        //     buffer.len(),
        //     (width * height) as usize,
        //     "Buffer size doesn't match dimensions"
        // );
        // let buffer64: Vec<u64> = buffer.iter().map(|&x| x as u64).collect();

        // let boxed_buffer = buffer64.into_boxed_slice();

        // // Leak the memory (intentionally) so it persists for the lifetime of the application or whatever the hell that means
        // let leaked_buffer = Box::leak(boxed_buffer);

        // let icon = minifb::Icon::Buffer(
        //     leaked_buffer.as_ptr(),
        //     leaked_buffer.len() as u32,
        // );
        // self.window.set_icon(icon);
        #[cfg(target_os = "windows")]
        {
            use std::str::FromStr;

            let ico_data = crate::graphics::create_ico(buffer);

            let temp_dir = std::env::temp_dir();
            let ico_path = temp_dir.join("temp_icon.ico");
            let string_path = match ico_path.to_str() {
                Some(v) => v.to_string(),
                None => return Errors::Unknown,
            };

            match std::fs::write(&ico_path, &ico_data) {
                Ok(()) => {}
                Err(_) => {
                    return Errors::FileAccessNotPossible {
                        path: string_path,
                    };
                }
            }
            if let Ok(p) = minifb::Icon::from_str(&string_path) {
                self.window.set_icon(p);
            } else {
                return Errors::Unknown;
            }

            return Errors::AllGood;
        }

        // For non-Windows platforms, try the buffer approach
        #[cfg(not(target_os = "windows"))]
        {
            let buffer64: Vec<u64> = buffer.iter().map(|&x| x as u64).collect();
            let boxed_buffer = buffer64.into_boxed_slice();
            let leaked_buffer = Box::leak(boxed_buffer);

            // Different platforms might expect different Icon constructor signatures
            // You might need to check minifb source to see exact parameters needed
            let icon = minifb::Icon::Buffer(
                leaked_buffer.as_ptr(),
                leaked_buffer.len() as u32,
                width,
                height,
            );

            self.window.set_icon(icon);
            return Errors::AllGood;
        }
        #[allow(unused)]
        return Errors::NotImplemented;
    }
    fn get_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        get_native_window_handle_from_minifb(&self.window)
    }
}

fn get_native_window_handle_from_minifb(
    window: &minifb::Window,
) -> raw_window_handle::RawWindowHandle {
    let window_handle = window.get_window_handle();

    #[cfg(target_os = "windows")]
    {
        let handle = raw_window_handle::Win32WindowHandle::new(
            // The window handle cannot be 0
            unsafe {
                std::num::NonZero::new(window_handle as isize)
                    .unwrap_unchecked()
            },
        );
        raw_window_handle::RawWindowHandle::Win32(handle)
    }
    #[cfg(target_os = "macos")]
    {
        let mut handle = raw_window_handle::AppKitWindowHandle::empty();
        handle.ns_view = window_handle;
        raw_window_handle::RawWindowHandle::AppKit(handle)
    }

    // #[cfg(all(target_os = "linux", not(feature = "wayland")))]
    // {
    //     let mut handle = raw_window_handle::XlibWindowHandle::empty();
    //     handle.window = window_handle;
    //     //handle.display = window.get_x11_display().cast();
    //     raw_window_handle::RawWindowHandle::Xlib(handle)
    // }

    // #[cfg(all(target_os = "linux", feature = "wayland"))]
    // {
    //     let mut handle = raw_window_handle::WaylandWindowHandle::empty();
    //     handle.surface = window_handle;
    //     //handle.display = window.get_wayland_display();
    //     raw_window_handle::RawWindowHandle::Wayland(handle)
    // }
}

#[cfg(target_os = "windows")]
impl Control for Framework {
    #[inline]
    fn set_size(&mut self, buffer: &Buffer) {
        Os::set_window_size(
            &self.get_window_handle(),
            (buffer.width as i32, buffer.height as i32),
        );
    }
    #[inline]
    fn get_size(&self) -> (i32, i32) {
        crate::system::Os::get_window_size(
            &get_native_window_handle_from_minifb(&self.window),
        )
    }
    #[inline]
    fn set_position(&mut self, xy: (i32, i32)) {
        self.window.set_position(xy.0 as isize, xy.1 as isize);
    }
    #[inline]
    fn get_position(&self) -> (i32, i32) {
        // MiniFB uses i32 before converting to isize meaning it is 100% safe to reconvert it to i32
        unsafe {
            self.window.get_position().try_tuple_into().unwrap_unchecked()
        }
    }
}

// #[cfg(feature = "ico")]
// fn encode_to_ico_format(
//     buffer: &[u32],
//     width: u32,
//     height: u32,
// ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//     // Create a new icon directory
//     let mut icon_dir = IconDir::new(ResourceType::Icon);

//     // Convert the RGBA u32 buffer to a Vec<u8> in BGRA format
//     // Windows .ico format typically expects BGRA ordering
//     let mut image_data = Vec::with_capacity(buffer.len() * 4);

//     for &pixel in buffer {
//         // Extract RGBA components from u32
//         let (r, g, b, _a) = u32_to_rgba_u8(pixel);
//         // ALPHA IS NOT READ CORRECTLY -> IT'S ALWAYS 0
//         // println!("Fix alpha channel not being read correctly");

//         // Push as BGRA
//         image_data.push(r);
//         image_data.push(g);
//         image_data.push(b);
//         image_data.push(0);
//     }

//     // Create icon image with proper transparency
//     let icon_image = IconImage::from_rgba_data(width, height, image_data);

//     // Add the image to the icon directory
//     icon_dir.add_entry(IconDirEntry::encode(&icon_image)?);

//     // Encode the icon directory to a Vec<u8>
//     let mut ico_data = Vec::new();
//     icon_dir.write(&mut ico_data)?;

//     Ok(ico_data)
// }

// Compile-time key mapping function
// const fn map_cursor_style(style: CursorStyle) -> minifb::CursorStyle {
//     match style {
//         CursorStyle::Default => minifb::CursorStyle::Arrow,
//         CursorStyle::HandClosed => minifb::CursorStyle::HandClosed,
//         CursorStyle::HandOpen => minifb::CursorStyle::HandOpen,
//         CursorStyle::Insertion => minifb::CursorStyle::Ibeam,
//         CursorStyle::Crosshair => minifb::CursorStyle::Crosshair,
//         CursorStyle::ResizeHorizontal => minifb::CursorStyle::ResizeLeftRight,
//         CursorStyle::ResizeVertical => minifb::CursorStyle::ResizeUpDown,
//         CursorStyle::ResizeAll => minifb::CursorStyle::ResizeAll,
//     }
// }
/// Maps mirls `MouseButtons` to `MiniFBs` `MouseButtons`
const fn map_mouse_button_to_minifb(
    button: MouseButton,
) -> Option<minifb::MouseButton> {
    match button {
        MouseButton::Left => Some(minifb::MouseButton::Left),
        MouseButton::Right => Some(minifb::MouseButton::Right),
        MouseButton::Middle => Some(minifb::MouseButton::Middle),
        MouseButton::Extra1
        | MouseButton::Extra2
        | MouseButton::Extra3
        | MouseButton::Extra4
        | MouseButton::Unsupported => None,
    }
}
/// Maps mirls `KeyCodes` to `MiniFBs` Keycodes
#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn map_keycode_to_minifb(key: KeyCode) -> minifb::Key {
    match key {
        // Letters
        KeyCode::A => minifb::Key::A,
        KeyCode::B => minifb::Key::B,
        KeyCode::C => minifb::Key::C,
        KeyCode::D => minifb::Key::D,
        KeyCode::E => minifb::Key::E,
        KeyCode::F => minifb::Key::F,
        KeyCode::G => minifb::Key::G,
        KeyCode::H => minifb::Key::H,
        KeyCode::I => minifb::Key::I,
        KeyCode::J => minifb::Key::J,
        KeyCode::K => minifb::Key::K,
        KeyCode::L => minifb::Key::L,
        KeyCode::M => minifb::Key::M,
        KeyCode::N => minifb::Key::N,
        KeyCode::O => minifb::Key::O,
        KeyCode::P => minifb::Key::P,
        KeyCode::Q => minifb::Key::Q,
        KeyCode::R => minifb::Key::R,
        KeyCode::S => minifb::Key::S,
        KeyCode::T => minifb::Key::T,
        KeyCode::U => minifb::Key::U,
        KeyCode::V => minifb::Key::V,
        KeyCode::W => minifb::Key::W,
        KeyCode::X => minifb::Key::X,
        KeyCode::Y => minifb::Key::Y,
        KeyCode::Z => minifb::Key::Z,

        // Numbers
        KeyCode::Num0 => minifb::Key::Key0,
        KeyCode::Num1 => minifb::Key::Key1,
        KeyCode::Num2 => minifb::Key::Key2,
        KeyCode::Num3 => minifb::Key::Key3,
        KeyCode::Num4 => minifb::Key::Key4,
        KeyCode::Num5 => minifb::Key::Key5,
        KeyCode::Num6 => minifb::Key::Key6,
        KeyCode::Num7 => minifb::Key::Key7,
        KeyCode::Num8 => minifb::Key::Key8,
        KeyCode::Num9 => minifb::Key::Key9,
        KeyCode::KeyPad0 => minifb::Key::NumPad0,
        KeyCode::KeyPad1 => minifb::Key::NumPad1,
        KeyCode::KeyPad2 => minifb::Key::NumPad2,
        KeyCode::KeyPad3 => minifb::Key::NumPad3,
        KeyCode::KeyPad4 => minifb::Key::NumPad4,
        KeyCode::KeyPad5 => minifb::Key::NumPad5,
        KeyCode::KeyPad6 => minifb::Key::NumPad6,
        KeyCode::KeyPad7 => minifb::Key::NumPad7,
        KeyCode::KeyPad8 => minifb::Key::NumPad8,
        KeyCode::KeyPad9 => minifb::Key::NumPad9,

        // Function Keys
        KeyCode::F1 => minifb::Key::F1,
        KeyCode::F2 => minifb::Key::F2,
        KeyCode::F3 => minifb::Key::F3,
        KeyCode::F4 => minifb::Key::F4,
        KeyCode::F5 => minifb::Key::F5,
        KeyCode::F6 => minifb::Key::F6,
        KeyCode::F7 => minifb::Key::F7,
        KeyCode::F8 => minifb::Key::F8,
        KeyCode::F9 => minifb::Key::F9,
        KeyCode::F10 => minifb::Key::F10,
        KeyCode::F11 => minifb::Key::F11,
        KeyCode::F12 => minifb::Key::F12,
        KeyCode::F13 => minifb::Key::F13,
        KeyCode::F14 => minifb::Key::F14,
        KeyCode::F15 => minifb::Key::F15,

        // Modifiers
        KeyCode::LeftShift => minifb::Key::LeftShift,
        KeyCode::RightShift => minifb::Key::RightShift,
        KeyCode::LeftControl => minifb::Key::LeftCtrl,
        KeyCode::RightControl => minifb::Key::RightCtrl,
        KeyCode::LeftAlt => minifb::Key::LeftAlt,
        KeyCode::RightAlt => minifb::Key::RightAlt,
        KeyCode::LeftSuper => minifb::Key::LeftSuper,
        KeyCode::RightSuper => minifb::Key::RightSuper,

        // Symbols
        KeyCode::Space => minifb::Key::Space,
        KeyCode::Enter | KeyCode::KeyPadEnter => minifb::Key::Enter,
        KeyCode::Escape => minifb::Key::Escape,
        KeyCode::Backspace => minifb::Key::Backspace,
        KeyCode::Tab => minifb::Key::Tab,

        // Arrows
        KeyCode::UpArrow => minifb::Key::Up,
        KeyCode::DownArrow => minifb::Key::Down,
        KeyCode::LeftArrow => minifb::Key::Left,
        KeyCode::RightArrow => minifb::Key::Right,

        // Extra
        KeyCode::Comma => minifb::Key::Comma,
        KeyCode::Period => minifb::Key::Period,
        KeyCode::Minus | KeyCode::KeyPadSubtract => minifb::Key::Minus,
        KeyCode::Equal => minifb::Key::Equal,
        KeyCode::LeftBracket => minifb::Key::LeftBracket,
        KeyCode::RightBracket => minifb::Key::RightBracket,
        KeyCode::Backslash => minifb::Key::Backslash,
        KeyCode::Semicolon => minifb::Key::Semicolon,
        KeyCode::Quote => minifb::Key::Apostrophe,
        // Other letters
        // Other
        KeyCode::ScrollLock => minifb::Key::ScrollLock,
        // Lock keys (new)
        KeyCode::CapsLock => minifb::Key::CapsLock,
        KeyCode::NumLock => minifb::Key::NumLock,

        // Editing keys
        KeyCode::Insert => minifb::Key::Insert,
        KeyCode::Delete => minifb::Key::Delete,
        KeyCode::Home => minifb::Key::Home,
        KeyCode::End => minifb::Key::End,
        KeyCode::PageUp => minifb::Key::PageUp,
        KeyCode::PageDown => minifb::Key::PageDown,

        // Keypad ops
        KeyCode::KeyPadDivide | KeyCode::Slash => minifb::Key::Slash,
        // Multimedia keys (unsupported in minifb)
        // Browser/OS keys
        // Platform-specific
        KeyCode::Menu => minifb::Key::Menu,
        KeyCode::Pause => minifb::Key::Pause,
        // Symbols not mapped before
        // Fallback
        _ => minifb::Key::Unknown,
    }
}

/// Maps `MiniFBs` `KeyCodes` to mirls Keycodes
#[must_use]
#[allow(clippy::too_many_lines)]
pub const fn map_minifb_to_keycode(key: minifb::Key) -> KeyCode {
    match key {
        // Letters
        minifb::Key::A => KeyCode::A,
        minifb::Key::B => KeyCode::B,
        minifb::Key::C => KeyCode::C,
        minifb::Key::D => KeyCode::D,
        minifb::Key::E => KeyCode::E,
        minifb::Key::F => KeyCode::F,
        minifb::Key::G => KeyCode::G,
        minifb::Key::H => KeyCode::H,
        minifb::Key::I => KeyCode::I,
        minifb::Key::J => KeyCode::J,
        minifb::Key::K => KeyCode::K,
        minifb::Key::L => KeyCode::L,
        minifb::Key::M => KeyCode::M,
        minifb::Key::N => KeyCode::N,
        minifb::Key::O => KeyCode::O,
        minifb::Key::P => KeyCode::P,
        minifb::Key::Q => KeyCode::Q,
        minifb::Key::R => KeyCode::R,
        minifb::Key::S => KeyCode::S,
        minifb::Key::T => KeyCode::T,
        minifb::Key::U => KeyCode::U,
        minifb::Key::V => KeyCode::V,
        minifb::Key::W => KeyCode::W,
        minifb::Key::X => KeyCode::X,
        minifb::Key::Y => KeyCode::Y,
        minifb::Key::Z => KeyCode::Z,

        // Numbers
        minifb::Key::Key0 => KeyCode::Num0,
        minifb::Key::Key1 => KeyCode::Num1,
        minifb::Key::Key2 => KeyCode::Num2,
        minifb::Key::Key3 => KeyCode::Num3,
        minifb::Key::Key4 => KeyCode::Num4,
        minifb::Key::Key5 => KeyCode::Num5,
        minifb::Key::Key6 => KeyCode::Num6,
        minifb::Key::Key7 => KeyCode::Num7,
        minifb::Key::Key8 => KeyCode::Num8,
        minifb::Key::Key9 => KeyCode::Num9,
        minifb::Key::NumPad0 => KeyCode::KeyPad0,
        minifb::Key::NumPad1 => KeyCode::KeyPad1,
        minifb::Key::NumPad2 => KeyCode::KeyPad2,
        minifb::Key::NumPad3 => KeyCode::KeyPad3,
        minifb::Key::NumPad4 => KeyCode::KeyPad4,
        minifb::Key::NumPad5 => KeyCode::KeyPad5,
        minifb::Key::NumPad6 => KeyCode::KeyPad6,
        minifb::Key::NumPad7 => KeyCode::KeyPad7,
        minifb::Key::NumPad8 => KeyCode::KeyPad8,
        minifb::Key::NumPad9 => KeyCode::KeyPad9,

        // Function Keys
        minifb::Key::F1 => KeyCode::F1,
        minifb::Key::F2 => KeyCode::F2,
        minifb::Key::F3 => KeyCode::F3,
        minifb::Key::F4 => KeyCode::F4,
        minifb::Key::F5 => KeyCode::F5,
        minifb::Key::F6 => KeyCode::F6,
        minifb::Key::F7 => KeyCode::F7,
        minifb::Key::F8 => KeyCode::F8,
        minifb::Key::F9 => KeyCode::F9,
        minifb::Key::F10 => KeyCode::F10,
        minifb::Key::F11 => KeyCode::F11,
        minifb::Key::F12 => KeyCode::F12,
        minifb::Key::F13 => KeyCode::F13,
        minifb::Key::F14 => KeyCode::F14,
        minifb::Key::F15 => KeyCode::F15,

        // Modifiers
        minifb::Key::LeftShift => KeyCode::LeftShift,
        minifb::Key::RightShift => KeyCode::RightShift,
        minifb::Key::LeftCtrl => KeyCode::LeftControl,
        minifb::Key::RightCtrl => KeyCode::RightControl,
        minifb::Key::LeftAlt => KeyCode::LeftAlt,
        minifb::Key::RightAlt => KeyCode::RightAlt,
        minifb::Key::LeftSuper => KeyCode::LeftSuper,
        minifb::Key::RightSuper => KeyCode::RightSuper,

        // Symbols
        minifb::Key::Space => KeyCode::Space,
        minifb::Key::Enter => KeyCode::Enter,
        minifb::Key::Escape => KeyCode::Escape,
        minifb::Key::Backspace => KeyCode::Backspace,
        minifb::Key::Tab => KeyCode::Tab,

        // Arrows
        minifb::Key::Up => KeyCode::UpArrow,
        minifb::Key::Down => KeyCode::DownArrow,
        minifb::Key::Left => KeyCode::LeftArrow,
        minifb::Key::Right => KeyCode::RightArrow,

        // Extras
        minifb::Key::Comma => KeyCode::Comma,
        minifb::Key::Period => KeyCode::Period,
        minifb::Key::Minus => KeyCode::Minus,
        minifb::Key::Equal => KeyCode::Equal,
        minifb::Key::LeftBracket => KeyCode::LeftBracket,
        minifb::Key::RightBracket => KeyCode::RightBracket,
        minifb::Key::Backslash => KeyCode::Backslash,
        minifb::Key::Semicolon => KeyCode::Semicolon,
        minifb::Key::Apostrophe => KeyCode::Quote,

        // Other
        minifb::Key::ScrollLock => KeyCode::ScrollLock,
        minifb::Key::CapsLock => KeyCode::CapsLock,
        minifb::Key::NumLock => KeyCode::NumLock,
        minifb::Key::Insert => KeyCode::Insert,
        minifb::Key::Delete => KeyCode::Delete,
        minifb::Key::Home => KeyCode::Home,
        minifb::Key::End => KeyCode::End,
        minifb::Key::PageUp => KeyCode::PageUp,
        minifb::Key::PageDown => KeyCode::PageDown,
        minifb::Key::Slash => KeyCode::Slash,
        minifb::Key::Menu => KeyCode::Menu,
        minifb::Key::Pause => KeyCode::Pause,

        // Unknown or unmapped
        _ => KeyCode::Unknown,
    }
}
