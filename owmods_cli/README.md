# owmods-cli

The CLI interface for the mod manager, built using clap.

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
