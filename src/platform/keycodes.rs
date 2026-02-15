#[cfg(all(feature = "system", feature = "keycodes"))]
use strum_macros::EnumIter;
#[cfg_attr(all(feature = "system", feature = "keycodes"), derive(EnumIter))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash, PartialOrd, Ord)]
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
    LeftHyper,
    RightHyper,
    AltControl,

    // Symbols / Punctuation
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    BackTab,
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
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,

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
    // TODO: when checking for MediaPlayPause, also check MediaPlay and MediaPause
    MediaPlayPause,
    MediaPlay,
    MediaPause,
    MediaStop,
    MediaNext,
    MediaPrev,
    VolumeUp,
    VolumeDown,
    Mute,
    MediaReverse,
    MediaFastForward,
    MediaRecord,

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

    // Convenient
    AnyShift,
    AnyAlt,
    AnyControl,
    AnySuper,

    // what
    F25,
    KeyPadEqual,
    World1,
    World2,
    SpecialControl,
    #[default]
    Unknown,
}

#[cfg(feature = "std")]
macro_rules! define_keys {
    ( $( $ident:ident => $name:expr ),* $(,)? ) => {
         impl KeyCode {
            /// Converts the requested key to a string representation of itself
            #[must_use]
            pub const fn to_string(&self) -> &'static str {
                match self {
                    $( Self::$ident => $name ),*
                }
            }
            /// Converts the requested key to a string representation of itself
            #[must_use]
            pub fn str_to_keycode(string: &str) -> Self {
                match string {
                    $( $name => Self::$ident ),*,
                    _=>Self::Unknown
                }
            }
            #[must_use]
            /// Converts self to `[Vec<Self>]`
            pub fn to_vec(&self) -> Vec<Self> {
                Vec::from([*self])
            }
        }
        impl StringToKeyCodes for String {
            fn to_keycodes(&self) -> Vec<KeyCode> {
                let mut list = Vec::new();
                for i in self.chars() {
                    list.push(i.to_keycode())
                }
                list
            }
            fn to_keycode(&self)->KeyCode{
                KeyCode::str_to_keycode(self)


            }
        }
        impl StringToKeyCodes for char {
            fn to_keycodes(&self) -> Vec<KeyCode> {
                Vec::from([self.to_keycode()])
            }
            fn to_keycode(&self)->KeyCode{
                KeyCode::str_to_keycode(&self.to_string())
            }
        }

        /// All available keys in string form in a list
        pub const AVAILABLE_KEY_NAMES: &[&str] = &[
            $( $name ),*
        ];

        /// All available keys in [`KeyCode`] form a list
        pub const AVAILABLE_KEYS: &[KeyCode] =&[
            $( KeyCode::$ident ),*];

        /// Get all available keys string form in a list
        #[must_use] pub const fn get_available_key_names() -> &'static [&'static str] {
            AVAILABLE_KEY_NAMES
        }
        /// Get all available keys in [`KeyCode`] form in a list
        #[must_use] pub const fn get_available_keys() -> &'static [KeyCode] {
            AVAILABLE_KEYS
        }
    };
}

#[cfg(feature = "std")]
/// Convert from a string to a `Vec<KeyCode>`
pub const trait StringToKeyCodes {
    /// Convert from a string to a `Vec<KeyCode>`
    fn to_keycodes(&self) -> Vec<KeyCode>;
    /// Converts a single text instance into the corresponding `KeyCode`
    fn to_keycode(&self) -> KeyCode;
}

impl KeyCode {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    /// Convert a keycode to a String -> Letters/Numbers return a String while function keys return None
    pub const fn to_user_friendly_string(&self) -> Option<&'static str> {
        match self {
            Self::A => Some("A"),
            Self::B => Some("B"),
            Self::C => Some("C"),
            Self::D => Some("D"),
            Self::E => Some("E"),
            Self::F => Some("F"),
            Self::G => Some("G"),
            Self::H => Some("H"),
            Self::I => Some("I"),
            Self::J => Some("J"),
            Self::K => Some("K"),
            Self::L => Some("L"),
            Self::M => Some("M"),
            Self::N => Some("N"),
            Self::O => Some("O"),
            Self::P => Some("P"),
            Self::Q => Some("Q"),
            Self::R => Some("R"),
            Self::S => Some("S"),
            Self::T => Some("T"),
            Self::U => Some("U"),
            Self::V => Some("V"),
            Self::W => Some("W"),
            Self::X => Some("X"),
            Self::Y => Some("Y"),
            Self::Z => Some("Z"),
            Self::Num0 | Self::KeyPad0 => Some("0"),
            Self::Num1 | Self::KeyPad1 => Some("1"),
            Self::Num2 | Self::KeyPad2 => Some("2"),
            Self::Num3 | Self::KeyPad3 => Some("3"),
            Self::Num4 | Self::KeyPad4 => Some("4"),
            Self::Num5 | Self::KeyPad5 => Some("5"),
            Self::Num6 | Self::KeyPad6 => Some("6"),
            Self::Num7 | Self::KeyPad7 => Some("7"),
            Self::Num8 | Self::KeyPad8 => Some("8"),
            Self::Num9 | Self::KeyPad9 => Some("9"),
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
            | Self::UpArrow
            | Self::DownArrow
            | Self::LeftArrow
            | Self::RightArrow
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
            | Self::AnyAlt
            | Self::AnySuper
            | Self::AnyControl
            | Self::AnyShift
            | Self::Unknown
            | Self::Enter
            | Self::KeyPadEnter
            | Self::BackTab
            | Self::Tab
            | Self::LeftHyper
            | Self::RightHyper
            | Self::AltControl
            | Self::MediaPlay
            | Self::MediaPause
            | Self::MediaReverse
            | Self::MediaFastForward
            | Self::MediaRecord
            | Self::SpecialControl => None,
            Self::Space => Some(" "),
            Self::Comma => Some(","),
            Self::Period | Self::KeyPadDecimal => Some("."),
            Self::Minus | Self::KeyPadSubtract => Some("-"),
            Self::Equal | Self::KeyPadEqual => Some("="),
            Self::LeftBracket => Some("["),
            Self::RightBracket => Some("]"),
            Self::Backslash => Some("\\"),
            Self::Semicolon => Some(";"),
            Self::Quote => Some("\""),
            Self::Tilde => Some("~"),
            Self::Slash | Self::KeyPadDivide => Some("/"),
            Self::Grave => Some("`"),
            Self::Apostrophe => Some("'"),
            Self::KeyPadMultiply => Some("*"),
            Self::KeyPadAdd => Some("+"),
            Self::AUmlautÄ => Some("Ä"),
            Self::UUmlautÜ => Some("Ü"),
            Self::OUmlautÖ => Some("Ö"),
            Self::SS => Some("ß"),
            Self::ACircumflexÂ => Some("Â"),
            Self::UAcuteÚ => Some("Ú"),
            Self::OCircumflexÔ => Some("Ô"),
            Self::ICircumflexÎ => Some("Î"),
            Self::ECircumflexÊ => Some("Ê"),
            Self::EthÐ => Some("Ð"),
            Self::OELigatureŒ => Some("Œ"),
            Self::AAcuteÁ => Some("Á"),
            Self::YAcuteÝ => Some("Ý"),
            Self::IUmlautÏ => Some("Ï"),
            Self::NTildeÑ => Some("Ñ"),
            Self::OGraveÒ => Some("Ò"),
            Self::UGraveÙ => Some("Ù"),
            Self::ARingÅ => Some("Å"),
            Self::AELigatureÆ => Some("Æ"),
            Self::OSlashØ => Some("Ø"),
            Self::IGraveÌ => Some("Ì"),
            Self::ThornÞ => Some("Þ"),
        }
    }
    // #[allow(clippy::too_many_lines)]
    // #[must_use]
    // /// Converts the requested key to a string representation of itself
    // pub const fn to_string(&self) -> &'static str {
    //     match self {
    //         // Letters
    //         Self::A => "A",
    //         Self::B => "B",
    //         Self::C => "C",
    //         Self::D => "D",
    //         Self::E => "E",
    //         Self::F => "F",
    //         Self::G => "G",
    //         Self::H => "H",
    //         Self::I => "I",
    //         Self::J => "J",
    //         Self::K => "K",
    //         Self::L => "L",
    //         Self::M => "M",
    //         Self::N => "N",
    //         Self::O => "O",
    //         Self::P => "P",
    //         Self::Q => "Q",
    //         Self::R => "R",
    //         Self::S => "S",
    //         Self::T => "T",
    //         Self::U => "U",
    //         Self::V => "V",
    //         Self::W => "W",
    //         Self::X => "X",
    //         Self::Y => "Y",
    //         Self::Z => "Z",

    //         // Numbers
    //         Self::Num0 => "0",
    //         Self::Num1 => "1",
    //         Self::Num2 => "2",
    //         Self::Num3 => "3",
    //         Self::Num4 => "4",
    //         Self::Num5 => "5",
    //         Self::Num6 => "6",
    //         Self::Num7 => "7",
    //         Self::Num8 => "8",
    //         Self::Num9 => "9",
    //         Self::KeyPad0 => "KeyPad 0",
    //         Self::KeyPad1 => "KeyPad 1",
    //         Self::KeyPad2 => "KeyPad 2",
    //         Self::KeyPad3 => "KeyPad 3",
    //         Self::KeyPad4 => "KeyPad 4",
    //         Self::KeyPad5 => "KeyPad 5",
    //         Self::KeyPad6 => "KeyPad 6",
    //         Self::KeyPad7 => "KeyPad 7",
    //         Self::KeyPad8 => "KeyPad 8",
    //         Self::KeyPad9 => "KeyPad 9",

    //         // Function Keys
    //         Self::F1 => "F1",
    //         Self::F2 => "F2",
    //         Self::F3 => "F3",
    //         Self::F4 => "F4",
    //         Self::F5 => "F5",
    //         Self::F6 => "F6",
    //         Self::F7 => "F7",
    //         Self::F8 => "F8",
    //         Self::F9 => "F9",
    //         Self::F10 => "F10",
    //         Self::F11 => "F11",
    //         Self::F12 => "F12",
    //         Self::F13 => "F13",
    //         Self::F14 => "F14",
    //         Self::F15 => "F15",
    //         Self::F16 => "F16",
    //         Self::F17 => "F17",
    //         Self::F18 => "F18",
    //         Self::F19 => "F19",
    //         Self::F20 => "F20",
    //         Self::F21 => "F21",
    //         Self::F22 => "F22",
    //         Self::F23 => "F23",
    //         Self::F24 => "F24",

    //         // Modifiers
    //         Self::LeftShift => "Left Shift",
    //         Self::RightShift => "Right Shift",
    //         Self::LeftControl => "Left Control",
    //         Self::RightControl => "Right Control",
    //         Self::LeftAlt => "Left Alt",
    //         Self::RightAlt => "Right Alt",
    //         Self::LeftSuper => "Left Super",
    //         Self::RightSuper => "Right Super",

    //         // Symbols / Punctuation
    //         Self::Space => "Space",
    //         Self::Enter => "Enter",
    //         Self::Escape => "Escape",
    //         Self::Backspace => "Backspace",
    //         Self::Tab => "Tab",
    //         Self::Comma => ",",
    //         Self::Period => ".",
    //         Self::Minus => "-",
    //         Self::Equal => "=",
    //         Self::LeftBracket => "[",
    //         Self::RightBracket => "]",
    //         Self::Backslash => "\\",
    //         Self::Semicolon => ";",
    //         Self::Quote => "\"",
    //         Self::Tilde => "~",
    //         Self::Slash => "/",
    //         Self::Grave => "`",
    //         Self::Apostrophe => "'",

    //         // Arrow keys
    //         Self::Up => "Up",
    //         Self::Down => "Down",
    //         Self::Left => "Left",
    //         Self::Right => "Right",

    //         // Editing keys
    //         Self::Insert => "Insert",
    //         Self::Delete => "Delete",
    //         Self::Home => "Home",
    //         Self::End => "End",
    //         Self::PageUp => "Page Up",
    //         Self::PageDown => "Page Down",

    //         // Lock keys
    //         Self::CapsLock => "Caps Lock",
    //         Self::NumLock => "Num Lock",
    //         Self::ScrollLock => "Scroll Lock",

    //         // Keypad operations
    //         Self::KeyPadDivide => "KeyPad /",
    //         Self::KeyPadMultiply => "KeyPad *",
    //         Self::KeyPadSubtract => "KeyPad -",
    //         Self::KeyPadAdd => "KeyPad +",
    //         Self::KeyPadDecimal => "KeyPad .",
    //         Self::KeyPadEnter => "KeyPad Enter",

    //         // International & special characters
    //         Self::AUmlautÄ => "Ä",
    //         Self::UUmlautÜ => "Ü",
    //         Self::OUmlautÖ => "Ö",
    //         Self::SS => "ß",
    //         Self::ACircumflexÂ => "Â",
    //         Self::UAcuteÚ => "Ú",
    //         Self::OCircumflexÔ => "Ô",
    //         Self::ICircumflexÎ => "Î",
    //         Self::ECircumflexÊ => "Ê",
    //         Self::EthÐ => "Ð",
    //         Self::OELigatureŒ => "Œ",
    //         Self::AAcuteÁ => "Á",
    //         Self::YAcuteÝ => "Ý",
    //         Self::IUmlautÏ => "Ï",
    //         Self::NTildeÑ => "Ñ",
    //         Self::OGraveÒ => "Ò",
    //         Self::UGraveÙ => "Ù",
    //         Self::ARingÅ => "Å",
    //         Self::AELigatureÆ => "Æ",
    //         Self::OSlashØ => "Ø",
    //         Self::IGraveÌ => "Ì",
    //         Self::ThornÞ => "Þ",

    //         // Multimedia keys
    //         Self::MediaPlayPause => "Media Play/Pause",
    //         Self::MediaStop => "Media Stop",
    //         Self::MediaNext => "Media Next",
    //         Self::MediaPrev => "Media Previous",
    //         Self::VolumeUp => "Volume Up",
    //         Self::VolumeDown => "Volume Down",
    //         Self::Mute => "Mute",

    //         // Browser/OS keys
    //         Self::BrowserBack => "Browser Back",
    //         Self::BrowserForward => "Browser Forward",
    //         Self::BrowserRefresh => "Browser Refresh",
    //         Self::BrowserHome => "Browser Home",
    //         Self::LaunchMail => "Launch Mail",
    //         Self::LaunchApp1 => "Launch App 1",
    //         Self::LaunchApp2 => "Launch App 2",

    //         // Platform-specific
    //         Self::Menu => "Menu",
    //         Self::PrintScreen => "Print Screen",
    //         Self::Pause => "Pause",
    //         Self::Application => "Application",

    //         // what
    //         Self::F25 => "F25",
    //         Self::KeyPadEqual => "KeyPad =",
    //         Self::World1 => "World 1",
    //         Self::World2 => "World 2",
    //         Self::Unknown => "Unknown",
    //     }
    // }
}
#[cfg(feature = "std")]
impl core::fmt::Display for KeyCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value = self.to_string();
        write!(f, "{value}")
    }
}
#[cfg(feature = "std")]
define_keys!(
            // Letters
            A => "A",
            B => "B",
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            H => "H",
            I => "I",
            J => "J",
            K => "K",
            L => "L",
            M => "M",
            N => "N",
            O => "O",
            P => "P",
            Q => "Q",
            R => "R",
            S => "S",
            T => "T",
            U => "U",
            V => "V",
            W => "W",
            X => "X",
            Y => "Y",
            Z => "Z",

            // Numbers
            Num0 => "0",
            Num1 => "1",
            Num2 => "2",
            Num3 => "3",
            Num4 => "4",
            Num5 => "5",
            Num6 => "6",
            Num7 => "7",
            Num8 => "8",
            Num9 => "9",
            KeyPad0 => "KeyPad 0",
            KeyPad1 => "KeyPad 1",
            KeyPad2 => "KeyPad 2",
            KeyPad3 => "KeyPad 3",
            KeyPad4 => "KeyPad 4",
            KeyPad5 => "KeyPad 5",
            KeyPad6 => "KeyPad 6",
            KeyPad7 => "KeyPad 7",
            KeyPad8 => "KeyPad 8",
            KeyPad9 => "KeyPad 9",

            // Function Keys
            F1 => "F1",
            F2 => "F2",
            F3 => "F3",
            F4 => "F4",
            F5 => "F5",
            F6 => "F6",
            F7 => "F7",
            F8 => "F8",
            F9 => "F9",
            F10 => "F10",
            F11 => "F11",
            F12 => "F12",
            F13 => "F13",
            F14 => "F14",
            F15 => "F15",
            F16 => "F16",
            F17 => "F17",
            F18 => "F18",
            F19 => "F19",
            F20 => "F20",
            F21 => "F21",
            F22 => "F22",
            F23 => "F23",
            F24 => "F24",

            // Modifiers
            LeftShift => "Left Shift",
            RightShift => "Right Shift",
            LeftControl => "Left Control",
            RightControl => "Right Control",
            LeftAlt => "Left Alt",
            RightAlt => "Right Alt",
            LeftSuper => "Left Super",
            RightSuper => "Right Super",
    LeftHyper=>"Hyper Left",
    RightHyper=>"Hyper Right",
    AltControl=>"Alt Control",

            // Symbols / Punctuation
            Space => "Space",
            Enter => "Enter",
            Escape => "Escape",
            Backspace => "Backspace",
            Tab => "Tab",
            Comma => ",",
            Period => ".",
            Minus => "-",
            Equal => "=",
            LeftBracket => "[",
            RightBracket => "]",
            Backslash => "\\",
            Semicolon => ";",
            Quote => "\"",
            Tilde => "~",
            Slash => "/",
            Grave => "`",
            Apostrophe => "'",

            // Arrow keys
            UpArrow => "Up",
            DownArrow => "Down",
            LeftArrow => "Left",
            RightArrow => "Right",

            // Editing keys
            Insert => "Insert",
            Delete => "Delete",
            Home => "Home",
            End => "End",
            PageUp => "Page Up",
            PageDown => "Page Down",

            // Lock keys
            CapsLock => "Caps Lock",
            NumLock => "Num Lock",
            ScrollLock => "Scroll Lock",

            // Keypad operations
            KeyPadDivide => "KeyPad /",
            KeyPadMultiply => "KeyPad *",
            KeyPadSubtract => "KeyPad -",
            KeyPadAdd => "KeyPad +",
            KeyPadDecimal => "KeyPad .",
            KeyPadEnter => "KeyPad Enter",

            // International & special characters
            AUmlautÄ => "Ä",
            UUmlautÜ => "Ü",
            OUmlautÖ => "Ö",
            SS => "ß",
            ACircumflexÂ => "Â",
            UAcuteÚ => "Ú",
            OCircumflexÔ => "Ô",
            ICircumflexÎ => "Î",
            ECircumflexÊ => "Ê",
            EthÐ => "Ð",
            OELigatureŒ => "Œ",
            AAcuteÁ => "Á",
            YAcuteÝ => "Ý",
            IUmlautÏ => "Ï",
            NTildeÑ => "Ñ",
            OGraveÒ => "Ò",
            UGraveÙ => "Ù",
            ARingÅ => "Å",
            AELigatureÆ => "Æ",
            OSlashØ => "Ø",
            IGraveÌ => "Ì",
            ThornÞ => "Þ",

            // Multimedia keys
            MediaPlayPause => "Media Play/Pause",
            MediaStop => "Media Stop",
            MediaNext => "Media Next",
            MediaPrev => "Media Previous",
            VolumeUp => "Volume Up",
            VolumeDown => "Volume Down",
            Mute => "Mute",
            MediaPause=>"Media Pause",
            MediaPlay=>"Media Play",
            MediaReverse=>"Media Reverse",
            MediaFastForward=>"Media Fast forward",
            MediaRecord=>"Media Record",

            // Browser/OS keys
            BrowserBack => "Browser Back",
            BrowserForward => "Browser Forward",
            BrowserRefresh => "Browser Refresh",
            BrowserHome => "Browser Home",
            LaunchMail => "Launch Mail",
            LaunchApp1 => "Launch App 1",
            LaunchApp2 => "Launch App 2",

            // Platform-specific
            Menu => "Menu",
            PrintScreen => "Print Screen",
            Pause => "Pause",
            Application => "Application",

            AnyShift=>"Shift",
            AnyControl=>"Control",
            AnyAlt=>"Alt",
            AnySuper=>"Super",

            // what
            F25 => "F25",
            KeyPadEqual => "KeyPad =",
            World1 => "World 1",
            World2 => "World 2",
            BackTab=>"BackTab",
            SpecialControl=>"Special Control",
            Unknown => "Unknown",);

#[must_use]
#[cfg(feature = "std")]
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
impl KeyCode {
    #[must_use]
    /// Get the number of the keycode if it is a number
    pub const fn get_number<
        T: crate::math::ConstOne
            + crate::math::ConstZero
            + crate::math::ConstNumbers128,
    >(
        &self,
    ) -> Option<T> {
        Some(match self {
            Self::Num0 => T::ZERO,
            Self::Num1 => T::ONE,
            Self::Num2 => T::CONST_2,
            Self::Num3 => T::CONST_3,
            Self::Num4 => T::CONST_4,
            Self::Num5 => T::CONST_5,
            Self::Num6 => T::CONST_6,
            Self::Num7 => T::CONST_7,
            Self::Num8 => T::CONST_8,
            Self::Num9 => T::CONST_9,
            _ => return None,
        })
    }
    #[must_use]
    /// Check if the given keycode is a number
    pub const fn is_number(&self) -> bool {
        self.get_number::<usize>().is_some()
    }
}
