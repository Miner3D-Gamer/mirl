import json
import subprocess
from typing import Any, Optional

search: dict[str, str | list[str]] = {
    "functions": ["pub fn", "pub const fn", "pub unsafe fn", "pub unsafe system fn"],
    "implementations": "impl",
    "traits": "pub trait",
    "macros": "pub macro",
    "constants": "pub const",
    "types": "pub type",
    "structs": "pub struct",
    "enum": "pub enum",
    "modules": "pub mod",
    "statics": "pub static",
    "import": "pub use",
}
REQUIRED_TRAITS = ["Debug", "Clone", "Copy", "PartialEq", "Eq", "Default", "Hash"]
TRAITS = {
    "Clone": "core::clone::Clone",
    "Default": "core::default::Default",
    "Debug": "core::fmt::Debug",
    "Copy": "core::marker::Copy",
    "PartialEq": "core::cmp::PartialEq",
    "Eq": "core::cmp::Eq",
    "Hash": "core::hash::Hash",
}
patcher = {
    "alloc": "core::alloc",
    "f32": "core::f32",
    "f64": "core::f64",
    "f128": "core::f128",
    "f16": "core::f16",
    "u8": "core::u8",
    "u16": "core::u16",
    "u32": "core::u32",
    "u64": "core::u64",
    "u128": "core::u128",
    "usize": "core::usize",
    "i8": "core::i8",
    "i16": "core::i16",
    "i32": "core::i32",
    "i64": "core::i64",
    "i128": "core::i128",
    "isize": "core::isize",
    "bool": "core::bool",
    "char": "core::char",
    "str": "core::str",
    "array": "core::array",
    "slice": "core::slice",
    "ptr": "core::ptr",
    "option": "core::option",
    "result": "core::result",
    "iter": "core::iter",
    "ops": "core::ops",
    "cmp": "core::cmp",
    "fmt": "core::fmt",
    "hash": "core::hash",
    "marker": "core::marker",
    "mem": "core::mem",
    "clone": "core::clone",
    "convert": "core::convert",
    "default": "core::default",
}
true = True
false = False
from search_functions import (
    filter_for_struct_inputs_and_enums,
    get_public_api,
    get_all_impl_for_struct,
)
import os

this = os.path.dirname(__file__)
cache_folder = "z_cache"

filter_name = "mirl"

########################################################################

data: list[str] = get_public_api("mirl", filter_name, cache_folder)


impl_lines = [x for x in data if x.startswith("impl")]

collected_impl: list[str] = []


# [print(x) for x in inputs]

inputs, variants = filter_for_struct_inputs_and_enums(data, search)
del data

new_temp: dict[str, list[str]] = {}

# TODO: Add support for sub types (inside <>): `core::option::Option<(f32, f32)>`
for x in inputs:
    name, type = x.split(": ")
    name = name[4:]
    if type.startswith("("):
        type = type[1:-1].split(", ")
    else:
        type = [type]
    for patch_key, patch_val in patcher.items():
        for idx, t in enumerate(type):
            if t.startswith(patch_key):
                type[idx] = patch_val + t[patch_key.__len__() :]
    if new_temp.__contains__(name):
        raise BaseException
    new_temp[name] = type
    


def flatten(lst: list[list | Any]) -> list[Any]:
    final = []
    for item in lst:
        if (
            isinstance(item, (list, tuple))
            or hasattr(item, "__iter__")
            and not isinstance(item, str)
        ):
            final.extend(flatten(list(item)))
        else:
            final.append(item)
    return final


def get_crate_name_of_item(item: str) -> Optional[str]:
    if not item.__contains__("::"):
        return None
    return item.split("::")[0]


required_crates: list[str] = [
    x
    for x in list(
        set(flatten([(get_crate_name_of_item(y) for y in x) for x in new_temp.values()]))
    )
    if not x is None
]

print(required_crates)


stuff = {}

for crate in required_crates:
    api = get_public_api(crate, filter_name, cache_folder)
    derives = get_all_impl_for_struct(api, TRAITS)
    stuff[crate] = derives

this_structs = get_all_impl_for_struct(
    get_public_api(filter_name, filter_name, cache_folder), TRAITS
)

for struct, existing in this_structs.items():
    print(struct)
    all_inputs = list(set(flatten([x[1] for x in new_temp.items() if x[0].startswith(struct)])))
    print(all_inputs)