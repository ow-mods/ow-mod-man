<!-- markdownlint-disable MD030 MD033 -->

# owmods_gui

<p align="center">
<img src="https://raw.githubusercontent.com/Bwc9876/ow-mod-man/main/owmods_gui/frontend/src/assets/images/logo.png" alt="OWMM Logo"/><br/>
The GUI Version Of The Manager.<br/>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/dev/owmods_core">Core</a><b> |</b>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/dev/owmods_cli">CLI</a><b> |</b>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/dev/owmods_gui"><b>GUI</b></a>
</p>

The GUI Version Of The Manager. Created with Tauri, React, Vite, TypeScript, and Pico CSS.

## Installation

Check out the [mods website](https://outerwildsmods.com/mod-manager/) for instructions.

## Building

Prerequisites:

- rust
- cargo
- pnpm

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
```

Builds are located in `target/` at the root of the repo.

### On Debian

On debian you need to install some dependencies with apt:

```sh
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev
```

### build.rs Error in `backend/`

Just create an empty `dist/` folder next to `backend`, tauri expects one to be there from vite but it isn't since you haven't ran it yet.
