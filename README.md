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

| Platform | Status     |
| -------- | ---------- |
| Windows  | ✅ Full    |
| Linux    | ⚠️ Partial |
| macOS    | 🚧 Barely  |
| Web      | ❓ Unknown |

## Features

### Default (disable with `default-features = false`)

- `texture_manager_cleanup` – Adds extra cleanup logic for 'automatic' texture unloading

### Optional

- `imagery` – Enables support for the `image` crate for image loading
- `svg_support` – Enables support for the `resvg` crate (used for SVG rendering, e.g., cursors in framework integration)
- `wayland` – Placeholder for Wayland support (not yet implemented)
- `minifb_backend` – Enables the framework backend using `minifb` and low-level system access
- `glfw_backend` – Enables the framework backend using `glfw`, OpenGL, and low-level system access
- `system` – Low-level system interaction using platform-specific crates (`x11`, `windows`, `winapi`)
- `full_backend_support` – Enables both `minifb_backend` and `glfw_backend` along with `svg_support`
- `all` – Enables all major features including `default`, `imagery`, and `full_backend_support`

---

### Hi there

What brought you to this place?

This is just a little big lib I built for easy function/struct reusability.

You can find the most random yet oddly specific things here.
