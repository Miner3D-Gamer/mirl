from typing import TypedDict
import os, subprocess
import json


# TODO: 
# - Make this optionally run continuously, changing the file when either the source or target files change
# - Make program detect values of crates and make them point to the right location
# - Add support for:
# [![Version](https://img.shields.io/crates/v/mirl_macros.svg)](https://crates.io/crates/mirl_macros)
# [![Documentation](https://docs.rs/mirl_macros/badge.svg)](https://docs.rs/mirl_macros)
# [![dependency status](https://deps.rs/repo/github/Miner3D-Gamer/mirl_macros/status.svg)](https://deps.rs/repo/github/Miner3D-Gamer/mirl_macros)
# [![Minimum Supported Rust Version](https://img.shields.io/badge/MSRV-1.99–nightly-blue)](https://github.com/rust-lang/rust/releases/tag/1.99.0)
# [![Changelog](https://img.shields.io/badge/CHANGELOG.md--555.svg)](https://github.com/Miner3D-Gamer/mirl_macros/blob/main/CHANGELOG.md)


template_folder = "./templates"


cwd = os.getcwd()

__this__ = os.path.dirname(__file__)
os.chdir(__this__)


class Website(TypedDict):
    link: str
    nicknames: list[str]
    # name: str


output = subprocess.run("cargo metadata --format-version 1", capture_output=True)
members = json.loads(output.stdout)["workspace_members"]
crates = [m.split("/")[-1].split("#")[0] for m in members]
# [print(m) for m in members]

# quit()


def snake_to_normal_casing(snake: str) -> str:
    return snake.replace("_", " ").title()

def normal_to_snake_casing(normal: str) -> str:
    return normal.lower().replace(" ", "_")


def names_from_segments(segments: list[str], divider: str) -> list[str]:
    output = []
    current = segments[0]
    output.append(current)
    for segment in segments[1:]:
        current += divider + segment
        output.append(current)

    return output


# print(names_from_segments(["a", "b", "c"], "."))

# quit()
default_website = "github.com"

websites: list[Website] = [
    {
        "link": "github.com/Miner3D-Gamer/%s",
        "nicknames": ["git"],
    },
    {
        "link": "crates.io/crates/%s",
        "nicknames": [],
    },
    {
        "link": "docs.rs/crate/%s",
        "nicknames": [],
    },
]
definitions = {}

for crate in crates:
    fancy_name = snake_to_normal_casing(crate)

    crate_names = [fancy_name, crate]
    for w in websites:
        link = w["link"]
        this_is_default = link.split("/")[0] == default_website
        link = link % crate

        definitions[link] = []
        nicknames = w["nicknames"]
        name = link.split("/")[0]
        names = names_from_segments(name.split("."), ".")
        for identifier in names + nicknames + ([""] if this_is_default else []):
            for crate_name in crate_names:
                local_name = crate_name + (
                    f" ({identifier})" if identifier.strip() != "" else ""
                )
                definitions[link].append(local_name)

crate_names


# {print(x, ":", sep=""): [print("\t", z) for z in y] for (x, y) in definitions.items()}
# print(len([item for sublist in definitions.values() for item in sublist]))

insertions = 0

files = os.listdir(template_folder)

for file in files:
    file_path = os.path.join(template_folder, file)
    with open(file_path, "r", encoding="utf-8") as f:
        data = f.read()
    if data[0] == "{":
        config, data = data.split("\n", 1)
        config = json.loads(config)
    else:
        config = {}
    path = config.get("path")
    if path == None:
        print("No path in config:", file_path)
        quit()

    for url, identifiers in definitions.items():
        for identifier in identifiers:
            potential = ["[%s]", "[`%s`]"]
            for p in potential:
                p = p % identifier
                if data.__contains__(p):
                    insertions += 1
                    data += p + f": {url}\n"

    with open(
        os.path.realpath(os.path.join(__this__, path, file)), "w", encoding="utf-8"
    ) as f:
        f.write(data.strip())

print(f"Made {insertions} insertions")
os.chdir(cwd)
