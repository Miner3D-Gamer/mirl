use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

use windows::{
    Win32::Foundation::RECT,
    Win32::Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC,
        GetPixel, ReleaseDC, SelectObject, SRCCOPY,
    },
    Win32::UI::WindowsAndMessaging::{GetDesktopWindow, GetShellWindow, GetWindowRect},
};

pub fn get_screen_resolution() -> (i32, i32) {
    let width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let height = unsafe { GetSystemMetrics(SM_CYSCREEN) };
    (width, height)
}
pub struct ScreenCapture {
    pub width: i32,
    pub height: i32,
    pub pixels: Vec<u32>,
}
pub fn capture_desktop_screen() -> Option<ScreenCapture> {
    unsafe {
        // Get the desktop window handle
        let desktop_hwnd = GetDesktopWindow();
        if desktop_hwnd.0 == 0 {
            return None;
        }

        // Get desktop dimensions
        let mut rect = RECT::default();
        if !GetWindowRect(desktop_hwnd, &mut rect).as_bool() {
            return None;
        }
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        // Get the device context for the desktop
        let hdc = GetDC(desktop_hwnd);
        if hdc.is_invalid() {
            return None;
        }

        // Create a compatible DC and bitmap
        let hdc_mem = CreateCompatibleDC(hdc);
        if hdc_mem.is_invalid() {
            ReleaseDC(desktop_hwnd, hdc);
            return None;
        }

        let hbitmap = CreateCompatibleBitmap(hdc, width, height);
        if hbitmap.is_invalid() {
            DeleteDC(hdc_mem);
            ReleaseDC(desktop_hwnd, hdc);
            return None;
        }

        // Select the bitmap into the compatible DC
        let old_obj = SelectObject(hdc_mem, hbitmap);

        // Copy screen contents to our bitmap
        let result = BitBlt(
            hdc_mem, 0, 0, width, height, hdc, rect.left, rect.top, SRCCOPY,
        );

        // Create a vector to store the pixel data
        // Calculate the correct size: width * height (as usize to prevent overflow)
        let size = (width as usize) * (height as usize);
        let mut pixels = vec![0u32; size];

        // Get the bitmap data
        let bmi = windows::Win32::Graphics::Gdi::BITMAPINFO {
            bmiHeader: windows::Win32::Graphics::Gdi::BITMAPINFOHEADER {
                biSize: std::mem::size_of::<windows::Win32::Graphics::Gdi::BITMAPINFOHEADER>()
                    as u32,
                biWidth: width,
                biHeight: -height, // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0, // BI_RGB is 0
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD::default()],
        };

        let _scan_lines = windows::Win32::Graphics::Gdi::GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut std::ffi::c_void),
            &bmi as *const _ as *mut _,
            windows::Win32::Graphics::Gdi::DIB_RGB_COLORS,
        );

        // Clean up
        SelectObject(hdc_mem, old_obj);
        DeleteObject(hbitmap);
        DeleteDC(hdc_mem);
        ReleaseDC(desktop_hwnd, hdc);

        if !result.as_bool() {
            return None;
        }

        Some(ScreenCapture {
            width,
            height,
            pixels,
        })
    }
}
pub fn capture_desktop_background() -> Option<ScreenCapture> {
    unsafe {
        // Get the shell window handle (desktop background + icons)
        let shell_hwnd = GetShellWindow();
        if shell_hwnd.0 == 0 {
            return None;
        }

        // Get desktop dimensions
        let mut rect = RECT::default();
        if !GetWindowRect(shell_hwnd, &mut rect).as_bool() {
            return None;
        }
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        // Get the device context for the shell window
        let hdc = GetDC(shell_hwnd);
        if hdc.is_invalid() {
            return None;
        }

        // Create a compatible DC and bitmap
        let hdc_mem = CreateCompatibleDC(hdc);
        if hdc_mem.is_invalid() {
            ReleaseDC(shell_hwnd, hdc);
            return None;
        }

        let hbitmap = CreateCompatibleBitmap(hdc, width, height);
        if hbitmap.is_invalid() {
            DeleteDC(hdc_mem);
            ReleaseDC(shell_hwnd, hdc);
            return None;
        }

        // Select the bitmap into the compatible DC
        let old_obj = SelectObject(hdc_mem, hbitmap);

        // Copy screen contents to our bitmap
        let result = BitBlt(
            hdc_mem, 0, 0, width, height, hdc, rect.left, rect.top, SRCCOPY,
        );

        // Create a vector to store the pixel data
        let size = (width as usize) * (height as usize);
        let mut pixels = vec![0u32; size];

        // Get the bitmap data
        let bmi = windows::Win32::Graphics::Gdi::BITMAPINFO {
            bmiHeader: windows::Win32::Graphics::Gdi::BITMAPINFOHEADER {
                biSize: std::mem::size_of::<windows::Win32::Graphics::Gdi::BITMAPINFOHEADER>()
                    as u32,
                biWidth: width,
                biHeight: -height, // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0, // BI_RGB is 0
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [windows::Win32::Graphics::Gdi::RGBQUAD::default()],
        };

        let _scan_lines = windows::Win32::Graphics::Gdi::GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as *mut std::ffi::c_void),
            &bmi as *const _ as *mut _,
            windows::Win32::Graphics::Gdi::DIB_RGB_COLORS,
        );

        // Clean up
        SelectObject(hdc_mem, old_obj);
        DeleteObject(hbitmap);
        DeleteDC(hdc_mem);
        ReleaseDC(shell_hwnd, hdc);

        if !result.as_bool() {
            return None;
        }

        Some(ScreenCapture {
            width,
            height,
            pixels,
        })
    }
}
pub fn get_desktop_pixel_color(x: i32, y: i32) -> u32 {
    unsafe {
        // Get the desktop window handle directly
        let desktop_hwnd = GetDesktopWindow();
        if desktop_hwnd.0 == 0 {
            return 0;
        }

        // Get the device context for the desktop window
        let hdc = GetDC(desktop_hwnd);
        if hdc.is_invalid() {
            return 0;
        }

        // Make sure we release the DC even if getting the pixel fails
        let result = std::panic::catch_unwind(|| {
            let pixel_color = GetPixel(hdc, x, y).0;
            if pixel_color == u32::MAX {
                return Err(windows::core::Error::new(
                    windows::core::HRESULT(0),
                    "Failed to get pixel color".into(),
                ));
            }
            return Ok(pixel_color);
            // // Convert to u32 first, then perform the bitwise operations
            // let color_u32: u32 = pixel_color;

            // Ok(PixelColor {
            //     r: (color_u32 & 0xFF) as u8,
            //     g: ((color_u32 >> 8) & 0xFF) as u8,
            //     b: ((color_u32 >> 16) & 0xFF) as u8,
            // })
        });

        // Always release the DC
        ReleaseDC(desktop_hwnd, hdc);

        match result {
            Ok(Ok(color)) => color,
            Ok(Err(_e)) => 0,
            Err(_) => 0,
        }
    }
}
