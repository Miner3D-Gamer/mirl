#[cfg(feature = "ico")]
use std::str::FromStr;

#[cfg(feature = "ico")]
use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use minifb::CursorStyle;

use super::framework_traits::{
    Control, ExtendedControl, ExtendedInput, ExtendedTiming, ExtendedWindow,
    Input, Output, Timing, Window,
};
#[cfg(feature = "resvg")]
use super::mouse::load_base_cursor_with_file;
#[cfg(feature = "resvg")]
use super::mouse::Cursor;
use super::Time;
use super::{time::NativeTime, Buffer};
use crate::extensions::*;
#[cfg(feature = "ico")]
use crate::graphics::u32_to_rgba;
use crate::platform::framework_traits::CursorStyleControl;
use crate::platform::keycodes::KeyCode;
use crate::platform::{MouseButton, WindowLevel};
use crate::system::action::Decoration;
use crate::system::action::Default;
/// Backend implementation using MiniFB
#[derive(Debug)]
pub struct Framework {
    window: minifb::Window,
    time: NativeTime,
    #[cfg(feature = "resvg")]
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

impl Window for Framework {
    /// Settings not accounted for:
    ///
    /// visible
    fn new(title: &str, settings: super::WindowSettings) -> Self {
        let width = settings.size.0;
        let height = settings.size.1;
        let mut window = minifb::Window::new(
            title,
            width as usize,
            height as usize,
            minifb_window_options_from_options(&settings),
        )
        .unwrap();

        window.set_position(settings.position.0, settings.position.1);
        crate::system::OsActions::set_window_borderless(
            &get_native_window_handle_from_minifb(&window),
            settings.borderless,
        );
        crate::system::OsActions::set_window_level(
            &get_native_window_handle_from_minifb(&window),
            settings.window_level,
        );

        Self {
            window,
            time: NativeTime::new(),
            #[cfg(feature = "resvg")]
            cursor_subclassed: false,
        }
    }
    #[inline]
    fn update(&mut self, buffer: &[u32]) {
        let s = self.window.get_size();
        self.window.update_with_buffer(buffer, s.0, s.1).unwrap();
    }

    #[inline]
    fn is_open(&self) -> bool {
        self.window.is_open()
    }
}

impl Input for Framework {
    #[inline]
    fn get_mouse_position(&self) -> Option<(isize, isize)> {
        let value =
            self.window.get_unscaled_mouse_pos(minifb::MouseMode::Pass)?;

        Some(value.tuple_2_into())
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
        return r;
    }
    #[inline]
    fn sleep(&self, time: std::time::Duration) {
        super::shared::sleep(time);
    }
}

impl ExtendedControl for Framework {
    #[inline]
    fn set_render_layer(&mut self, level: WindowLevel) {
        crate::system::OsActions::set_window_level(
            &self.get_window_handle(),
            level,
        );
    }
    #[inline]
    fn maximize(&mut self) {
        super::shared::maximize(&self.window.get_window_handle());
    }
    #[inline]
    fn minimize(&mut self) {
        super::shared::minimize(&self.window.get_window_handle());
    }
    #[inline]
    fn restore(&mut self) {
        super::shared::restore(&self.window.get_window_handle());
    }
    fn is_maximized(&self) -> bool {
        super::shared::is_window_maximized(&self.window.get_window_handle())
    }
    fn is_minimized(&self) -> bool {
        super::shared::is_window_minimized(&self.window.get_window_handle())
    }
}

impl<MouseManagerScrollAccuracy: num_traits::Float>
    ExtendedInput<MouseManagerScrollAccuracy> for Framework
{
    #[inline]
    fn get_mouse_scroll(
        &self,
    ) -> Option<(MouseManagerScrollAccuracy, MouseManagerScrollAccuracy)> {
        let t = self.window.get_scroll_wheel();
        if t.is_none() {
            return None;
        }
        let (x, y) = t.unwrap();
        return Some((x, y).tuple_2_into());
    }
    fn get_all_keys_down(&self) -> Vec<KeyCode> {
        return super::keyboard::get_all_pressed_keys();
    }
}

#[cfg(feature = "resvg")]
impl CursorStyleControl for Framework {
    #[inline]
    fn set_cursor_style(&mut self, style: &Cursor) {
        #[cfg(target_os = "windows")]
        {
            if !self.cursor_subclassed {
                unsafe {
                    super::mouse::cursors_windows::subclass_window(
                        self.get_window_handle(),
                        *style,
                    );
                }
                self.cursor_subclassed = true;
            }
        }
        super::mouse::use_cursor(style, None);
    }
    fn load_custom_cursor(
        &mut self,
        size: crate::extensions::U2,
        main_color: u32,
        secondary_color: u32,
    ) -> super::mouse::Cursors {
        super::mouse::Cursors::load(
            size,
            main_color,
            secondary_color,
            load_base_cursor_with_file,
        )
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
    #[cfg(feature = "ico")]
    #[inline]
    fn set_icon(&mut self, buffer: &[u32], width: u32, height: u32) {
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
            let ico_data = encode_to_ico_format(buffer, width, height);

            let temp_dir = std::env::temp_dir();
            let ico_path = temp_dir.join("temp_icon.ico");

            std::fs::write(&ico_path, &ico_data)
                .expect("Failed to write temporary icon file");

            self.window.set_icon(
                minifb::Icon::from_str(ico_path.to_str().unwrap()).unwrap(),
            );
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
        }
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
            std::num::NonZero::new(window_handle as isize).unwrap(),
        );
        raw_window_handle::RawWindowHandle::Win32(handle)
    }
    #[cfg(target_os = "macos")]
    {
        let mut handle = raw_window_handle::AppKitWindowHandle::empty();
        handle.ns_view = window_handle;
        raw_window_handle::RawWindowHandle::AppKit(handle)
    }

    #[cfg(all(target_os = "linux", not(feature = "wayland")))]
    {
        let mut handle = raw_window_handle::XlibWindowHandle::empty();
        handle.window = window_handle;
        //handle.display = window.get_x11_display().cast();
        raw_window_handle::RawWindowHandle::Xlib(handle)
    }

    #[cfg(all(target_os = "linux", feature = "wayland"))]
    {
        let mut handle = raw_window_handle::WaylandWindowHandle::empty();
        handle.surface = window_handle;
        //handle.display = window.get_wayland_display();
        raw_window_handle::RawWindowHandle::Wayland(handle)
    }
}

#[cfg(target_os = "windows")]
impl Control for Framework {
    #[inline]
    fn set_size(&mut self, buffer: &Buffer) {
        super::shared::resize(
            &self.window.get_window_handle(),
            (buffer.width, buffer.height),
        );
    }
    #[inline]
    fn get_size(&self) -> (isize, isize) {
        return crate::system::OsActions::get_window_size(
            &get_native_window_handle_from_minifb(&self.window),
        )
        .tuple_2_into();
    }
    #[inline]
    fn set_position(&mut self, xy: (isize, isize)) {
        self.window.set_position(xy.0, xy.1);
    }
    #[inline]
    fn get_position(&self) -> (isize, isize) {
        self.window.get_position()
    }
}

#[cfg(feature = "ico")]
fn encode_to_ico_format(buffer: &[u32], width: u32, height: u32) -> Vec<u8> {
    // Create a new icon directory
    let mut icon_dir = IconDir::new(ResourceType::Icon);

    // Convert the RGBA u32 buffer to a Vec<u8> in BGRA format
    // Windows .ico format typically expects BGRA ordering
    let mut image_data = Vec::with_capacity(buffer.len() * 4);

    for &pixel in buffer {
        // Extract RGBA components from u32
        let (r, g, b, _a) = u32_to_rgba(pixel);
        // ALPHA IS NOT READ CORRECTLY -> IT'S ALWAYS 0
        // println!("Fix alpha channel not being read correctly");

        // Push as BGRA
        image_data.push(b);
        image_data.push(g);
        image_data.push(r);
        image_data.push(255);
    }

    // Create icon image with proper transparency
    let icon_image = IconImage::from_rgba_data(width, height, image_data);

    // Add the image to the icon directory
    icon_dir.add_entry(
        IconDirEntry::encode(&icon_image).expect("Failed to encode icon image"),
    );

    // Encode the icon directory to a Vec<u8>
    let mut ico_data = Vec::new();
    icon_dir.write(&mut ico_data).expect("Failed to write icon data");

    ico_data
}

// Compile-time key mapping function
// const fn map_cursor_style(style: CursorStyle) -> minifb::CursorStyle {
//     match style {
//         CursorStyle::Default => minifb::CursorStyle::Arrow,
//         CursorStyle::ClosedHand => minifb::CursorStyle::ClosedHand,
//         CursorStyle::OpenHand => minifb::CursorStyle::OpenHand,
//         CursorStyle::Insertion => minifb::CursorStyle::Ibeam,
//         CursorStyle::Crosshair => minifb::CursorStyle::Crosshair,
//         CursorStyle::ResizeHorizontal => minifb::CursorStyle::ResizeLeftRight,
//         CursorStyle::ResizeVertical => minifb::CursorStyle::ResizeUpDown,
//         CursorStyle::ResizeAll => minifb::CursorStyle::ResizeAll,
//     }
// }
/// Maps mirls MouseButtons to MiniFBs MouseButtons
const fn map_mouse_button_to_minifb(
    button: MouseButton,
) -> Option<minifb::MouseButton> {
    match button {
        MouseButton::Left => Some(minifb::MouseButton::Left),
        MouseButton::Right => Some(minifb::MouseButton::Right),
        MouseButton::Middle => Some(minifb::MouseButton::Middle),
        MouseButton::Extra1 => None,
        MouseButton::Extra2 => None,
        MouseButton::Extra3 => None,
        MouseButton::Extra4 => None,
        MouseButton::Unsupported => None,
    }
}
/// Maps mirls KeyCodes to MiniFBs Keycodes
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

        KeyCode::F16 => minifb::Key::Unknown,
        KeyCode::F17 => minifb::Key::Unknown,
        KeyCode::F18 => minifb::Key::Unknown,
        KeyCode::F19 => minifb::Key::Unknown,
        KeyCode::F20 => minifb::Key::Unknown,
        KeyCode::F21 => minifb::Key::Unknown,
        KeyCode::F22 => minifb::Key::Unknown,
        KeyCode::F23 => minifb::Key::Unknown,
        KeyCode::F24 => minifb::Key::Unknown,

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
        KeyCode::Enter => minifb::Key::Enter,
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
        KeyCode::Minus => minifb::Key::Minus,
        KeyCode::Equal => minifb::Key::Equal,
        KeyCode::LeftBracket => minifb::Key::LeftBracket,
        KeyCode::RightBracket => minifb::Key::RightBracket,
        KeyCode::Backslash => minifb::Key::Backslash,
        KeyCode::Semicolon => minifb::Key::Semicolon,
        KeyCode::Quote => minifb::Key::Apostrophe,
        KeyCode::Tilde => minifb::Key::Unknown,

        // Other letters
        KeyCode::AUmlautÄ => minifb::Key::Unknown,
        KeyCode::UUmlautÜ => minifb::Key::Unknown,
        KeyCode::OUmlautÖ => minifb::Key::Unknown,
        KeyCode::SS => minifb::Key::Unknown,
        KeyCode::ACircumflexÂ => minifb::Key::Unknown,
        KeyCode::UAcuteÚ => minifb::Key::Unknown,
        KeyCode::OCircumflexÔ => minifb::Key::Unknown,
        KeyCode::ICircumflexÎ => minifb::Key::Unknown,
        KeyCode::ECircumflexÊ => minifb::Key::Unknown,
        KeyCode::EthÐ => minifb::Key::Unknown,
        KeyCode::OELigatureŒ => minifb::Key::Unknown,
        KeyCode::AAcuteÁ => minifb::Key::Unknown,
        KeyCode::YAcuteÝ => minifb::Key::Unknown,
        KeyCode::IUmlautÏ => minifb::Key::Unknown,
        KeyCode::NTildeÑ => minifb::Key::Unknown,
        KeyCode::OGraveÒ => minifb::Key::Unknown,
        KeyCode::UGraveÙ => minifb::Key::Unknown,
        KeyCode::ARingÅ => minifb::Key::Unknown,
        KeyCode::AELigatureÆ => minifb::Key::Unknown,
        KeyCode::OSlashØ => minifb::Key::Unknown,
        KeyCode::IGraveÌ => minifb::Key::Unknown,
        KeyCode::ThornÞ => minifb::Key::Unknown,

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
        KeyCode::KeyPadDivide => minifb::Key::Slash,
        KeyCode::KeyPadMultiply => minifb::Key::Unknown,
        KeyCode::KeyPadSubtract => minifb::Key::Minus,
        KeyCode::KeyPadAdd => minifb::Key::Unknown,
        KeyCode::KeyPadDecimal => minifb::Key::Unknown,
        KeyCode::KeyPadEnter => minifb::Key::Enter,

        // Multimedia keys (unsupported in minifb)
        KeyCode::MediaPlayPause => minifb::Key::Unknown,
        KeyCode::MediaStop => minifb::Key::Unknown,
        KeyCode::MediaNext => minifb::Key::Unknown,
        KeyCode::MediaPrev => minifb::Key::Unknown,
        KeyCode::VolumeUp => minifb::Key::Unknown,
        KeyCode::VolumeDown => minifb::Key::Unknown,
        KeyCode::Mute => minifb::Key::Unknown,

        // Browser/OS keys
        KeyCode::BrowserBack => minifb::Key::Unknown,
        KeyCode::BrowserForward => minifb::Key::Unknown,
        KeyCode::BrowserRefresh => minifb::Key::Unknown,
        KeyCode::BrowserHome => minifb::Key::Unknown,
        KeyCode::LaunchMail => minifb::Key::Unknown,
        KeyCode::LaunchApp1 => minifb::Key::Unknown,
        KeyCode::LaunchApp2 => minifb::Key::Unknown,

        // Platform-specific
        KeyCode::Menu => minifb::Key::Menu,
        KeyCode::PrintScreen => minifb::Key::Unknown,
        KeyCode::Pause => minifb::Key::Pause,
        KeyCode::Application => minifb::Key::Unknown,

        // Symbols not mapped before
        KeyCode::Slash => minifb::Key::Slash,
        KeyCode::Grave => minifb::Key::Unknown,
        // Fallback
        _ => minifb::Key::Unknown,
    }
}

/// Maps MiniFBs KeyCodes to mirls Keycodes
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
