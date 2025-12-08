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
