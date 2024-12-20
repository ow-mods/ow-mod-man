<!-- markdownlint-disable MD030 MD033 -->

# Outer Wilds Mod Manager GUI

<p align="center">
<a href="https://github.com/ow-mods/ow-mod-man"><img src="https://raw.githubusercontent.com/ow-mods/ow-mod-man/main/.github/assets/logo-gui.png" alt="OWMM Logo"/></a><br/>
The GUI Version Of The Outer Wilds Mod Manager.<br/>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_core">Core</a><b> |</b>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_cli">CLI</a><b> |</b>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_gui"><b>GUI</b></a>
</p>

<hr/>

[![Latest](https://img.shields.io/github/v/release/ow-mods/ow-mod-man)](https://github.com/ow-mods/ow-mod-man/releases/latest)
[![Flathub](https://img.shields.io/flathub/v/com.outerwildsmods.owmods_gui)](https://flathub.org/apps/com.outerwildsmods.owmods_gui)
[![AUR](https://img.shields.io/aur/version/owmods-gui-bin)](https://aur.archlinux.org/packages/owmods-gui-bin)
[![GPL-3 licensed](https://img.shields.io/aur/license/owmods-gui-bin)](https://github.com/ow-mods/ow-mod-man/blob/main/LICENSE)
[![CI JS](https://github.com/ow-mods/ow-mod-man/actions/workflows/ci_js.yml/badge.svg?branch=main)](https://github.com/ow-mods/ow-mod-man/actions/workflows/ci_js.yml)
[![CI RS](https://github.com/ow-mods/ow-mod-man/actions/workflows/ci_rs.yml/badge.svg?branch=main)](https://github.com/ow-mods/ow-mod-man/actions/workflows/ci_rs.yml)
[![Release](https://github.com/ow-mods/ow-mod-man/actions/workflows/release_gui.yml/badge.svg)](https://github.com/ow-mods/ow-mod-man/actions/workflows/release_gui.yml)

The GUI interface for the [Outer Wilds Mod Manager](https://github.com/ow-mods/ow-mod-man), this package is responsible for providing a streamlined way to manage, install, and validate your mods. As well as running the game. This is achieved using the [owmods_core](https://crates.io/crates/owmods_core) package.

## Installation

Check out the [mods website](https://outerwildsmods.com/mod-manager/) for instructions.

### Steam Deck Installation

Go into the Discover app in desktop mode and search "Outer Wilds" and the manager should appear.

## Building

Prerequisites:

- rust
- cargo
- node
- npm

You'll need the tauri CLI installed, so run:

```sh
cargo install tauri-cli
```

Clone the repo:

```sh
git clone https://github.com/ow-mods/ow-mod-man/
```

Go into `owmods_gui/frontend` and run:

```sh
npm i
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
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev librsvg2-dev
```

### build.rs Error in `backend/`

Just create an empty `dist/` folder next to `backend`, tauri expects one to be there from vite but it isn't since you haven't ran it yet.

## Screenshots

![The main screen of the app](https://github.com/ow-mods/ow-mod-man/raw/dev/.github/assets/screenshots/main.png)
![The updates tab allowing you to update mods](https://github.com/ow-mods/ow-mod-man/raw/dev/.github/assets/screenshots/update.png)
![The logs screen when starting the game](https://github.com/ow-mods/ow-mod-man/raw/dev/.github/assets/screenshots/logs.png)
![The settings menu for easy OWML tweaking](https://github.com/ow-mods/ow-mod-man/raw/dev/.github/assets/screenshots/settings.png)
![The about modal](https://github.com/ow-mods/ow-mod-man/raw/dev/.github/assets/screenshots/about.png)
