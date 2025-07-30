#[cfg(feature = "system")]
use crate::extensions::*;
use crate::platform::file_data::FileData;
/// Time trait, IDK
pub trait Time {
    /// Get time in seconds
    fn get_elapsed_time(&self) -> f64;
}
/// A cursor style, what else to say?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Create a new file system access-er, files that are not defined in `required_files` are not guaranteed to exist
    ///
    /// # Errors
    /// If the required files cannot be found, an error will return
    fn new(
        required_files: Vec<&'static str>,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
    /// # Get the contents of a file
    ///
    /// # Errors
    /// If the file is not found or otherwise accessible an error is returned
    fn get_file_contents(
        &self,
        path: &str,
    ) -> Result<FileData, Box<dyn std::error::Error>>;
    /// Write the desired data into the specified file in byte format
    ///
    /// # Errors
    /// If the file cannot be written to, it errors ¯\_(ツ)_/¯
    fn write_to_file(&self, path: &str, contents: &[u8])
        -> std::io::Result<()>;
    /// Get all file paths in the requested folder
    fn get_files_in_folder(&self, path: &str) -> Vec<String>;
    /// Get all sub folder paths in the requested folder
    fn get_folders_in_folder(&self, path: &str) -> Vec<String>;
    /// Join 2 paths together
    fn join(&self, path1: &str, path2: &str) -> String;
    /// Checks if a file exists
    fn does_file_exist(&self, path: &str) -> bool;
    /// Debug function to see what folders the implementation searched in
    fn get_searched_folders(&self)->Vec<String>;
}
/// Supported (and unsupported) mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, EnumIter, Eq)]
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
    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// Convert a keycode to a String -> Letters/Numbers return a String while function keys return None
    pub fn to_user_friendly_string(&self) -> Option<String> {
        match self {
            // Letters
            Self::A => Some("A".to_string()),
            Self::B => Some("B".to_string()),
            Self::C => Some("C".to_string()),
            Self::D => Some("D".to_string()),
            Self::E => Some("E".to_string()),
            Self::F => Some("F".to_string()),
            Self::G => Some("G".to_string()),
            Self::H => Some("H".to_string()),
            Self::I => Some("I".to_string()),
            Self::J => Some("J".to_string()),
            Self::K => Some("K".to_string()),
            Self::L => Some("L".to_string()),
            Self::M => Some("M".to_string()),
            Self::N => Some("N".to_string()),
            Self::O => Some("O".to_string()),
            Self::P => Some("P".to_string()),
            Self::Q => Some("Q".to_string()),
            Self::R => Some("R".to_string()),
            Self::S => Some("S".to_string()),
            Self::T => Some("T".to_string()),
            Self::U => Some("U".to_string()),
            Self::V => Some("V".to_string()),
            Self::W => Some("W".to_string()),
            Self::X => Some("X".to_string()),
            Self::Y => Some("Y".to_string()),
            Self::Z => Some("Z".to_string()),

            // Numbers
            Self::Num0 | Self::KeyPad0 => Some("0".to_string()),
            Self::Num1 | Self::KeyPad1 => Some("1".to_string()),
            Self::Num2 | Self::KeyPad2 => Some("2".to_string()),
            Self::Num3 | Self::KeyPad3 => Some("3".to_string()),
            Self::Num4 | Self::KeyPad4 => Some("4".to_string()),
            Self::Num5 | Self::KeyPad5 => Some("5".to_string()),
            Self::Num6 | Self::KeyPad6 => Some("6".to_string()),
            Self::Num7 | Self::KeyPad7 => Some("7".to_string()),
            Self::Num8 | Self::KeyPad8 => Some("8".to_string()),
            Self::Num9 | Self::KeyPad9 => Some("9".to_string()),
            // Function Keys
            // Multimedia keys - return None for action keys
            // Browser/OS keys - return None for action keys
            // Platform-specific - return None for action keys
            // Special
            Self::F1
            | Self::F2
            | Self::F3
            | Self::F4
            | Self::F5
            | Self::F6
            | Self::F7
            | Self::F8
            | Self::F9
            | Self::F10
            | Self::F11
            | Self::F12
            | Self::F13
            | Self::F14
            | Self::F15
            | Self::F16
            | Self::F17
            | Self::F18
            | Self::F19
            | Self::F20
            | Self::F21
            | Self::F22
            | Self::F23
            | Self::F24
            | Self::F25
            | Self::LeftShift
            | Self::RightShift
            | Self::LeftControl
            | Self::RightControl
            | Self::LeftAlt
            | Self::RightAlt
            | Self::LeftSuper
            | Self::RightSuper
            | Self::Escape
            | Self::Backspace
            | Self::Up
            | Self::Down
            | Self::Left
            | Self::Right
            | Self::Insert
            | Self::Delete
            | Self::Home
            | Self::End
            | Self::PageUp
            | Self::PageDown
            | Self::CapsLock
            | Self::NumLock
            | Self::ScrollLock
            | Self::MediaPlayPause
            | Self::MediaStop
            | Self::MediaNext
            | Self::MediaPrev
            | Self::VolumeUp
            | Self::VolumeDown
            | Self::Mute
            | Self::BrowserBack
            | Self::BrowserForward
            | Self::BrowserRefresh
            | Self::BrowserHome
            | Self::LaunchMail
            | Self::LaunchApp1
            | Self::LaunchApp2
            | Self::Menu
            | Self::PrintScreen
            | Self::Pause
            | Self::Application
            | Self::World1
            | Self::World2
            | Self::Unknown => None,
            // Modifiers - return None for action keys
            // Symbols / Punctuation
            Self::Space => Some(" ".to_string()),
            Self::Enter | Self::KeyPadEnter => Some("\n".to_string()),
            Self::Tab => Some("\t".to_string()),
            Self::Comma => Some(",".to_string()),
            Self::Period | Self::KeyPadDecimal => Some(".".to_string()),
            Self::Minus | Self::KeyPadSubtract => Some("-".to_string()),
            Self::Equal | Self::KeyPadEqual => Some("=".to_string()),
            Self::LeftBracket => Some("[".to_string()),
            Self::RightBracket => Some("]".to_string()),
            Self::Backslash => Some("\\".to_string()),
            Self::Semicolon => Some(";".to_string()),
            Self::Quote => Some("\"".to_string()),
            Self::Tilde => Some("~".to_string()),
            Self::Slash | Self::KeyPadDivide => Some("/".to_string()),
            Self::Grave => Some("`".to_string()),
            Self::Apostrophe => Some("'".to_string()),

            // Arrow keys - return None for action keys
            // Editing keys - return None for action keys
            // Lock keys - return None for action keys
            // Keypad operations - return just the symbol
            Self::KeyPadMultiply => Some("*".to_string()),
            Self::KeyPadAdd => Some("+".to_string()),
            // International & special characters
            Self::AUmlautÄ => Some("Ä".to_string()),
            Self::UUmlautÜ => Some("Ü".to_string()),
            Self::OUmlautÖ => Some("Ö".to_string()),
            Self::SS => Some("ß".to_string()),
            Self::ACircumflexÂ => Some("Â".to_string()),
            Self::UAcuteÚ => Some("Ú".to_string()),
            Self::OCircumflexÔ => Some("Ô".to_string()),
            Self::ICircumflexÎ => Some("Î".to_string()),
            Self::ECircumflexÊ => Some("Ê".to_string()),
            Self::EthÐ => Some("Ð".to_string()),
            Self::OELigatureŒ => Some("Œ".to_string()),
            Self::AAcuteÁ => Some("Á".to_string()),
            Self::YAcuteÝ => Some("Ý".to_string()),
            Self::IUmlautÏ => Some("Ï".to_string()),
            Self::NTildeÑ => Some("Ñ".to_string()),
            Self::OGraveÒ => Some("Ò".to_string()),
            Self::UGraveÙ => Some("Ù".to_string()),
            Self::ARingÅ => Some("Å".to_string()),
            Self::AELigatureÆ => Some("Æ".to_string()),
            Self::OSlashØ => Some("Ø".to_string()),
            Self::IGraveÌ => Some("Ì".to_string()),
            Self::ThornÞ => Some("Þ".to_string()),
        }
    }
}
impl std::fmt::Display for KeyCode {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            // Letters
            Self::A => "A".to_string(),
            Self::B => "B".to_string(),
            Self::C => "C".to_string(),
            Self::D => "D".to_string(),
            Self::E => "E".to_string(),
            Self::F => "F".to_string(),
            Self::G => "G".to_string(),
            Self::H => "H".to_string(),
            Self::I => "I".to_string(),
            Self::J => "J".to_string(),
            Self::K => "K".to_string(),
            Self::L => "L".to_string(),
            Self::M => "M".to_string(),
            Self::N => "N".to_string(),
            Self::O => "O".to_string(),
            Self::P => "P".to_string(),
            Self::Q => "Q".to_string(),
            Self::R => "R".to_string(),
            Self::S => "S".to_string(),
            Self::T => "T".to_string(),
            Self::U => "U".to_string(),
            Self::V => "V".to_string(),
            Self::W => "W".to_string(),
            Self::X => "X".to_string(),
            Self::Y => "Y".to_string(),
            Self::Z => "Z".to_string(),

            // Numbers
            Self::Num0 => "0".to_string(),
            Self::Num1 => "1".to_string(),
            Self::Num2 => "2".to_string(),
            Self::Num3 => "3".to_string(),
            Self::Num4 => "4".to_string(),
            Self::Num5 => "5".to_string(),
            Self::Num6 => "6".to_string(),
            Self::Num7 => "7".to_string(),
            Self::Num8 => "8".to_string(),
            Self::Num9 => "9".to_string(),
            Self::KeyPad0 => "KeyPad 0".to_string(),
            Self::KeyPad1 => "KeyPad 1".to_string(),
            Self::KeyPad2 => "KeyPad 2".to_string(),
            Self::KeyPad3 => "KeyPad 3".to_string(),
            Self::KeyPad4 => "KeyPad 4".to_string(),
            Self::KeyPad5 => "KeyPad 5".to_string(),
            Self::KeyPad6 => "KeyPad 6".to_string(),
            Self::KeyPad7 => "KeyPad 7".to_string(),
            Self::KeyPad8 => "KeyPad 8".to_string(),
            Self::KeyPad9 => "KeyPad 9".to_string(),

            // Function Keys
            Self::F1 => "F1".to_string(),
            Self::F2 => "F2".to_string(),
            Self::F3 => "F3".to_string(),
            Self::F4 => "F4".to_string(),
            Self::F5 => "F5".to_string(),
            Self::F6 => "F6".to_string(),
            Self::F7 => "F7".to_string(),
            Self::F8 => "F8".to_string(),
            Self::F9 => "F9".to_string(),
            Self::F10 => "F10".to_string(),
            Self::F11 => "F11".to_string(),
            Self::F12 => "F12".to_string(),
            Self::F13 => "F13".to_string(),
            Self::F14 => "F14".to_string(),
            Self::F15 => "F15".to_string(),
            Self::F16 => "F16".to_string(),
            Self::F17 => "F17".to_string(),
            Self::F18 => "F18".to_string(),
            Self::F19 => "F19".to_string(),
            Self::F20 => "F20".to_string(),
            Self::F21 => "F21".to_string(),
            Self::F22 => "F22".to_string(),
            Self::F23 => "F23".to_string(),
            Self::F24 => "F24".to_string(),

            // Modifiers
            Self::LeftShift => "Left Shift".to_string(),
            Self::RightShift => "Right Shift".to_string(),
            Self::LeftControl => "Left Control".to_string(),
            Self::RightControl => "Right Control".to_string(),
            Self::LeftAlt => "Left Alt".to_string(),
            Self::RightAlt => "Right Alt".to_string(),
            Self::LeftSuper => "Left Super".to_string(),
            Self::RightSuper => "Right Super".to_string(),

            // Symbols / Punctuation
            Self::Space => "Space".to_string(),
            Self::Enter => "Enter".to_string(),
            Self::Escape => "Escape".to_string(),
            Self::Backspace => "Backspace".to_string(),
            Self::Tab => "Tab".to_string(),
            Self::Comma => ",".to_string(),
            Self::Period => ".".to_string(),
            Self::Minus => "-".to_string(),
            Self::Equal => "=".to_string(),
            Self::LeftBracket => "[".to_string(),
            Self::RightBracket => "]".to_string(),
            Self::Backslash => "\\".to_string(),
            Self::Semicolon => ";".to_string(),
            Self::Quote => "\"".to_string(),
            Self::Tilde => "~".to_string(),
            Self::Slash => "/".to_string(),
            Self::Grave => "`".to_string(),
            Self::Apostrophe => "'".to_string(),

            // Arrow keys
            Self::Up => "Up".to_string(),
            Self::Down => "Down".to_string(),
            Self::Left => "Left".to_string(),
            Self::Right => "Right".to_string(),

            // Editing keys
            Self::Insert => "Insert".to_string(),
            Self::Delete => "Delete".to_string(),
            Self::Home => "Home".to_string(),
            Self::End => "End".to_string(),
            Self::PageUp => "Page Up".to_string(),
            Self::PageDown => "Page Down".to_string(),

            // Lock keys
            Self::CapsLock => "Caps Lock".to_string(),
            Self::NumLock => "Num Lock".to_string(),
            Self::ScrollLock => "Scroll Lock".to_string(),

            // Keypad operations
            Self::KeyPadDivide => "KeyPad /".to_string(),
            Self::KeyPadMultiply => "KeyPad *".to_string(),
            Self::KeyPadSubtract => "KeyPad -".to_string(),
            Self::KeyPadAdd => "KeyPad +".to_string(),
            Self::KeyPadDecimal => "KeyPad .".to_string(),
            Self::KeyPadEnter => "KeyPad Enter".to_string(),

            // International & special characters
            Self::AUmlautÄ => "Ä".to_string(),
            Self::UUmlautÜ => "Ü".to_string(),
            Self::OUmlautÖ => "Ö".to_string(),
            Self::SS => "ß".to_string(),
            Self::ACircumflexÂ => "Â".to_string(),
            Self::UAcuteÚ => "Ú".to_string(),
            Self::OCircumflexÔ => "Ô".to_string(),
            Self::ICircumflexÎ => "Î".to_string(),
            Self::ECircumflexÊ => "Ê".to_string(),
            Self::EthÐ => "Ð".to_string(),
            Self::OELigatureŒ => "Œ".to_string(),
            Self::AAcuteÁ => "Á".to_string(),
            Self::YAcuteÝ => "Ý".to_string(),
            Self::IUmlautÏ => "Ï".to_string(),
            Self::NTildeÑ => "Ñ".to_string(),
            Self::OGraveÒ => "Ò".to_string(),
            Self::UGraveÙ => "Ù".to_string(),
            Self::ARingÅ => "Å".to_string(),
            Self::AELigatureÆ => "Æ".to_string(),
            Self::OSlashØ => "Ø".to_string(),
            Self::IGraveÌ => "Ì".to_string(),
            Self::ThornÞ => "Þ".to_string(),

            // Multimedia keys
            Self::MediaPlayPause => "Media Play/Pause".to_string(),
            Self::MediaStop => "Media Stop".to_string(),
            Self::MediaNext => "Media Next".to_string(),
            Self::MediaPrev => "Media Previous".to_string(),
            Self::VolumeUp => "Volume Up".to_string(),
            Self::VolumeDown => "Volume Down".to_string(),
            Self::Mute => "Mute".to_string(),

            // Browser/OS keys
            Self::BrowserBack => "Browser Back".to_string(),
            Self::BrowserForward => "Browser Forward".to_string(),
            Self::BrowserRefresh => "Browser Refresh".to_string(),
            Self::BrowserHome => "Browser Home".to_string(),
            Self::LaunchMail => "Launch Mail".to_string(),
            Self::LaunchApp1 => "Launch App 1".to_string(),
            Self::LaunchApp2 => "Launch App 2".to_string(),

            // Platform-specific
            Self::Menu => "Menu".to_string(),
            Self::PrintScreen => "Print Screen".to_string(),
            Self::Pause => "Pause".to_string(),
            Self::Application => "Application".to_string(),

            // what
            Self::F25 => "F25".to_string(),
            Self::KeyPadEqual => "KeyPad =".to_string(),
            Self::World1 => "World 1".to_string(),
            Self::World2 => "World 2".to_string(),
            Self::Unknown => "Unknown".to_string(),
        };
        write!(f, "{value}")
    }
}

#[must_use]
/// Convert a list of keycodes into a String and return the ones that weren't convertible
pub fn keycodes_to_str(keycodes: &Vec<KeyCode>) -> (String, Vec<KeyCode>) {
    let mut functions = Vec::new();
    let mut output = String::new();
    for key_code in keycodes {
        let value = key_code.to_user_friendly_string();
        if let Some(key) = value {
            for l in key.chars() {
                output.push(l);
            }
        } else {
            functions.push(*key_code);
        }
    }
    (output, functions)
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
    /// Position on screen
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
#[derive(PartialEq, Copy, Clone, Debug, Eq)]
/// The render level of the window the os should use
/// Any window on the same render level will move in front of every other window on that same level if the user clicks them
pub enum WindowLevel {
    /// Render layer on the bottom -> Always under '[`WindowLevel::Normal`]'
    AlwaysOnBottom,
    /// Render layer sandwiched in the middle of '[`WindowLevel::AlwaysOnBottom`]' and '[`WindowLevel::AlwaysOnTop`]'
    Normal,
    /// Render layer on the top -> Always on top of '[`WindowLevel::Normal`]'
    AlwaysOnTop,
}

#[cfg(all(feature = "resvg", feature = "system"))]
pub use mouse::Cursor;

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
pub mod mouse;
#[cfg(feature = "system")]
/// Traits used by the backends
pub mod framework_traits;

/// Why bother reading files if you can't process them? Let [`file_data::FileData`] fix that.
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
#[derive(Debug)]
pub struct DoubleBuffer {
    front: Buffer,
    back: Buffer,
    front_is_back: std::sync::atomic::AtomicBool, // true if front buffer is the "back" buffer
}

impl DoubleBuffer {
    /// Creates a new instance of the double buffer starting out empty
    #[must_use]
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
    /// Recommended is using [`crate::math::UniformRange`] in conjunction with this struct
    ///
    /// # Panics
    /// If the dimensions are invalid
    pub const fn new<A: num_traits::ToPrimitive>(screen_size: (S, S)) -> Self {
        Self {
            screen_width: screen_size.0,
            screen_height: screen_size.1,
        }
    }
    /// Convert a percentage into screen a coordinate horizontally
    pub fn percentile_to_x<T: num_traits::Num + num_traits::NumCast>(
        &self,
        p: S,
    ) -> Option<T> {
        num_traits::NumCast::from(p * self.screen_width)
    }
    /// Convert a percentage into screen a coordinate vertically
    pub fn percentile_to_y<T: num_traits::Num + num_traits::NumCast>(
        &self,
        p: S,
    ) -> Option<T> {
        num_traits::NumCast::from(p * self.screen_height)
    }
    /// Convert a horizontal screen coordinate into a percentage
    pub fn x_to_percentile<T: num_traits::Num + num_traits::NumCast>(
        &self,
        x: T,
    ) -> Option<S> {
        if let Some(value) = S::from(x) {
            return Some(value / self.screen_width);
        }
        None
    }
    /// Convert a vertical screen coordinate into a percentage
    pub fn y_to_percentile<T: num_traits::Num + num_traits::NumCast>(
        &self,
        y: T,
    ) -> Option<S> {
        if let Some(value) = S::from(y) {
            return Some(value / self.screen_height);
        }
        None
    }
}
