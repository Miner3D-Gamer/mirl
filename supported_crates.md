## Supported Crates:

> All mirl crates support these crates(/features) when logically possible:

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

## May see support in the future:

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

> TODO: Provide links to the below mentioned crates

---

---
