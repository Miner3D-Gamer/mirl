use std::{
    mem, ptr,
    sync::{Arc, Mutex, OnceLock},
};

use raw_window_handle::RawWindowHandle;
use winapi::{
    shared::{
        minwindef::{FALSE, LPARAM, UINT, WPARAM},
        windef::HWND,
    },
    um::winuser::{
        CallWindowProcW, DefWindowProcW, GetRawInputData, GetWindowLongPtrW,
        RegisterRawInputDevices, SetWindowLongPtrW, GWLP_WNDPROC, RAWINPUT,
        RAWINPUTDEVICE, RAWINPUTHEADER, RIDEV_INPUTSINK, RIDEV_REMOVE,
        RID_INPUT, RIM_TYPEMOUSE, WM_INPUT, WNDPROC,
    },
};

use super::{MouseDelta, RawMouseInputTrait};
use crate::settings::MapType;

/// Global mouse delta storage (since there's only one mouse)
static GLOBAL_MOUSE_DELTA: OnceLock<Arc<Mutex<MouseDelta>>> = OnceLock::new();

/// Global storage for original window procedures
static ORIGINAL_PROCS: OnceLock<Arc<Mutex<MapType<isize, WNDPROC>>>> =
    OnceLock::new();

/// Get or initialize the global mouse delta storage
fn get_global_delta() -> &'static Arc<Mutex<MouseDelta>> {
    GLOBAL_MOUSE_DELTA
        .get_or_init(|| Arc::new(Mutex::new(MouseDelta::default())))
}

/// Get or initialize the original procedures storage
fn get_original_procs() -> &'static Arc<Mutex<MapType<isize, WNDPROC>>> {
    ORIGINAL_PROCS.get_or_init(|| Arc::new(Mutex::new(MapType::new())))
}

/// Update the global mouse delta
fn update_global_delta(delta: MouseDelta) {
    if let Ok(mut global_delta) = get_global_delta().lock() {
        global_delta.dx += delta.dx;
        global_delta.dy += delta.dy;
    }
}

/// Reset the global mouse delta to zero
fn reset_global_delta() {
    if let Ok(mut global_delta) = get_global_delta().lock() {
        *global_delta = MouseDelta::default();
    }
}

/// Get the original window procedure for a given HWND
fn get_original_proc(hwnd: HWND) -> Option<WNDPROC> {
    get_original_procs().lock().ok()?.get(&(hwnd as isize)).copied()
}

/// Store the original window procedure for a given HWND
fn store_original_proc(hwnd: HWND, proc: WNDPROC) {
    if let Ok(mut procs) = get_original_procs().lock() {
        procs.insert(hwnd as isize, proc);
    }
}

/// Remove the stored original window procedure for a given HWND
fn remove_original_proc(hwnd: HWND) {
    if let Ok(mut procs) = get_original_procs().lock() {
        procs.remove(&(hwnd as isize));
    }
}

/// Process raw input and update global delta
unsafe fn process_raw_input(lparam: LPARAM) {
    // Get the size of the raw input data
    let mut size: UINT = 0;
    let result = GetRawInputData(
        lparam as *mut _,
        RID_INPUT,
        ptr::null_mut(),
        &raw mut size,
        mem::size_of::<RAWINPUTHEADER>() as UINT,
    );

    if result == u32::MAX || size == 0 {
        return;
    }

    // Allocate buffer for raw input data
    let mut buffer = vec![0u8; size as usize];

    // Get the actual raw input data
    let result = GetRawInputData(
        lparam as *mut _,
        RID_INPUT,
        buffer.as_mut_ptr().cast(),
        &raw mut size,
        mem::size_of::<RAWINPUTHEADER>() as UINT,
    );

    if result == u32::MAX {
        return;
    }
    #[allow(clippy::cast_ptr_alignment)]
    let raw_input = &*buffer.as_ptr().cast::<RAWINPUT>();

    // Check if this is mouse input
    if raw_input.header.dwType == RIM_TYPEMOUSE {
        let mouse = &raw_input.data.mouse();

        // Get raw mouse movement (delta values)
        let dx = mouse.lLastX;
        let dy = mouse.lLastY;

        // Only update if there's actual movement
        if dx != 0 || dy != 0 {
            let delta = MouseDelta {
                dx,
                dy,
            };
            update_global_delta(delta);
        }
    }
}

/// Window procedure that intercepts `WM_INPUT` messages
unsafe extern "system" fn raw_mouse_window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> isize {
    // Handle WM_INPUT messages to capture raw mouse input
    if msg == WM_INPUT {
        process_raw_input(lparam);
        // Don't return here - let the original procedure handle it too
    }

    // Always call the original window procedure
    get_original_proc(hwnd).map_or_else(
        || DefWindowProcW(hwnd, msg, wparam, lparam),
        |original_proc| {
            CallWindowProcW(original_proc, hwnd, msg, wparam, lparam)
        },
    )
}

/// Raw mouse input handler
#[derive(Debug, Clone)]
pub struct RawMouseInputWindows {
    hwnd: HWND,
    original_proc: WNDPROC,
}

impl RawMouseInputTrait for RawMouseInputWindows {
    fn new(handle: RawWindowHandle) -> Result<Self, &'static str> {
        let hwnd = match handle {
            RawWindowHandle::Win32(win32_handle) => {
                win32_handle.hwnd.get() as HWND
            }
            _ => return Err("Only Win32 window handles are supported"),
        };

        if hwnd.is_null() {
            return Err("Invalid window handle");
        }

        unsafe {
            // Get the current window procedure before modifying it
            let original_proc_ptr = GetWindowLongPtrW(hwnd, GWLP_WNDPROC);
            if original_proc_ptr == 0 {
                return Err("Failed to get original window procedure");
            }

            let original_proc: WNDPROC = std::mem::transmute(original_proc_ptr);

            // Store the original procedure in our global map
            store_original_proc(hwnd, original_proc);

            #[allow(clippy::fn_to_numeric_cast)]
            // Set our custom window procedure
            let result = SetWindowLongPtrW(
                hwnd,
                GWLP_WNDPROC,
                raw_mouse_window_proc as isize,
            );

            if result == 0 {
                remove_original_proc(hwnd);
                return Err("Failed to set window procedure");
            }

            // Register for raw mouse input
            let rid = RAWINPUTDEVICE {
                usUsagePage: 0x01,        // Generic Desktop Controls
                usUsage: 0x02,            // Mouse
                dwFlags: RIDEV_INPUTSINK, // Receive input even when not in foreground
                hwndTarget: hwnd,         // Target window
            };

            let register_result = RegisterRawInputDevices(
                &raw const rid,
                1,
                mem::size_of::<RAWINPUTDEVICE>() as UINT,
            );

            if register_result == FALSE {
                // Restore original procedure on failure
                SetWindowLongPtrW(hwnd, GWLP_WNDPROC, original_proc_ptr);
                remove_original_proc(hwnd);
                return Err("Failed to register raw input device");
            }

            Ok(Self {
                hwnd,
                original_proc,
            })
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        let delta = get_global_delta()
            .lock()
            .map_or_else(|_| MouseDelta::default(), |buffer| *buffer);

        // Reset delta after reading (so each call gets the accumulated delta since last call)
        reset_global_delta();

        (delta.dx, delta.dy)
    }
}

impl Drop for RawMouseInputWindows {
    fn drop(&mut self) {
        unsafe {
            #[allow(clippy::missing_transmute_annotations)]
            // Restore original window procedure
            SetWindowLongPtrW(
                self.hwnd,
                GWLP_WNDPROC,
                std::mem::transmute(self.original_proc),
            );

            // Remove from our global storage
            remove_original_proc(self.hwnd);

            // Unregister raw input
            let rid = RAWINPUTDEVICE {
                usUsagePage: 0x01,
                usUsage: 0x02,
                dwFlags: RIDEV_REMOVE,
                hwndTarget: ptr::null_mut(),
            };

            RegisterRawInputDevices(
                &raw const rid,
                1,
                mem::size_of::<RAWINPUTDEVICE>() as UINT,
            );
        }
    }
}
