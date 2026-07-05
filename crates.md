# **Mirl**

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

---

## Extensions

Extends functionality of existing structs, enums, and primitives.

#### • [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions)

Extends rust and supported crates with new functions using (>200) traits.
Also adds back functionality lost when using `no_std`. This includes but isn't limited to extended data conversion, more const values, function deduplication, convenience functions, cross crate compatibility.

(95% Done. Nothing obvious is missing but I can feel that there are definitely still some holes to be patched)

#### • [Mirl Extensions Conversion](https://github.com/Miner3D-Gamer/mirl_extensions_conversion)

A subset of [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) housing `FromPatch` and `TryFromPatch` as well as `IntoPatch` and `TryIntoPatch`.

`core::convert::From/TryFrom` have a lot of holes, especially when it comes to converting numbers. Did you know that turning `Vec<T1>` into `Vec<T2` is impossible with `core::convert::From/TryFrom`? This lib not only resolves these but also adds other conversions and supports a few external crates as well.

(90% (Never) Done. There may still be hidden holes and other libs could be supported)

#### • [Mirl Extensions Math](https://github.com/Miner3D-Gamer/mirl_extensions_math)

A subset of [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) with traits purely related to math and mathematical constants. (80% Done. More constants could be added as well as support for other math libs)

<details>
<summary>Core crates</summary>

#### • [Mirl Extensions Core](https://github.com/Miner3D-Gamer/mirl_extensions_core)

A subset of [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) with traits that cannot be auto implemented like attributes, plus some logic related traits.

(Done, more attributes for auto impls would be nice though)

## </details>

---

## Windowing

Create and manage a window to draw on

#### • [Mirl Windowing](https://github.com/Miner3D-Gamer/mirl_windowing)

A modular windowing wrapper, switch between backends by changing a single line

(90% Done, need to check if the api is intuitive)

#### • [Mirl Windowing Minifb](https://github.com/Miner3D-Gamer/mirl_windowing_core)

A backend implementation of [Mirl Windowing Core](https://github.com/Miner3D-Gamer/mirl_windowing_core) using [Minifb](https://crates.io/minifb)

(The backend is 90% implemented)

#### • [Mirl Windowing Glfw](https://github.com/Miner3D-Gamer/mirl_windowing_core)

A backend implementation of [Mirl Windowing Core](https://github.com/Miner3D-Gamer/mirl_windowing_core) using [Glfw](https://crates.io/glfw)

(The backend is 65% implemented)

#### • [Mirl Windowing Console](https://github.com/Miner3D-Gamer/mirl_windowing_core)

A backend implementation of [Mirl Windowing Core](https://github.com/Miner3D-Gamer/mirl_windowing_core) using nothing but the display and input powers of the console/terminal

(The backend is 20% implemented)

<details>
<summary>Core crates</summary>

#### • [Mirl Windowing Core](https://github.com/Miner3D-Gamer/mirl_windowing_core)

Defines the core objects for implementing a backend

(95% Done. Nothing obvious can be improved.)

</details>

---

---

## Platform

Crates related to the device or [Windowing](#windowing)

#### • [Mirl System](https://github.com/Miner3D-Gamer/mirl_system)

A Os abstraction layer that also includes the standart moving window, get user path, and so on but also a lot of niche os specific behavior like controlling the Z Layer/Taskbar progress/Decorations/Position/Cpu Priority/Transparency/More of any window + A ton of other stuff

(15% Done, 1/4 OS. More pronounced error handling and more platforms than windows needs to be added)

#### • [Mirl Rendering](https://github.com/Miner3D-Gamer/mirl_rendering)

Draw shapes and text on buffers to display them on [Mirl Windowing](https://github.com/Miner3D-Gamer/mirl_windowing) or save them on disk using [Mirl Graphics](https://github.com/Miner3D-Gamer/mirl_graphics)

(55% Done. Outdated crate but still usable. More shapes and textured shapes need to be added.)

#### • [Mirl Buffer](https://github.com/Miner3D-Gamer/mirl_buffer)

A library containing `Buffer`, `ConstBuffer`, and a few traits. Without any dependencies, this exists so any other mirl lib can import it without circulating dependencies.

(95% Done. Nothing obvious can be improved.)

#### • [Mirl Buffer Interpolation](https://github.com/Miner3D-Gamer/mirl_buffer_interpolation)

A helper library for [Mirl Buffer](https://github.com/Miner3D-Gamer/mirl_buffer) which adds proper resizing using several algorithms to choose from.

(6s0% Done. Seamless integration with `Buffer` needs to be added.)

#### • [Mirl Graphics](https://github.com/Miner3D-Gamer/mirl_graphics)

Manipulate color and draw on Buffers (limited drawing capabilities compared to [Mirl Rendering](https://github.com/Miner3D-Gamer/mirl_rendering))

(99% Done. Nothing obvious to be improved)

#### • [Mirl Input](https://github.com/Miner3D-Gamer/mirl_input)

Structs, enums, and functions relating to keyboard/mouse handling. This crate does not concern itself with the OS or hardware unlike [Mirl System](https://github.com/Miner3D-Gamer/mirl_system).

(90% Done. Some polish/qof like complete conversions coverage are missing.)

---

---

## Codec

Crates relating to codecs/formatting

#### • [Mirl Codec Info](https://github.com/Miner3D-Gamer/mirl_codec_info)

When codec parsers parse codecs, they keep the value but discard where it actually came from (65% Done, parsing and marshalling support for more formats)

#### • [Mirl Values](https://github.com/Miner3D-Gamer/mirl_values)

Every codec parser/marshaller has their own value type (serde_json, toml, yaml, json5, ...) which are all incompatible with one another. So here is a Value enum that tries to unify them. (90% Done, more codec crates could be supported)

<details>
<summary>Core crates</summary>

#### • [Mirl Values Core](https://github.com/Miner3D-Gamer/mirl_values_core)

A crate solely housing Simple and Container Wrappers. Alone it's almost useless, use [Mirl Values](https://github.com/Miner3D-Gamer/mirl_values) for actual functionality. (90% Done, more codec creates could be supported)

## </details>

---

## Math

Math related crates :p

#### • [Mirl Geometry](https://github.com/Miner3D-Gamer/mirl_geometry)

Focusing on geometry (1-4d) and expressions

(0% Done, this needs a total rewrite)

<details>
<summary>Core crates</summary>

#### • [Mirl Geometry Core](https://github.com/Miner3D-Gamer/mirl_geometry-core)

It is recommended to use [Mirl Geometry](https://github.com/Miner3D-Gamer/mirl_geometry) instead of this lib. This lib only exists to resolve dependency recursion issues.

Defines geometry and math functions

(0% Done, parent crate needs a total rewrite)

## </details>

---

---

## Helper

Crates to help with development

#### • [Mirl Derive](https://github.com/Miner3D-Gamer/mirl_derive)

Less duplicate code when deriving other crates. Doesn't use nightly. (85% Done. More crates could be supported)

#### • [Mirl Build Tools](https://github.com/Miner3D-Gamer/mirl_build_tools)

A compile time library currently only used to pretty print a warning page into the console when someone tries to use a mirl crate on stable rust. This crate does not require nightly. (99% Done. Nothing obvious can be improved.)

---

---

## Other

#### • [Mirl Collections](https://github.com/Miner3D-Gamer/mirl_collections)

A library containing type wrappers with extra functionality, most notably a `BinaryData` struct for file contents (plugin extendible) (75% Done. `NonEmptyVec` needs a rewrite to be more performant.)

#### • [Mirl Terminal](https://github.com/Miner3D-Gamer/mirl_terminal)

Functions and integrations for better interacting with the console/terminal.

More or less just a crossterm wrapper to make the usage of the crate in the mirl ecosystem more seamless.

#### • [Mirl std Exposed ](https://github.com/Miner3D-Gamer/mirl_std_exposed)

This crate publicly contains private functions(/structs/enums) from std/core

---

---

## Deprecated

All functionality will be extracted from these crates. Then they will be deleted. No crate should depend on them.

#### • [Mirl Consts]()

Most values here could just exist in in a [Extensions](#extensions) crates.

This lib is unreleased and will soon be consumed by [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) or [Mirl Extensions Math](https://github.com/Miner3D-Gamer/mirl_extensions_math) (10% Done. Only time and byte related consts exist. Needs to be expanded.)

#### • [Mirl](https://github.com/Miner3D-Gamer/mirl)

The old kitchen sink. A crate with objects I don't know where else to put (50% Done. This lib will dissolve into others when logically possible)

#### • [Mirl Core](https://github.com/Miner3D-Gamer/mirl_core)

Similar to [mirl](https://github.com/Miner3D-Gamer/mirl), a kitchen sink. Whereas mirl pulls from all other crates here, the other crates pull from here (40% This lib will dissolve into others when logically possible)

---

---

None of the libs inexplicitly panic or cause UB, the done-meter is a rough estimation of how close they are to their "true potential" (without being extended by other crates), none will ever really be at a true 100% because perfection doesn't exist.
