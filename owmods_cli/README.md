<!-- markdownlint-disable MD030 MD033 -->

# Outer Wilds Mod Manager CLI

<p align="center">
<a href="https://github.com/ow-mods/ow-mod-man"><img src="https://raw.githubusercontent.com/ow-mods/ow-mod-man/main/.github/assets/logo-cli.png" alt="OWMM Logo"/></a><br/>
The CLI interface for the Outer Wilds Mod Manager, built using clap.<br/>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_core">Core</a><b> |</b>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_cli"><b>CLI</b></a><b> |</b>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_gui">GUI</a>
</p>

<hr />

[![crates.io](https://img.shields.io/crates/v/owmods_cli.svg)](https://crates.io/crates/owmods_cli)
[![GPL-3 licensed](https://img.shields.io/crates/l/owmods_cli.svg)](https://github.com/ow-mods/ow-mod-man/blob/main/LICENSE)
[![CI RS](https://github.com/ow-mods/ow-mod-man/actions/workflows/ci_rs.yml/badge.svg?branch=main)](https://github.com/ow-mods/ow-mod-man/actions/workflows/ci_rs.yml)
[![Release](https://github.com/ow-mods/ow-mod-man/actions/workflows/release_cli.yml/badge.svg)](https://github.com/ow-mods/ow-mod-man/actions/workflows/release_cli.yml)
[![AUR](https://img.shields.io/aur/version/owmods-cli-bin)](https://aur.archlinux.org/packages/owmods-cli-bin)

The CLI interface for the [Outer Wilds Mod Manager](https://github.com/ow-mods/ow-mod-man), this package is responsible for providing a streamlined way to manage, install, and validate your mods. As well as running the game. This is achieved using the [owmods_core](https://crates.io/crates/owmods_core) package.

## Usage

Run `owmods help` for a list of commands

### Shortcuts

Some command shortcuts exist for convenience

- `install` -> `i`
- `install-zip` -> `iz`
- `install-url` -> `iu`
- `list` -> `ls`
- `update` -> `up`
- `enable` -> `e`
- `disable` -> `d`
- `uninstall` -> `rm`
- `readme` -> `man`

### Autocomplete

How to use value hints and generate shell completions.
Usage with zsh:

```console
owmods generate-completions zsh > /usr/local/share/zsh/site-functions/_owmods
compinit
```

Bash:

```console
owmods generate-completions bash > /usr/local/share/bash-completions/completions/_owmods
compinit
```

Fish:

```console
owmods generate-completions fish > owmods_autocomplete.fish
. ./owmods_autocomplete.fish
```

Check [clap_complete docs](https://docs.rs/clap_complete/latest/clap_complete/shells/enum.Shell.html#variants) for a list of all supported shells.

## Building

This package requires rust and cargo to build.

Run `cargo build -p owmods_cli --release` from the **root of the repo** to create a release binary.
