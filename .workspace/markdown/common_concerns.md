# Rustly Concerns

## Nightly/Features

While "stable" rust already offers a lot of functionality, access to nightly features allows for even greater customization, speed, and usability. Using these libs, I have yet to come across any UB issues from the used features.

## Unsafe

The unsafe keyword is used in plenty of places when it is 100% known that no UB will occur.

Examples would include indexed loops using `list.get_unchecked(idx)` instead `list[idx]` as to avoid the silent overload behaviour of usize list indexing.

## Ai

Ai assistance has been used in the following three aspects:

- Boilerplate
- Repetitive tasks (Like renaming a bunch of enum values)
- Low level api calls (Everything Microsoft tainted is overly complex so these functions have gone through many iteration to work and be safe)
- Some Helper tools in the `.helper` folder as their quality is not as significant

Everything else is made, planned, and organized by Man. Please don't try this at home,

## Crate/Relation Complexity

A lot of additional crate features are gated behind flags, making sure that only the required dependencies are ever loaded by default.

This may be a matter to be looked into as flags are not always imminently clear to users.

<sub>
It is not recommended to use any `core` crate as they may change or dissolve in the future more rapidly than the others.
</sub>

<sub>
The graph of Mirl is quite the net, however as long as it works, it works!</sub>

## `no_std` support

`no_std` support is a side effect, is not a focus.

[Mirl Extensions](https://github.com/Miner3D-Gamer/mirl_extensions) is the only crate that has special support for `no_std` as to add back lost functionality.
In the other libs, everything that can be `no_std` and makes sense to be, is.

Performance/Customizability sacrifices will not be made to add more support to `no_std`
