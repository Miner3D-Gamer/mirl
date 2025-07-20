#[cfg(feature = "system")]
use crate::extensions::*;
use crate::platform::file_data::FileData;
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
    fn new(required_files: Vec<&'static str>) -> Self
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
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, EnumIter)]
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

impl KeyCode {
    /// Convert a keycode to a String -> Letters/Numbers return a String while function keys return None
    pub fn to_user_friendly_string(&self) -> Option<String> {
        match self {
            // Letters
            KeyCode::A => Some("A".to_string()),
            KeyCode::B => Some("B".to_string()),
            KeyCode::C => Some("C".to_string()),
            KeyCode::D => Some("D".to_string()),
            KeyCode::E => Some("E".to_string()),
            KeyCode::F => Some("F".to_string()),
            KeyCode::G => Some("G".to_string()),
            KeyCode::H => Some("H".to_string()),
            KeyCode::I => Some("I".to_string()),
            KeyCode::J => Some("J".to_string()),
            KeyCode::K => Some("K".to_string()),
            KeyCode::L => Some("L".to_string()),
            KeyCode::M => Some("M".to_string()),
            KeyCode::N => Some("N".to_string()),
            KeyCode::O => Some("O".to_string()),
            KeyCode::P => Some("P".to_string()),
            KeyCode::Q => Some("Q".to_string()),
            KeyCode::R => Some("R".to_string()),
            KeyCode::S => Some("S".to_string()),
            KeyCode::T => Some("T".to_string()),
            KeyCode::U => Some("U".to_string()),
            KeyCode::V => Some("V".to_string()),
            KeyCode::W => Some("W".to_string()),
            KeyCode::X => Some("X".to_string()),
            KeyCode::Y => Some("Y".to_string()),
            KeyCode::Z => Some("Z".to_string()),

            // Numbers
            KeyCode::Num0 => Some("0".to_string()),
            KeyCode::Num1 => Some("1".to_string()),
            KeyCode::Num2 => Some("2".to_string()),
            KeyCode::Num3 => Some("3".to_string()),
            KeyCode::Num4 => Some("4".to_string()),
            KeyCode::Num5 => Some("5".to_string()),
            KeyCode::Num6 => Some("6".to_string()),
            KeyCode::Num7 => Some("7".to_string()),
            KeyCode::Num8 => Some("8".to_string()),
            KeyCode::Num9 => Some("9".to_string()),
            KeyCode::KeyPad0 => Some("0".to_string()),
            KeyCode::KeyPad1 => Some("1".to_string()),
            KeyCode::KeyPad2 => Some("2".to_string()),
            KeyCode::KeyPad3 => Some("3".to_string()),
            KeyCode::KeyPad4 => Some("4".to_string()),
            KeyCode::KeyPad5 => Some("5".to_string()),
            KeyCode::KeyPad6 => Some("6".to_string()),
            KeyCode::KeyPad7 => Some("7".to_string()),
            KeyCode::KeyPad8 => Some("8".to_string()),
            KeyCode::KeyPad9 => Some("9".to_string()),

            // Function Keys
            KeyCode::F1 => None,
            KeyCode::F2 => None,
            KeyCode::F3 => None,
            KeyCode::F4 => None,
            KeyCode::F5 => None,
            KeyCode::F6 => None,
            KeyCode::F7 => None,
            KeyCode::F8 => None,
            KeyCode::F9 => None,
            KeyCode::F10 => None,
            KeyCode::F11 => None,
            KeyCode::F12 => None,
            KeyCode::F13 => None,
            KeyCode::F14 => None,
            KeyCode::F15 => None,
            KeyCode::F16 => None,
            KeyCode::F17 => None,
            KeyCode::F18 => None,
            KeyCode::F19 => None,
            KeyCode::F20 => None,
            KeyCode::F21 => None,
            KeyCode::F22 => None,
            KeyCode::F23 => None,
            KeyCode::F24 => None,
            KeyCode::F25 => None,

            // Modifiers - return None for action keys
            KeyCode::LeftShift => None,
            KeyCode::RightShift => None,
            KeyCode::LeftControl => None,
            KeyCode::RightControl => None,
            KeyCode::LeftAlt => None,
            KeyCode::RightAlt => None,
            KeyCode::LeftSuper => None,
            KeyCode::RightSuper => None,

            // Symbols / Punctuation
            KeyCode::Space => Some(" ".to_string()),
            KeyCode::Enter => Some("\n".to_string()),
            KeyCode::Escape => None,
            KeyCode::Backspace => None,
            KeyCode::Tab => Some("\t".to_string()),
            KeyCode::Comma => Some(",".to_string()),
            KeyCode::Period => Some(".".to_string()),
            KeyCode::Minus => Some("-".to_string()),
            KeyCode::Equal => Some("=".to_string()),
            KeyCode::LeftBracket => Some("[".to_string()),
            KeyCode::RightBracket => Some("]".to_string()),
            KeyCode::Backslash => Some("\\".to_string()),
            KeyCode::Semicolon => Some(";".to_string()),
            KeyCode::Quote => Some("'".to_string()),
            KeyCode::Tilde => Some("~".to_string()),
            KeyCode::Slash => Some("/".to_string()),
            KeyCode::Grave => Some("`".to_string()),
            KeyCode::Apostrophe => Some("'".to_string()),

            // Arrow keys - return None for action keys
            KeyCode::Up => None,
            KeyCode::Down => None,
            KeyCode::Left => None,
            KeyCode::Right => None,

            // Editing keys - return None for action keys
            KeyCode::Insert => None,
            KeyCode::Delete => None,
            KeyCode::Home => None,
            KeyCode::End => None,
            KeyCode::PageUp => None,
            KeyCode::PageDown => None,

            // Lock keys - return None for action keys
            KeyCode::CapsLock => None,
            KeyCode::NumLock => None,
            KeyCode::ScrollLock => None,

            // Keypad operations - return just the symbol
            KeyCode::KeyPadDivide => Some("/".to_string()),
            KeyCode::KeyPadMultiply => Some("*".to_string()),
            KeyCode::KeyPadSubtract => Some("-".to_string()),
            KeyCode::KeyPadAdd => Some("+".to_string()),
            KeyCode::KeyPadDecimal => Some(".".to_string()),
            KeyCode::KeyPadEnter => Some("\n".to_string()),
            KeyCode::KeyPadEqual => Some("=".to_string()),

            // International & special characters
            KeyCode::AUmlautÄ => Some("Ä".to_string()),
            KeyCode::UUmlautÜ => Some("Ü".to_string()),
            KeyCode::OUmlautÖ => Some("Ö".to_string()),
            KeyCode::SS => Some("ß".to_string()),
            KeyCode::ACircumflexÂ => Some("Â".to_string()),
            KeyCode::UAcuteÚ => Some("Ú".to_string()),
            KeyCode::OCircumflexÔ => Some("Ô".to_string()),
            KeyCode::ICircumflexÎ => Some("Î".to_string()),
            KeyCode::ECircumflexÊ => Some("Ê".to_string()),
            KeyCode::EthÐ => Some("Ð".to_string()),
            KeyCode::OELigatureŒ => Some("Œ".to_string()),
            KeyCode::AAcuteÁ => Some("Á".to_string()),
            KeyCode::YAcuteÝ => Some("Ý".to_string()),
            KeyCode::IUmlautÏ => Some("Ï".to_string()),
            KeyCode::NTildeÑ => Some("Ñ".to_string()),
            KeyCode::OGraveÒ => Some("Ò".to_string()),
            KeyCode::UGraveÙ => Some("Ù".to_string()),
            KeyCode::ARingÅ => Some("Å".to_string()),
            KeyCode::AELigatureÆ => Some("Æ".to_string()),
            KeyCode::OSlashØ => Some("Ø".to_string()),
            KeyCode::IGraveÌ => Some("Ì".to_string()),
            KeyCode::ThornÞ => Some("Þ".to_string()),

            // Multimedia keys - return None for action keys
            KeyCode::MediaPlayPause => None,
            KeyCode::MediaStop => None,
            KeyCode::MediaNext => None,
            KeyCode::MediaPrev => None,
            KeyCode::VolumeUp => None,
            KeyCode::VolumeDown => None,
            KeyCode::Mute => None,

            // Browser/OS keys - return None for action keys
            KeyCode::BrowserBack => None,
            KeyCode::BrowserForward => None,
            KeyCode::BrowserRefresh => None,
            KeyCode::BrowserHome => None,
            KeyCode::LaunchMail => None,
            KeyCode::LaunchApp1 => None,
            KeyCode::LaunchApp2 => None,

            // Platform-specific - return None for action keys
            KeyCode::Menu => None,
            KeyCode::PrintScreen => None,
            KeyCode::Pause => None,
            KeyCode::Application => None,

            // Special
            KeyCode::World1 => None,
            KeyCode::World2 => None,
            KeyCode::Unknown => None,
        }
    }
    /// Convert the keycode into a string
    pub fn to_string(&self) -> String {
        match self {
            // Letters
            KeyCode::A => "A".to_string(),
            KeyCode::B => "B".to_string(),
            KeyCode::C => "C".to_string(),
            KeyCode::D => "D".to_string(),
            KeyCode::E => "E".to_string(),
            KeyCode::F => "F".to_string(),
            KeyCode::G => "G".to_string(),
            KeyCode::H => "H".to_string(),
            KeyCode::I => "I".to_string(),
            KeyCode::J => "J".to_string(),
            KeyCode::K => "K".to_string(),
            KeyCode::L => "L".to_string(),
            KeyCode::M => "M".to_string(),
            KeyCode::N => "N".to_string(),
            KeyCode::O => "O".to_string(),
            KeyCode::P => "P".to_string(),
            KeyCode::Q => "Q".to_string(),
            KeyCode::R => "R".to_string(),
            KeyCode::S => "S".to_string(),
            KeyCode::T => "T".to_string(),
            KeyCode::U => "U".to_string(),
            KeyCode::V => "V".to_string(),
            KeyCode::W => "W".to_string(),
            KeyCode::X => "X".to_string(),
            KeyCode::Y => "Y".to_string(),
            KeyCode::Z => "Z".to_string(),

            // Numbers
            KeyCode::Num0 => "0".to_string(),
            KeyCode::Num1 => "1".to_string(),
            KeyCode::Num2 => "2".to_string(),
            KeyCode::Num3 => "3".to_string(),
            KeyCode::Num4 => "4".to_string(),
            KeyCode::Num5 => "5".to_string(),
            KeyCode::Num6 => "6".to_string(),
            KeyCode::Num7 => "7".to_string(),
            KeyCode::Num8 => "8".to_string(),
            KeyCode::Num9 => "9".to_string(),
            KeyCode::KeyPad0 => "KeyPad 0".to_string(),
            KeyCode::KeyPad1 => "KeyPad 1".to_string(),
            KeyCode::KeyPad2 => "KeyPad 2".to_string(),
            KeyCode::KeyPad3 => "KeyPad 3".to_string(),
            KeyCode::KeyPad4 => "KeyPad 4".to_string(),
            KeyCode::KeyPad5 => "KeyPad 5".to_string(),
            KeyCode::KeyPad6 => "KeyPad 6".to_string(),
            KeyCode::KeyPad7 => "KeyPad 7".to_string(),
            KeyCode::KeyPad8 => "KeyPad 8".to_string(),
            KeyCode::KeyPad9 => "KeyPad 9".to_string(),

            // Function Keys
            KeyCode::F1 => "F1".to_string(),
            KeyCode::F2 => "F2".to_string(),
            KeyCode::F3 => "F3".to_string(),
            KeyCode::F4 => "F4".to_string(),
            KeyCode::F5 => "F5".to_string(),
            KeyCode::F6 => "F6".to_string(),
            KeyCode::F7 => "F7".to_string(),
            KeyCode::F8 => "F8".to_string(),
            KeyCode::F9 => "F9".to_string(),
            KeyCode::F10 => "F10".to_string(),
            KeyCode::F11 => "F11".to_string(),
            KeyCode::F12 => "F12".to_string(),
            KeyCode::F13 => "F13".to_string(),
            KeyCode::F14 => "F14".to_string(),
            KeyCode::F15 => "F15".to_string(),
            KeyCode::F16 => "F16".to_string(),
            KeyCode::F17 => "F17".to_string(),
            KeyCode::F18 => "F18".to_string(),
            KeyCode::F19 => "F19".to_string(),
            KeyCode::F20 => "F20".to_string(),
            KeyCode::F21 => "F21".to_string(),
            KeyCode::F22 => "F22".to_string(),
            KeyCode::F23 => "F23".to_string(),
            KeyCode::F24 => "F24".to_string(),

            // Modifiers
            KeyCode::LeftShift => "Left Shift".to_string(),
            KeyCode::RightShift => "Right Shift".to_string(),
            KeyCode::LeftControl => "Left Control".to_string(),
            KeyCode::RightControl => "Right Control".to_string(),
            KeyCode::LeftAlt => "Left Alt".to_string(),
            KeyCode::RightAlt => "Right Alt".to_string(),
            KeyCode::LeftSuper => "Left Super".to_string(),
            KeyCode::RightSuper => "Right Super".to_string(),

            // Symbols / Punctuation
            KeyCode::Space => "Space".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Escape => "Escape".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::Comma => ",".to_string(),
            KeyCode::Period => ".".to_string(),
            KeyCode::Minus => "-".to_string(),
            KeyCode::Equal => "=".to_string(),
            KeyCode::LeftBracket => "[".to_string(),
            KeyCode::RightBracket => "]".to_string(),
            KeyCode::Backslash => "\\".to_string(),
            KeyCode::Semicolon => ";".to_string(),
            KeyCode::Quote => "'".to_string(),
            KeyCode::Tilde => "~".to_string(),
            KeyCode::Slash => "/".to_string(),
            KeyCode::Grave => "`".to_string(),
            KeyCode::Apostrophe => "'".to_string(),

            // Arrow keys
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),

            // Editing keys
            KeyCode::Insert => "Insert".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "Page Up".to_string(),
            KeyCode::PageDown => "Page Down".to_string(),

            // Lock keys
            KeyCode::CapsLock => "Caps Lock".to_string(),
            KeyCode::NumLock => "Num Lock".to_string(),
            KeyCode::ScrollLock => "Scroll Lock".to_string(),

            // Keypad operations
            KeyCode::KeyPadDivide => "KeyPad /".to_string(),
            KeyCode::KeyPadMultiply => "KeyPad *".to_string(),
            KeyCode::KeyPadSubtract => "KeyPad -".to_string(),
            KeyCode::KeyPadAdd => "KeyPad +".to_string(),
            KeyCode::KeyPadDecimal => "KeyPad .".to_string(),
            KeyCode::KeyPadEnter => "KeyPad Enter".to_string(),

            // International & special characters
            KeyCode::AUmlautÄ => "Ä".to_string(),
            KeyCode::UUmlautÜ => "Ü".to_string(),
            KeyCode::OUmlautÖ => "Ö".to_string(),
            KeyCode::SS => "ß".to_string(),
            KeyCode::ACircumflexÂ => "Â".to_string(),
            KeyCode::UAcuteÚ => "Ú".to_string(),
            KeyCode::OCircumflexÔ => "Ô".to_string(),
            KeyCode::ICircumflexÎ => "Î".to_string(),
            KeyCode::ECircumflexÊ => "Ê".to_string(),
            KeyCode::EthÐ => "Ð".to_string(),
            KeyCode::OELigatureŒ => "Œ".to_string(),
            KeyCode::AAcuteÁ => "Á".to_string(),
            KeyCode::YAcuteÝ => "Ý".to_string(),
            KeyCode::IUmlautÏ => "Ï".to_string(),
            KeyCode::NTildeÑ => "Ñ".to_string(),
            KeyCode::OGraveÒ => "Ò".to_string(),
            KeyCode::UGraveÙ => "Ù".to_string(),
            KeyCode::ARingÅ => "Å".to_string(),
            KeyCode::AELigatureÆ => "Æ".to_string(),
            KeyCode::OSlashØ => "Ø".to_string(),
            KeyCode::IGraveÌ => "Ì".to_string(),
            KeyCode::ThornÞ => "Þ".to_string(),

            // Multimedia keys
            KeyCode::MediaPlayPause => "Media Play/Pause".to_string(),
            KeyCode::MediaStop => "Media Stop".to_string(),
            KeyCode::MediaNext => "Media Next".to_string(),
            KeyCode::MediaPrev => "Media Previous".to_string(),
            KeyCode::VolumeUp => "Volume Up".to_string(),
            KeyCode::VolumeDown => "Volume Down".to_string(),
            KeyCode::Mute => "Mute".to_string(),

            // Browser/OS keys
            KeyCode::BrowserBack => "Browser Back".to_string(),
            KeyCode::BrowserForward => "Browser Forward".to_string(),
            KeyCode::BrowserRefresh => "Browser Refresh".to_string(),
            KeyCode::BrowserHome => "Browser Home".to_string(),
            KeyCode::LaunchMail => "Launch Mail".to_string(),
            KeyCode::LaunchApp1 => "Launch App 1".to_string(),
            KeyCode::LaunchApp2 => "Launch App 2".to_string(),

            // Platform-specific
            KeyCode::Menu => "Menu".to_string(),
            KeyCode::PrintScreen => "Print Screen".to_string(),
            KeyCode::Pause => "Pause".to_string(),
            KeyCode::Application => "Application".to_string(),

            // what
            KeyCode::F25 => "F25".to_string(),
            KeyCode::KeyPadEqual => "KeyPad =".to_string(),
            KeyCode::World1 => "World 1".to_string(),
            KeyCode::World2 => "World 2".to_string(),
            KeyCode::Unknown => "Unknown".to_string(),
        }
    }
}

/// Convert a list of keycodes into a String and return the ones that weren't convertible
pub fn keycodes_to_str(keycodes: &Vec<KeyCode>) -> (String, Vec<KeyCode>) {
    let mut functions = Vec::new();
    let mut output = String::new();
    for key_code in keycodes {
        let value = key_code.to_user_friendly_string();
        if let Some(key) = value {
            for l in key.chars() {
                output.push(l)
            }
        } else {
            functions.push(*key_code)
        }
    }
    return (output, functions);
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

#[cfg(all(feature = "resvg", feature = "system"))]
pub use cursors::Cursor;

mod buffer;
pub use buffer::Buffer;

// Windows
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "minifb_backend")]
/// The minifb version of the backend
pub mod minifb;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "glfw_backend")]
/// The glfw version of the backend
pub mod glfw;

#[cfg(all(feature = "resvg", feature = "system"))]
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
//     fn is_point_visible(&self, xy: (isize,isize)) -> bool {
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
            front: Buffer::new_empty(width, height),
            back: Buffer::new_empty(width, height),
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
