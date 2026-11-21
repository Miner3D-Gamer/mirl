import subprocess
import json
import platform
import os

# TODO: Add a cache. Every run takes too long. Simple hashing of what has and hasn't changed

system = platform.system().lower()

output = subprocess.run(
    "cargo metadata --format-version 1",
    capture_output=True,
    text=True,
    shell=True,
)
if output.returncode != 0:
    print(output.stderr)
    raise SystemExit(output.returncode)

crates = json.loads(output.stdout)["workspace_members"]
crates = [m.split("/")[-1].split("#")[0] for m in crates]

crates.sort()

attributes = {
    # "mirl_buffer": {"requires_std": True},
    "mirl_buffer_interpolation": {"requires_std": True},
    "mirl_std_exposed": {"requires_std": True},
    "mirl_codec_info": {"requires_std": True},
    "mirl_values": {"requires_std": True},
    "mirl_values_core": {"requires_std": True},
    "mirl_gui": {"requires_external_component": True},
    "mirl_system": {"requires_external_component": True},
    "mirl_windowing": {"requires_external_component": True},
    "mirl_windowing_console": {"requires_external_component": "wasm32"},
    "mirl_windowing_minifb": {"requires_external_component": True},
    "mirl_windowing_glfw": {"requires_external_component": True},
    "mirl_windowing_core": {"requires_external_component": "wasm32"},
    "mirl_input": {"requires_external_component": "wasm32"},
    "mirl_metadata": {"extra_command_parameter": "--no-default-features --features serde_json"},
}

tripled = {
    "wasm32-wasip1": {},
    "wasm32-unknown-unknown": {},
    # "x86_64-unknown-none": {"std_unsupported": True},
    "x86_64-unknown-linux-gnu": {"requires_external_component": True},
    "x86_64-pc-windows-msvc": {},
    # "x86_64-pc-windows-gnu": {"requires_external_component": True},
    "aarch64-unknown-linux-gnu": {"requires_external_component": True},
    "aarch64-pc-windows-msvc": {},
    "aarch64-apple-darwin": {"requires_external_component": True},
    "armv7-unknown-linux-gnueabihf": {"requires_external_component": True},
    "i686-unknown-linux-gnu": {"requires_external_component": True},
    "riscv64gc-unknown-linux-gnu": {"requires_external_component": True},
}


def skip_for_reason(target, member, reason):
    print(f"> Skipped {member} on {target} as {reason}")


def requires_external_component_for_target(requirement, target):
    """
    True:
        Skip when the target doesn't match the host platform.

    str:
        Skip when the target contains that substring.

    list[str]:
        Skip when the target contains any listed substring.

    False or None:
        Don't skip.
    """
    if requirement is None or requirement is False:
        return False

    if requirement is True:
        return system not in target

    if isinstance(requirement, str):
        return requirement in target

    if isinstance(requirement, list):
        return any(component in target for component in requirement)

    raise TypeError(f"Invalid requires_external_component value: {requirement!r}")


for target in tripled:
    output = subprocess.run(
        f"rustup target add {target}",
        shell=True,
    )
    if output.returncode != 0:
        print(f"ERROR: Target {target} returned with code {output.returncode}")
        raise SystemExit(output.returncode)

env = os.environ.copy()
env["CARGO_TERM_COLOR"] = "always"

for target, extra in tripled.items():
    std_unsupported = extra.get("std_unsupported", False)
    extra_argument = extra.get("extra_argument", "")

    target_requires_external = extra.get("requires_external_component")

    if requires_external_component_for_target(target_requires_external, target):
        print(
            f"Skipping Target {target} as the target requires "
            "platform-dependent external tools"
        )
        continue

    for member in crates:
        member_info = attributes.get(member, {})

        extra_command_parameter = member_info.get("extra_command_parameter", "")

        member_requires_std = member_info.get("requires_std", False)
        member_requires_external = member_info.get("requires_external_component")

        if requires_external_component_for_target(member_requires_external, target):
            skip_for_reason(
                target,
                member,
                "an external (platform-specific) tool is required "
                "for the compiled target",
            )
            continue

        if std_unsupported and member_requires_std:
            skip_for_reason(
                target,
                member,
                "std is required but not supported",
            )
            continue

        command = (
            f"cargo check --target {target} {extra_argument} "
            f"--package {member} "
            + ("--no-default-features" if std_unsupported else "")
            + " "
            + extra_command_parameter
        )

        print("Executing command:", command)

        output = subprocess.run(
            command,
            capture_output=True,
            text=True,
            env=env,
            shell=True,
        )

        if output.returncode != 0:
            print(output.stdout)
            print(output.stderr)

            print(
                f"ERROR:\n"
                f"Target {target} compiling for {member} "
                f"returned with code {output.returncode}\n"
                f"Using the command: `{command}`"
            )

            infos = {
                "error calling dlltool 'dlltool.exe': program not found": "Follow the instruction guide of "
                "https://stackoverflow.com/questions/79640934/"
                "how-to-get-dlltool-exe-for-rust-gnu-toolchain-on-windows"
            }

            for key, val in infos.items():
                if key in output.stderr:
                    print(f"Tip: {val}")

            raise SystemExit(output.returncode)
