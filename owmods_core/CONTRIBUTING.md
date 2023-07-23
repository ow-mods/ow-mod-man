# Contributing

To build this project you'll need rust and cargo.

This package is called `owmods_core` so anytime you want to perform cargo commands on it **do not do it in this folder**, do it from the root of the repo and add `-p owmods_core` to your cargo command.

Ex: `cargo add tokio` should become `cargo add clap -p owmods_core`.

## Typeshare

Upon editing any structs marked with `#[typeshare]`, you'll need to regenerate TypeScript bindings for the GUI. See the [typeshare section](https://github.com/ow-mods/ow-mod-man/blob/main/owmods_gui/CONTRIBUTING.md#Typeshare) of the GUI contributing file for more info.

## Tests

To run tests run `cargo test -p owmods_core`

## Formatting & Linting

Please format and lint your code before pushing:

```sh
cargo fmt
cargo lint
```

Git hooks are setup to run clippy on every commit, meaning they may take longer.
