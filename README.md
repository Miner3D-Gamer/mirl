# Mirl

**Miners Rust Lib** – A modular utility library for math, graphics, system-level functions, and more. A little bit of everything:

## Content

### Math and Geometry

- Core math utilities and extensions
- Basic collision detection
- U1, U2, U4 integer types with `num-traits` support

### Graphics and Rendering

- Color manipulation (ARGB `u32`)
- Buffer rendering for text, lines, triangles, blocks, circles, images, outlines
- Console rendering utilities
- Support for generating .bmp, .ico, and .cur

### Windowing System

- Modular rendering backend
  - **MiniFB** (full support)
  - **GLFW** (partial support)
- Input handling (mouse, keyboard)
- Cursor and icon management (Custom cursors)
- Window manipulation (position, size, opacity, z-order, visibility, etc.)

### Miscellaneous

- Discord webhook integration
- Rust extensions (`.is_number()`, `.sign()`, `.average()`, tuple comparison)
- Screenshot utilities
- Simple keybinding system
- etc...

## Compatibility

| Platform | Status     | Info                                                |
| -------- | ---------- | --------------------------------------------------- |
| Windows  | ✅ Full    | Native implementation                               |
| Linux    | ⚠️ Partial | Currently fixing                                    |
| macOS    | 🚧 Barely  | Untested                                            |
| Web      | ⚠️ Partial | GLFW and MiniFB backends are automatically disabled |

## Features/Flags

### Default (disabled with `default-features = false`)

- `texture_manager_cleanup` – Adds extra cleanup logic for 'automatic' texture unloading

### Optional

- `imagery` – Enables support for the `image` crate for image loading
- `svg_support` – Enables SVG rendering via the `resvg` and `tempfile` crates (used for things like cursor support)
- `wayland` – Placeholder for Linux Wayland support (not yet implemented)
- `minifb_backend` – Enables the framework backend using `minifb` and requires low-level system access
- `glfw_backend` – Enables the framework backend using `glfw`, OpenGL, and requires low-level system access
- `font_support` – Adds support for `fontdue` and `once_cell` for font rendering
- `system` – Low-level system interaction using platform-specific crates (`x11`, `windows`, `winapi`, `raw-window-handle`)
- `full_backend_support` – Enables all major backends: `minifb_backend`, `glfw_backend`, `keycode_support`, and `svg_support`
- `all` – Enables all commonly used features: `default`, `imagery`, and `full_backend_support`
- `f128`- Enables support for 128-bit floating point numbers since they are not yet stable
- `keycode_support` - Enables the ability to interact with keyboard
- `do_not_compile_misc` - Stops the experimental misc module from compiling
- `keyboard_query` - Get the currently pressed keys -> Required for MiniFB
- `cursor_show_hotspot` - A debug option for adding a red dot to the hotspot of a customly loaded cursor
- `discord_support` - Support for sending stuff to discord webhooks

### Other

This lib is heavily guided by clippy and as such:
- Almost everything has a short docstring
- Execution stopping functions/macros like panic! or unwrap() are only ever used in custom panic/unwrap functions

---

### Hi there

What brought you to this strange place?

While a lot of the lib is stable and won't be touched again by me again, in the name of speed I will not hesitate to improve what already exists.
I believe my goal is to create so many functions/structs/etc., that just work no matter what you throw at them, until I'm able to write entire projects in just a few lines of code.

This is just a little big lib I built for easy function/struct/etc. reusability across my never ending stream of unfinished projects.
Even if most of the functions in here will never be used again, considering there are ~3k functions, ~50 enums, ~100 structs, >200 traits, >1k trait implementations; you are sure to find _something_

You can find the most random yet oddly specific things here.
Enjoy! Or don't, honestly...
