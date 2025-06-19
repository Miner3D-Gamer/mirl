#![allow(static_mut_refs)]
#![allow(non_snake_case)]

use std::ptr;
use std::sync::Mutex;
use winapi::ctypes::c_int;
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
    HC_ACTION, KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    WM_SYSKEYDOWN, WM_SYSKEYUP,
};

use crate::platform::shared::KeyManager;
use crate::platform::KeyCode;

static mut HOOK: HHOOK = ptr::null_mut();

// Key event structure
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key_code: KeyCode,
    pub raw_vk_code: u32,
    pub is_key_down: bool,
    pub is_system_key: bool,
    pub timestamp: std::time::SystemTime,
}

static mut KEYBOARD_MANAGER_WINDOWS: Mutex<KeyManager> =
    Mutex::new(KeyManager::new());

// Low-level keyboard hook procedure
unsafe extern "system" fn low_level_keyboard_proc(
    n_code: c_int,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code >= HC_ACTION {
        let kb_struct = *(l_param as *const KBDLLHOOKSTRUCT);

        let is_key_down = match w_param as u32 {
            WM_KEYDOWN | WM_SYSKEYDOWN => true,
            WM_KEYUP | WM_SYSKEYUP => false,
            _ => return CallNextHookEx(HOOK, n_code, w_param, l_param),
        };

        if let Ok(manager) = KEYBOARD_MANAGER_WINDOWS.get_mut() {
            manager.set_key_state(
                vk_code_to_keycode(kb_struct.vkCode),
                is_key_down,
            );
        }
    }

    CallNextHookEx(HOOK, n_code, w_param, l_param)
}

// Convert Windows VK codes to KeyCode enum
const fn vk_code_to_keycode(vk_code: u32) -> KeyCode {
    match vk_code {
        // Letters A-Z (0x41-0x5A)
        0x41 => KeyCode::A,
        0x42 => KeyCode::B,
        0x43 => KeyCode::C,
        0x44 => KeyCode::D,
        0x45 => KeyCode::E,
        0x46 => KeyCode::F,
        0x47 => KeyCode::G,
        0x48 => KeyCode::H,
        0x49 => KeyCode::I,
        0x4A => KeyCode::J,
        0x4B => KeyCode::K,
        0x4C => KeyCode::L,
        0x4D => KeyCode::M,
        0x4E => KeyCode::N,
        0x4F => KeyCode::O,
        0x50 => KeyCode::P,
        0x51 => KeyCode::Q,
        0x52 => KeyCode::R,
        0x53 => KeyCode::S,
        0x54 => KeyCode::T,
        0x55 => KeyCode::U,
        0x56 => KeyCode::V,
        0x57 => KeyCode::W,
        0x58 => KeyCode::X,
        0x59 => KeyCode::Y,
        0x5A => KeyCode::Z,

        // Numbers 0-9 (0x30-0x39)
        0x30 => KeyCode::Num0,
        0x31 => KeyCode::Num1,
        0x32 => KeyCode::Num2,
        0x33 => KeyCode::Num3,
        0x34 => KeyCode::Num4,
        0x35 => KeyCode::Num5,
        0x36 => KeyCode::Num6,
        0x37 => KeyCode::Num7,
        0x38 => KeyCode::Num8,
        0x39 => KeyCode::Num9,

        // Keypad numbers (0x60-0x69)
        0x60 => KeyCode::KeyPad0,
        0x61 => KeyCode::KeyPad1,
        0x62 => KeyCode::KeyPad2,
        0x63 => KeyCode::KeyPad3,
        0x64 => KeyCode::KeyPad4,
        0x65 => KeyCode::KeyPad5,
        0x66 => KeyCode::KeyPad6,
        0x67 => KeyCode::KeyPad7,
        0x68 => KeyCode::KeyPad8,
        0x69 => KeyCode::KeyPad9,

        // Function keys F1-F24 (0x70-0x87)
        0x70 => KeyCode::F1,
        0x71 => KeyCode::F2,
        0x72 => KeyCode::F3,
        0x73 => KeyCode::F4,
        0x74 => KeyCode::F5,
        0x75 => KeyCode::F6,
        0x76 => KeyCode::F7,
        0x77 => KeyCode::F8,
        0x78 => KeyCode::F9,
        0x79 => KeyCode::F10,
        0x7A => KeyCode::F11,
        0x7B => KeyCode::F12,
        0x7C => KeyCode::F13,
        0x7D => KeyCode::F14,
        0x7E => KeyCode::F15,
        0x7F => KeyCode::F16,
        0x80 => KeyCode::F17,
        0x81 => KeyCode::F18,
        0x82 => KeyCode::F19,
        0x83 => KeyCode::F20,
        0x84 => KeyCode::F21,
        0x85 => KeyCode::F22,
        0x86 => KeyCode::F23,
        0x87 => KeyCode::F24,

        // Modifiers
        0xA0 => KeyCode::LeftShift,
        0xA1 => KeyCode::RightShift,
        0xA2 => KeyCode::LeftControl,
        0xA3 => KeyCode::RightControl,
        0xA4 => KeyCode::LeftAlt,
        0xA5 => KeyCode::RightAlt,
        0x5B => KeyCode::LeftSuper,  // VK_LWIN
        0x5C => KeyCode::RightSuper, // VK_RWIN

        // Special keys
        0x20 => KeyCode::Space,
        0x0D => KeyCode::Enter,
        0x1B => KeyCode::Escape,
        0x08 => KeyCode::Backspace,
        0x09 => KeyCode::Tab,

        // Punctuation
        0xBC => KeyCode::Comma,        // VK_OEM_COMMA
        0xBE => KeyCode::Period,       // VK_OEM_PERIOD
        0xBD => KeyCode::Minus,        // VK_OEM_MINUS
        0xBB => KeyCode::Equal,        // VK_OEM_PLUS
        0xDB => KeyCode::LeftBracket,  // VK_OEM_4
        0xDD => KeyCode::RightBracket, // VK_OEM_6
        0xDC => KeyCode::Backslash,    // VK_OEM_5
        0xBA => KeyCode::Semicolon,    // VK_OEM_1
        0xDE => KeyCode::Quote,        // VK_OEM_7
        0xC0 => KeyCode::Grave,        // VK_OEM_3
        0xBF => KeyCode::Slash,        // VK_OEM_2

        // Arrow keys
        0x26 => KeyCode::Up,
        0x28 => KeyCode::Down,
        0x25 => KeyCode::Left,
        0x27 => KeyCode::Right,

        // Editing keys
        0x2D => KeyCode::Insert,
        0x2E => KeyCode::Delete,
        0x24 => KeyCode::Home,
        0x23 => KeyCode::End,
        0x21 => KeyCode::PageUp,
        0x22 => KeyCode::PageDown,

        // Lock keys
        0x14 => KeyCode::CapsLock,
        0x90 => KeyCode::NumLock,
        0x91 => KeyCode::ScrollLock,

        // Keypad operations
        0x6F => KeyCode::KeyPadDivide, // VK_DIVIDE
        0x6A => KeyCode::KeyPadMultiply, // VK_MULTIPLY
        0x6D => KeyCode::KeyPadSubtract, // VK_SUBTRACT
        0x6B => KeyCode::KeyPadAdd,    // VK_ADD
        0x6E => KeyCode::KeyPadDecimal, // VK_DECIMAL

        // Media keys
        0xB3 => KeyCode::MediaPlayPause, // VK_MEDIA_PLAY_PAUSE
        0xB2 => KeyCode::MediaStop,      // VK_MEDIA_STOP
        0xB0 => KeyCode::MediaNext,      // VK_MEDIA_NEXT_TRACK
        0xB1 => KeyCode::MediaPrev,      // VK_MEDIA_PREV_TRACK
        0xAF => KeyCode::VolumeUp,       // VK_VOLUME_UP
        0xAE => KeyCode::VolumeDown,     // VK_VOLUME_DOWN
        0xAD => KeyCode::Mute,           // VK_VOLUME_MUTE

        // Browser keys
        0xA6 => KeyCode::BrowserBack, // VK_BROWSER_BACK
        0xA7 => KeyCode::BrowserForward, // VK_BROWSER_FORWARD
        0xA8 => KeyCode::BrowserRefresh, // VK_BROWSER_REFRESH
        0xAC => KeyCode::BrowserHome, // VK_BROWSER_HOME

        // Application launch keys
        0xB4 => KeyCode::LaunchMail, // VK_LAUNCH_MAIL
        0xB6 => KeyCode::LaunchApp1, // VK_LAUNCH_APP1
        0xB7 => KeyCode::LaunchApp2, // VK_LAUNCH_APP2

        // Other keys
        0x5D => KeyCode::Menu,        // VK_APPS
        0x2C => KeyCode::PrintScreen, // VK_SNAPSHOT
        0x13 => KeyCode::Pause,       // VK_PAUSE

        _ => KeyCode::Unknown,
    }
}

pub struct KeyLogger {
    hook: HHOOK,
}

impl KeyLogger {
    pub fn new() -> Self {
        unsafe {
            let h_instance = GetModuleHandleW(ptr::null());
            if h_instance.is_null() {
                panic!("Failed to get module handle");
            }

            let hook = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(low_level_keyboard_proc),
                h_instance,
                0,
            );

            if hook.is_null() {
                panic!("Failed to set keyboard hook");
            }

            HOOK = hook;

            KeyLogger {
                hook,
            }
        }
    }

    pub fn start_listening(&self) -> Result<(), String> {
        std::thread::spawn(|| {
            unsafe {
                let mut msg: MSG = std::mem::zeroed();

                // Message loop to keep the hook active
                loop {
                    let result = GetMessageW(&mut msg, ptr::null_mut(), 0, 0);
                    if result == 0 || result == -1 {
                        break;
                    }
                }
            }
        });

        Ok(())
    }
    pub fn get_manager(&self) -> KeyManager {
        let temp;
        unsafe {
            temp = *KEYBOARD_MANAGER_WINDOWS.lock().unwrap();
        }
        return temp.clone();
    }
}

impl Drop for KeyLogger {
    fn drop(&mut self) {
        unsafe {
            if !self.hook.is_null() {
                UnhookWindowsHookEx(self.hook);
                HOOK = ptr::null_mut();
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_vk_code_conversion() {
//         assert_eq!(vk_code_to_keycode(0x0D), KeyCode::Enter);
//         assert_eq!(vk_code_to_keycode(0x20), KeyCode::Space);
//         assert_eq!(vk_code_to_keycode(0x1B), KeyCode::Escape);
//         assert_eq!(vk_code_to_keycode(0x41), KeyCode::A);
//         assert_eq!(vk_code_to_keycode(0x5A), KeyCode::Z);
//         assert_eq!(vk_code_to_keycode(0x30), KeyCode::Num0);
//         assert_eq!(vk_code_to_keycode(0x70), KeyCode::F1);
//         assert_eq!(vk_code_to_keycode(0x999), KeyCode::Unknown);
//     }
// }
