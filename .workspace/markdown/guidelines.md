## Internal Mirl Guidelines

To reduce chaos and set a quality baseline, there are guidelines every mirl lib needs to follow.

Everything not mentioned is fair game.

THIS IS A WIP.
DO IMPROVE!

<details>
<summary>

## Cargo.toml

</summary>

Crates should be explicitly referencing the workspace in these aspects:

```toml
[package]
rust-version.workspace = true
edition.workspace = true # 2024
license.workspace = true

[lints]
workspace = true
```

Crates must also include the following:

Inside `cargo.toml`:

```toml
[build-dependencies]
mirl_build_tools = {workspace = true}
```

and inside `build.rs`:

```rust
//! Check [`mirl_build_tools::setup`] for documentation
fn main() {
    mirl_build_tools::setup((bool) is nightly required?)
}
```

This setup function does the following:

- Ensure the user is using nightly if required
- Ensures `miri` flag is set when `miri` is used and a `miri` flag exist, same goes for `test`
- Adds `{name}_present` to env so it can be used in `#[cfg({name}_present)]`

</details>

<details>
<summary>

## Supported crates/flags

</summary>

All of these are required when logically applicable

> Internal

- `std` (Enabled by default if not optional)
- `c_compatible` (Marks all structs/enums with `#[repr(C)]` if not already marked otherwise)
- `all` (Enables all non debug flags)
- `_checking` (Enables `all` + any debug flags)

> Enums

- `all_enum_extensions` (Enables all the features below)
- `strum`
- `enum_ext`

> Codec

- `all_codecs` (Enables all the features below)
- `serde`
- `wincode`
- `bitcode`
- `zerocopy`
- `compactly`

When defining the version of a crate, using any prefix like ">=" is not allowed.

</details>

<details>
<summary>

## README.md documentation

</summary>

> All crates must follow this layout:

Titles are h3 (`###`).
Text under titles are normal unless otherwise specified.

# {Name} ({Version})

### {Short Name} - {Description}

> {Core lib disclaimer if the crate is a core crate, who's the parent?}

### Purpose

{What does this lib do/solve?}

<details>
<summary>

### Flags

</summary>

**Flag Category Name**

- Flag - Description
- ~~Unsupported Flag~~ (Reason unsupported)

{As list with each custom flag having a description, unsupported flags that are in guidelines should be crossed out}

</details>

### Entry Points

{Where someone would start using the lib, may include tutorial}

### Disclaimer

{What does his lib not do?}

### Planned

{What is there to do/improve in the future?}

### Origin

{Trivia -> How/Why has this lib come to be?}

</details>

<summary>

## Automation

</summary>

(`cargo install binstall`)

Use `cargo sort` to format the `Cargo.toml` (`cargo binstall sort`)

Use `cargo stale` to detect outdated libs (`cargo binstall sort`)

</details>
