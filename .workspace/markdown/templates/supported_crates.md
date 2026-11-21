{"path": "./docs"}

# Crates [Mirl] has special support for

> All mirl crates support these crates(/features) when logically possible:

**Core**

- `std` (Default)
- `c_compatible`
- `all` (Enables all flags that add content)

**Codec**

- `serde`
- `bitcode`
- `wincode` (Temporarily disabled due to wincode macro issues. `bitcode` recommended as alternative.)
- `zerocopy`
- `compactly`(Temporarily disabled due to wincode macro issues.)

**Enum**

- `strum`
- `enum_ext`

## Currently Testing:

- [Bevy Reflect](https://docs.rs/bevy/latest/bevy/reflect/) - Edit struct/enum fields from inside code instead of being limited to hardcoding everything
- [Enum Ordinalize](https://docs.rs/enum-ordinalize/latest/enum_ordinalize/) - More enum support
- [enum_map](https://docs.rs/enum-map/latest/enum_map/) - Lets you create hashmaps from enums
- [rkyv](https://docs.rs/rkyv/latest/rkyv/) - RAW conversion between bytes and structs
- [parity_scale_codec](https://docs.rs/parity-scale-codec/latest/parity_scale_codec/) - And another serializing/deserializing crate

## May see support in the future:

- [Document Features](https://docs.rs/document-features/latest/document_features/) - Copy Cargo.toml metadata into the lib docstring
- [Document Dependencies](https://docs.rs/dep_doc/latest/dep_doc/) - Auto generates docs for dependencies
- [Dyn Clone](https://docs.rs/dyn-clone/latest/dyn_clone/) - Make &dyn {trait} cloneable
- [DynEq](https://docs.rs/dyn-eq/latest/dyn_eq/) - Make &dyn {trait} comparable
- [Schemars](https://docs.rs/schemars/latest/schemars/) - More accurate json support than `serde`
- [Borsch](https://docs.rs/borsh/latest/borsh/) - Like `rkyv` but more secure
- [Nano Serde](https://docs.rs/nanoserde/latest/nanoserde/) - Like `serde` but even faster
- [Borsh](https://docs.rs/borsh/latest/borsh/) - Another serializing/deserializing crate
- [Müsli](https://docs.rs/musli/latest/musli/) - And another serializing/deserializing
- [minicbor](https://docs.rs/minicbor/latest/minicbor/) - You'd never gonna believe this
- [num_enum](https://docs.rs/num_enum/latest/num_enum/) - Turn an enum into a number and a number into an enum
- [typed_builder](https://docs.rs/typed-builder/latest/typed_builder/) - Creates builder functions for a given struct
- [bon](https://docs.rs/bon/latest/bon) - Creates builder functions for structs and functions

(`mirl_macros` takes care of the codec, enum, and c_compatible flags for all structs/enums. If anyone reading this knows any other libs with easy derives, gladly open a ticket in `mirl_macros`.)

> TODO: Provide links to the supported crates stated above
> TODO: Automate the inclusions of these crates so no one needs to manually define the required flags and dependencies

---

---
