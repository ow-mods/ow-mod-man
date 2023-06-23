<!-- markdownlint-disable MD030 MD033 -->

# Outer Wilds Mod Manager GUI

<p align="center">
<a href="https://github.com/Bwc9876/ow-mod-man"><img src="https://raw.githubusercontent.com/Bwc9876/ow-mod-man/main/.github/assets/logo-gui.png" alt="OWMM Logo"/></a><br/>
The GUI Version Of The Outer Wilds Mod Manager.<br/>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/main/owmods_core">Core</a><b> |</b>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/main/owmods_cli">CLI</a><b> |</b>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/main/owmods_gui"><b>GUI</b></a>
</p>

<hr/>

[![Latest](https://img.shields.io/github/v/release/Bwc9876/ow-mod-man)](https://github.com/Bwc9876/ow-mod-man/releases/latest)
[![Flathub](https://img.shields.io/flathub/v/com.outerwildsmods.owmods_gui)](https://flathub.org/apps/com.outerwildsmods.owmods_gui)
[![AUR](https://img.shields.io/aur/version/owmods-gui-bin)](https://aur.archlinux.org/packages/owmods-gui-bin)
[![GPL-3 licensed](https://img.shields.io/aur/license/owmods-gui-bin)](https://github.com/Bwc9876/ow-mod-man/blob/main/LICENSE)
[![CI](https://github.com/Bwc9876/ow-mod-man/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Bwc9876/ow-mod-man/actions/workflows/ci.yml)
[![Release](https://github.com/Bwc9876/ow-mod-man/actions/workflows/release_gui.yml/badge.svg)](https://github.com/Bwc9876/ow-mod-man/actions/workflows/release_gui.yml)

The GUI interface for the [Outer Wilds Mod Manager](https://github.com/Bwc9876/ow-mod-man), this package is responsible for providing a streamlined way to manage, install, and validate your mods. As well as running the game. This is achieved using the [owmods_core](https://crates.io/crates/owmods_core) package.

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

## Screenshots

![The main screen of the app](https://github.com/Bwc9876/ow-mod-man/raw/dev/.github/assets/screenshots/main.png)
![The logs screen when starting the game](https://github.com/Bwc9876/ow-mod-man/raw/dev/.github/assets/screenshots/logs.png)
![The about modal](https://github.com/Bwc9876/ow-mod-man/raw/dev/.github/assets/screenshots/about.png)
