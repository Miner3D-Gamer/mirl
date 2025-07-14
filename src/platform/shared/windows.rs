extern crate winapi;
use std::os::raw::c_void;

use winapi::shared::windef::HWND;
use winapi::um::winuser::{IsIconic, IsZoomed};
use winapi::um::winuser::{
    SetWindowPos,
    ShowWindow, //HWND_NOTOPMOST, HWND_TOPMOST,
    SWP_NOMOVE,
    // SWP_NOSIZE,
    SWP_NOZORDER, // SWP_SHOWWINDOW,
    SW_MAXIMIZE,
    SW_MINIMIZE,
    SW_RESTORE,
};
/// Maximize the given window -> Expand its borders to fit the screen
pub fn maximize(window: &*mut c_void) {
    let hwnd = *window as HWND;
    unsafe {
        ShowWindow(hwnd, SW_MAXIMIZE);
    }
}
/// Hide away the given window
pub fn minimize(window: &*mut c_void) {
    let hwnd = *window as HWND;
    unsafe {
        ShowWindow(hwnd, SW_MINIMIZE);
    }
}
/// Unhide un-away the given window
pub fn restore(window: &*mut c_void) {
    let hwnd = *window as HWND;
    unsafe {
        ShowWindow(hwnd, SW_RESTORE);
    }
}

// pub fn always_on_top(window: &*mut c_void, always_on_top_bool: bool) {
//     if always_on_top_bool {
//         topmost(window);
//     } else {
//         not_topmost(window);
//     }
// }

// pub fn topmost(window: &*mut c_void) {
//     let hwnd = *window as HWND;
//     unsafe {
//         SetWindowPos(
//             hwnd,
//             HWND_TOPMOST,
//             0,
//             0,
//             0,
//             0,
//             SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
//         );
//     }
// }
// pub fn not_topmost(window: &*mut c_void) {
//     let hwnd = *window as HWND;
//     unsafe {
//         SetWindowPos(
//             hwnd,
//             HWND_NOTOPMOST,
//             0,
//             0,
//             0,
//             0,
//             SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
//         );
//     }
// }
/// Resize the current window to the specified size
pub fn resize<T: num_traits::ToPrimitive>(window: &*mut c_void, size: (T, T)) {
    let hwnd = *window as HWND;

    unsafe {
        // Resize the window to the given width and height
        SetWindowPos(
            hwnd,
            std::ptr::null_mut(), // No changes to the window's position
            0,
            0,
            size.0.to_i32().unwrap(),
            size.1.to_i32().unwrap(),
            SWP_NOZORDER | SWP_NOMOVE, // Keep the current position, just resize
        );
    }
}
/// Wether the window is minimized on windows
pub fn is_window_minimized(window: &*mut c_void) -> bool {
    let hwnd = *window as HWND;
    unsafe { IsIconic(hwnd) != 0 }
}
/// Get the numerical value of a window
pub fn get_window_handle(window: &*mut c_void) -> HWND {
    let hwnd = *window as HWND;
    hwnd
}
/// Wether a window is maximized on windows
pub fn is_window_maximized(window: &*mut c_void) -> bool {
    let hwnd = *window as HWND;
    unsafe { IsZoomed(hwnd) != 0 }
}
