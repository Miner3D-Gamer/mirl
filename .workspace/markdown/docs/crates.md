# **[Mirl]**

<sub>Miner's Rust Libs (Mrl didn't sound that good)</sub>

> Disclaimer:
>
> 1.  This mini ecosystem is still a WIP due to the recent split.
> 2.  Most of these libs require nightly for access to the `feature` keyword which for faster runtime execution, `no_std` support, and better customizability.

Before the 30 smaller crates, this was a singular kitchen sink lib. But now;

## These aspects are now offered in specialized packages:

> More info about each crate in their respective `README.md`

### Categories:

- [Extensions](#extensions)
- [Widowing](#windowing)
- [Codec](#codec)
- [Other](#other)
- [Deprecated](#deprecated)

### Status Definitions:

- <a id="status-5"></a>[**5.**](#status-5) Has enough quality and testing for a release. (Like any other code, there may be hidden bugs to fix once reported.)

- <a id="status-4"></a>[**4.**](#status-4) Has enough quality and testing for a release. Yet more polish would be nice.

- <a id="status-3"></a>[**3.**](#status-3) Does not have "release" quality, may spout warnings, panic, or do other stuff that'll be fixed/implemented update by update. Yet it is ready to be used if needed.

- <a id="status-2"></a>[**2.**](#status-2) Unfinished. Not yet ready for usage.

- <a id="status-1"></a>[**1.**](#status-1) Abandoned or (almost) completely useless

---

## Extensions

Extends functionality of existing structs, enums, and primitives.

| Status | Further Development                 |
| ------ | ----------------------------------- |
| [5/5]  | Individual crates could be improved |

### • [Mirl Extensions]

Extends rust and supported crates with new functions using (>200) traits.
Also adds back functionality lost when using `no_std`. This includes but isn't limited to extended data conversion, more const values, function deduplication, convenience functions, cross crate compatibility.

| Status | Further Development                                                                                |
| ------ | -------------------------------------------------------------------------------------------------- |
| [5/5]  | Nothing obvious is missing but I can feel that there are definitely still some holes to be patched |

### • [Mirl Extensions Conversion]

A subset of [Mirl Extensions] housing `FromPatch` and `TryFromPatch` as well as `IntoPatch` and `TryIntoPatch`.

`core::convert::From/TryFrom` have a lot of holes, especially when it comes to converting numbers. Did you know that turning `Vec<T1>` into `Vec<T2` is impossible with `core::convert::From/TryFrom`? This lib not only resolves these but also adds other conversions and supports a few external crates as well.

| Status | Further Development                                               |
| ------ | ----------------------------------------------------------------- |
| [4/5]  | There may still be hidden holes and other libs could be supported |

### • [Mirl Extensions Math]

A subset of [Mirl Extensions] with traits purely related to math and mathematical constants. (80% Done. More constants could be added as well as support for other math libs)

| Status | Further Development                   |
| ------ | ------------------------------------- |
| [4/5]  | More number crates could be supported |

<details>
<summary>Core crates</summary>

### • [Mirl Extensions Core]

A subset of [Mirl Extensions] with traits that cannot be auto implemented like attributes, plus some logic related traits.

Used when implementing for types

| Status | Further Development            |
| ------ | ------------------------------ |
| [5/5]  | Done - Additions upon requests |

## </details>

---

## Windowing

Create and manage a window to draw on, get input from, and customize.

| Status | Further Development                                                                                  |
| ------ | ---------------------------------------------------------------------------------------------------- |
| [4/5]  | Console backend needs to be implemented and the other two backends need to be checked for edge cases |

### • [Mirl Windowing]

A modular windowing wrapper, switch between backends by changing a single line

| Status | Further Development |
| ------ | ------------------- |
| [5/5]  | No further ideas    |

### • [Mirl Windowing Minifb]

A backend implementation of [Mirl Windowing Core] using [Minifb](https://crates.io/minifb)

| Status | Further Development                                  |
| ------ | ---------------------------------------------------- |
| [4/5]  | Not all traits from [Mirl Windowing] are implemented |

### • [Mirl Windowing Glfw]

A backend implementation of [Mirl Windowing Core] using [Glfw](https://crates.io/glfw)

| Status | Further Development                                  |
| ------ | ---------------------------------------------------- |
| [3/5]  | Not all traits from [Mirl Windowing] are implemented |

### • [Mirl Windowing Console]

A backend implementation of [Mirl Windowing Core] using nothing but the display and input powers of the console/terminal

| Status | Further Development                                     |
| ------ | ------------------------------------------------------- |
| [2/5]  | Fix window IO and implement all [Mirl Windowing] traits |

<details>
<summary>Core crates</summary>

### • [Mirl Windowing Core]

| Status | Further Development              |
| ------ | -------------------------------- |
| [5/5]  | Clipboard operations are missing |

</details>

---

---

## Graphics

| Status | Further Development                                                                                              |
| ------ | ---------------------------------------------------------------------------------------------------------------- |
| [3/5]  | The whole concept of rendering need to be rethunk to create a modular API that supports both cpu and gpu drawing |

### • [Mirl Rendering]

Draw shapes and text on buffers to display them on [Mirl Windowing] or save them on disk using [Mirl Graphics]

| Status | Further Development                                                                                                                                               |
| ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [2/5]  | Outdated crate but still usable. The architecture behind it needs to be overhauled to a unified yet dynamic api that allows for both cpu and gpu drawing backends |

### • [Mirl Graphics]

Manipulate color and draw on Buffers (limited drawing capabilities compared to [Mirl Rendering])

| Status | Further Development                                     |
| ------ | ------------------------------------------------------- |
| [4/5]  | Some parts of the api feels weird and idk how to fix it |

### • [Mirl Gui]

A retained mode dynamic gui.
Can be integrated into [Mirl Windowing] implementations with just 2 lines of code.

Essentially the `MyDearGui` version for [Mirl]

| Status | Further Development                                     |
| ------ | ------------------------------------------------------- |
| [4/5]  | Some parts of the api feels weird and idk how to fix it |

<details>
<summary>Core crates</summary>

### • [Mirl Rendering Core]

> In development

| Status | Further Development                                               |
| ------ | ----------------------------------------------------------------- |
| [1/5]  | The new baseline for [Mirl Rendering] with a smarter architecture |

</details>

---

---

## Buffer

### • [Mirl Buffer]

A library containing `Buffer`, `ConstBuffer`, `TextBuffer` and a few traits. Without any dependencies, this exists so any other mirl lib can import it without circulating dependencies.

| Status | Further Development                                            |
| ------ | -------------------------------------------------------------- |
| [5/5]  | Replace the u32 inside `Buffer` and `ConstBuffer` to a generic |

### • [Mirl Buffer Interpolation]

A helper library for [Mirl Buffer] which adds proper resizing using several algorithms to choose from.

| Status | Further Development            |
| ------ | ------------------------------ |
| [2/5]  | Provide a proper api for users |

---

## Platform

| Status  | Further Development                                          |
| ------- | ------------------------------------------------------------ |
| [3-4/5] | Reimplement [Mirl Rendering] and [Mirl Buffer Interpolation] |

Crates related to the device or [Windowing](#windowing)

### • [Mirl System]

A Os abstraction layer that also includes the standard moving window, get user path, and so on but also a lot of niche os specific behavior like controlling the Z Layer/Taskbar progress/Decorations/Position/Cpu Priority/Transparency/More of any window + A ton of other stuff

| Status | Further Development                                                             |
| ------ | ------------------------------------------------------------------------------- |
| [3/5]  | More pronounced error handling and more platforms than windows need to be added |

### • [Mirl Input]

Structs, enums, and functions relating to keyboard/mouse handling but does not concern itself with the OS/hardware unlike [Mirl System].

| Status | Further Development                                            |
| ------ | -------------------------------------------------------------- |
| [4/5]  | Some polish/qof like complete conversions coverage are missing |

---

---

## Codec

Crates relating to codecs/formatting

### • [Mirl Codec Info]

When codec parsers parse codecs, they keep the value but discard where it actually came from. Also allows the marshalling of json back into its serialized form.

| Status | Further Development |
| ------ | ------------------- |
| [2/5]  | Fix toml format     |

Dilemma:
The main reason for this crate to exist is because I could not find a crate that recorded the value location of json files. Since then I've found several crates which implement this behavior, e.g. [`https://crates.io/crates/serde_spanned`](https://crates.io/crates/serde_spanned).

Should I abandon this crate or continue development with the following features:

- Format/Codec Mixing (Start with json, continue in css, end in toml)
- Comment support
- Support for defining a format/codec structure and automatically parsing & validating a serialized format/codec for automatic error generation

### • [Mirl Values]

Every codec parser/marshaller has their own value type (serde_json, toml, yaml, json5, ...) which are all incompatible with one another. So here is a Value enum that tries to unify them.

| Status | Further Development  |
| ------ | -------------------- |
| [4/5]  | Support more formats |

<details>
<summary>Core crates</summary>

### • [Mirl Values Core]

A crate solely housing Simple and Container Wrappers. Alone it's almost useless, use [Mirl Values] for actual functionality.

| Status | Further Development  |
| ------ | -------------------- |
| [4/5]  | Support more formats |

## </details>

---

## Math

Math related crates :p

| Status  | Further Development              |
| ------- | -------------------------------- |
| [1-4/5] | Treat the individual crate needs |

### • [Mirl Geometry]

Focusing on geometry (1-4d) and expressions

| Status | Further Development |
| ------ | ------------------- |
| [1/5]  | None Planned        |

It was planned to be used in tangent with a custom game engine.
This game engine never came to be as I simply don't have the motivation for it.

As such, this crate was never actually implemented - existing alternatives are recommended.

### • [Mirl Narrowed Floats]

Focusing on geometry (1-4d) and expressions

| Status | Further Development |
| ------ | ------------------- |
| [4/5]  | No Ideas            |

<details>
<summary>Core crates</summary>

### • [Mirl Geometry Core]

It is recommended to use [Mirl Geometry] instead of this lib. This lib only exists to resolve dependency recursion issues.

Defines geometry and math functions

| Status | Further Development |
| ------ | ------------------- |
| [1/5]  | None Planned        |

It was planned to be used in tangent with a custom game engine.
This game engine never came to be as I simply don't have the motivation for it.

As such, this crate was never actually implemented - existing alternatives are recommended.

## </details>

---

---

## Helper

Crates to help with development

### • [Mirl Macros]

Instead of having every derive opt-in, derives are opt-out using [`#[mirl_macros::derive_all]`]
Has a list of supported crates, chooses what to implement based on a per-object basis

Automatically checks what crates you depend on and what features you've defined to configure that supported crates automatically get derived on your objects

| Status | Further Development |
| ------ | ------------------- |
| [4/5]  | No Ideas            |

(This crate is waiting for [Mirl Macros Core] to get fixed)

### • [Mirl Macros Lite]

Instead of having every derive opt-in, derives are opt-out using [`#[mirl_macros_lite::derive_all]`]
Has a list of supported crates, chooses what to implement based on a per-object basis

Does not automatically check what dependencies you support or which features you've defined.
These stats need to be defined individually on each defined object.

It is recommended to use [Mirl Macros] for less manual work.

| Status | Further Development |
| ------ | ------------------- |
| [4/5]  | No Ideas            |

(This crate is waiting for [Mirl Macros Core] to get fixed)

### • [Mirl Metadata]

Instead of having every derive opt-in, derives are opt-out using [`#[mirl_macros_lite::derive_all]`]
Has a list of supported crates, chooses what to implement based on a per-object basis

Does not automatically check what dependencies you support or which features you've defined.
These stats need to be defined individually on each defined object.

It is recommended to use [Mirl Macros] for less manual work.

| Status | Further Development                                        |
| ------ | ---------------------------------------------------------- |
| [4/5]  | Add the obscure missing fields (metadata, target, resolve) |

### • [Mirl Nightly Check]

Check if nightly is used.
If not, throws a nicely formatted, user friendly, error message.

| Status | Further Development        |
| ------ | -------------------------- |
| [4/5]  | I forgot this even existed |

### • [Mirl Build Tools]

This compile time library crate does a lot but does not require nightly.

- Automatic nightly detection: Warns user with proper error screen instead of compile errors that the `feature` keyword can only be used in nightly
- Warns user when they a crate has a `miri` or `test` flag which has not been activated when using `miri` or `test`
- Checks that feature flags include the feature flag of dependencies when they have the same name
- Adds functions to better deal with `cargo.toml` and `cargo.lock`
- Other mics stuff

| Status | Further Development                                                                                                                                       |
| ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [2/5]  | Port all functionality to [Mirl Build Tools Core], then implement dependency based features like custom prelude imports, automatic nightly features, etc. |

<details>
<summary>Core crates</summary>

### • [Mirl Macros Core]

The core lib of [Mirl Macros] and [Mirl Macros Lite]

Handles all the logic of deciding which object should implement what.

| Status | Further Development                                               |
| ------ | ----------------------------------------------------------------- |
| [2/5]  | Fix generating configured derives for crates that aren't imported |

### • [Mirl Build Tools Core]

The core lib of [Mirl Build Tools], does not require nightly.

All the essentials of controlling the environment and compilation of a given crate.

| Status | Further Development                                                                                                                                    |
| ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [2/5]  | Import all functionality from [Mirl Build Tools], implement trait collector (into `traits.rs`), and automatic dependency file (into `dependencies.rs`) |

</details>

---

---

## Other

### • [Mirl Collections]

A library containing type wrappers with extra functionality, most notably a `BinaryData` struct for file contents (plugin extendible)

| Status | Further Development                                 |
| ------ | --------------------------------------------------- |
| [4/5]  | `NonEmptyVec` needs a rewrite to be more performant |

### • [Mirl Terminal]

Functions and integrations for better interacting with the console/terminal.

More or less just a crossterm wrapper to make the usage of the crate in the mirl ecosystem more seamless.

| Status | Further Development     |
| ------ | ----------------------- |
| [1/5]  | Requires implementation |

### • [Mirl Std Exposed]

This crate publicly contains private functions(/structs/enums) from std/core

| Status | Further Development                                                                                                                |
| ------ | ---------------------------------------------------------------------------------------------------------------------------------- |
| [1/5]  | Create script that automatically scans the entirely of `std` and copies it with the only difference being that everything's public |

---

---

## Deprecated

All functionality will be extracted from these crates. Then they will be deleted. No crate should depend on them.

All deprecated crates `mirl`, `mirl_core`, and `mirl_consts` have been removed.

`mirl`: The original kitchen sink crate before functionality was split
`mirl_core`: A secondary kitchen sink crate, accidentally created when splitting `mirl`
`mirl_consts`: A collection of const values about various units, swallowed by [Mirl Extensions] and its sub crates (Mostly [Mirl Extensions Math])

---

---

None of the libs inexplicably panic or cause UB.

I rate my own crates very harshly because my quality standards for myself are very high.

<!-- Custom definitions -->

[1-5/5]: #status-definitions
[2-5/5]: #status-definitions
[3-5/5]: #status-definitions
[4-5/5]: #status-definitions
[1-4/5]: #status-definitions
[2-4/5]: #status-definitions
[3-4/5]: #status-definitions
[1-2/5]: #status-definitions
[2-2/5]: #status-definitions
[1-3/5]: #status-definitions
[5/5]: #status-5
[3/5]: #status-4
[2/5]: #status-3
[4/5]: #status-2
[1/5]: #status-1
[Mirl Buffer]: github.com/Miner3D-Gamer/mirl_buffer
[Mirl Buffer Interpolation]: github.com/Miner3D-Gamer/mirl_buffer_interpolation
[Mirl Codec Info]: github.com/Miner3D-Gamer/mirl_codec_info
[Mirl Values]: github.com/Miner3D-Gamer/mirl_values
[Mirl Values Core]: github.com/Miner3D-Gamer/mirl_values_core
[Mirl Extensions]: github.com/Miner3D-Gamer/mirl_extensions
[Mirl Extensions Conversion]: github.com/Miner3D-Gamer/mirl_extensions_conversion
[Mirl Extensions Core]: github.com/Miner3D-Gamer/mirl_extensions_core
[Mirl Extensions Math]: github.com/Miner3D-Gamer/mirl_extensions_math
[Mirl Graphics]: github.com/Miner3D-Gamer/mirl_graphics
[Mirl Gui]: github.com/Miner3D-Gamer/mirl_gui
[Mirl Rendering]: github.com/Miner3D-Gamer/mirl_rendering
[Mirl Rendering Core]: github.com/Miner3D-Gamer/mirl_rendering_core
[Mirl Build Tools]: github.com/Miner3D-Gamer/mirl_build_tools
[Mirl Macros]: github.com/Miner3D-Gamer/mirl_macros
[Mirl Geometry]: github.com/Miner3D-Gamer/mirl_geometry
[Mirl Geometry Core]: github.com/Miner3D-Gamer/mirl_geometry_core
[Mirl Collections]: github.com/Miner3D-Gamer/mirl_collections
[Mirl Std Exposed]: github.com/Miner3D-Gamer/mirl_std_exposed
[Mirl Input]: github.com/Miner3D-Gamer/mirl_input
[Mirl System]: github.com/Miner3D-Gamer/mirl_system
[Mirl Terminal]: github.com/Miner3D-Gamer/mirl_terminal
[Mirl Windowing]: github.com/Miner3D-Gamer/mirl_windowing
[Mirl Windowing Console]: github.com/Miner3D-Gamer/mirl_windowing_console
[Mirl Windowing Core]: github.com/Miner3D-Gamer/mirl_windowing_core
[Mirl Windowing Glfw]: github.com/Miner3D-Gamer/mirl_windowing_glfw
[Mirl Windowing Minifb]: github.com/Miner3D-Gamer/mirl_windowing_minifb
[Mirl Metadata]: github.com/Miner3D-Gamer/mirl_metadata
[Mirl Nightly Check]: github.com/Miner3D-Gamer/mirl_nightly_check
[Mirl Build Tools Core]: github.com/Miner3D-Gamer/mirl_build_tools_core
[Mirl Macros Lite]: github.com/Miner3D-Gamer/mirl_macros_lite
[Mirl Macros Core]: github.com/Miner3D-Gamer/mirl_macros_core
[Mirl Narrowed Floats]: github.com/Miner3D-Gamer/mirl_narrowed_floats