# Version 8.0.0

# Changes:

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
- test_flags.py is over 10 times faster now, expect more flags in the future
