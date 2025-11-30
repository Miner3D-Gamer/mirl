# [Mirl](https://github.com/Miner3D-Gamer/mirl)

[**Miners Rust Lib**](https://crates.io/crates/mirl) – A modular utility library featuring <a href="#windowing--platform" style="color: inherit; text-decoration: underline;">windowing & platform</a>, <a href="#graphics--rendering" style="color: inherit; text-decoration: underline;">2D rendering</a>, <a href="#windowing--platform" style="color: inherit; text-decoration: underline;">input handling</a>, <a href="#graphics--rendering" style="color: inherit; text-decoration: underline;">color manipulation</a>, <a href="#system-integration" style="color: inherit; text-decoration: underline;">system integration</a>, <a href="#math--geometry" style="color: inherit; text-decoration: underline;">math utilities</a>, <a href="#extensions--utilities" style="color: inherit; text-decoration: underline;">extensive type extensions</a>, _and <a href="#miscellaneous" style="color: inherit; text-decoration: underline;">more</a>_.

## Compatibility

| Platform | Status     | Info                          |
| -------- | ---------- | ----------------------------- |
| Windows  | ✅ Full    | Native implementation         |
| Linux    | ⚠️ Partial | Currently fixing              |
| macOS    | 🚧 Barely  | Untested                      |
| Web      | ⚠️ Partial | Everything but IO should work |

## How to get started (flags: `minifb`/`glfw`/`all_backends`):

MiniFB is recommended but every backend has their own unique limitations/quirks which is the reason this lib exists in the first place

```rust
use mirl::platform::framework_traits::Window;
fn main() {
    let mut buffer = mirl::platform::Buffer::new_empty((800, 600));
    let mut window = mirl::platform::{minifb/glfw}::Framework::new(
        "Example window",
        mirl::platform::WindowSettings::default(&buffer),
    ).unwrap();
    while window.is_open() {
        buffer.clear();

        // Draw here, use mirl::render for simple presets/helper functions

        window.update(&buffer);
    }
}
```

For a debugging window lib "similar" to `Dear ImGui` you can use the [`dear_mirl_gui`](https://crates.io/crates/dear_mirl_gui) crate (which is `RmMode`)

## Features/Flags

### Default (disabled with `default-features = false`)

- `texture_manager_cleanup` – Adds extra cleanup logic for 'automatic' texture unloading

## Features/Flags

### Default (disabled with `default-features = false`)

- `texture_manager_cleanup` – Adds extra cleanup logic for 'automatic' texture unloading
- `num_traits` – Enables `num-traits` dependency which is relied on by many functions and structs for modular number support

### Optional

- `imagery` – Enables support for the `image` crate for image loading
- `svg` – Enables SVG rendering via the `resvg` and `tempfile` crates (used for things like cursor support)
- `minifb` – Enables the framework backend using `minifb` and requires low-level system access
- `glfw` – Enables the framework backend using `glfw`, OpenGL, and requires low-level system access
- `font_support` – Adds support for `fontdue` and `once_cell` for font rendering
- `system` – Low-level system interaction using platform-specific crates (`x11`, `windows`, `winapi`, `raw-window-handle`)
- `all_backends` – Enables all major backends: `minifb_backend`, `glfw_backend`, `keycode_support`, and `svg`
- `all` – Enables all commonly used features: `default`, `imagery`, and `full_backend_support`
- `f128`- Enables support for 128-bit floating point numbers since they are not yet stable
- `keycodes` - Enables the ability to interact with keyboard
- `keyboard_query` - Get the currently pressed keys -> Required for MiniFB
- `cursor_show_hotspot` - A debug option for adding a red dot to the hotspot of a customly loaded cursor
- `discord` - Support for sending stuff to discord webhooks
- `parking_lot` - Enables the `parking_lot` dependency for more efficient synchronization primitives
- `ahash` - Enables the `ahash` dependency for faster HashMap implementations
- `random` - Enables random number generation support

### Other

This lib is heavily guided by clippy and as such:

- Almost everything has a short docstring
- Execution stopping functions/macros like panic! or unwrap() are only ever used in custom panic/unwrap functions

### To add:

- Clipboard support
- Sound support
- More terminal functionality
- Network support

---

## Content

### Core Types & Buffer Management

- `Buffer` - ARGB `u32` pixel buffer with manipulation operations
- Pixel-level operations (safe/unsafe accessors)
- Buffer transformations (flip, rotate, trim, resize)
- Collision detection from buffers

### Windowing & Platform

- Multi-backend window system
  - [**MiniFB**](https://crates.io/crates/minifb) (full support)
  - [**GLFW**](https://crates.io/crates/glfw) (partial support)
- Window manipulation (position, size, opacity, z-order, visibility, borderless, etc.)
- Input handling (keyboard, mouse, raw input)
- Custom cursor loading and management
- File system abstraction
- Platform-specific features (Windows focus)

### Graphics & Rendering

- **2D Rendering**
  - Text (standard, antialiased, stretched)
  - Shapes (rectangles, circles, triangles, lines)
  - Buffer-to-buffer blitting
- **Color Utilities**
  - RGB/HSL conversion and manipulation
  - Hex/ARGB conversion
  - Color interpolation and shifting
  - Brightness and saturation adjustment
- **Image Processing**
  - Multiple interpolation modes (Nearest, Linear, Cubic, Lanczos, Gaussian, etc.)
  - Image resizing and filtering
  - SVG rasterization
  - Format conversion (BMP, ICO, CUR generation)
- **Texture Management**
  - Lazy loading and caching
  - Automatic cleanup of unused textures

### System Integration

- **Window Control** (Windows-specific)
  - Window positioning and sizing
  - Minimization, maximization, restoration
  - Z-order and window level management
  - Opacity and click-through
  - Taskbar progress indicators
- **System Actions**
  - Screen capture (full screen, desktop background)
  - CPU priority control
  - Screen resolution queries

### Math & Geometry

- **Collision Detection**
  - Rectangle-rectangle intersection
  - Circle-rectangle collision
  - Point containment tests
- **Number Types**
  - `U1`, `U2`, `U4` - sub-byte unsigned integers with `num-traits` support
  - `UniformRange` - normalized [0.0, 1.0] float representation
- **Math Extensions**
  - Vector normalization and interpolation
  - Bounded type traits
  - Angle conversions (degrees/radians)
- **Positioning**
  - `Point2D` and `Point3D` - 2D and 3D point structures

### Extensions & Utilities

- **List Operations**
  - Finding differences, duplicates, regions
  - Push-or-replace for max-sized lists
  - Averaging and combining
- **String Extensions**
  - Text justification and centering
  - Tab expansion
  - Number validation
- **Cell & Tuple Operations**
  - Saturating arithmetic
  - Clamping and sign manipulation
  - Tuple-wise operations and comparisons
- **Type Conversions**
  - Tuple into conversions (const and runtime)
  - Result mapping helpers
  - Sign/unsigned mapping
  - `TryFromPatch` with Number, HashMap, BTreeMap, Vec, and String conversions

### Miscellaneous

- **Discord Integration** - Webhook payloads with embeds, components, and attachments
- **Keybinding System** - Action-based keybind handling with priority
- **Direction Types** - Cardinal and extended directions with rotation
- **Scrollable Camera** - 2D viewport with scroll bounds
- **Console Utilities** - Colored output and input handling
- **Time Constants** - Duration conversion helpers
- **Map Trait** - Unified interface for working with different map types
- **Unsafe Option Unwrapping** - `EasyUnwrapUnchecked`, used when you know that a value isn't None using `.easy_unwrap_unchecked` (Skips the unsafe {} step)

### Updating

<sup>What do the version numbers mean?</sup>

- Major: Changes have been made across the library that are almost sure to affect anyone using it
- Minor: Changes to specific areas that only may affect your use case
- Patch: No breaking API change, only fixes and additions

> Note that these rules do not apply to modules (and sub modules) named `misc`, they are collections of objects that have not yet been assigned a proper place to stay

### Hi there

What brought you to this strange place?

This is just a little big lib I built for easy function/struct/etc. reusability across my never ending stream of unfinished projects.
Even if many of the functions in here will never be used again, considering there are ~1.5k free standing functions, ~40 enums, ~60 structs, ~100 traits, ~500 trait implementations; you are sure to find _something_ of use

My philosophy follows 3 things:

- Modularity: Why put a limit on things?
- Usability What is something worth when it's unusable?
- Speed: No questions. More speed more better.

You can find the most random yet oddly specific things here.
Enjoy! Or don't, honestly...

If you use the lib in a public project, let me know; I'd genuinely love to see what other people create with the lib

---
