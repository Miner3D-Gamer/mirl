import subprocess
from typing import Any, Optional
from collections import defaultdict
import json

filter_name = "mirl"

result = subprocess.run(
    ["cargo", "public-api", "--features", "all"],
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    text=True,
    encoding="utf-8",
    errors="replace",
)
print("Processing")
file = (result.stdout or "") + (result.stderr or "")

data = file.split("\n")


search = {
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
}

collected_impl: list[str] = []

found = {}

further = []
for name, allowed_variants in search.items():
    found[name] = {}
    if isinstance(allowed_variants, str):
        found[name] = 0
    else:
        found[name][">total"] = 0
        for variant in allowed_variants:
            found[name][variant] = 0

blacklisted_functions = [
    "hash",
    "fmt",
    "eq",
    "deref",
    "deref_mut",
    "borrow",
    "borrow_mut",
    "type_id",
    "to_owned",
    "clone_into",
    "try_into",
    "try_from",
    "into",
    "try_into_value",
    "equivilant",
    "drop",
    "clone",
    "or",
    "and",
    "Hasher>",
    "repeat_value",
    "init",
    "equivalent",
    "pointer",
    "mut_pointer",
    "Copy>",
    "vzip",
    "clone_to_uninit",
    "le",
    "cmp",
    "gt",
    "lt",
    "ge",
]


def get_function_name(line: str) -> str:
    values = line.split("::")
    for got in values:
        if got.__contains__("("):
            got = got.split("(")[0]
            break
    if got.__contains__("<"):
        got = got.split("<")[0]
    if got == "pub fn ":
        got = get_function_name(line[8:])
        # print("Special:", got, line[8:])
    return got


for line in data:
    if line == "":
        continue
    if line[0] == " ":
        continue
    hit = False
    for name, allowed_variants in search.items():
        if isinstance(allowed_variants, str):
            singular = True
            allowed_variants = [allowed_variants]
        else:
            singular = False

        for variant in allowed_variants:
            if line.startswith(variant):
                for variant in allowed_variants:
                    if line.__contains__(variant):
                        if name == "functions":
                            # print(line)
                            function_name = get_function_name(line)
                            if function_name in blacklisted_functions:
                                break
                            # print("'%s'" % function_name)
                        if variant == "impl":
                            collected_impl.append(line)
                        if singular:
                            found[name] += 1
                        else:
                            found[name][variant] += 1
                            found[name][">total"] += 1
                        hit = True
                        break
                break
        if hit:
            break

    if hit:
        continue

    further.append(line)


# print("\n".join(collected_impl))


found["struct_fields"] = len(further)


self_impl: list[str] = []
other_impl: list[tuple[str, str]] = []
other_optional_impl: list[tuple[str, str, str]] = []

for impl in collected_impl:
    impl = impl[4:]
    if impl.startswith("<"):
        depth = 0
        i = 0
        while i < len(impl):
            if impl[i] == "<":
                depth += 1
            elif impl[i] == ">":
                depth -= 1
                if depth == 0:
                    i += 1
                    break
            i += 1
        impl = impl[i:]
    else:
        impl = impl[1:]
    impl = impl.lstrip(" !")

    if not impl.__contains__(" for "):
        self_impl.append(impl)
        continue

    trait, more = impl.split(" for ")
    if not more.__contains__(" where "):
        other_impl.append((trait, more))
        continue

    struct, condition = more.split(" where ")
    other_optional_impl.append((trait, struct, condition))

doesnt_count = [
    "core::marker::Copy",
    "core::fmt::Debug",
    "core::cmp::Eq",
    "core::cmp::PartialEq",
    "core::clone::Clone",
    "core::marker::Freeze",
    "core::marker::Sync",
    "core::marker::Unpin",
    "core::panic::unwind_safe::RefUnwindSafe",
    "core::panic::unwind_safe::UnwindSafe",
    "core::marker::StructuralPartialEq",
    "core::marker::Send",
    "core::cmp::PartialOrd",
    "core::cmp::Ord",
    "core::hash::Hash",
    "core::default::Default",
]
my_impl = ["mirl", "core", "num_traits"]

blacklist = ["crossbeam_epoch", "tracing", "either", "strum", "serde"]

impls = {}
inside = 0
outside = 0
self = "mirl"

for i in other_impl:
    if i[0] in doesnt_count:
        continue
    if any([i[0].startswith(x) for x in blacklist]):
        continue
    if not any([i[0].startswith(x) for x in my_impl]):
        print(f"Impl '{i[0]}' for '{i[1]}'")
        quit()
    # print(f"Impl '{i[0]}' for '{i[1]}'")
    crate = i[0].split("::")[0]
    if impls.get(crate) == None:
        impls[crate] = 0
    impls[crate] += 1
    if i[1].startswith(self):
        inside += 1
    else:
        outside += 1


def divide_dict_values(d, x):
    for key, value in d.items():
        if isinstance(value, dict):
            divide_dict_values(value, x)
        elif isinstance(value, (int, float)):
            d[key] = value / x


def mul_dict_values(d, x):
    for key, value in d.items():
        if isinstance(value, dict):
            mul_dict_values(value, x)
        elif isinstance(value, (int, float)):
            d[key] = value * x


found["implementations"] = {
    ">total": inside + outside,
    "origin": impls,
    "for_structs_inside": inside,
    "for_structs_outside": outside,
}
from copy import deepcopy

to_be_divided = deepcopy(found)

days = 355
year = 365

divide_dict_values(to_be_divided, days)
to_be_multiplied = deepcopy(to_be_divided)
mul_dict_values(to_be_multiplied, year)

print(json.dumps(found, indent=4))
print(json.dumps(to_be_divided, indent=4))
print(json.dumps(to_be_multiplied, indent=4))


def add(dict) -> int:
    if isinstance(dict, int):
        return dict
    total = 0

    for key, item in dict.items():
        if isinstance(item, int):
            total += item
        else:
            total += item[">total"]
    return total

for_percentage = deepcopy(found)
for_percentage.pop("struct_fields")
for_percentage.pop("modules")
total = add(for_percentage)

for key, items in for_percentage.items():
    if isinstance(items, int):
        amount = items
    else:
        amount = items[">total"]

    percentage = amount / total
    print("%s is taking up %s percent" % (key, percentage * 100))
