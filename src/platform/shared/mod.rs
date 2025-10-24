use super::{time::NativeTime, Time};
/// Get the boxed native time struct
#[inline(always)]
#[must_use]
pub fn get_time() -> Box<dyn Time> {
    // Native environment: using SystemTime to get current time
    Box::new(NativeTime::new())
}
/// Sample fps, needs to be called continuously to work properly
#[inline(always)]
pub fn sample_fps<T: Time>(since: &T) -> (NativeTime, f64) {
    let delta_time = since.get_elapsed_time();
    (NativeTime::new(), delta_time)
}
/// Make the thread suspend the current program for the duration of the duration
#[inline(always)]
pub fn sleep(time: std::time::Duration) {
    std::thread::sleep(time);
}
/// Log the given object to the console, not good but it works
#[inline(always)]
pub fn log(t: &str) {
    println!("{t}");
}
/// A struct to manage pressed keys
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct KeyManager {
    // Letters
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
    h: bool,
    i: bool,
    j: bool,
    k: bool,
    l: bool,
    m: bool,
    n: bool,
    o: bool,
    p: bool,
    q: bool,
    r: bool,
    s: bool,
    t: bool,
    u: bool,
    v: bool,
    w: bool,
    x: bool,
    y: bool,
    z: bool,

    // Numbers
    num0: bool,
    num1: bool,
    num2: bool,
    num3: bool,
    num4: bool,
    num5: bool,
    num6: bool,
    num7: bool,
    num8: bool,
    num9: bool,
    key_pad0: bool,
    key_pad1: bool,
    key_pad2: bool,
    key_pad3: bool,
    key_pad4: bool,
    key_pad5: bool,
    key_pad6: bool,
    key_pad7: bool,
    key_pad8: bool,
    key_pad9: bool,

    // Function Keys
    f1: bool,
    f2: bool,
    f3: bool,
    f4: bool,
    f5: bool,
    f6: bool,
    f7: bool,
    f8: bool,
    f9: bool,
    f10: bool,
    f11: bool,
    f12: bool,
    f13: bool,
    f14: bool,
    f15: bool,
    f16: bool,
    f17: bool,
    f18: bool,
    f19: bool,
    f20: bool,
    f21: bool,
    f22: bool,
    f23: bool,
    f24: bool,

    // Modifiers
    left_shift: bool,
    right_shift: bool,
    left_control: bool,
    right_control: bool,
    left_alt: bool,
    right_alt: bool,
    left_super: bool,
    right_super: bool,

    // Symbols / Punctuation
    space: bool,
    enter: bool,
    escape: bool,
    backspace: bool,
    tab: bool,
    comma: bool,
    period: bool,
    minus: bool,
    equal: bool,
    left_bracket: bool,
    right_bracket: bool,
    backslash: bool,
    semicolon: bool,
    quote: bool,
    tilde: bool,
    slash: bool,
    grave: bool,

    // Arrow keys
    up: bool,
    down: bool,
    left: bool,
    right: bool,

    // Editing keys
    insert: bool,
    delete: bool,
    home: bool,
    end: bool,
    page_up: bool,
    page_down: bool,

    // Lock keys
    caps_lock: bool,
    num_lock: bool,
    scroll_lock: bool,

    // Keypad operations
    key_pad_divide: bool,
    key_pad_multiply: bool,
    key_pad_subtract: bool,
    key_pad_add: bool,
    key_pad_decimal: bool,
    key_pad_enter: bool,

    // International & special characters
    a_umlaut_ä: bool,
    u_umlaut_ü: bool,
    o_umlaut_ö: bool,
    ss: bool,
    â: bool,
    ú: bool,
    ô: bool,
    î: bool,
    ê: bool,
    ð: bool,
    œ: bool,
    á: bool,
    ý: bool,
    i_umlaut_ï: bool,
    ñ: bool,
    ò: bool,
    é: bool,
    å: bool,
    æ: bool,
    ø: bool,
    í: bool,
    þ: bool,
    ù: bool,
    ì: bool,

    // Multimedia keys
    media_play_pause: bool,
    media_stop: bool,
    media_next: bool,
    media_prev: bool,
    volume_up: bool,
    volume_down: bool,
    mute: bool,

    // Browser/OS keys
    browser_back: bool,
    browser_forward: bool,
    browser_refresh: bool,
    browser_home: bool,
    launch_mail: bool,
    launch_app1: bool,
    launch_app2: bool,

    // Platform-specific
    menu: bool,
    print_screen: bool,
    pause: bool,
    application: bool,

    apostrophe: bool,
    f25: bool,

    key_pad_equal: bool,
    world_1: bool,
    world_2: bool,
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyManager {
    /// Create a new `KeyManager` instance, keep in mind that key manager itself does not check if a key is down.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            a: false,
            b: false,
            c: false,
            d: false,
            e: false,
            f: false,
            g: false,
            h: false,
            i: false,
            j: false,
            k: false,
            l: false,
            m: false,
            n: false,
            o: false,
            p: false,
            q: false,
            r: false,
            s: false,
            t: false,
            u: false,
            v: false,
            w: false,
            x: false,
            y: false,
            z: false,
            num0: false,
            num1: false,
            num2: false,
            num3: false,
            num4: false,
            num5: false,
            num6: false,
            num7: false,
            num8: false,
            num9: false,
            key_pad0: false,
            key_pad1: false,
            key_pad2: false,
            key_pad3: false,
            key_pad4: false,
            key_pad5: false,
            key_pad6: false,
            key_pad7: false,
            key_pad8: false,
            key_pad9: false,
            f1: false,
            f2: false,
            f3: false,
            f4: false,
            f5: false,
            f6: false,
            f7: false,
            f8: false,
            f9: false,
            f10: false,
            f11: false,
            f12: false,
            f13: false,
            f14: false,
            f15: false,
            f16: false,
            f17: false,
            f18: false,
            f19: false,
            f20: false,
            f21: false,
            f22: false,
            f23: false,
            f24: false,
            left_shift: false,
            right_shift: false,
            left_control: false,
            right_control: false,
            left_alt: false,
            right_alt: false,
            left_super: false,
            right_super: false,
            space: false,
            enter: false,
            escape: false,
            backspace: false,
            tab: false,
            comma: false,
            period: false,
            minus: false,
            equal: false,
            left_bracket: false,
            right_bracket: false,
            backslash: false,
            semicolon: false,
            quote: false,
            tilde: false,
            slash: false,
            grave: false,
            up: false,
            down: false,
            left: false,
            right: false,
            insert: false,
            delete: false,
            home: false,
            end: false,
            page_up: false,
            page_down: false,
            caps_lock: false,
            num_lock: false,
            scroll_lock: false,
            key_pad_divide: false,
            key_pad_multiply: false,
            key_pad_subtract: false,
            key_pad_add: false,
            key_pad_decimal: false,
            key_pad_enter: false,
            a_umlaut_ä: false,
            u_umlaut_ü: false,
            o_umlaut_ö: false,
            ss: false,
            â: false,
            ú: false,
            ô: false,
            î: false,
            ê: false,
            ð: false,
            œ: false,
            á: false,
            ý: false,
            i_umlaut_ï: false,
            ñ: false,
            ò: false,
            é: false,
            å: false,
            æ: false,
            ø: false,
            í: false,
            þ: false,
            media_play_pause: false,
            media_stop: false,
            media_next: false,
            media_prev: false,
            volume_up: false,
            volume_down: false,
            mute: false,
            browser_back: false,
            browser_forward: false,
            browser_refresh: false,
            browser_home: false,
            launch_mail: false,
            launch_app1: false,
            launch_app2: false,
            menu: false,
            print_screen: false,
            pause: false,
            application: false,
            key_pad_equal: false,
            apostrophe: false,
            f25: false,
            world_1: false,
            world_2: false,
            ì: false,
            ù: false,
        }
    }
    /// Checks if a key is pressed
    #[must_use]
    pub const fn is_key_pressed(&self, keycode: KeyCode) -> bool {
        map_keycode(keycode, self)
    }
    /// Sets if a key is down
    pub const fn set_key_state(&mut self, keycode: KeyCode, value: bool) {
        set_keycode(keycode, self, value);
    }
    /// Get every pressed key (by checking if every single one is pressed)
    #[must_use]
    pub fn get_all_pressed_keys(&self) -> Vec<KeyCode> {
        let mut key_codes = Vec::new();
        for variant in KeyCode::iter() {
            if self.is_key_pressed(variant) {
                key_codes.push(variant);
            }
        }
        key_codes
    }
}

use strum::IntoEnumIterator;

/// A struct to manage the pressed mouse keys + scroll
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
pub struct MouseManager {
    scroll_x: f32,
    scroll_y: f32,
    left_mouse_button: bool,
    right_mouse_button: bool,
    middle_mouse_button: bool,
    // Other mouse buttons not supported due to me not having a mouse that supports them
    extra1_button: bool,
    extra2_button: bool,
    // Never mind
    extra3_button: bool,
    extra4_button: bool,
}

impl Default for MouseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseManager {
    /// Create a new `MouseManager` instance, keep in mind that it doesn't check if any keys are down/scroll itself
    #[must_use]
    pub const fn new() -> Self {
        Self {
            scroll_x: 0.0,
            scroll_y: 0.0,
            left_mouse_button: false,
            right_mouse_button: false,
            middle_mouse_button: false,
            extra1_button: false,
            extra2_button: false,
            extra3_button: false,
            extra4_button: false,
        }
    }
    /// Set the scroll
    pub const fn set_scroll(&mut self, xy: (f32, f32)) {
        self.scroll_x = xy.0;
        self.scroll_y = xy.1;
    }
    /// Add to the scroll
    pub fn add_scroll(&mut self, xy: (f32, f32)) {
        self.scroll_x += xy.0;
        self.scroll_y += xy.1;
    }
    /// Reset the scroll
    pub const fn reset_scroll(&mut self) {
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;
    }
    #[must_use]
    /// Get the mouse wheel scroll
    pub const fn get_scroll(&self) -> (f32, f32) {
        (self.scroll_x, self.scroll_y)
    }
    #[must_use]
    /// Check if a mouse button is pressed
    pub const fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        map_button(button, self)
    }
    /// Set the state of a mouse button
    pub const fn set_mouse_button_state(
        &mut self,
        button: MouseButton,
        value: bool,
    ) {
        set_mouse_button(button, self, value);
    }
}
use super::MouseButton;
use crate::platform::keycodes::KeyCode;
#[must_use]
/// Get the value [`MouseButton`] of [`MouseManager`]
pub const fn map_button(
    button: MouseButton,
    mouse_manager: &MouseManager,
) -> bool {
    match button {
        MouseButton::Left => mouse_manager.left_mouse_button,
        MouseButton::Right => mouse_manager.right_mouse_button,
        MouseButton::Middle => mouse_manager.middle_mouse_button,
        MouseButton::Extra1 => mouse_manager.extra1_button,
        MouseButton::Extra2 => mouse_manager.extra2_button,
        MouseButton::Extra3 => mouse_manager.extra3_button,
        MouseButton::Extra4 => mouse_manager.extra4_button,
        MouseButton::Unsupported => false,
    }
}
/// Set the value [`MouseButton`] of [`MouseManager`]
pub const fn set_mouse_button(
    button: MouseButton,
    mouse_manager: &mut MouseManager,
    value: bool,
) {
    match button {
        MouseButton::Left => mouse_manager.left_mouse_button = value,
        MouseButton::Right => mouse_manager.right_mouse_button = value,
        MouseButton::Middle => mouse_manager.middle_mouse_button = value,
        MouseButton::Extra1 => mouse_manager.extra1_button = value,
        MouseButton::Extra2 => mouse_manager.extra2_button = value,
        MouseButton::Extra3 => mouse_manager.extra3_button = value,
        MouseButton::Extra4 => mouse_manager.extra4_button = value,
        MouseButton::Unsupported => (),
    }
}

/// Get the value [`KeyCode`] of [`KeyManager`]
#[must_use]
pub const fn map_keycode(keycode: KeyCode, key_manager: &KeyManager) -> bool {
    match keycode {
        KeyCode::A => key_manager.a,
        KeyCode::B => key_manager.b,
        KeyCode::C => key_manager.c,
        KeyCode::D => key_manager.d,
        KeyCode::E => key_manager.e,
        KeyCode::F => key_manager.f,
        KeyCode::G => key_manager.g,
        KeyCode::H => key_manager.h,
        KeyCode::I => key_manager.i,
        KeyCode::J => key_manager.j,
        KeyCode::K => key_manager.k,
        KeyCode::L => key_manager.l,
        KeyCode::M => key_manager.m,
        KeyCode::N => key_manager.n,
        KeyCode::O => key_manager.o,
        KeyCode::P => key_manager.p,
        KeyCode::Q => key_manager.q,
        KeyCode::R => key_manager.r,
        KeyCode::S => key_manager.s,
        KeyCode::T => key_manager.t,
        KeyCode::U => key_manager.u,
        KeyCode::V => key_manager.v,
        KeyCode::W => key_manager.w,
        KeyCode::X => key_manager.x,
        KeyCode::Y => key_manager.y,
        KeyCode::Z => key_manager.z,
        KeyCode::Num0 => key_manager.num0,
        KeyCode::Num1 => key_manager.num1,
        KeyCode::Num2 => key_manager.num2,
        KeyCode::Num3 => key_manager.num3,
        KeyCode::Num4 => key_manager.num4,
        KeyCode::Num5 => key_manager.num5,
        KeyCode::Num6 => key_manager.num6,
        KeyCode::Num7 => key_manager.num7,
        KeyCode::Num8 => key_manager.num8,
        KeyCode::Num9 => key_manager.num9,
        KeyCode::KeyPad0 => key_manager.key_pad0,
        KeyCode::KeyPad1 => key_manager.key_pad1,
        KeyCode::KeyPad2 => key_manager.key_pad2,
        KeyCode::KeyPad3 => key_manager.key_pad3,
        KeyCode::KeyPad4 => key_manager.key_pad4,
        KeyCode::KeyPad5 => key_manager.key_pad5,
        KeyCode::KeyPad6 => key_manager.key_pad6,
        KeyCode::KeyPad7 => key_manager.key_pad7,
        KeyCode::KeyPad8 => key_manager.key_pad8,
        KeyCode::KeyPad9 => key_manager.key_pad9,
        KeyCode::F1 => key_manager.f1,
        KeyCode::F2 => key_manager.f2,
        KeyCode::F3 => key_manager.f3,
        KeyCode::F4 => key_manager.f4,
        KeyCode::F5 => key_manager.f5,
        KeyCode::F6 => key_manager.f6,
        KeyCode::F7 => key_manager.f7,
        KeyCode::F8 => key_manager.f8,
        KeyCode::F9 => key_manager.f9,
        KeyCode::F10 => key_manager.f10,
        KeyCode::F11 => key_manager.f11,
        KeyCode::F12 => key_manager.f12,
        KeyCode::F13 => key_manager.f13,
        KeyCode::F14 => key_manager.f14,
        KeyCode::F15 => key_manager.f15,
        KeyCode::F16 => key_manager.f16,
        KeyCode::F17 => key_manager.f17,
        KeyCode::F18 => key_manager.f18,
        KeyCode::F19 => key_manager.f19,
        KeyCode::F20 => key_manager.f20,
        KeyCode::F21 => key_manager.f21,
        KeyCode::F22 => key_manager.f22,
        KeyCode::F23 => key_manager.f23,
        KeyCode::F24 => key_manager.f24,
        KeyCode::Application => key_manager.application,
        KeyCode::Escape => key_manager.escape,
        KeyCode::Enter => key_manager.enter,
        KeyCode::Backspace => key_manager.backspace,
        KeyCode::Tab => key_manager.tab,
        KeyCode::Space => key_manager.space,
        KeyCode::Minus => key_manager.minus,
        KeyCode::Equal => key_manager.equal,
        KeyCode::LeftBracket => key_manager.left_bracket,
        KeyCode::RightBracket => key_manager.right_bracket,
        KeyCode::Backslash => key_manager.backslash,
        KeyCode::Semicolon => key_manager.semicolon,
        KeyCode::Grave => key_manager.grave,
        KeyCode::Comma => key_manager.comma,
        KeyCode::Period => key_manager.period,
        KeyCode::Slash => key_manager.slash,
        KeyCode::CapsLock => key_manager.caps_lock,
        KeyCode::ScrollLock => key_manager.scroll_lock,
        KeyCode::NumLock => key_manager.num_lock,
        KeyCode::BrowserBack => key_manager.browser_back,
        KeyCode::BrowserForward => key_manager.browser_forward,
        KeyCode::KeyPadAdd => key_manager.key_pad_add,
        KeyCode::KeyPadSubtract => key_manager.key_pad_subtract,
        KeyCode::KeyPadMultiply => key_manager.key_pad_multiply,
        KeyCode::KeyPadDivide => key_manager.key_pad_divide,
        KeyCode::KeyPadEnter => key_manager.key_pad_enter,
        KeyCode::KeyPadDecimal => key_manager.key_pad_decimal,
        KeyCode::LeftShift => key_manager.left_shift,
        KeyCode::LeftControl => key_manager.left_control,
        KeyCode::LeftAlt => key_manager.left_alt,
        KeyCode::LeftSuper => key_manager.left_super,
        KeyCode::RightShift => key_manager.right_shift,
        KeyCode::RightControl => key_manager.right_control,
        KeyCode::RightAlt => key_manager.right_alt,
        KeyCode::RightSuper => key_manager.right_super,
        KeyCode::Menu => key_manager.menu,
        KeyCode::Quote => key_manager.quote,
        KeyCode::Tilde => key_manager.tilde,
        KeyCode::UpArrow => key_manager.up,
        KeyCode::DownArrow => key_manager.down,
        KeyCode::LeftArrow => key_manager.left,
        KeyCode::RightArrow => key_manager.right,
        KeyCode::PageUp => key_manager.page_up,
        KeyCode::PageDown => key_manager.page_down,
        KeyCode::Home => key_manager.home,
        KeyCode::End => key_manager.end,
        KeyCode::Insert => key_manager.insert,
        KeyCode::Delete => key_manager.delete,

        KeyCode::AUmlautÄ => key_manager.a_umlaut_ä,

        KeyCode::ECircumflexÊ => key_manager.ê,
        KeyCode::OGraveÒ => key_manager.ô,
        KeyCode::UGraveÙ => key_manager.ù,
        KeyCode::ARingÅ => key_manager.å,
        KeyCode::AELigatureÆ => key_manager.æ,
        KeyCode::OSlashØ => key_manager.ø,
        KeyCode::UUmlautÜ => key_manager.u_umlaut_ü,
        KeyCode::IGraveÌ => key_manager.ì,
        KeyCode::BrowserHome => key_manager.browser_home,
        KeyCode::BrowserRefresh => key_manager.browser_refresh,
        KeyCode::LaunchApp1 => key_manager.launch_app1,
        KeyCode::LaunchApp2 => key_manager.launch_app2,
        KeyCode::LaunchMail => key_manager.launch_mail,
        KeyCode::MediaNext => key_manager.media_next,
        KeyCode::MediaPlayPause => key_manager.media_play_pause,
        KeyCode::MediaPrev => key_manager.media_prev,
        KeyCode::MediaStop => key_manager.media_stop,
        KeyCode::Mute => key_manager.mute,
        KeyCode::PrintScreen => key_manager.print_screen,
        KeyCode::SS => key_manager.ss,
        KeyCode::VolumeDown => key_manager.volume_down,
        KeyCode::VolumeUp => key_manager.volume_up,
        KeyCode::AAcuteÁ => key_manager.á,
        KeyCode::ACircumflexÂ => key_manager.â,
        KeyCode::ICircumflexÎ => key_manager.î,
        KeyCode::IUmlautÏ => key_manager.i_umlaut_ï,
        KeyCode::EthÐ => key_manager.ð,
        KeyCode::NTildeÑ => key_manager.ñ,
        KeyCode::OCircumflexÔ => key_manager.ò,
        KeyCode::OUmlautÖ => key_manager.o_umlaut_ö,
        KeyCode::UAcuteÚ => key_manager.ú,
        KeyCode::YAcuteÝ => key_manager.ý,
        KeyCode::ThornÞ => key_manager.þ,
        KeyCode::OELigatureŒ => key_manager.œ,

        KeyCode::AnyAlt => key_manager.right_alt || key_manager.left_alt,
        KeyCode::AnyControl => {
            key_manager.right_control || key_manager.left_control
        }
        KeyCode::AnyShift => key_manager.right_shift || key_manager.left_shift,
        KeyCode::AnySuper => key_manager.right_super || key_manager.left_super,

        KeyCode::Pause => key_manager.pause,
        KeyCode::Apostrophe => key_manager.apostrophe,
        KeyCode::F25 => key_manager.f25,
        KeyCode::KeyPadEqual => key_manager.key_pad_equal,
        KeyCode::World1 => key_manager.world_1,
        KeyCode::World2 => key_manager.world_2,

        KeyCode::Unknown => false,
    }
}

/// Set the value [`KeyCode`] of [`KeyManager`]
pub const fn set_keycode(
    keycode: KeyCode,
    key_manager: &mut KeyManager,
    value: bool,
) {
    match keycode {
        KeyCode::A => key_manager.a = value,
        KeyCode::B => key_manager.b = value,
        KeyCode::C => key_manager.c = value,
        KeyCode::D => key_manager.d = value,
        KeyCode::E => key_manager.e = value,
        KeyCode::F => key_manager.f = value,
        KeyCode::G => key_manager.g = value,
        KeyCode::H => key_manager.h = value,
        KeyCode::I => key_manager.i = value,
        KeyCode::J => key_manager.j = value,
        KeyCode::K => key_manager.k = value,
        KeyCode::L => key_manager.l = value,
        KeyCode::M => key_manager.m = value,
        KeyCode::N => key_manager.n = value,
        KeyCode::O => key_manager.o = value,
        KeyCode::P => key_manager.p = value,
        KeyCode::Q => key_manager.q = value,
        KeyCode::R => key_manager.r = value,
        KeyCode::S => key_manager.s = value,
        KeyCode::T => key_manager.t = value,
        KeyCode::U => key_manager.u = value,
        KeyCode::V => key_manager.v = value,
        KeyCode::W => key_manager.w = value,
        KeyCode::X => key_manager.x = value,
        KeyCode::Y => key_manager.y = value,
        KeyCode::Z => key_manager.z = value,
        KeyCode::Num0 => key_manager.num0 = value,
        KeyCode::Num1 => key_manager.num1 = value,
        KeyCode::Num2 => key_manager.num2 = value,
        KeyCode::Num3 => key_manager.num3 = value,
        KeyCode::Num4 => key_manager.num4 = value,
        KeyCode::Num5 => key_manager.num5 = value,
        KeyCode::Num6 => key_manager.num6 = value,
        KeyCode::Num7 => key_manager.num7 = value,
        KeyCode::Num8 => key_manager.num8 = value,
        KeyCode::Num9 => key_manager.num9 = value,
        KeyCode::KeyPad0 => key_manager.key_pad0 = value,
        KeyCode::KeyPad1 => key_manager.key_pad1 = value,
        KeyCode::KeyPad2 => key_manager.key_pad2 = value,
        KeyCode::KeyPad3 => key_manager.key_pad3 = value,
        KeyCode::KeyPad4 => key_manager.key_pad4 = value,
        KeyCode::KeyPad5 => key_manager.key_pad5 = value,
        KeyCode::KeyPad6 => key_manager.key_pad6 = value,
        KeyCode::KeyPad7 => key_manager.key_pad7 = value,
        KeyCode::KeyPad8 => key_manager.key_pad8 = value,
        KeyCode::KeyPad9 => key_manager.key_pad9 = value,
        KeyCode::F1 => key_manager.f1 = value,
        KeyCode::F2 => key_manager.f2 = value,
        KeyCode::F3 => key_manager.f3 = value,
        KeyCode::F4 => key_manager.f4 = value,
        KeyCode::F5 => key_manager.f5 = value,
        KeyCode::F6 => key_manager.f6 = value,
        KeyCode::F7 => key_manager.f7 = value,
        KeyCode::F8 => key_manager.f8 = value,
        KeyCode::F9 => key_manager.f9 = value,
        KeyCode::F10 => key_manager.f10 = value,
        KeyCode::F11 => key_manager.f11 = value,
        KeyCode::F12 => key_manager.f12 = value,
        KeyCode::F13 => key_manager.f13 = value,
        KeyCode::F14 => key_manager.f14 = value,
        KeyCode::F15 => key_manager.f15 = value,
        KeyCode::F16 => key_manager.f16 = value,
        KeyCode::F17 => key_manager.f17 = value,
        KeyCode::F18 => key_manager.f18 = value,
        KeyCode::F19 => key_manager.f19 = value,
        KeyCode::F20 => key_manager.f20 = value,
        KeyCode::F21 => key_manager.f21 = value,
        KeyCode::F22 => key_manager.f22 = value,
        KeyCode::F23 => key_manager.f23 = value,
        KeyCode::F24 => key_manager.f24 = value,
        KeyCode::Application => key_manager.application = value,
        KeyCode::Escape => key_manager.escape = value,
        KeyCode::Enter => key_manager.enter = value,
        KeyCode::Backspace => key_manager.backspace = value,
        KeyCode::Tab => key_manager.tab = value,
        KeyCode::Space => key_manager.space = value,
        KeyCode::Minus => key_manager.minus = value,
        KeyCode::Equal => key_manager.equal = value,
        KeyCode::LeftBracket => key_manager.left_bracket = value,
        KeyCode::RightBracket => key_manager.right_bracket = value,
        KeyCode::Backslash => key_manager.backslash = value,
        KeyCode::Semicolon => key_manager.semicolon = value,
        KeyCode::Grave => key_manager.grave = value,
        KeyCode::Comma => key_manager.comma = value,
        KeyCode::Period => key_manager.period = value,
        KeyCode::Slash => key_manager.slash = value,
        KeyCode::CapsLock => key_manager.caps_lock = value,
        KeyCode::ScrollLock => key_manager.scroll_lock = value,
        KeyCode::NumLock => key_manager.num_lock = value,
        KeyCode::BrowserBack => key_manager.browser_back = value,
        KeyCode::BrowserForward => key_manager.browser_forward = value,
        KeyCode::KeyPadAdd => key_manager.key_pad_add = value,
        KeyCode::KeyPadSubtract => key_manager.key_pad_subtract = value,
        KeyCode::KeyPadMultiply => key_manager.key_pad_multiply = value,
        KeyCode::KeyPadDivide => key_manager.key_pad_divide = value,
        KeyCode::KeyPadEnter => key_manager.key_pad_enter = value,
        KeyCode::KeyPadDecimal => key_manager.key_pad_decimal = value,
        KeyCode::LeftShift => key_manager.left_shift = value,
        KeyCode::LeftControl => key_manager.left_control = value,
        KeyCode::LeftAlt => key_manager.left_alt = value,
        KeyCode::LeftSuper => key_manager.left_super = value,
        KeyCode::RightShift => key_manager.right_shift = value,
        KeyCode::RightControl => key_manager.right_control = value,
        KeyCode::RightAlt => key_manager.right_alt = value,
        KeyCode::RightSuper => key_manager.right_super = value,
        KeyCode::Menu => key_manager.menu = value,
        KeyCode::Quote => key_manager.quote = value,
        KeyCode::Tilde => key_manager.tilde = value,
        KeyCode::UpArrow => key_manager.up = value,
        KeyCode::DownArrow => key_manager.down = value,
        KeyCode::LeftArrow => key_manager.left = value,
        KeyCode::RightArrow => key_manager.right = value,
        KeyCode::PageUp => key_manager.page_up = value,
        KeyCode::PageDown => key_manager.page_down = value,
        KeyCode::Home => key_manager.home = value,
        KeyCode::End => key_manager.end = value,
        KeyCode::Insert => key_manager.insert = value,
        KeyCode::Delete => key_manager.delete = value,
        KeyCode::AUmlautÄ => key_manager.a_umlaut_ä = value,
        KeyCode::ECircumflexÊ => key_manager.ê = value,
        KeyCode::OUmlautÖ => key_manager.ô = value,
        KeyCode::UGraveÙ => key_manager.ù = value,
        KeyCode::ARingÅ => key_manager.å = value,
        KeyCode::AELigatureÆ => key_manager.æ = value,
        KeyCode::OSlashØ => key_manager.ø = value,
        KeyCode::UUmlautÜ => key_manager.u_umlaut_ü = value,
        KeyCode::IGraveÌ => key_manager.ì = value,
        KeyCode::BrowserHome => key_manager.browser_home = value,
        KeyCode::BrowserRefresh => key_manager.browser_refresh = value,
        KeyCode::LaunchApp1 => key_manager.launch_app1 = value,
        KeyCode::LaunchApp2 => key_manager.launch_app2 = value,
        KeyCode::LaunchMail => key_manager.launch_mail = value,
        KeyCode::MediaNext => key_manager.media_next = value,
        KeyCode::MediaPlayPause => key_manager.media_play_pause = value,
        KeyCode::MediaPrev => key_manager.media_prev = value,
        KeyCode::MediaStop => key_manager.media_stop = value,
        KeyCode::Mute => key_manager.mute = value,
        KeyCode::PrintScreen => key_manager.print_screen = value,
        KeyCode::SS => key_manager.ss = value,
        KeyCode::VolumeDown => key_manager.volume_down = value,
        KeyCode::VolumeUp => key_manager.volume_up = value,
        KeyCode::AAcuteÁ => key_manager.á = value,
        KeyCode::ACircumflexÂ => key_manager.â = value,
        KeyCode::ICircumflexÎ => key_manager.î = value,
        KeyCode::IUmlautÏ => key_manager.i_umlaut_ï = value,
        KeyCode::EthÐ => key_manager.ð = value,
        KeyCode::NTildeÑ => key_manager.ñ = value,
        KeyCode::OGraveÒ => key_manager.ò = value,
        KeyCode::OCircumflexÔ => key_manager.o_umlaut_ö = value,
        KeyCode::UAcuteÚ => key_manager.ú = value,
        KeyCode::YAcuteÝ => key_manager.ý = value,
        KeyCode::ThornÞ => key_manager.þ = value,
        KeyCode::OELigatureŒ => key_manager.œ = value,
        KeyCode::AnyAlt => {
            key_manager.right_alt = value;
            key_manager.left_alt = value;
        }
        KeyCode::AnyShift => {
            key_manager.right_shift = value;
            key_manager.left_shift = value;
        }
        KeyCode::AnyControl => {
            key_manager.right_control = value;
            key_manager.left_control = value;
        }
        KeyCode::AnySuper => {
            key_manager.right_super = value;
            key_manager.left_super = value;
        }
        KeyCode::Pause => key_manager.pause = value,
        KeyCode::Apostrophe => key_manager.apostrophe = value,
        KeyCode::F25 => key_manager.f25 = value,
        KeyCode::KeyPadEqual => key_manager.key_pad_equal = value,
        KeyCode::World1 => key_manager.world_1 = value,
        KeyCode::World2 => key_manager.world_2 = value,
        KeyCode::Unknown => (),
    }
}

// #[cfg(target_os = "windows")]
// mod windows;
// #[cfg(target_os = "windows")]
// pub use windows::*;
