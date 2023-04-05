<!-- markdownlint-disable MD030 MD033 -->

# Outer Wilds Mod Manager CLI

<p align="center">
<a href="https://github.com/Bwc9876/ow-mod-man"><img src="https://raw.githubusercontent.com/Bwc9876/ow-mod-man/main/owmods_gui/frontend/src/assets/images/logo.png" alt="OWMM Logo"/></a><br/>
The CLI interface for the mod manager, built using clap.<br/>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/main/owmods_core">Core</a><b> |</b>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/main/owmods_cli"><b>CLI</b></a><b> |</b>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/main/owmods_gui">GUI</a>
</p>

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

## Building

This package requires rust and cargo to build.

Run `cargo build -p owmods_cli --release` from the **root of the repo** to create a release binary.
