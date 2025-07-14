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

- `system` – Low-level OS interaction (required by `platform`)
- `platform` – Native window creation and management
- `texture_manager_cleanup` – Adds cleanup logic for 'automatic' texture unloading

### Optional

- `imagery` – Enables support for the bulky `image` crate
- `svg_support` – Enables support for `resvg` crate, required by cursor support in framework integration

---

### Hi there

What brought you to this place?

This is just a little big lib I built for easy function/struct reusability.

You can find the most random yet oddly specific things here.
