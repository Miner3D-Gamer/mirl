use crossterm::{event, ExecutableCommand};

#[cfg(feature = "keycodes")]
use crate::platform::keycodes::KeyCode;
use crate::{
    console::color_data_to_console,
    graphics::resize_buffer_nearest,
    platform::windowing::{traits::*, WindowError, WindowUpdateError},
    prelude::{ConcatenateString, RepeatData},
};

/// Backend implementation using just the console
#[derive(Debug)]
pub struct Framework {
    /// The title of the current window
    pub name: String,
    /// If the usual os menu should be shown
    pub os_menu: bool,
    /// If the title is visible
    pub visible_title: bool,
    /// If the contents should even be displayed at all
    pub visible: bool,
    /// If the window is still open
    pub is_open: bool,
    /// The width of the buffer it should hold
    pub width: usize,
    /// The height of the buffer it should hold
    pub height: usize,
    /// The the size of the buffer last frame, needed to smartly clear the screen
    pub last_size: (usize, usize),
    /// The recorded mouse position
    pub mouse_pos: std::sync::Arc<std::sync::Mutex<(f32, f32)>>,
    /// If any of the typical mouse buttons are detected
    pub mouse_buttons: std::sync::Arc<std::sync::Mutex<[bool; 3]>>,
    /// All currently pressed keys
    #[cfg(feature = "keycodes")]
    pub pressed_keys:
        std::sync::Arc<std::sync::Mutex<std::collections::HashSet<KeyCode>>>,
}

impl NewWindow for Framework {
    fn new(
        title: &str,
        settings: crate::prelude::WindowSettings,
        // #[cfg(feature = "svg")] cursor: Option<Cursor>,
    ) -> Result<Self, super::WindowError>
    where
        Self: Sized,
    {
        crossterm::execute!(std::io::stdout(), event::EnableMouseCapture)
            .map_err(|x| {
                WindowError::Misc(format!(
                    "Error while trying to activate mouse capturing: {x}"
                ))
            })?;
        Ok(Self {
            name: title.to_string(),
            os_menu: settings.os_menu,
            visible: settings.visible,
            visible_title: settings.title_visible,
            height: settings.size.0 as usize,
            width: settings.size.1 as usize,
            last_size: (0, 0),
            is_open: true,
            mouse_pos: std::sync::Arc::new(std::sync::Mutex::new((0.0, 0.0))),
            mouse_buttons: std::sync::Arc::new(std::sync::Mutex::new(
                [false; 3],
            )),
            #[cfg(feature = "keycodes")]
            pressed_keys: std::sync::Arc::new(std::sync::Mutex::new(
                std::collections::HashSet::new(),
            )),
        })
    }
}
impl Framework {
    /// Poll for mouse and keyboard events
    fn poll_events(&self) -> Result<(), WindowError> {
        if event::poll(std::time::Duration::from_millis(100)).map_err(|x| {
            WindowError::Misc(format!("Error while trying to poll events: {x}"))
        })? {
            match event::read() {
                Ok(crossterm::event::Event::Mouse(mouse_event)) => {
                    // Update position
                    *self.mouse_pos.lock().map_err(|x| {
                        WindowError::Misc(format!(
                            "Error while trying to lock mouse pos: {x}"
                        ))
                    })? = (
                        f32::from(mouse_event.column),
                        f32::from(mouse_event.row),
                    );

                    // Update button states
                    match mouse_event.kind {
                        crossterm::event::MouseEventKind::Down(btn) => {
                            let idx = button_to_index(btn);
                            self.mouse_buttons.lock().map_err(|x| {
                                WindowError::Misc(format!(
                        "Error while trying to lock mouse buttons (1): {x}"
                    ))
                            })?[idx] = true;
                        }
                        crossterm::event::MouseEventKind::Up(btn) => {
                            let idx = button_to_index(btn);
                            self.mouse_buttons.lock().map_err(|x| {
                                WindowError::Misc(format!(
                        "Error while trying to lock mouse buttons (2): {x}"
                    ))
                            })?[idx] = false;
                        }
                        _ => {}
                    }
                }
                #[cfg(feature = "keycodes")]
                Ok(crossterm::event::Event::Key(key_event)) => {
                    let key = crossterm_to_keycode(key_event.code);
                    let mut keys = self.pressed_keys.lock().map_err(|x| {
                        WindowError::Misc(format!(
                            "Error while trying to lock pressed keys: {x}"
                        ))
                    })?;
                    match key_event.kind {
                        crossterm::event::KeyEventKind::Press => {
                            keys.insert(key);
                        }
                        crossterm::event::KeyEventKind::Release => {
                            keys.remove(&key);
                        }
                        crossterm::event::KeyEventKind::Repeat => {}
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    return Err(WindowError::Misc(format!(
                        "Error while trying to read event: {e}"
                    )));
                }
            }
        }
        Ok(())
    }
}

impl MouseInput for Framework {
    fn get_mouse_position(&self) -> Option<(f32, f32)> {
        Some(*self.mouse_pos.lock().ok()?)
    }

    fn is_mouse_down(&self, button: crate::platform::MouseButton) -> bool {
        let idx = match button {
            crate::platform::MouseButton::Left => 0,
            crate::platform::MouseButton::Right => 1,
            crate::platform::MouseButton::Middle => 2,
            _ => return false,
        };
        self.mouse_buttons.lock().is_ok_and(|x| x[idx])
    }
}
const fn button_to_index(btn: crossterm::event::MouseButton) -> usize {
    match btn {
        crossterm::event::MouseButton::Left => 0,
        crossterm::event::MouseButton::Right => 1,
        crossterm::event::MouseButton::Middle => 2,
    }
}
impl Window for Framework {
    fn update_raw(
        &mut self,
        pixels: &[u32],
        width: usize,
        height: usize,
    ) -> Result<(), super::WindowError> {
        let new = resize_buffer_nearest(
            pixels,
            width,
            height,
            self.width,
            self.height,
        );
        let (w, h) = crossterm::terminal::size().map_err(|_x| {
            WindowError::Misc("Unable to get console size".to_string())
        })?;
        let w = w as usize - 2;
        let h = h as usize - 2;
        if w < self.width {
            let ratio = self.width as f32 / self.height as f32;
            self.width = w;
            self.height = (w as f32 / ratio).round() as usize;
        }
        if h < self.height {
            let ratio = self.width as f32 / self.height as f32;
            self.height = h;
            self.width = (h as f32 / ratio).round() as usize;
        }

        let data = color_data_to_console(&new, self.width, self.height);

        if self.last_size != (self.width, self.height) {
            std::io::stdout()
                .execute(crossterm::terminal::Clear(
                    crossterm::terminal::ClearType::All,
                ))
                .map_err(|x| {
                    WindowUpdateError::Misc(format!(
                        "Unable to clear screen: {x}"
                    ))
                })?;
            self.last_size = (self.width, self.height);
        }
        std::io::stdout().execute(crossterm::cursor::MoveTo(0, 0)).map_err(
            |x| WindowUpdateError::Misc(format!("Unable to move cursor: {x}")),
        )?;

        let mut final_text = format!("> {}", self.name);
        final_text.push_str(&format!(
            "\n+{}+",
            '-'.repeat_value(self.width).concatenate()
        ));
        for line in data {
            final_text.push_str(&format!("\n|{line}|"));
        }
        final_text.push_str(&format!(
            "\n+{}+",
            '-'.repeat_value(self.width).concatenate()
        ));

        println!("{final_text}");
        self.poll_events()?;

        Ok(())
    }

    fn is_open(&self) -> bool {
        self.is_open
    }
    fn close_and_clean_up(&mut self) {
        // TODO: ADD PROPER ERRORING
        let _ =
            crossterm::execute!(std::io::stdout(), event::DisableMouseCapture)
                .map_err(|x| {
                    WindowError::Misc(format!(
                        "Error while trying to activate mouse capturing: {x}"
                    ))
                });
        self.is_open = false;
    }
}

impl RenderLayer for Framework {
    #[inline]
    fn set_render_layer(&mut self, _level: crate::platform::WindowLevel) {
        // We can only have 1 window at the time
    }
}

impl Control for Framework {
    fn get_position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn set_position(&mut self, _xy: (i32, i32)) {}

    fn set_size(&mut self, buffer: &crate::prelude::Buffer) {
        self.width = buffer.width;
        self.height = buffer.height;
    }

    fn get_size(&self) -> (i32, i32) {
        (self.width as i32, self.height as i32)
    }
}
impl Timing for Framework {
    #[inline]
    fn get_time(&self) -> Box<dyn crate::platform::Time> {
        crate::platform::shared::get_time()
    }
    #[inline]
    fn sleep(&self, time: std::time::Duration) {
        crate::platform::shared::sleep(time);
    }
}

impl Output for Framework {
    #[inline]
    fn log(&self, t: &str) {
        crate::platform::shared::log(t);
    }
}

impl Visibility for Framework {
    #[inline]
    fn maximize(&mut self) {
        let Ok((w, h)) = crossterm::terminal::size().map_err(|_x| {
            WindowError::Misc("Unable to get console size".to_string())
        }) else {
            return;
        };
        let w = w as usize;
        let h = h as usize;

        if w > self.width {
            let ratio = self.height as f32 / self.width as f32;
            self.width = w;
            self.height = (w as f32 / ratio).round() as usize;
        }
        if h < self.height {
            let ratio = self.width as f32 / self.height as f32;
            self.height = h;
            self.width = (h as f32 / ratio).round() as usize;
        }
    }
    #[inline]
    fn minimize(&mut self) {
        self.visible = false;
    }
    #[inline]
    fn restore(&mut self) {
        self.visible = true;
    }
    fn is_maximized(&self) -> bool {
        let Ok((w, _h)) = crossterm::terminal::size().map_err(|_x| {
            WindowError::Misc("Unable to get console size".to_string())
        }) else {
            return false;
        };
        let w = w as usize;
        w == self.width + 2
    }
    fn is_minimized(&self) -> bool {
        !self.visible
    }
}

impl ExtendedWindow for Framework {
    fn set_title(&mut self, title: &str) {
        self.name = title.to_string();
    }
}
#[cfg(feature = "keycodes")]
const fn crossterm_to_keycode(key: crossterm::event::KeyCode) -> KeyCode {
    match key {
        crossterm::event::KeyCode::Char('a' | 'A') => KeyCode::A,
        crossterm::event::KeyCode::Char('b' | 'B') => KeyCode::B,
        crossterm::event::KeyCode::Char('c' | 'C') => KeyCode::C,
        crossterm::event::KeyCode::Char('d' | 'D') => KeyCode::D,
        crossterm::event::KeyCode::Char('e' | 'E') => KeyCode::E,
        crossterm::event::KeyCode::Char('f' | 'F') => KeyCode::F,
        crossterm::event::KeyCode::Char('g' | 'G') => KeyCode::G,
        crossterm::event::KeyCode::Char('h' | 'H') => KeyCode::H,
        crossterm::event::KeyCode::Char('i' | 'I') => KeyCode::I,
        crossterm::event::KeyCode::Char('j' | 'J') => KeyCode::J,
        crossterm::event::KeyCode::Char('k' | 'K') => KeyCode::K,
        crossterm::event::KeyCode::Char('l' | 'L') => KeyCode::L,
        crossterm::event::KeyCode::Char('m' | 'M') => KeyCode::M,
        crossterm::event::KeyCode::Char('n' | 'N') => KeyCode::N,
        crossterm::event::KeyCode::Char('o' | 'O') => KeyCode::O,
        crossterm::event::KeyCode::Char('p' | 'P') => KeyCode::P,
        crossterm::event::KeyCode::Char('q' | 'Q') => KeyCode::Q,
        crossterm::event::KeyCode::Char('r' | 'R') => KeyCode::R,
        crossterm::event::KeyCode::Char('s' | 'S') => KeyCode::S,
        crossterm::event::KeyCode::Char('t' | 'T') => KeyCode::T,
        crossterm::event::KeyCode::Char('u' | 'U') => KeyCode::U,
        crossterm::event::KeyCode::Char('v' | 'V') => KeyCode::V,
        crossterm::event::KeyCode::Char('w' | 'W') => KeyCode::W,
        crossterm::event::KeyCode::Char('x' | 'X') => KeyCode::X,
        crossterm::event::KeyCode::Char('y' | 'Y') => KeyCode::Y,
        crossterm::event::KeyCode::Char('z' | 'Z') => KeyCode::Z,
        crossterm::event::KeyCode::Char('0') => KeyCode::Num0,
        crossterm::event::KeyCode::Char('1') => KeyCode::Num1,
        crossterm::event::KeyCode::Char('2') => KeyCode::Num2,
        crossterm::event::KeyCode::Char('3') => KeyCode::Num3,
        crossterm::event::KeyCode::Char('4') => KeyCode::Num4,
        crossterm::event::KeyCode::Char('5') => KeyCode::Num5,
        crossterm::event::KeyCode::Char('6') => KeyCode::Num6,
        crossterm::event::KeyCode::Char('7') => KeyCode::Num7,
        crossterm::event::KeyCode::Char('8') => KeyCode::Num8,
        crossterm::event::KeyCode::Char('9') => KeyCode::Num9,
        crossterm::event::KeyCode::Enter => KeyCode::Enter,
        crossterm::event::KeyCode::Esc => KeyCode::Escape,
        crossterm::event::KeyCode::Backspace => KeyCode::Backspace,
        crossterm::event::KeyCode::Tab => KeyCode::Tab,
        crossterm::event::KeyCode::Char(' ') => KeyCode::Space,
        crossterm::event::KeyCode::Left => KeyCode::LeftArrow,
        crossterm::event::KeyCode::Right => KeyCode::RightArrow,
        crossterm::event::KeyCode::Up => KeyCode::UpArrow,
        crossterm::event::KeyCode::Down => KeyCode::DownArrow,
        crossterm::event::KeyCode::F(1) => KeyCode::F1,
        crossterm::event::KeyCode::F(2) => KeyCode::F2,
        crossterm::event::KeyCode::F(3) => KeyCode::F3,
        crossterm::event::KeyCode::F(4) => KeyCode::F4,
        crossterm::event::KeyCode::F(5) => KeyCode::F5,
        crossterm::event::KeyCode::F(6) => KeyCode::F6,
        crossterm::event::KeyCode::F(7) => KeyCode::F7,
        crossterm::event::KeyCode::F(8) => KeyCode::F8,
        crossterm::event::KeyCode::F(9) => KeyCode::F9,
        crossterm::event::KeyCode::F(10) => KeyCode::F10,
        crossterm::event::KeyCode::F(11) => KeyCode::F11,
        crossterm::event::KeyCode::F(12) => KeyCode::F12,
        crossterm::event::KeyCode::Insert => KeyCode::Insert,
        crossterm::event::KeyCode::Delete => KeyCode::Delete,
        crossterm::event::KeyCode::Home | event::KeyCode::KeypadBegin => {
            KeyCode::Home
        }
        crossterm::event::KeyCode::End => KeyCode::End,
        crossterm::event::KeyCode::PageUp => KeyCode::PageUp,
        crossterm::event::KeyCode::PageDown => KeyCode::PageDown,
        crossterm::event::KeyCode::Char('-') => KeyCode::Minus,
        crossterm::event::KeyCode::Char('=') => KeyCode::Equal,
        crossterm::event::KeyCode::Char('[') => KeyCode::LeftBracket,
        crossterm::event::KeyCode::Char(']') => KeyCode::RightBracket,
        crossterm::event::KeyCode::Char('\\') => KeyCode::Backslash,
        crossterm::event::KeyCode::Char(';') => KeyCode::Semicolon,
        crossterm::event::KeyCode::Char('\'') => KeyCode::Quote,
        crossterm::event::KeyCode::Char(',') => KeyCode::Comma,
        crossterm::event::KeyCode::Char('.') => KeyCode::Period,
        crossterm::event::KeyCode::Char('/') => KeyCode::Slash,
        crossterm::event::KeyCode::Char('`') => KeyCode::Grave,
        event::KeyCode::BackTab => KeyCode::BackTab,
        event::KeyCode::F(_)
        | event::KeyCode::Char(_)
        | event::KeyCode::Null => KeyCode::Unknown,
        event::KeyCode::CapsLock => KeyCode::CapsLock,
        event::KeyCode::ScrollLock => KeyCode::ScrollLock,
        event::KeyCode::NumLock => KeyCode::NumLock,
        event::KeyCode::PrintScreen => KeyCode::PrintScreen,
        event::KeyCode::Pause => KeyCode::Pause,
        event::KeyCode::Menu => KeyCode::Menu,
        event::KeyCode::Media(media_key_code) => match media_key_code {
            event::MediaKeyCode::Play => KeyCode::MediaPlay,
            event::MediaKeyCode::Pause => KeyCode::MediaPause,
            event::MediaKeyCode::PlayPause => KeyCode::MediaPlayPause,
            event::MediaKeyCode::Reverse | event::MediaKeyCode::Rewind => {
                KeyCode::MediaReverse
            } // I'm breaking the [KeyCode] rules but until someone explains to me the difference between rewind and reverse, I am keeping it like this
            event::MediaKeyCode::Stop => KeyCode::MediaStop,
            event::MediaKeyCode::FastForward => KeyCode::MediaFastForward,

            event::MediaKeyCode::TrackNext => KeyCode::MediaNext,
            event::MediaKeyCode::TrackPrevious => KeyCode::MediaPrev,
            event::MediaKeyCode::Record => KeyCode::MediaRecord,
            event::MediaKeyCode::LowerVolume => KeyCode::VolumeDown,
            event::MediaKeyCode::RaiseVolume => KeyCode::VolumeUp,
            event::MediaKeyCode::MuteVolume => KeyCode::Mute,
        },
        event::KeyCode::Modifier(modifier_key_code) => {
            match modifier_key_code {
                event::ModifierKeyCode::LeftShift => KeyCode::LeftShift,
                event::ModifierKeyCode::LeftControl
                | event::ModifierKeyCode::LeftMeta => KeyCode::LeftControl,
                event::ModifierKeyCode::LeftAlt => KeyCode::LeftAlt,
                event::ModifierKeyCode::LeftSuper => KeyCode::LeftSuper,
                event::ModifierKeyCode::LeftHyper => KeyCode::LeftHyper,
                event::ModifierKeyCode::RightShift => KeyCode::RightShift,
                event::ModifierKeyCode::RightControl
                | event::ModifierKeyCode::RightMeta => KeyCode::RightControl,
                event::ModifierKeyCode::RightAlt => KeyCode::RightAlt,
                event::ModifierKeyCode::RightSuper => KeyCode::RightSuper,
                event::ModifierKeyCode::RightHyper => KeyCode::RightHyper,
                event::ModifierKeyCode::IsoLevel3Shift => KeyCode::AltControl,
                event::ModifierKeyCode::IsoLevel5Shift => {
                    KeyCode::SpecialControl
                }
            }
        }
    }
}

#[cfg(feature = "keycodes")]
impl KeyboardInput for Framework {
    fn is_key_down(&self, key: KeyCode) -> bool {
        self.pressed_keys.lock().unwrap().contains(&key)
    }
}
