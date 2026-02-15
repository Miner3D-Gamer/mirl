# [Mirl](https://github.com/Miner3D-Gamer/mirl)

[**Miners Rust Lib**](https://crates.io/crates/mirl) - A modular utility library featuring [<a href="#windowing--platform" style="color: inherit; text-decoration: none;">windowing</a>](#windowing--platform), [<a href="#graphics--rendering" style="color: inherit; text-decoration: none;">2D rendering</a>](#graphics--rendering), [<a href="#windowing--platform" style="color: inherit; text-decoration: none;">input handling</a>](#windowing--platform), [<a href="#graphics--rendering" style="color: inherit; text-decoration: none;">color manipulation</a>](#graphics--rendering), [<a href="#system-integration" style="color: inherit; text-decoration: none;">system integration</a>](#system-integration), [<a href="#math--geometry" style="color: inherit; text-decoration: none;">math utilities</a>](#math--geometry), [<a href="#extensions--utilities" style="color: inherit; text-decoration: none;">extensive type extensions</a>](#extensions--utilities), _and [<a href="#miscellaneous" style="color: inherit; text-decoration: none;">more</a>](#miscellaneous)_. [<a href="#miscellaneous" style="color: inherit; text-decoration: none;">Content is listed at the <u>bottom</u></a>](#content).

<!-- HTML clean version:
[**Miners Rust Lib**](https://crates.io/crates/mirl) - A modular utility library featuring [windowing](#windowing--platform), [2D rendering](#graphics--rendering), [input handling](#windowing--platform), [color manipulation](#graphics--rendering), [system integration](#system-integration), [math utilities](#math--geometry), [extensive type extensions](#extensions--utilities), _and [more](#miscellaneous)_. [Content is listed at the <u>bottom</u>](#content)
-->

### Why would you may want to use this lib

This is a one for all library, though it theoretically started as a minifb patcher

It offers a "little bit" of content in an unreasonable amount of aspects and loves to ever self improve

## How to get started with a simple window:

> If you want to enable everything this library has to offer you can use the `all` flag

MiniFB is recommended but every backend has their own unique limitations/quirks which is the reason this lib exists in the first place

<sub>(Flags: `minifb`/`glfw`/`console`/`all_backends`)</sub>

```rust
use mirl::platform::windowing::traits::*;
fn main() {
    let mut buffer = mirl::prelude::Buffer::new_empty((800, 600));
    // More backends options can be found in `mirl::platform::windowing`
    let mut window = mirl::prelude::Framework::new(
        "Example window",
        mirl::prelude::WindowSettings::default(&buffer),
    )
    .unwrap();
    while window.is_open() {
        buffer.clear();

        // Draw here, use mirl::render for simple presets/helper functions

        window.update(&buffer);
    }
}
```

**For a debugging window lib "similar" to `Dear ImGui` you can use the [`dear_mirl_gui`](https://crates.io/crates/dear_mirl_gui) crate** (which is `RmMode`)

## Compatibility

| Platform | Status     | Info                                     |
| -------- | ---------- | ---------------------------------------- |
| Windows  | ✅ Full    | Native implementation                    |
| Linux    | ❓ Partial | Natively untested; `cargo check` passes. |
| macOS    | ❓ Partial | Natively untested; `cargo check` passes. |
| Web      | ❓ Partial | Natively untested; `cargo check` passes. |

## Features/Flags

<details open>
<summary>List of supported Flags</summary>

### Default (disabled with `default-features = false`)

- `num_traits_with_std` - Enables `num-traits` with std support, which is relied on by many functions and structs for modular number support

### Optional

#### Core Features

- `std` - Enables the standard rust library which most of the lib relies on

#### Supported libs

- `num_traits` - Enables `num-traits` support for the small u items
- `serde` - Enables `serde` support for almost all structs/enums
- `indexmap` - Enables `indexmap` support for `FromPatch` and `TryFromPatch`
- `internal_use_indexmap` - Replaces all relevant uses of `HashMap` with `indexmap::IndexMap`
- `ahash` - Does nothing
- `internal_use_ahash` - Replaces all relevant uses of `RandomState` in hashmaps to `ahash::Ahasher`

#### Graphics & Rendering

- `imagery` - Enables support for the `image` crate for image loading
- `svg` - Enables SVG rasterization via the `resvg` and `tempfile` crates (used for things like cursor support)
- `texture_manager_cleanup` - Extra cleanup functions for the texture manager (requires `std`)

#### Backend Support

- `minifb` - Enables the framework backend using `minifb` (includes `system`, `keycodes`, `keyboard_query`, `svg`, and `std`) - Not compatible with Web
- `glfw` - Enables the framework backend using `glfw` and OpenGL (includes `system`, `keycodes`, and `std`) - Not compatible with Web
- `console` - Experimental backend that displays the buffer in the console using `crossterm`
- `all_backends` - Enables all major backends: `minifb`, `glfw`, `font_support`, and `console`

#### Font Support

- `font_support` - Adds support for `fontdue` font rendering with `once_cell` for caching and `parking_lot` for synchronization (requires `std`)

#### System Integration

- `system` - Low-level system interaction using platform-specific crates (`x11`, `windows`, `winapi`, `raw-window-handle`) (requires `std`)

#### Input Handling

- `keycodes` - Enables the ability to interact with keyboard using `strum` for enum iteration (requires `std`)
- `keyboard_query` - Get the currently pressed keys using `device_query` - Required for MiniFB (requires `std`)
- `cursor_show_hotspot` - A debug option for adding a red dot to the hotspot of a custom cursor

#### External Services

- `discord` - Support for sending payloads to Discord webhooks (requires `serde`, `serde_json`, `reqwest`, and `std`)

#### Performance & Utilities

- `parking_lot` - Enables the `parking_lot` dependency for more efficient synchronization primitives
- `ahash` - Enables the `ahash` dependency for faster HashMap implementations (requires `std`)
- `random` - Enables random number generation support using `rand` (requires `std`)

#### Platform-Specific

- `mac_cc_installed` - Activate when on macOS with a C compiler installed (required for libraries like minifb to compile on macOS)

#### Development & Testing

- `miri` - Set this flag when using Miri for undefined behavior detection

#### Meta Features

- `all` - Enables all major features: `default`, `imagery`, `all_backends`, `discord`, `random`, and `console`

</details>

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
- `ConstBuffer` - Compile-time constant buffer support

### Windowing & Platform

- Multi-backend window system
  - [**MiniFB**](https://crates.io/crates/minifb) (full support)
  - [**GLFW**](https://crates.io/crates/glfw) (partial support)
  - **Console** (Uses console instead of standalone window) (Experimental)
- Window manipulation (position, size, opacity, z-order, visibility, borderless, etc.)
- Input handling (keyboard, mouse, raw input)
- Mouse snapshot support for capturing complete input state
- Custom cursor loading and management
- File system abstraction
- Platform-specific features (Windows focus)

### Graphics & Rendering

- **2D Rendering**
  - Text (standard, antialiased with alpha cutoff control, stretched)
  - Shapes (rectangles, circles, triangles, lines)
  - Buffer-to-buffer blitting with multiple interpolation modes
  - Smart drawing functions for optimized rendering
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
  - Index-based texture access
  - Optional automatic cleanup of unused textures

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
  - Rectangle-rectangle intersection (standard and strict)
  - Circle-rectangle collision
  - Point containment tests
  - Rectangle direction calculations
- **Number Types**
  - `U1`, `U2`, `U4` - sub-byte unsigned integers with `num-traits` support
  - `UniformRange` - normalized [0.0, 1.0] float representation
  - Support for `f16` and `f128` where possible
- **Math Extensions**
  - Vector normalization and interpolation
  - Bounded type traits
  - Angle conversions (degrees/radians)
  - Power-of-two utilities
  - No std math support

### Extensions & Utilities

- **List Operations**
  - Finding differences, duplicates, regions
  - Push-or-replace for max-sized lists
  - Averaging and combining
- **String Extensions**
  - Text justification and centering
  - Tab expansion
  - Number validation
- **Container (Cell, Option) & Tuple Operations**
  - Saturating arithmetic
  - Clamping and sign manipulation
  - Tuple-wise operations and comparisons
  - Round, Sqrt, Abs, Floor, and Ceil operations
- **Type Conversions**
  - Tuple into conversions (const and runtime)
  - Result mapping helpers
  - Sign/unsigned mapping
  - `TryFromPatch` with Number, HashMap, BTreeMap, Vec, and String conversions (Fixes holes like f64 from f32 and adds a whole lot more)
  - `TryIntoPatch` an automatically implementing `TryFromPatch` counterpart

### Miscellaneous

- **Discord Integration** - Webhook payloads with embeds, components, and attachments
- **Keybinding System** - Action-based keybind handling with priority
- **Direction Types** - Cardinal and extended directions with rotation and const trait support
- **Scrollable Camera** - 2D viewport with scroll bounds
- **Console Utilities** - Colored output and input handling
- **Time Constants** - Duration conversion helpers
- **Unsafe Option Unwrapping** - `EasyUnwrapUnchecked`, used when you know that a value isn't None using `.easy_unwrap_unchecked` (Skips the unsafe {} step. This trait/function violates the rust philosophy, [unlike hopefully the rest of the lib,] yet it is distinct enough from the default unwraps to make it's accidental usage highly unlikely)
- **Dependencies Module** - Reexports of all dependencies mirl uses

---

### Updating

<sup>What do the version numbers mean?</sup>

- Major: Changes have been made across the library that are almost sure to affect anyone using it
- Minor: A breaking change has been made but it is likely not be to the area of the library you're working with
- Patch: No breaking API change, only fixes and additions

<sup>
> Or in summary:

- Patch: You can update the lib without any changes to your code

- Minor: (Focusing on single area) You can probably update safely _but there is a chance of something erroring_

- Major: You will most likely have to fix something

  </sup>

> Note that these rules do not apply to modules (and sub modules) named `misc`, they are collections of objects that have not yet been assigned a proper place to stay

The reason there are so many Major updates are:

- Even the tiniest change that would affect the majority of users must be tagged as major
- Every update experimental items are added and when they get polished, the major version number must be incremented

---

### Hi there

What brought you to this strange place?

This is just a little big lib I built for easy function/struct/etc. reusability across my never ending stream of unfinished projects.
Even if many of the functions in here will never be used again, considering there are ~2.5k functions, ~35 enums, ~80 structs, ~200 traits, ~1.3 trait implementations; you are sure to find _something_ of use

My philosophy follows 3 things:

| Modularity                                             | Stability                                   | Speed                   |
| ------------------------------------------------------ | ------------------------------------------- | ----------------------- |
| Interchangeable configurations and less duplicate code | What is something worth when it's unusable? | More speed: more better |

You can find the most oddly specific things here you'd otherwise need to write yourself or you can't find anything at all.
Enjoy! Or don't, honestly...

If you use the lib in a public project, let me know; I'd genuinely love to see what other people create with the lib

---

<details>
<summary>Unimportant stats</summary>

The lib was created on the 21.12.2024.

At the time of writing it is the 1.12.2025. (I will not update this frequently)

Having existed for almost a year here are the stats (pub only):

| Type            | Amount | Average per day | Average per year | Coverage         |
| --------------- | ------ | --------------- | ---------------- | ---------------- |
| functions       | 3545   | 9.98            | 3644.85          | 60.20%           |
| implementations | 1296   | 3.65            | 1332.50          | 22.01%           |
| traits          | 205    | 0.57            | 210.77           | 3.48%            |
| enums           | 25     | 0.07            | 25.70            | 0.42%            |
| structs         | 67     | 1.88            | 68.88            | 1.13%            |
| statics         | 2      | 0.005           | 2.05             | 0.03%            |
| consts          | 155    | 0.43            | 159.36           | 2.63%            |
| macros          | 8      | 0.02            | 8.22             | 0.13%            |
| modules         | 55     | 0.15            | 56.54            | 64.45 fn per mod |

(Not displayed: Types)

</details>

---

### Warnings

While both f16 and f128 are fully supported by this lib, most linkers do not support them yet. As such it is not recommended to actually use these types
