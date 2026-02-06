# 9.0.1

Added `Unwrap` and `UnwrapDefault` to `mirl::extensions`

This trait is automatically implemented for `Option` but will be extended in the future to `Result` and the still in progress `mirl::misc::FoundReturn`

# [Changelog](#changelog-overview)/Migration Guide v9.0.0

Disclaimer: `mirl::math::geometry` is not in a state that I like. Expect changes to the api there before the next major update.

## Quick Start - Most Common Changes

If you need to get your code compiling quickly, these are the changes that affect most projects:

```rust
// 1. Buffer import location
use mirl::platform::Buffer;  // ❌ Old
use mirl::render::Buffer;    // ✅ New

// 2. Buffer constructor
Buffer::new(w, h, data);          // ❌ Old
Buffer::new_with_data(w, h, data); // ✅ New

// 3. Collision functions
buffer.create_collision(x, y, w, h);     // ❌ Old
buffer.to_collision((x, y), (w, h));     // ✅ New

// 5. Draw buffer functions (names swapped!)
draw_buffer_on_buffer_1_to_1(...)  // ❌ Old (1:1 pixel copy)
draw_buffer_on_buffer(...)         // ✅ New (1:1 pixel copy)

draw_buffer_on_buffer(...)         // ❌ Old (stretched)
draw_buffer_on_buffer_stretched(...) // ✅ New (stretched)
```

For complete migration details, continue reading below.

---

## Table of Contents

1. [Critical Breaking Changes](#critical-breaking-changes) - Start here: Buffer relocation, constructor changes
2. [Module & Import Changes](#module--import-changes) - Namespace reorganization and new module locations
3. [Trait Reorganization](#trait-reorganization) - Trait renames, splits, and new patterns
4. [Function Signature Changes](#function-signature-changes) - Parameter changes and return type updates
5. [Type & Constant Changes](#type--constant-changes) - Type renames and constant updates
6. [New Features](#new-features) - Exciting additions you can start using
7. [Additional Notes](#additional-notes) - Performance, debugging, and quality improvements
8. [Migration Checklist](#migration-checklist) - Step-by-step migration tracking

---

## Critical Breaking Changes

These changes will likely affect most projects and should be addressed first.

### 1. Buffer Module Relocation

**Impact: HIGH** - Affects all code using `Buffer`

`Buffer` has moved from `mirl::platform` to `mirl::render` (but it is recommended to use `mirl::prelude`) to better reflect its primary use case.

```rust
// Before
use mirl::platform::Buffer;
use mirl::Buffer;

// After
use mirl::render::Buffer;
```

**Why this change?** The buffer is primarily a rendering construct, not a platform-specific one.

### 2. Collision Functions: Rename

Two changes in one: renamed functions AND changed to tuple-based parameters.

```rust
// Before
let collision = buffer.create_collision_usize(x, y, width, height);
let collision = buffer.create_collision(x, y, width, height);

// After
let collision = buffer.to_collision_usize((x, y), (width, height));
let collision = buffer.to_collision((x, y), (width, height));
```

---

## Module & Import Changes

### Namespace Reorganization

#### Math Module

The collision module has been reorganized into a layered geometry system.

```rust
// Before
use mirl::math::collision::Rectangle;
use mirl::math::positioning::*;

// After
use mirl::math::geometry::d2::Rectangle;
// positioning module removed - use tuples directly
// mirl's tuple math provides equivalent functionality
```

**What happened to positioning?** Tuples with mirl's mathematical extensions now handle positioning more elegantly than a dedicated module.

#### Time Constants

```rust
// Before
use mirl::time::{SECONDS_IN_MINUTE, MINUTES_IN_HOUR};

// After
use mirl::constants::time::{SECONDS_IN_MINUTE, MINUTES_IN_HOUR};
```

#### Text Positioning

```rust
// Before
use mirl::misc::TextPos;

// After
use mirl::text::position::TextPos;
```

**Bonus:** New utilities added alongside `TextPos` in the text module.

#### Dependencies

```rust
// Before
use mirl::prelude::fontdue;

// After
use mirl::dependencies::fontdue;
// All dependencies now in dedicated module
```

### Feature Flag Changes

- `num_traits` is no longer a feature flag
- `texture_manager_cleanup` removed from `default` features

---

## Trait Reorganization

### Windowing Traits

#### Framework Traits

Simple rename for clarity.

```rust
// Before
use mirl::platform::framework_traits::{Framework, ExtendedFramework};

struct MyFramework;
impl Framework for MyFramework { ... }
impl ExtendedFramework for MyFramework { ... }

// After
use mirl::platform::windowing::traits::{WindowingFramework, ExtendedWindowingFramework};

struct MyFramework;
impl WindowingFramework for MyFramework { ... }
impl ExtendedWindowingFramework for MyFramework { ... }
```

**Note:** `WindowingFramework` now requires `Debug` - ensure your implementations derive or implement it.

#### Control Traits - Trait Splitting

`ExtendedControl` has been split into two separate traits for better granularity.

```rust
// Before
use mirl::platform::windowing::traits::ExtendedControl;

impl ExtendedControl for MyControl {
    fn new(...) -> Self { ... }
    fn get_window_handle(&self) -> ... { ... }
}

// After
use mirl::platform::windowing::traits::{NewWindow, GetWindowHandle};

impl NewWindow for MyControl {
    fn new(...) -> Self { ... }
}

impl GetWindowHandle for MyControl {
    fn get_window_handle(&self) -> ... { ... }
}
```

**Why split?** Allows implementations to opt into only the functionality they need and GetWindowHandle is dyn compatible now.

#### Window Helper Trait - Function Rename

The function has been renamed back to the simpler `update`.

```rust
// Before
window_helper.update_with_buffer(&buffer);

// After
window_helper.update(&buffer);
```

**Migration tip:** Global find-and-replace `update_with_buffer` → `update`

### Direction Traits

```rust
// Before
use mirl::directions::NormalDirections;
if direction.is_direction_allowed_u8() { ... }

// After
use mirl::directions::{NormalDirections, IsDirectionTrue};
if direction.is_direction_true() { ... }
```

**Technical note:** Direction traits have been split and many are now `const`, enabling compile-time direction validation.

### Conversion Traits - FromPatch Introduction

Safe conversions extracted from `TryFromPatch` into new `FromPatch` trait (`TryFromPatch` automatically derives `FromPatch`).

```rust
use mirl::extensions::{FromPatch, TryFromPatch};

// Safe conversion (infallible)
let value: u32 = u32::from_patch(some_u8);
let value = some_u8.into_patch(); // Also works

// Fallible conversion (can fail)
let value: u32 = u32::try_from_patch(some_i32)?;
let value = some_i32.try_into_patch()?; // Also works
```

**Key difference:** `FromPatch` for conversions that always succeed, `TryFromPatch` for conversions that might fail.

---

## Function Signature Changes

### Tuple-Based Parameters (Consistency Update)

#### Buffer Resizing

```rust
// Before
buffer.resize_content(new_x, new_y);

// After
buffer.resize_content((new_x, new_y));
```

### Drawing Functions

#### Draw Buffer Naming Clarification

**Impact: MEDIUM-HIGH** - Affects all buffer composition code

Functions renamed to better indicate their behavior.

```rust
// Before
draw_buffer_on_buffer(target, source, ...);        // Was 1:1 pixel mapping
draw_buffer_on_buffer_1_to_1(target, source, ...); // Also 1:1 pixel mapping

// After
draw_buffer_on_buffer(target, source, ...);          // 1:1 pixel mapping
draw_buffer_on_buffer_stretched(target, source, ...); // Scaling/stretching
```

**Quick reference:**

- `draw_buffer_on_buffer` - Direct pixel copy (no scaling)
- `draw_buffer_on_buffer_stretched` - Scaled rendering with interpolation

### Text Rendering

#### Antialiasing Control

More granular control over text antialiasing with alpha cutoff values.

```rust
// Before
draw_text_switch(buffer, text, antialiased: true, ...);
draw_text_switch(buffer, text, antialiased: false, ...);

// After
draw_text_switch(buffer, text, antialiased: Some(128), ...); // Antialiased with cutoff
draw_text_switch(buffer, text, antialiased: None, ...);      // Aliased
```

### Mouse Input

#### Position Type Change

Mouse positions now use `f32` for sub-pixel precision.

```rust
// Before
let pos: Option<(i32, i32)> = window.get_mouse_position();
if let Some((x, y)) = pos {
    println!("Mouse at {}, {}", x, y);
}

// After
let pos: Option<(f32, f32)> = window.get_mouse_position();
if let Some((x, y)) = pos {
    println!("Mouse at {:.2}, {:.2}", x, y);
    // Round if you need integer coordinates:
    let (x_int, y_int) = (x.round() as i32, y.round() as i32);
}
```

#### Scroll Return Type

```rust
// Before
let scroll: Option<(f32, f32)> = window.get_mouse_scroll();
if let Some((dx, dy)) = scroll {
    // Handle scroll
}

// After
let (dx, dy): (f32, f32) = window.get_mouse_scroll();
// Always returns a value, (0.0, 0.0) when no scroll occurred
if dx != 0.0 || dy != 0.0 {
    // Handle scroll
}
```

### Color Interpolation

#### Unified Generic Function

**Impact: LOW** - Simplification

```rust
// Before
interpolate_color_rgb_u32_f32(color1, color2, t_f32);
interpolate_color_rgb_u32_f64(color1, color2, t_f64);

// After
interpolate_color_rgb_u32(color1, color2, t); // Works with f32, f64, f16, f128
```

The compiler infers the float type automatically.

---

## Type & Constant Changes

### Math Constants

```rust
// Before
use mirl::math::ConstPartialOrd; // (nightly-only workaround)
use mirl::math::TwoTillTen;

// After
// ConstPartialOrd removed (use nightly feature directly)
use mirl::math::ConstNumbers128; // Now supports 0-127
```

### Filesystem

```rust
// Before
use mirl::platform::filesystem::FileSystem;

// After
use mirl::platform::filesystem::FileSystemTrait;
```

---

## New Features

### Mouse Snapshot - Unified Mouse State

Capture complete mouse state in a single call for consistent frame data.

```rust
use mirl::platform::mouse::MouseSnapshot;
use mirl::platform::windowing::traits::WindowInputHelper;

// Old way - multiple calls
let pos = window.get_mouse_position();
let scroll = window.get_mouse_scroll();
let button1 = window.get_mouse_button(MouseButton::Left);

// New way - single snapshot
let snapshot = window.get_mouse_snapshot();
// All state captured in one call
```

### Console Backend (Experimental-Alpha)

Display your buffer in the terminal instead of a window.

This is not yet recommended but it is available.

### Rectangle Operations

#### Strict Intersection

New precise intersection check.

```rust
// Existing (inclusive)
if rect1.do_areas_intersect(&rect2) { ... }

// New (strict - edges don't count)
if rect1.do_areas_intersect_strict(&rect2) { ... }
```

### TextureManager Enhancements

#### Explicit Lazy Loading

Better control over when textures are loaded.

```rust
// Before: get() would load if not present (implicit)
let texture = manager.get("texture_key"); // Might load, might not

// After: explicit behavior
let texture = manager.get("texture_key");        // Errors if not loaded
let texture = manager.get_or_load("texture_key"); // Loads if needed

// Migration: Replace old get() calls with get_or_load() for same behavior (When using `imaginary` feature)
```

### Mathematical Extensions - Comprehensive Math Traits (Also no_std replacements)

#### Basic Math Operations

```rust
use mirl::extensions::{Round, Sqrt, Abs, Floor, Ceil};

let rounded = value.round();
let root = value.sqrt();
let absolute = value.abs();
```

#### Tuple Math Operations

Apply operations to entire tuples at once.

```rust
use mirl::extensions::{TupleRound, TupleSqrt, TupleAbs, TupleFloor, TupleCeil};

let position = (3.7, 4.2);
let rounded_pos = position.tuple_round(); // (4.0, 4.0)

let vector = (-3.5, 7.8);
let abs_vector = vector.tuple_abs(); // (3.5, 7.8)
```

#### Power of Two Utilities

```rust
use mirl::extensions::{NextPowerOfTwo, NextPowerOfTwoWithExponent, NextPowerOfTwoExponent};

let size = 100;
let next_pow = size.next_power_of_two(); // 128
let (pow, exp) = size.next_power_of_two_with_exponent(); // (128, 7)
let exp = size.next_power_of_two_exponent(); // 7
```

### Extended Float Support

- Added `f16` support throughout the library
- Existing `f128` support expanded
- Most mathematical operations now work with `f16`, `f32`, `f64`, and `f128`

### Enhanced Number Constants

```rust
// Before
use mirl::math::TwoTillTen; // Only 2-10

// After
use mirl::math::ConstNumbers128; // Constants 0-127
```

### Platform Module Improvements

`mirl::platform::mouse` now available without the `system` feature flag (individual items flagged as needed).

---

## Changelog overview

### Standard Library Independence

More of the library now works without the `std` feature flag.

**What changed:**

- Core replaces std functionality wherever possible
- Implemented a lot of math traits for numbers

### Float Type Support Expansion

MIRL now supports a wider range of float types.

**Supported float types:**

- `f16` - Half precision (NEW)
- `f32` - Single precision
- `f64` - Double precision
- `f128` - Quadruple precision (expanded support, reminder that your native compiler itself might not support `f128`)

### Debug Requirement for WindowingFramework

Custom windowing framework implementations must now implement `Debug`.

```rust
// Quick fix: Just derive it
#[derive(Debug)]
struct MyFramework {
    // ...
}

// Or implement manually if needed
impl Debug for MyFramework {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MyFramework")
            .field("complex_value", &"<Unknown>")
            .finish()
    }
}
```

### Derive Improvements

Many structs and enums gained additional trait implementations.

**Commonly added derives:**

- `Ord` and `PartialOrd` - Enables comparison and sorting
- `Hash` - Enables use as HashMap/HashSet keys
- `Default` - Provides sensible default values

**Benefit:** More types work out-of-the-box with standard Rust patterns and collections.

### Const Support Expansion

More operations can now be performed at compile time.

**What's new:**

- Many trait implementations marked as `impl const`
- Direction validation can happen at compile time
- More mathematical operations in const contexts

### Performance Improvements

#### Fixed TRANSPARENCY_INTERPOLATION

Transparency blending in `draw_buffer_on_buffer` now works correctly, producing more accurate visual results.

#### Inlining Optimizations

Various functions marked for inlining, improving performance in hot paths. The compiler can now better optimize drawing and mathematical operations.

### Feature Flag Changes

#### Removal of num_traits

**What this means:**

- These traits have been reimplemented to support `f16` and `128` as well as give extended functionality
- Improvements regarding `const` trait definition and implementations

### ConstBuffer Improvements

`mirl::render::ConstBuffer` is now much more functional.

**What changed:**

- Fixed existing function implementations
- Almost all `Buffer` functions now available
- Can be used as a drop-in replacement for cases where speed is more important than configurability

**When to use:** When you have fixed sized buffers and want compile-time speed improvements.

### Cargo Version Requirement

**Updated to cargo 1.94.0**

Ensure your development environment is up to date:

```bash
rustup update
cargo --version  # Should be 1.94.0 or later
```

---

## Questions or Issues?

If you encounter migration issues not covered in this guide, please:

1. Check the raw changelog for additional context
2. Report issues on the project's issue tracker

---

# Version 8.0.0

> Enhanced cross-platform support

## Breaking Changes:

### Big changes:

- Moved multiple items to new modules:
  | Item | Old Location | New Location |
  |-|-|-|
  | `minifb` | `mirl::platform::minifb` | `mirl::platform::frameworks::minifb` |
  | `glfw` | `mirl::platform::glfw` | `mirl::platform::frameworks::glfw` |
  | `framework_traits` | `mirl::platform::framework_traits` | `mirl::platform::frameworks::traits` |

- Split `mirl::platform::framework_traits::Window::update` into two functions:
  - `update_raw` - Update without buffer
  - `update_with_buffer` - Update with buffer data

- Renamed methods:
  | Old Name | New Name |
  |-|-|
  | `WindowSettings::set_position_to_middle_of_screen` | `WindowSettings::center_window` |
  |`mirl::platform::frameworks::Errors`|`mirl::platform::frameworks::WindowError`|

### Minor changes:

- `mirl::platform::framework_traits::Errors` changes:
  - Removed `AllGood` - Functions now return `Result<(), Errors>` instead
  - Removed `Unknown` - Functions now return `Errors::Misc` with description instead
- Removed `get_delta_time` from `mirl::platform::frameworks::Timing`
- Removed `native_time` field from all frameworks
- `mirl::constants::bytes` constants are now `u128` instead of a mix of `usize` and `u128`

## Migration Guide

- **Update module imports:**
  - `mirl::platform::minifb` → `mirl::platform::frameworks::minifb`
  - `mirl::platform::glfw` → `mirl::platform::frameworks::glfw`
  - `mirl::platform::framework_traits` → `mirl::platform::frameworks::traits`

  Note: Old paths are temporarily re-exported with deprecation warnings for easier migration

- **Update Window trait usage:**
  - Replace `window.update()` with `window.update_raw()` or `window.update_with_buffer()`
  - Use `mirl::platform::frameworks::traits::DeprecatedCompatibilityHelper` for temporary compatibility

- **Update error handling:**
  - Replace `Errors::AllGood` checks with `Result<(), Errors>` pattern matching
  - Replace `Errors::Unknown` with `Errors::Misc`
  - Use new `WindowCreationError` and `WindowUpdateError` enums for explicit error types

- **Update trait implementations:**
  - Split `set_icon` from `ExtendedWindow` into `MouseInput`
  - Update `Input` implementations to use `MouseInput` and `KeyboardInput`
  - Update `ExtendedInput` implementations to use `ExtendedMouseInput` and `ExtendedKeyboardInput`
  - Split `CursorStyleControl` into `LoadCursorStyle` and `UseCursorStyle`
  - Split `ExtendedControl` into `Visibility` and `RenderLayer`

- **Update WindowSettings:**
  - Replace `set_position_to_middle_of_screen()` with `center_window()`

- **Update timing code:**
  - Remove `get_delta_time()` from `mirl::platform::frameworks::Timing`
  - Remove `native_time` field for both backends

## Added:

- `WindowSettings::fullscreen` - Set window to fullscreen mode
- Implemented `std::error::Error` to `mirl::platform::frameworks::WindowError`
- `mirl::platform::frameworks::traits::DeprecatedCompatibilityHelper` - Temporary helper for migration with `update` function
- `WindowCreationError` enum for explicit window creation error handling
- `WindowUpdateError` enum for explicit window update error handling
- `Rectangle::get_pos` - Get position helper function
- `Rectangle::get_size` - Get size helper function
- `mirl::compile_time_dependency_errors` - Formal compile time errors for unusual flag combinations
- Platform compilation support:
  - `wasm32-unknown-unknown` - Compiles with some missing features like IO and framework options, almost no functions implemented for web in `mirl::system::Os`
  - `x86_64-apple-darwin` / `aarch64-apple-darwin` - Compiles with MiniFB disabled, no functions implemented for mac in `mirl::system::Os`

## Changes:

- Made all error conversion functions public in `mirl::platform::minifb`

## Misc:

- Improved test_flags.py performance (2x faster) so expect more configurability in the future
- Updated source code formatting settings

---

# Version 7.1.2

Fixed some oversights which prevented some flag combinations from compiling.

---

# Version 7.1.1

## Changes

- `mirl::platform::file_system::native::NativeFileSystem::{get_files_in_folder, get_folders_in_folders}` now searches paths in this order until an element is found:
  1. Provided path
  2. Executable path
  3. Current working directory
  4. `src` directory

- `mirl::platform::WindowSettings` is no longer gated behind the `system` feature.

- When the `font_support` feature is active, `fontdue` is re-exported through `mirl::prelude`.

- `mirl::platform::WindowSettings` is now included in `mirl::prelude`.

---

# Version 7.1.0

> Fewer flag restrictions and smarter dependency flagging

## Breaking Changes:

- Moved items to new modules:
  | Item | Old Location | New Location |
  |-|-|-|
  | List functions | `mirl::extensions::list` | mirl::extensions::lists::helper_functions` |

- `mirl::platform::mouse` now compiles even when the `keycodes` and `svg` features are disabled
- `mirl::platform::framework_traits` now compiles even when the `keycodes` feature is disabled

## Migration Guide

- **Update module imports:**
  - `mirl::extensions::list::*` → `mirl::extensions::lists::helper_functions::*`

- **Removed:**
  - `mirl::platform::mouse::cursor_resolution` - No longer needed
  - `mirl::platform::mouse::resolution_to_quality` - No longer needed

## Changes:

- Disabled optional/default features from dependencies for faster compile times

---

# Version 7.0.0

> This is another step towards stabilizing the library for future releases

## Breaking Changes:

### Big changes:

- Moved multiple items to new modules:
  | Item | Old Location | New Location |
  |-|-|-|
  | `corner_type_to_cursor_style` | `mirl::misc` | `mirl::directions::misc` |
  | `corner_type_and_delta_to_metric_change` | `mirl::misc` | `mirl::directions::misc` |
  | `NormalDirections` | `mirl::misc` | `mirl::directions` |
  | `invert_color_below` | `mirl::misc` | `Buffer::invert_color_below` |
  | `invert_color_if_same` | `mirl::misc` | `Buffer::invert_color_if_same` |
  | `fade_out_edges` | `mirl::misc` | `Buffer::fade_out_edges` |
  | `Point2D` | `mirl::misc` | `mirl::math::positioning` |
  | `Point3D` | `mirl::misc` | `mirl::math::positioning` |
  | `discord` | `mirl::misc` | `mirl::network` |

- `mirl::platform::framework_traits::CursorStyleControl` now uses `mirl::platform::mouse::CursorResolution` for cursor resolution instead of `mirl::extensions::u2::U2`

### Minor changes:

- `Buffer::get_pixel_isize` now returns 0 when out of bounds
- `mirl::platform::file_system::native::NativeFileSystem` will now also scan the src folder for files/folders
- Dependencies are now optional:
  - `parking_lot` is now optional and only imported when needed
  - `ahash` is now optional and replaces `std::collections::HashMap` under the `ahash` flag
  - `num-traits` is now optional and enabled by default using the `num_traits` flag

## Migration Guide

- **Update module imports:**
  - `mirl::misc::corner_type_to_cursor_style` → `mirl::directions::misc::corner_type_to_cursor_style`
  - `mirl::misc::corner_type_and_delta_to_metric_change` → `mirl::directions::misc::corner_type_and_delta_to_metric_change`
  - `mirl::misc::NormalDirections` → `mirl::directions::NormalDirections`
  - `mirl::misc::invert_color_below` → `Buffer::invert_color_below`
  - `mirl::misc::invert_color_if_same` → `Buffer::invert_color_if_same`
  - `mirl::misc::fade_out_edges` → `Buffer::fade_out_edges`
  - `mirl::misc::Point2D` → `mirl::math::positioning::Point2D`
  - `mirl::misc::Point3D` → `mirl::math::positioning::Point3D`
  - `mirl::misc::discord` → `mirl::network::discord`

- **Update cursor style control:**
  - Replace `mirl::extensions::u2::U2` with `mirl::platform::mouse::CursorResolution` for cursor resolution

- **Update feature flags:**
  - Enable `ahash` flag to use ahash-based HashMaps
  - `num_traits` flag is enabled by default

- **Removed:**
  - `do_not_compile_misc` flag
  - `f128` flag (Now always supported)

## Added:

- `Buffer::get_pixel_option` - Get pixel with `Option` return type
- `Buffer::get_pixel_isize_option` - Get pixel at signed coordinates with `Option` return type
- `Map` trait to `mirl::misc` - Unifies interaction with underlying map types
- `line_and_column_from_offset` to `mirl::misc` - Convert text offset to line/column position
- `mirl::misc::TextPosition` methods:
  - `advance_char` - Advance position by one character
  - `advance_char_by` - Advance position by N characters
  - `advance_line` - Advance position by one line
  - `advance_line_by` - Advance position by N lines
- `mirl::misc::EasyUnwrapUnchecked` trait for `Option<T>` - Unsafe unwrap for known non-None values
- Enhanced `mirl::extensions::TryFromPatch` support:
  - `HashMap<K1, V1>` to `HashMap<K2, V2>` conversion
  - `BTreeMap<K1, V1>` to `BTreeMap<K2, V2>` conversion
  - String to Number conversion
  - `Vec<T1>` to `Vec<T2>` conversion
- Added `mirl::settings` for more global crate settings in the future

## Changes:

- Certain flag/feature combinations will no longer error - Objects relying on disabled objects are now also properly disabled

---

# Version 6.0.0:

> This is an attempt of stabilizing the library for future releases.

## Breaking Changes:

### Big changes:

- Renamed traits (for better error handling):
  | Old Name | New Name |
  |-|-|
  | FromPatch | TryFromPatch |
  | TupleInto | TryTupleInto |
  | ConstTupleInto | ConstTryTupleInto |
- Some `FileData` functions are now prepended `to_` instead of `as_`
- `Buffer::create_collision` now returns `Option<mirl::math::collision::Rectangle>` instead of `mirl::math::collision::Rectangle`, None is returned when `T` cannot hold the usize size of the buffer
- Renamed flags:
  | Old Flag Name | New Flag Name |
  |-|-|
  | svg_support | svg |
  | full_backend_support | all_backends |
  | minifb_backend | minifb |
  | glfw_backend | glfw |
  | discord_support | discord |
  | random_support | random |
  | keycode_support | keycodes |

### Minor changes:

- `pixmap_to_dynamic_image` now returns `Option<image::DynamicImage>` instead of `image::DynamicImage`
- `RangeExtension` trait now returns `Option<T>` instead of `T`
- `radians` and `degrees` inside `mirl::math` now return `Option<T>` instead of `T`
- Loading cursors now returns `std::result::Result<Cursor, LoadCursorError>` instead of `std::result::Result<Cursor, String>`
- `mirl::platform::framework_traits` now uses i32 instead of isize
- Removed unused `wayland` flag

## Migration Guide

- **Update traits:**
  - Replace `FromPatch` → `TryFromPatch`
  - Replace `TupleInto` → `TryTupleInto`
  - Replace `ConstTupleInto` → `ConstTryTupleInto`

- **Update `FileData` methods:**
  - Functions previously prefixed with `as_` may now be `to_`

- **Adjust for optional returns:**
  - `Buffer::create_collision` now returns `Option<Rectangle>`
  - `pixmap_to_dynamic_image` now returns `Option<DynamicImage>`
  - `RangeExtension` methods now return `Option<T>`
  - `mirl::math::degrees` methods now return `Option<T>`
  - `mirl::math::radians` methods now return `Option<T>`

- **Cursor loading:**
  - Now returns `Result<Cursor, LoadCursorError>` instead of `Result<Cursor, String>`

- **Platform trait updates:**
  - `mirl::platform::framework_traits` now uses `i32` instead of `isize`

- **Update feature flags:**
  - `svg_support` → `svg`
  - `full_backend_support` → `all_backends`
  - `minifb_backend` → `minifb`
  - `glfw_backend` → `glfw`
  - `discord_support` → `discord`
  - `random_support` → `random`
  - `keycode_support` → `keycodes`

- **Removed:**
  - `wayland` flag

## Added:

- Experimental 'std' flag, when disabled 90% of the library does so as well to be able to compile

---

# Version 5.1.0:

Added:

- RepeatDataInContainer trait inside mirl::extensions, examples: Box<T> -> Box<Vec<T>>, Option<T> -> Option<Vec<T>>
- mirl::prelude with some frequently used and useful stuff
- ListGetNewItemsCloned trait inside mirl::extensions that returns Vec<T> instead of Vec<&T>

Changed:

- File System trait moved to own file
- File System trait no longer asks for a list of required files
- Renamed mirl::graphics::color_presets to mirl::graphics::colors
- Loosened version requirements for serde and serde_json

---

# Version 5.0.0:

Added:

- mirl::extensions::RangeExtension for additional x..y helper functions
- 'random_support' flag for generating a random color under mirl::graphics::random
- ScrollableCamera in mirl::misc

Changed:

- Renamed FromPatch::from_number to FromPatch::from_value
- Creating a new mirl::platform::Buffer now takes in a size tuple instead of two separate variables

Misc:

- Un-note worthy changes across the lib

---

# Version 4.7.1:

Removed:

- tuple ops after 12 as they aren't even supported by the default std

---

# Version 4.7.0:

Added:

- Custom from (mirl::extensions::FromPatch) to patch some gabs in the from provided by std, especially in the number-type area
- Added const tuple conversion functions

Changes:

- Rewrote tuple conversion which should be faster now
- Made buffer collision creation more dynamic
- Tuple conversion is no longer tuple\_{num}\_into() but instead tuple_into() for a tuple of any size up to twelve items

Removed:

- Got rid of the 'do_not_compile_extension_tuple_support' flag since the new impl isn't as long

---

# Version 4.6.0:

Added:

- Ability to set the size of a window
- Moved functions related to Iconifying from the awkward mirl::platform::shared to the Os struct in mirl::system

Removed:

- MouseManagerScrollAccuracy from all framework traits as it was annoying. f32 is enough accuracy for the scroll wheel.

---

# Version 4.5.0:

Added

- Added primitive collision detection between collision types
- Added `mirl::misc::fade_out_buffer` for fading out the edges of a buffer
- Fixed the `system` flag preventing Linux users from compiling (full functionality in the workings)

Changed

- Removed the `PURE_` prefix from color presets
- Deprecated `system/info`; its features will be merged into `system/actions`, forming a unified `system` section
- Fixed rectangles with invalid input values causing crashes when the `SAFE` flag is set
- Overhauled `README.md` again in the hopes to improve clarity and intuitiveness

---

# Version 4.4.1:

Changes:

- Loosened dependencies

---

# Version 4.4.0:

Added

- Added support for retrieving the hitbox of a window (window size + OS padding) [Windows]
- Added the ability to set the CPU priority of a running process [Windows]
- Added functions to set color channels of a `u32`: `with_alpha`, `with_red`, `with_blue`, and `with_green`
- Added utility functions for sending data to Discord webhooks
- Added functions for converting a buffer to `.bmp` and `.ico`, and made the `.cur` conversion function public in `mirl::platform::mouse::cursors_windows`

Changed

- Fixed `get_window_size` returning the window hitbox instead of the actual size [Windows]
- Updated rendering input handling for rectangle drawing functions
- Renamed graphical functions using `u32` to clarify that `u32` is the default type instead of `u8`
- Improved `ScreenNormalizer`
- Fixed application icon transparency issues

Removed

- Removed the now-obsolete `ico_support` flag. A single custom function now replaces all functionality of its previous library and fixes its alpha channel bug.

---

# Earlier versions:

Unknown; github history can be used for the completion of the list
