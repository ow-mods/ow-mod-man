# Contributing

To build this project you'll need rust, cargo, and pnpm.

This package is called `owmods_gui` so anytime you want to perform cargo commands on it **do not do it in this folder**, do it from the root of the repo and add `-p owmods_gui` to your cargo command.

Ex: `cargo add tokio` should become `cargo add clap -p owmods_gui`.

## Setup on Linux

Please follow the [tauri docs](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux) for instructions on installing the necessary system packages.

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

This will generate `types.d.ts` in `owmods_gui/frontend/src/types.d.ts`.

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

## Connection Refused Error When Using Protocol Installs

This is a result of your OS falsely thinking the dev version of the manager should handle protocols, causing it to open a window with only the frontend of the tauri application, which will fail. To remedy this simply open a release version of the manager and it will re-register as the handler for the owmods URI.
