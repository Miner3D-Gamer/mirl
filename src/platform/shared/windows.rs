extern crate winapi;
use std::os::raw::c_void;

use winapi::shared::windef::HWND;
use winapi::um::winuser::{IsIconic, IsZoomed};

use winapi::um::winuser::{
    SetWindowPos, ShowWindow, HWND_NOTOPMOST, HWND_TOPMOST, SWP_NOMOVE,
    SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW, SW_MAXIMIZE, SW_MINIMIZE,
    SW_RESTORE,
};

pub fn maximize(window: &*mut c_void) {
    let hwnd = *window as HWND;
    unsafe {
        ShowWindow(hwnd, SW_MAXIMIZE);
    }
}

pub fn minimize(window: &*mut c_void) {
    let hwnd = *window as HWND;
    unsafe {
        ShowWindow(hwnd, SW_MINIMIZE);
    }
}

pub fn restore(window: &*mut c_void) {
    let hwnd = *window as HWND;
    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
    }
}
pub fn always_ontop(window: &*mut c_void, always_ontop: bool) {
    if always_ontop {
        topmost(window);
    } else {
        not_topmost(window);
    }
}

pub fn topmost(window: &*mut c_void) {
    let hwnd = *window as HWND;
    unsafe {
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
        );
    }
}
pub fn not_topmost(window: &*mut c_void) {
    let hwnd = *window as HWND;
    unsafe {
        SetWindowPos(
            hwnd,
            HWND_NOTOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
        );
    }
}

pub fn resize(window: &*mut c_void, width: i32, height: i32) {
    let hwnd = *window as HWND;

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

pub fn is_window_minimized(window: &*mut c_void) -> bool {
    let hwnd = *window as HWND;
    unsafe { IsIconic(hwnd) != 0 }
}

pub fn get_window_handle(window: &*mut c_void) -> HWND {
    let hwnd = *window as HWND;
    hwnd
}

pub fn is_window_maximized(window: &*mut c_void) -> bool {
    let hwnd = *window as HWND;
    unsafe { IsZoomed(hwnd) != 0 }
}
