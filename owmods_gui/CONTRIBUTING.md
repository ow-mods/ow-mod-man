# Contributing

To build this project you'll need rust, cargo, and pnpm.

This package is called `owmods_gui` so anytime you want to perform cargo commands on it **do not do it in this folder**, do it from the root of the repo and add `-p owmods_gui` to your cargo command.

Ex: `cargo add tokio` should become `cargo add clap -p owmods_gui`.

## pnpm

The frontend for this package is made with TS so you need to install related dependencies. First cd in to `owmods_gui/frontend`, then run `pnpm i`

## Typeshare

Upon editing any structs marked with `#[typeshare]`, you'll need to regenerate TypeScript bindings.

To do this, you need to install the typeshare cli:

```sh
cargo install typeshare-cli
```

Then run the `gen-types` pnpm command:

```sh
cd owmods_gui/frontend
pnpm gen-types
```

This will generate `types.d.ts` in `owmods_gui/frontend/src/types.d.ts`, **make sure to format this file with prettier (`pnpm prettify`)**.

## Formatting & Linting

Please format and lint your code before pushing:

```sh
cargo fmt
cargo lint
```

And lint and format the frontend as well:

```sh
cd owmods_gui/frontend
pnpm lint
pnpm prettify
```

Git hooks are setup to run clippy on every commit, meaning they may take longer.
