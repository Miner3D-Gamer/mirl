# **[Mirl]**

<sub>Miner's Rust Libs (Mrl didn't sound that good)</sub>

> Disclaimer:
>
> 1.  This mini ecosystem is still a WIP due to the recent split.
> 2.  Most of these libs require nightly for access to the `feature` keyword which for faster runtime execution, `no_std` support, and better customizability.

Before the 30 smaller crates, this was a singular kitchen sink lib. But now;

## These aspects are now offered in specialized packages:

> More info about each crate in their respective `README.md`

**Categories:**

- [Extensions](#extensions)
- [Widowing](#windowing)
- [Codec](#codec)
- [Other](#other)
- [Deprecated](#deprecated)

**Status Definitions:**

- S: Has enough quality and testing for a release. Like any other code, there may be hidden bugs to fix once reported.
- U: Does not have "release" quality, may spout warnings, panic, or do other stuff that'll be fixed/implemented update by update

---

Table

| Crate           | Description                                                                                                                                                                                                                                                                                | Status | Further Development                                                                                |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------ | -------------------------------------------------------------------------------------------------- |
| [Mirl Extensions] | Extends rust and supported crates with new functions using (>200) traits. Also adds back functionality lost when using `no_std`. This includes but isn't limited to extended data conversion, more const values, function deduplication, convenience functions, cross crate compatibility. | S      | Nothing obvious is missing but I can feel that there are definitely still some holes to be patched |

## Extensions

Extends functionality of existing structs, enums, and primitives.

### • [Mirl Extensions]

### • [Mirl Extensions Conversion]

A subset of [Mirl Extensions] housing `FromPatch` and `TryFromPatch` as well as `IntoPatch` and `TryIntoPatch`.

`core::convert::From/TryFrom` have a lot of holes, especially when it comes to converting numbers. Did you know that turning `Vec<T1>` into `Vec<T2` is impossible with `core::convert::From/TryFrom`? This lib not only resolves these but also adds other conversions and supports a few external crates as well.

(90% (Never) Done. There may still be hidden holes and other libs could be supported)

### • [Mirl Extensions Math]

A subset of [Mirl Extensions] with traits purely related to math and mathematical constants. (80% Done. More constants could be added as well as support for other math libs)

<details>
<summary>Core crates</summary>

### • [Mirl Extensions Core]

A subset of [Mirl Extensions] with traits that cannot be auto implemented like attributes, plus some logic related traits.

(Done, more attributes for auto impls would be nice though)

## </details>

---

## Windowing

Create and manage a window to draw on

### • [Mirl Windowing]

A modular windowing wrapper, switch between backends by changing a single line

(90% Done, need to check if the api is intuitive)

### • [Mirl Windowing Minifb]

A backend implementation of [Mirl Windowing Core] using [Minifb](https://crates.io/minifb)

(The backend is 90% implemented)

### • [Mirl Windowing Glfw]

A backend implementation of [Mirl Windowing Core] using [Glfw](https://crates.io/glfw)

(The backend is 65% implemented)

### • [Mirl Windowing Console]

A backend implementation of [Mirl Windowing Core] using nothing but the display and input powers of the console/terminal

(The backend is 20% implemented)

<details>
<summary>Core crates</summary>

### • [Mirl Windowing Core]

Defines the core objects for implementing a backend

(95% Done. Nothing obvious can be improved.)

</details>

---

---

## Platform

Crates related to the device or [Windowing](#windowing)

### • [Mirl System]

A Os abstraction layer that also includes the standard moving window, get user path, and so on but also a lot of niche os specific behavior like controlling the Z Layer/Taskbar progress/Decorations/Position/Cpu Priority/Transparency/More of any window + A ton of other stuff

(15% Done, 1/4 OS. More pronounced error handling and more platforms than windows needs to be added)

### • [Mirl Rendering]

Draw shapes and text on buffers to display them on [Mirl Windowing] or save them on disk using [Mirl Graphics]

(55% Done. Outdated crate but still usable. More shapes and textured shapes need to be added.)

### • [Mirl Buffer]

A library containing `Buffer`, `ConstBuffer`, and a few traits. Without any dependencies, this exists so any other mirl lib can import it without circulating dependencies.

(95% Done. Nothing obvious can be improved.)

### • [Mirl Buffer Interpolation]

A helper library for [Mirl Buffer] which adds proper resizing using several algorithms to choose from.

(60% Done. Seamless integration with `Buffer` needs to be added.)

### • [Mirl Graphics]

Manipulate color and draw on Buffers (limited drawing capabilities compared to [Mirl Rendering])

(99% Done. Nothing obvious to be improved)

### • [Mirl Input]

Structs, enums, and functions relating to keyboard/mouse handling. This crate does not concern itself with the OS or hardware unlike [Mirl System].

(90% Done. Some polish/qof like complete conversions coverage are missing.)

---

---

## Codec

Crates relating to codecs/formatting

### • [Mirl Codec Info]

When codec parsers parse codecs, they keep the value but discard where it actually came from

(65% Done, parsing and marshalling support for more formats)

### • [Mirl Values]

Every codec parser/marshaller has their own value type (serde_json, toml, yaml, json5, ...) which are all incompatible with one another. So here is a Value enum that tries to unify them.

(90% Done, more codec crates could be supported)

<details>
<summary>Core crates</summary>

### • [Mirl Values Core]

A crate solely housing Simple and Container Wrappers. Alone it's almost useless, use [Mirl Values] for actual functionality.

(90% Done, more codec creates could be supported)

## </details>

---

## Math

Math related crates :p

### • [Mirl Geometry]

Focusing on geometry (1-4d) and expressions

(0% Done, this needs a total rewrite)

<details>
<summary>Core crates</summary>

### • [Mirl Geometry Core]

It is recommended to use [Mirl Geometry] instead of this lib. This lib only exists to resolve dependency recursion issues.

Defines geometry and math functions

(0% Done, parent crate needs a total rewrite)

## </details>

---

---

## Helper

Crates to help with development

### • [Mirl Macros]

Instead of having every derive opt-in, derives are opt-out using [`#[mirl_macros::derive_all]`]

(85% Done. Spews warnings.)

### • [Mirl Build Tools]

This compile time library crate does a lot but does not require nightly.

- Automatic nightly detection: Warns user with proper error screen instead of compile errors that the `feature` keyword can only be used in nightly
- Warns user when they a crate has a `miri` or `test` flag which has not been activated when using `miri` or `test`
- Checks that feature flags include the feature flag of dependencies when they have the same name
- Adds functions to better deal with `cargo.toml` and `cargo.lock`
- Other mics stuff

(90% Done. Dependency collector, automatic feature activation based on imported crates, and automatic prelude extension based on imported crates could be added.)

---

---

## Other

### • [Mirl Collections]

A library containing type wrappers with extra functionality, most notably a `BinaryData` struct for file contents (plugin extendible)

(75% Done. `NonEmptyVec` needs a rewrite to be more performant.)

### • [Mirl Terminal]

Functions and integrations for better interacting with the console/terminal.

More or less just a crossterm wrapper to make the usage of the crate in the mirl ecosystem more seamless.

### • [Mirl Std Exposed]

This crate publicly contains private functions(/structs/enums) from std/core

---

---

## Deprecated

All functionality will be extracted from these crates. Then they will be deleted. No crate should depend on them.

All deprecated crates have been removed. These were `mirl`, `mirl_core`, and `mirl_consts`

---

---

None of the libs inexplicably panic or cause UB, the done-meter is a rough estimation of how satisfied I am with their current state, none will ever really be at a true 100% because perfection doesn't exist.

[Mirl]: https://github.com/Miner3D-Gamer/mirl
[Mirl Extensions]: https://github.com/Miner3D-Gamer/mirl_extensions
[Mirl Extensions Conversion]: https://github.com/Miner3D-Gamer/mirl_extensions_conversion
[Mirl Extensions Math]: https://github.com/Miner3D-Gamer/mirl_extensions_math
[Mirl Extensions Core]: https://github.com/Miner3D-Gamer/mirl_extensions_core
[Mirl Windowing]: https://github.com/Miner3D-Gamer/mirl_windowing
[Mirl Windowing Minifb]: https://github.com/Miner3D-Gamer/mirl_windowing_core
[Mirl Windowing Glfw]: https://github.com/Miner3D-Gamer/mirl_windowing_core
[Mirl Windowing Console]: https://github.com/Miner3D-Gamer/mirl_windowing_core
[Mirl Windowing Core]: https://github.com/Miner3D-Gamer/mirl_windowing_core
[Mirl System]: https://github.com/Miner3D-Gamer/mirl_system
[Mirl Rendering]: https://github.com/Miner3D-Gamer/mirl_rendering
[Mirl Buffer]: https://github.com/Miner3D-Gamer/mirl_buffer
[Mirl Buffer Interpolation]: https://github.com/Miner3D-Gamer/mirl_buffer_interpolation
[Mirl Graphics]: https://github.com/Miner3D-Gamer/mirl_graphics
[Mirl Input]: https://github.com/Miner3D-Gamer/mirl_input
[Mirl Codec Info]: https://github.com/Miner3D-Gamer/mirl_codec_info
[Mirl Values]: https://github.com/Miner3D-Gamer/mirl_values
[Mirl Values Core]: https://github.com/Miner3D-Gamer/mirl_values_core
[Mirl Geometry]: https://github.com/Miner3D-Gamer/mirl_geometry
[Mirl Geometry Core]: https://github.com/Miner3D-Gamer/mirl_geometry_core
[Mirl Macros]: https://github.com/Miner3D-Gamer/mirl_macros
[Mirl Macros Lite]: https://github.com/Miner3D-Gamer/mirl_macros_lite
[Mirl Build Tools]: https://github.com/Miner3D-Gamer/mirl_build_tools
[Mirl Build Tools Core]: https://github.com/Miner3D-Gamer/mirl_build_tools_core
[Mirl Metadata]: https://github.com/Miner3D-Gamer/mirl_metadata
[Mirl Collections]: https://github.com/Miner3D-Gamer/mirl_collections
[Mirl Terminal]: https://github.com/Miner3D-Gamer/mirl_terminal
[Mirl Std Exposed]: https://github.com/Miner3D-Gamer/mirl_std_exposed
[Mirl Narrowed Floats]: https://github.com/Miner3D-Gamer/mirl_narrowed_floats
