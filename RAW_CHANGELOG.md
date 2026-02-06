# Version 9.0.0

### Changes:

- Split `new` function from `mirl::platform::windowing::traits::ExtendedControl` into `NewWindow`
- Added new experimental backend: `console` which will display the given buffer in the console instead of a standalone window
- Split `get_window_handle` function from `mirl::platform::windowing::traits::ExtendedWindow` into `GetWindowHandle`
- Renamed `Framework` trait from `mirl::platform::windowing::traits` into `WindowingFramework`
- Renamed `ExtendedFramework` trait from `mirl::platform::windowing::traits` into `ExtendedWindowingFramework`
- Replaced a bunch of references to `std` with `core` meaning more stuff is available without the `std` flag
- `mirl::render::text::aliased::*` now wants an alpha cutoff for finer visual control
- `draw_text_switch` and `draw_text_switch_isize` inside `mirl::render::text` now take `Option<u8>` for `antialiased` instead of `bool` to comply with the changes made in `mirl::render::text::aliased`
- Renamed `draw_buffer_on_buffer` in `mirl::render` to `draw_buffer_on_buffer_stretched`
- Renamed `draw_buffer_on_buffer_1_to_1` in `mirl::render` to `draw_buffer_on_buffer`
- Fixed `TRANSPARENCY_INTERPOLATION` of `mirl::draw_buffer_on_buffer`
- Added more number trait identifiers in `mirl::extensions`: `Round`, `Sqrt`, `Abs`, `Floor`, and `Ceil`
- Added more tuple operation traits in `mirl::extensions`: `TupleRound`, `TupleSqrt`, `TupleAbs`, `TupleFloor`, and `TupleCeil`
- Misc un-note worthy improvements concerning stuff line inlining
- Added `get_from_idx` and `get_idx` to `mirl::graphics::TextureManager`
- Updated `mirl::render::draw_buffer_on_buffer_stretched` to support more interpolation modes
- Updated `mirl::Buffer::resize_content` to take in a tuple position instead of 2 separate variables
- Renamed `update_with_buffer` of `mirl::platform::windowing::trait::WindowHelper` back to `update`
- Removed `texture_manager_cleanup` flag from `default`
- `mirl::platform::windowing::WindowingFramework` now requires the struct to have `Debug` implemented
- Updated code to work with cargo `1.94.0`
- Removed the reexport of `fontdue` in `mirl::prelude`
- Added `mirl::dependencies` which reexport all dependencies mirl uses
- Added `get_direction_from_point_to_point` and `get_direction_from_rectangle_to_rectangle` to `mirl::directions::misc`
- Added `NextPowerOfTwo`, `NextPowerOfTwoWithExponent`, and `NextPowerOfTwoExponent` trait to `mirl::extensions`
- Changed inputs of `mirl::math::collision::Rectangle::new` from `(x: T, y: T, width: T, height: T)` to `(pos: (T, T), size: (T, T))`
- Changed inputs of `create_collision_usize` and `create_collision` of `mirl::platform::buffer::Buffer` to take a tuple pos instead of separate x and y coordinates
- Added `mirl::render::draw_buffer_on_buffer_smart` and `mirl::render::draw_buffer_on_buffer_stretched_smart`
- Added `do_areas_intersect_strict` to `mirl::math::collision::Rectangle`
- In addition to f128, now also supports f16 wherever possible
- Merged `interpolate_color_rgb_u32_f32` and `interpolate_color_rgb_u32_f64` into `interpolate_color_rgb_u32` as it has nothing to do with floats in `mirl::graphics`
- Split traits inside `mirl::directions` into separate traits to be able to make them const
- Marked a bunch of `impl` as `impl const`
- Replaced `is_direction_allowed_u8` in `mirl::directions::NormalDirections` with `is_direction_true` and added trait `IsDirectionTrue` which `is_direction_true` uses
- Added `mirl::platform::mouse::MouseSnapShot` which can be gotten using the `mirl::platform::windowing::traits::WindowInputHelper::get_mouse_snapshot` trait function
- Changed output of `mirl::platform::windowing::traits::ExtendedMouseInput::get_mouse_scroll` from `Option<(f32, f32)>` to `(f32, f32)`
- Changed output of `mirl::platform::windowing::traits::MouseInput::get_mouse_position` from `Option<(i32, i32)>` to `Option<(f32, f32)>` and subsequently also for `mirl::platform::windowing::traits::RelativeMousePos::get_mouse_position_relative` (also fixed this trait as it was adding instead of subbing positions)
- `mirl::platform::mouse` is now available when `system` flag is not enabled, items that need `system` are individually flagged
- Fixed `mirl::platform::Buffer::ConstBuffer` and implemented almost all functions of `mirl::platform::Buffer`
- Looked through pretty much all structs and enums and added missing derives (Mostly `Ord`, `PartialOrd`, `Hash`, and `Default` were added)
- Added `expressions` branch to `mirl::math` for easier manipulation and usage of common expressions
- Removed `positioning` branch from `mirl::math` as a simple x dimensional tuple is already well equipped for positioning, especially with the tuple math added my mirl
- Renamed `mirl::math::collision` to `mirl::math::geometry::d2`, instead of a simple hardcoded shapes it now uses a layered system where one features embeds another (Shape -> Position -> Rotation)
- Renamed all `create_collision*` functions of `Buffer` to `to_collision*`
- Removed `num_traits` as a feature locking flag. The flag now only exists for the small u support (`all` vs only `std` flag now only has a 10k line difference or ~22.5%)
- Moved `Buffer` from `mirl::platform` to `mirl::render`, the buffer belongs to where you use as the main focus, not where you use it as just another variable to keep track of
- Renamed `new` to `new_with_data` of `Buffer` and `ConstBuffer` in `mirl::render`
- Moved time related constants from `mirl::time` to `mirl::constants::time` and added a few more
- Moved `TextPos` out of `mirl::misc` into `mirl::text::position` and added a few more things. Also added new traits in `mirl::extensions` to more seamlessly integrate it with already existing types
- Added Buffer traits -> Instead of hardcoding the drawing functions to only work with `mirl::render::Buffer`, any struct implementing the traits inside `mirl::render::buffer_traits` will now work with most of the builtin drawing functions. By default they are implemented for `mirl::render::Buffer` and `mirl::render::ConstBuffer`
- Removed `mirl::math::ConstPartialOrd`, as it is supported by a nightly feature
- Repurposed `TwoTillTen` in `mirl::math` to `ConstNumbers128`, supporting all numbers from 0 to 127
- Added `FromPatch` (and `into_patch`) and moved all safe conversions from `TryFromPatch` (and `TryIntoPatch`). `TryFromPatch` automatically supports all conversions defined in `FromPatch`
- Renamed `FileSystem` trait to `FileSystemTrait`
- Fixed `TextureManager` in `mirl::graphics` to use a generic for its file system instead of a dyn
- Split lazy loading of `TextureManager` in `mirl::graphics` from `get` into its own functions: `get_or_load`

---

# Version 8.0.0

### Changes:

- Split `mirl::platform::framework_traits::Window::update` into `update_raw` and `update_with_buffer`
- Removed `mirl::platform::framework_traits::Errors::AllGood`, all functions that would return this now return `Result<(), Errors>` instead
- Removed `mirl::platform::framework_traits::Errors::Unknown`, all functions that would return this now return `Errors::Misc` instead which has a description
- Renamed `mirl::platform::WindowSettings::set_position_to_middle_of_screen` to `mirl::platform::WindowSettings::center_window`
- Added `fullscreen(mut self)` to `mirl::platform::WindowSettings`
- Added `mirl::platform::framework_traits::DeprecatedCompatibilityHelper` which currently readds the `update` function for easier migration
- Added `WindowCreationError` and `WindowUpdateError` enums to `mirl::platform::framework_traits` for more explicit error handling
- In `mirl::platform::minifb` pubbed all functions that were converting minifb errors to local error enums
- Added `get_pos` and `get_size` helper functions to `mirl::math::collision::rectangle::Rectangle`
- Added `mirl::compile_time_dependency_errors` which will give you a formal compile time error when using unusual ways of enabling flags partially
- Inside `mirl::constants::bytes`, all constants are now `u128`
- Moved `minifb`, `glfw`, and `framework_traits` from `mirl::platform` to `mirl::platform::frameworks` and renamed `framework_traits` to `traits`
- Removed `get_delta_time` from `mirl::platform::frameworks::Timing`
- Removed `native_time` field from all frameworks
- Split `set_icon` from `mirl::platform::frameworks::traits::ExtendedWindow` into their own trait `mirl::platform::frameworks::traits::IconControl`
- Split `mirl::platform::frameworks::traits::Input` into `MouseInput`, and `KeyboardInput`
- Split `mirl::platform::frameworks::traits::ExtendedInput` into `ExtendedMouseInput`, and `ExtendedKeyboardInput`
- Split `mirl::platform::frameworks::traits::CursorStyleControl` into `LoadCursorStyle`, and `UseCursorStyle`
- Re-pubbed `glfw`, `minifb`, and `frameworks::traits` as `framework_traits` under `mirl::platform` with a deprecation note each
- Split `mirl::platform::frameworks::traits::ExtendedControl` into their own trait `Visibility` and `RenderLayer`
- Mirl finally also compiled for `wasm32-unknown-unknown`, some features are missing like IO and Framework options but it is compiling, almost no functions have been implemented for web (`mirl::system::Os`)
- Mirl now compiles to `x86_64-apple-darwin`/`aarch64-apple-darwin`, MiniFB is disabled but GLFW should theoretically work, no functions have been implemented for mac (`mirl::system::Os`)
- Renamed `Errors` inside `mirl::platform::frameworks` to `WindowError`
- Added `std::error::Error` to `mirl::platform::frameworks::WindowError`

# Misc:

- Changed the settings of my local formatter for a better source code scouting experience
- test_flags.py is over 3 times faster now, expect more flags in the future
