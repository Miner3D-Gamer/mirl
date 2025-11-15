# Version 5.1.0:

Added:
- RepeatDataInContainer trait inside crate::extensions, examples: Box<T> -> Box<Vec<T>>, Option<T> -> Option<Vec<T>>
- mirl::prelude with some frequently used and useful stuff
- ListGetNewItemsCloned trait inside crate::extensions that returns Vec<T> instead of Vec<&T>

Changed:
- File System trait moved to own file
- File System trait no longer asks for a list of required files
- Renamed crate::graphics::color_presets to crate::graphics::colors
- Loosened version requirements for serde and serde_json

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

# Version 4.7.1:

Removed:

- tuple ops after 12 as they aren't even supported by the default std

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

# Version 4.6.0:

Added:

- Ability to set the size of a window
- Moved functions related to Iconifying from the awkward crate::platform::shared to the Os struct in crate::system

Removed:

- MouseManagerScrollAccuracy from all framework traits as it was annoying. f32 is enough accuracy for the scroll wheel.

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

# Version 4.4.1:

Changes:

- Loosened dependencies

# Version 4.4.0:

Added

- Added support for retrieving the hitbox of a window (window size + OS padding) [Windows]
- Added the ability to set the CPU priority of a running process [Windows]
- Added functions to set color channels of a `u32`: `with_alpha`, `with_red`, `with_blue`, and `with_green`
- Added utility functions for sending data to Discord webhooks
- Added functions for converting a buffer to `.bmp` and `.ico`, and made the `.cur` conversion function public in `crate::platform:🐭:cursors_windows`

Changed

- Fixed `get_window_size` returning the window hitbox instead of the actual size [Windows]
- Updated rendering input handling for rectangle drawing functions
- Renamed graphical functions using `u32` to clarify that `u32` is the default type instead of `u8`
- Improved `ScreenNormalizer`
- Fixed application icon transparency issues

Removed

- Removed the now-obsolete `ico_support` flag. A single custom function now replaces all functionality of its previous library and fixes its alpha channel bug.

# Earlier versions:

Unknown; github history can be used for the completion of the list
