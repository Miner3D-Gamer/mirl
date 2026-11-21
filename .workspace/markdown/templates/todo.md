{"path": "./docs"}

# TODO in [Mirl]:

## Helper scripts:

Create:

- If a flag exists and a dependency has the same flag, that flag needs to be included (Needs to be reimplemented in [`mirl_build_tools`] using [`mirl_metadata`])
- Generate all relevant combination of flags and test if each one compiles
- Create a python script that checks if all crates follow the correct standards in terms of `README.md` and `CHANGELOG.md`

## Crates

- Readd 'find-features-that-dependency-shares' for [`mirl_build_tools`]
- Add support for `alloc` on `no_std`
- Linux support for [`mirl_system`]
- Extend [`mirl_extensions_math`] with length units and temperature
- Implement [`mirl_geometry]` and uncomment all code in the other libs that depend on it OR deprecate it
- Improve the `RawCursor` to `.ico` formatting experience
- Check if glfw backend renderers red and blue color channels are swapped
- Check if all `README.md` are up to date quality wise
- Do the almost 200 `TODO` that are in the codebase
- Find a way to mark traits that you have to implement yourself and traits that are automatically implemented + Document all notable traits in the respective `README.md`

## New Crates

- Clipboard support (Unified wrapper around other crates)
- Sound support (Unified wrapper around other crates)
- Functionality for creating the format used inside dot engine visualizers

## Testing

- Test the cursors system more extensively
- Write tests for every lib

## Other

There are over 100 todo in this workspace, work them out
