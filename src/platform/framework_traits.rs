use crate::render::U2;

use super::{cursors::Cursor, Buffer, KeyCode, MouseButton, Time};

// Most basic of framework functionality
pub trait Framework: Window + Input + Output + Timing {}
impl<T: Window + Input + Output + Timing> Framework for T {}

pub trait ExtendedFramework:
    Framework
    + ExtendedInput
    + ExtendedWindow
    + ExtendedTiming
    + Control
    + ExtendedControl
{
}
impl<T> ExtendedFramework for T where
    T: Framework
        + ExtendedInput
        + ExtendedWindow
        + ExtendedTiming
        + Control
        + ExtendedControl
{
}

pub trait Window {
    fn new(buffer: &Buffer, title: &str) -> Self
    where
        Self: Sized;

    fn update(&mut self, buffer: &[u32]);
    fn is_open(&self) -> bool;
    fn clean_up(&self) {}
}

pub trait Input {
    fn get_mouse_position(&self) -> Option<(f64, f64)>;
    fn is_key_down(&self, key: KeyCode) -> bool;
    fn is_mouse_down(&self, button: MouseButton) -> bool;
}
pub trait Output {
    fn log<T: std::fmt::Debug>(&self, t: T);
}

pub trait Timing {
    fn get_time(&self) -> Box<dyn Time>;
    fn sleep(&self, time: u64);
    fn sample_fps(&mut self) -> u64;
}
pub trait ExtendedTiming {
    fn set_target_fps(&mut self, fps: usize);
}
pub trait ExtendedInput {
    fn get_mouse_scroll(&self) -> Option<(f64, f64)>;
}

pub trait ExtendedWindow {
    fn set_title(&mut self, title: &str);
    /// Width/Height should be something like 32x32 or 48x48
    fn set_icon(&mut self, buffer: &[u32], width: u32, height: u32);
    fn set_cursor_style(&mut self, style: &Cursor);
    fn load_custom_cursor(
        &mut self,
        size: U2,
        main_color: u32,
        secondary_color: u32,
    ) -> super::cursors::Cursors;
}

pub trait Control {
    fn move_window(&mut self, x: isize, y: isize) {
        let current = self.get_position();
        self.set_position(current.0 + x, current.1 + y);
    }
    fn get_position(&self) -> (isize, isize);
    fn set_position(&mut self, x: isize, y: isize);
    fn set_size(&mut self, buffer: &Buffer);
    fn get_size(&self) -> (usize, usize);
}
pub trait ExtendedControl {
    fn set_always_ontop(&mut self, always_ontop: bool);
    fn minimize(&mut self);
    fn maximize(&mut self);
    fn restore(&mut self);
    fn is_minimized(&self) -> bool;
    fn is_maximized(&self) -> bool;
}
