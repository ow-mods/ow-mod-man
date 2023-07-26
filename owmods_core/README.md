<!-- markdownlint-disable MD030 MD033 -->

# Outer Wilds Mod Manager Core Package

<p align="center">
<a href="https://github.com/ow-mods/ow-mod-man"><img src="https://raw.githubusercontent.com/ow-mods/ow-mod-man/main/.github/assets/logo-core.png" alt="OWMM Logo"/></a><br/>
The core library for the Outer Wilds Mod Manager.<br/>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_core"><b>Core</b></a><b> |</b>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_cli">CLI</a><b> |</b>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_gui">GUI</a>
</p>

<hr/>

[![crates.io](https://img.shields.io/crates/v/owmods_core.svg)](https://crates.io/crates/owmods_core)
[![Documentation](https://docs.rs/owmods_core/badge.svg)](https://docs.rs/owmods_core)
[![GPL-3 licensed](https://img.shields.io/crates/l/owmods_core.svg)](https://github.com/ow-mods/ow-mod-man/blob/main/LICENSE)
[![CI RS](https://github.com/ow-mods/ow-mod-man/actions/workflows/ci_rs.yml/badge.svg?branch=main)](https://github.com/ow-mods/ow-mod-man/actions/workflows/ci_rs.yml)
[![Release](https://github.com/ow-mods/ow-mod-man/actions/workflows/release_core.yml/badge.svg)](https://github.com/ow-mods/ow-mod-man/actions/workflows/release_core.yml)

The core library for the [Outer Wilds Mod Manager](https://github.com/ow-mods/ow-mod-man), this package is responsible for basically everything from fetching the db to downloading mods to validating local mods to launching the game.  

## Usage

`cargo add owmods_core`

### Analytics

In order to send analytics events you'll need to set the `ANALYTICS_API_KEY` environment variable before compilation.

### Configuration

This package operates using the global manager configuration located in `~/.local/share/ow-mod-man` (and similar on other OSs).

You can change these paths by passing a different path in a `Some` variant to `Config::get` and `Config::default`.

You can change the OWML path by simply passing a different path to `OWMLConfig::get`

## Building

To build this package locally clone the repo `git clone https://github.com/ow-mods/ow-mod-man` and run `cargo build -p owmods_core`
