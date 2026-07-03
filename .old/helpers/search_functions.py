import os, subprocess

this = os.path.dirname(__file__)
true = True
false = False


def get_public_api(name: str, filter_name: str, cache_folder: str) -> list[str]:
    path = os.path.join(this, cache_folder, name + ".txt")
    if os.path.exists(path):
        with open(path, "r", encoding="utf-8") as f:
            return f.read().split("\n")
    else:
        os.system(f"cargo api")
        if name == filter_name:
            command = ["cargo", "public-api", "--features", "all"]
        else:
            command = ["cargo", "public-api", "-p", name]
        result = subprocess.run(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            encoding="utf-8",
            errors="replace",
        )
        file = (result.stdout or "") + (result.stderr or "")
        if result.returncode != 0:
            raise RuntimeError("Unable to get public-api", file)
        lines = list(set(file.split("\n")))
        with open(path, "w", encoding="utf-8") as f:
            f.write("\n".join(lines))
        return lines


def filter_for_struct_inputs_and_enums(
    data: list[str], search: dict[str, str | list[str]]
) -> tuple[list[str], list[str]]:
    further: list[str] = []

    for line in data:
        if line == "":
            continue
        if line[0] == " ":
            continue

        hit = false
        for _name, allowed_variants in search.items():
            if any(
                [
                    line.startswith(variant)
                    for variant in (
                        allowed_variants
                        if isinstance(allowed_variants, list)
                        else [allowed_variants]
                    )
                ]
            ):
                # print(allowed_variants)
                hit = true
                break
        if hit:
            continue
        further.append(line)

    inputs: list[str] = []
    variants = []
    for x in further:
        if x.__contains__(": "):
            inputs.append(x)
        else:
            variants.append(x)
    return inputs, variants


def get_all_impl_for_struct(
    data: list[str], traits: dict[str, str]
) -> dict[str, list[str]]:

    implementations = {}

    for i in data:
        i = i[4:]
        i = skip_generics(i)
        if not i.startswith("core:"):
            continue

        what = any([x for x in [i.startswith(item + " ") for item in traits.values()]])
        if not what:
            continue
        trait, item = i.split(" for ")

        # print(trait, "-", item, "-", i)
        if implementations.get(item) == None:
            implementations[item] = []
        implementations[item].append(trait)

    unduplicated = {}
    for key, item in implementations.items():
        unduplicated[key] = list(set(item))
    return unduplicated


def skip_generics(text: str) -> str:
    if text.startswith("<"):
        depth = 0
        i = 0
        while i < len(text):
            if text[i] == "<":
                depth += 1
            elif text[i] == ">":
                depth -= 1
                if depth == 0:
                    i += 1
                    break
            i += 1
        text = text[i + 1 :]
    else:
        text = text[1:]

    return text


# def get_function_name(line: str) -> str:
#     values = line.split("::")
#     got = ""
#     for got in values:
#         if got.__contains__("("):
#             got = got.split("(")[0]
#             break
#     if got.__contains__("<"):
#         got = got.split("<")[0]
#     if got == "pub fn ":
#         got = get_function_name(line[8:])
#         # print("Special:", got, line[8:])
#     return got
