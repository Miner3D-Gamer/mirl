## Internal Mirl Guidelines

To reduce chaos and set a quality baseline, there are guidelines every mirl lib needs to follow.

THIS IS A WIP.
DO IMPROVE!

<details>
<summary>

## Cargo.toml

</summary>

Crates should be explicitly referencing the workspace in these aspects:

```toml
[package]
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
    mirl_build_tools::setup(`{is nightly required?} (bool)`)
}
```

</details>

<details >
<summary>

## Supported crates/flags

</summary>

All of these are required when logically applicable

> Internal

- `std` (Enabled by default if not optional)
- `c_compatible`
- `all` (Enables all non debug flags)
- `_checking` (Enables `all` as any debug flags)

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

> {Core lib disclaimer if the crate is a core crate}

### Purpose

{What does this lib do/solve?}

<details>
<summary>

### Flags

**Flag Category Name**

- Flag - Description
- ~~Unsupported Flag~~ (Reason unsupported)

{As list with each custom flag having a description, unsupported flags that are in guidelines should be crossed out}

</summary>
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
</details>
