# Mirl

**Miners Rust Lib** – A library with a little bit of everything:

- Math functions
- Graphics / color manipulation
- Buffer rendering (Text, Lines, Triangles, Blocks, Circles, Images)
- Console functions
- Dynamic window/rendering system
  - Supported: MiniFB (Full), GLFW (Mostly)
- Screenshots and other window manipulations
- Rust extensions (e.g., `.is_number()`, `.sign()`, `.average()`, comparing tuples – functionality that _should_ be built-in)
- U1, U2, U4 with num-traits support (More to come)

## Compatibility

| Platform | Status     | Info                                                |
| -------- | ---------- | --------------------------------------------------- |
| Windows  | ✅ Full    | Native implementation                               |
| Linux    | ⚠️ Partial | `system` flag not supported                         |
| macOS    | 🚧 Barely  | Untested                                            |
| Web      | ⚠️ Partial | GLFW and MiniFB backends are automatically disabled |

## Features

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
- `full_backend_support` – Enables all major backends: `minifb_backend`, `glfw_backend`, `svg_support`, and `ico_support`
- `all` – Enables all commonly used features: `default`, `imagery`, and `full_backend_support`

---

### Hi there

What brought you to this place?

This is just a little big lib I built for easy function/struct reusability.

You can find the most random yet oddly specific things here.
Enjoy! Or don't, honestly...
