# Contributing

To build this project you'll need rust and cargo.

This package is called `owmods_cli` so anytime you want to perform cargo commands on it **do not do it in this folder**, do it from the root of the repo and add `-p owmods_cli` to your cargo command.

Ex: `cargo add clap` should become `cargo add clap -p owmods_cli`.

## Formatting & Linting

Please format and lint your code before pushing:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
```

Git hooks are setup to run clippy on every commit, meaning they may take longer.
