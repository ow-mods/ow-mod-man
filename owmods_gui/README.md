# owmods_gui

> **CURRENTLY A WORK IN PROGRESS!!**

The GUI Version Of The Manager. Created with Tauri, React, Vite, TypeScript, and Pico CSS.

## Installation

Check out the [mods website](https://outerwildsmods.com/mod-manager/) for instructions.

## Building

You'll need the tauri CLI installed, so run:

```sh
cargo install tauri-cli
```

Clone the repo:

```sh
git clone https://github.com/Bwc9876/ow-mod-man/
```

Go into `owmods_gui/frontend` and run:

```sh
pnpm i
```

To install dependencies, then go back to the root of the repo.

And finally you can run a dev environment:

```sh
cargo tauri dev
```

... Or Build:

```sh
cargo tauri build
````

Builds are located in `target/` at the root of the repo.

## build.rs Error in `backend/`

Just create an empty `dist/` folder next to `backend`, tauri expects one to be there from vite but it isn't since you haven't ran it yet.

## Contributing

**Format and lint your code with `pnpm prettify` and `pnpm lint`**.
