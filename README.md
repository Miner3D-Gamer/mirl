# Mirl

<sub>Miner's Rust Libs (Mrl didn't sound that good)</sub>

> Disclaimer:
>
> 1.  This mini ecosystem is still a WIP due to the recent split.
> 2.  Most of these libs require nightly for access to the `feature` keyword which for faster runtime execution, `no_std` support, and better customizability.

Before the over 20 smaller crates, this was a singular kitchen sink lib. But now,

### These aspects are now offered in specialized packages:

#### Extensions

- [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) - Extends rust and supported crates with new functions using (>150) traits. Also adds back functionality lost when using `no_std`. This includes but isn't limited to extended data conversion, more const values, function deduplication, convenience functions, cross crate compatibility. (95% Done. Nothing obvious is missing but I can feel that there might be a hole here or there)

- [Mirl Extensions Core](https://github.com/Miner3D-Gamer/mirl_extensions_core) - A subset of [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) with traits that cannot be auto implemented like attributes, plus some logic related traits. (Done, more attributes for auto impls would be nice.)

- [Mirl Extensions Conversion](https://github.com/Miner3D-Gamer/mirl_extensions_conversion) - A subset of [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) housing `FromPatch` and `TryFromPatch` as well as `IntoPatch` and `TryIntoPatch`. `core::convert::From/TryFrom` have a lot of holes, especially when it comes to converting numbers. Did you know that turning `Vec<T1>` into `Vec<T2` is impossible with `core::convert::From/TryFrom`? This lib not only resolves these but also adds other conversions and supports a few external crates as well. (90% (Never) Done. There may still be hidden holes and other libs could be supported)

- [Mirl Extensions Math](https://github.com/Miner3D-Gamer/mirl_extensions_math) - A subset of [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) with traits purely related to math and mathematical constants. (80% Done. More constants could be added as well as support for other math libs)

#### Windowing

- [Mirl Windowing](https://github.com/Miner3D-Gamer/mirl_windowing) - A modular windowing wrapper, switch between backends by changing a single line (65% Done, the glfw backend is missing some features and the console backend is unimplemented)

- [Mirl Windowing Core](https://github.com/Miner3D-Gamer/mirl_windowing_core) - Defines the core objects for implementing a backend

- [Mirl Windowing Minifb](https://github.com/Miner3D-Gamer/mirl_windowing_core) - A backend implementation of [Mirl Windowing Core](https://github.com/Miner3D-Gamer/mirl_windowing_core) using [Minifb](https://crates.io/minifb)
- [Mirl Windowing Glfw](https://github.com/Miner3D-Gamer/mirl_windowing_core) - A backend implementation of [Mirl Windowing Core](https://github.com/Miner3D-Gamer/mirl_windowing_core) using [Glfw](https://crates.io/glfw)
- [Mirl Windowing Console](https://github.com/Miner3D-Gamer/mirl_windowing_core) - A backend implementation of [Mirl Windowing Core](https://github.com/Miner3D-Gamer/mirl_windowing_core) using nothing but the display and input powers of the console/terminal

#### Windowing Extended Loop

- [Mirl System](https://github.com/Miner3D-Gamer/mirl_system) - A unified Os abstraction layer that also includes a lot of os specific behavior like controlling the Z Layer/Taskbar progress/Decorations/Position/Cpu Priority/Transparency/More of any window + A ton of other OS stuff (15% Done, 1/4 OS. More pronounced error handling and more platforms than windows needs to be added)

- [Mirl Rendering](https://github.com/Miner3D-Gamer/mirl_rendering) - Draw shapes and text on buffers to display them on [Mirl Windowing](https://github.com/Miner3D-Gamer/mirl_windowing) or save them on disk using [Mirl Graphics](https://github.com/Miner3D-Gamer/mirl_graphics) (55% Done. More shapes and textured shapes need to be added.)

- [Mirl Buffer](https://github.com/Miner3D-Gamer/mirl_buffer) - A library containing `Buffer`, `ConstBuffer`, and a few traits. Without any dependencies, this exists so any other mirl lib can import it without circulating dependencies. (95% Done. Nothing obvious can be improved.)

- [Mirl Buffer Interpolation](https://github.com/Miner3D-Gamer/mirl_buffer_interpolation) - A helper library for [Mirl Buffer](https://github.com/Miner3D-Gamer/mirl_buffer) which adds proper resizing using several algorithms to choose from. (70% Done. Seamless integration with `Buffer` needs to be added.)

- [Mirl Graphics](https://github.com/Miner3D-Gamer/mirl_graphics) - Manipulate color and draw on Buffers (limited drawing capabilities compared to [Mirl Rendering](https://github.com/Miner3D-Gamer/mirl_rendering)) (30% Done. The color conversion functions need to be renamed.)

- [Mirl Input](https://github.com/Miner3D-Gamer/mirl_input) - Structs, enums, and functions relating to keyboard/mouse handling. This crate does not concern itself with the OS or hardware unlike [Mirl System](https://github.com/Miner3D-Gamer/mirl_system). (90% Done. Polish like better conversions are missing.)

#### Codec

- [Mirl Values](https://github.com/Miner3D-Gamer/mirl_values) - Every codec parser/marshaller has their own value type (serde_json, toml, yaml, json5, ...) which are all incompatible with one another. So here is a Value enum that tries to unify them. (90% Done, more codec crates could be supported)

- [Mirl Values Core](https://github.com/Miner3D-Gamer/mirl_values_core) - A crate solely housing Simple and Container Wrappers. Alone it's almost useless, use [Mirl Values](https://github.com/Miner3D-Gamer/mirl_values) for actual functionality. (90% Done, more codec creates could be supported)

- [Mirl Codec Info](https://github.com/Miner3D-Gamer/mirl_codec_info) - When codec parsers parse codecs, they keep the value but discard where it actually came from (65% Done, parsing and marshalling support for more formats)

#### Helper

- [Mirl Derive](https://github.com/Miner3D-Gamer/mirl_derive) - Less duplicate code when deriving other crates. Doesn't use nightly. (85% Done. More crates could be supported)

- [Mirl Build Tools](https://github.com/Miner3D-Gamer/mirl_build_tools) - A compile time library currently only used to pretty print a warning page into the console when someone tries to use a mirl crate on stable rust. This crate does not require nightly. (99% Done. Nothing obvious can be improved.)

#### Other

- [Mirl Geometry](https://github.com/Miner3D-Gamer/mirl_geometry) - Focusing on geometry (1-4d) and expressions (0% Done, this needs a total rewrite)

- [Mirl](https://github.com/Miner3D-Gamer/mirl) - The old kitchen sink. A crate with objects I don't know where else to put (50% Done. This lib will dissolve into others when logically possible)

- [Mirl Core](https://github.com/Miner3D-Gamer/mirl_core) - Similar to [mirl](https://github.com/Miner3D-Gamer/mirl), a kitchen sink. Whereas mirl pulls from all other crates here, the other crates pull from here (40% This lib will dissolve into others when logically possible)

- [Mirl Consts]() - Const values. This lib is unreleased and will soon be consumed by [Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) or [Mirl Extensions Math](https://github.com/Miner3D-Gamer/mirl_extensions_math) (10% Done. Only time and byte related consts exist. Needs to be expanded.)

- [Mirl Collections](https://github.com/Miner3D-Gamer/mirl_collections) - A library containing type wrappers with extra functionality, most notably a `BinaryData` struct for file contents (plugin extendible) (75% Done. `NonEmptyVec` needs a rewrite to be more performant.)

- [Mirl Terminal](https://github.com/Miner3D-Gamer/mirl_terminal) - Functions and integrations for better interacting with the console/terminal

None of the libs inexplicitly panic or cause UB, the done-meter is a rough estimation of how close they are to their "true potential" (without being extended by other crates), none will ever really be at a true 100% because perfection doesn't exist.

#### External crates that may see support in the future:

- [Document Features](https://docs.rs/document-features/latest/document_features/) - Copy Cargo.toml metadata into the lib docstring
- [Document Dependencies](https://docs.rs/dep_doc/latest/dep_doc/) - Auto generates docs for dependencies
- [rkyv](https://docs.rs/rkyv/latest/rkyv/) - RAW conversion between bytes and structs
- [Dyn Clone](https://docs.rs/dyn-clone/latest/dyn_clone/) - Make &dyn {trait} cloneable
- [Arbitrary](https://docs.rs/arbitrary/latest/arbitrary) - Conversion between bytes and structs
- [Schemars](https://docs.rs/schemars/latest/schemars/) - More accurate json support than `serde`
- [Borsch](https://docs.rs/borsh/latest/borsh/) - Like `rkyv` but more secure
- [Nano Serde](https://docs.rs/nanoserde/latest/nanoserde/) - Like `serde` but even faster
- [Bevy Reflect](https://docs.rs/bevy/latest/bevy/reflect/) - Edit struct/enum fields from inside code instead of being limited to hardcoding everything

(`mirl_derive` takes care of the codec, enum, and c_compatible flags for all structs/enums. If anyone reading this knows any other libs with easy derives, gladly open a ticket in `mirl_derive`.)

#### All mirl crates support these features(/crates) when logically possible:

> TODO: Provide links to the below mentioned crates

**Core**

- `std` (Default)
- `c_compatible`
- `all` (Enables all flags that add content)

**Codec**

- `serde`
- `bitcode`
- `wincode` (Temporarily disabled due to wincode macro issues. bitcode recommended)
- `zerocopy`
- `compactly`

**Enum**

- `strum`
- `enum_ext`

As well as miri to reassure than no UB occurs

---

## Rustly Concerns

#### Nightly/Features

While "stable" rust already offers a lot of functionality, access to nightly features allows for even greater customization, speed, and usability. Using these libs, I have yet to come across UB issues from the used features.

#### Unsafe

The unsafe keyword is used in plenty of places when it is 100% known that no UB will occur.

Examples would include indexed loops using `list.get_unchecked(idx)` instead `list[idx]` as to avoid the silent overload behaviour of usize list indexing.

#### Ai

Ai assistance has been used in the following three aspects:

- Boilerplate
- Repetitive tasks (Like renaming a bunch of enum values)
- Low level api calls (Everything Microsoft tainted is overly complex so these functions have gone through many iteration to work and be safe)
- Some Helper tools in the `.helper` folder

Everything else is made, planned, and organized by Man power. Please don't try this at home, 

#### Dependency Complexity

A lot of additional crate features are gated behind flags, making sure that only the required dependencies are ever loaded by default.

<sub>The graph of Mirl is quite the net, however as long as it works, it works!</sub>

### `no_std` support

`no_std` support is a side effect, is not a focus.

[Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) is the only crate that has special support for `no_std` as to add back lost functionality.
In the other libs, everything that can be `no_std` and makes sense to be, is.

### Expectations: Configurability, Speed, Usability

(Safety is a given so I won't be mentioning it)

**Customizability**: If you don't like something, replace it.

This also means all structs, enums, traits, and even fields that are normally private are public. (If a field comes with a warning in the docs, you take the safety responsibility upon accessing it directly.)

You have full control.
I shouldn't be the one limiting you.

**Speed**: We like performance, we like when things go fast. There is nothing else to it.

**Usability**: A consistent and logical API as well as QOL is always appreciated. This point is why the crates are in alpha, usability is alright but there is headroom for improvements.

---

<details> 
<summary>Internal Mirl Guidelines</summary>

To reduce chaos and set a quality baseline, there are guidelines every mirl lib needs to follow.

THIS IS A WIP.
DO IMPROVE!

<details>
<summary>cargo.toml</summary>

Crates should be explicitly referencing the workspace in these aspects:

```toml
[package]
edition.workspace = true
repository.workspace = true
license.workspace = true
authors.workspace = true

[lints]
workspace = true
```

Crates should include the following if they use nightly:

Inside `cargo.toml`:

```toml
[build-dependencies]
mirl_build_tools = {workspace = true}
```

and inside `build.rs`:

```rust
//! Check [`mirl_build_tools`] for documentation
fn main() {
    mirl_build_tools::setup()
}
```

</details>

<details>
<summary>Supported crates/flags</summary>

**Required**

> Enums

- `all_enum_extensions`
- `strum`
- `enum_ext`

> Codec

- `all_codecs`
- `serde`
- `wincode`
- `bitcode`
- `zerocopy`
- `wincode`

> Internal

- `c_compatible`
- `all` (Enables all non debug flags)
- `_checking` (Enables all content adding flags as well as debug flags)

**Optional**

> Internal

- `std` (Enabled by default if not optional)

</details>

<details>
<summary>README.md documentation</summary>

**All crates must follow this layout:**

Titles are bold.
Text under titles are normal unless otherwise specified.

- Name (h3) + Version (Bracketed)
- Description (h4)
- Subcrates [Optional] (Linequote)
- Flags {As list with a one line of description, except the flags mentioned in README.md which should be on one line with the unsupported ones crossed out}
- Entry Points (List)
- Purpose {What does this lib do/solve?}
- Disclaimer {What does his lib not do?}
- Planned {What is there to do/improve in the future?}
- Origin {(Trivia) How/Why has this lib come to be?}

</details>
</details>

TODO:

1. Write helper scripts:
   - If a flag exists and a dependency has the same flag, that flag needs to be included
   - Generate all relevant combination of flags and test if each one compiles
2. Impl/Fix all over 100 `TODO` sprinkled throughout the crates
3. Improve the `RawCursor` to `.ico` formatting experience
4. Implement `mirl_geometry` and uncomment all code in the other libs that depend on it
5. Test the cursors system more extensively
6. Check if glfw backend renderers red and blue color channels are swapped
7. dissolve `mirl_consts` into `mirl_extensions_math` and extend it with length units and temperature
8. Check if all `README.md` are up to date quality wise
