extern crate winapi;
use minifb::Window;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    SetWindowPos, ShowWindow, SWP_NOMOVE, SWP_NOZORDER, SW_MAXIMIZE,
    SW_MINIMIZE, SW_RESTORE,
};

pub fn maximize(window: &Window) {
    let hwnd = window.get_window_handle() as HWND;
    unsafe {
        ShowWindow(hwnd, SW_MAXIMIZE);
    }
}

pub fn minimize(window: &Window) {
    let hwnd = window.get_window_handle() as HWND;
    unsafe {
        ShowWindow(hwnd, SW_MINIMIZE);
    }
}

pub fn restore(window: &Window) {
    let hwnd = window.get_window_handle() as HWND;
    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
    }
}

pub fn resize(window: &Window, width: i32, height: i32) {
    let hwnd = window.get_window_handle() as HWND;

    unsafe {
        // Resize the window to the given width and height
        SetWindowPos(
            hwnd,
            std::ptr::null_mut(), // No changes to the window's position
            0,
            0,
            width as i32,
            height as i32,
            SWP_NOZORDER | SWP_NOMOVE, // Keep the current position, just resize
        );
    }
}
