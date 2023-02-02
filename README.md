# Outer Wilds Mod Manager

> **CURRENT A WORK IN PROGRESS!!**

This is the monorepo for the new [Outer Wilds](https://www.mobiusdigitalgames.com/outer-wilds.html) Mod Manager, completely re-implemented in Rust/Tauri.

## Packages

- [owmods_core](owmods_core): The core library, shared between the CLI and the GUI
- [owmods_cli](owmods_cli): The CLI interface for the manager, made with Clap
- [owmods_gui](owmods_gui): The GUI interface for the manager

## Platform Support

| **Platform** | **Supported** |
|:------------:|:-------------:|
| **Windows**  | ✅             |
| **Linux**    | ✅             |
| **Deck**     | ❓             |
| **MacOS**    | 😐             |
  

## Release Profile

To reduce binary sizes, some features have been enabled that will slow down release compile time, but make a significantly smaller binary.

## Contributing

You'll need rust and cargo.

First:

```sh
git clone https://github.com/Bwc9876/ow-mod-man/
```

Then check each package for guide on running and contributing.

**Please format your code (`cargo fmt`) and lint it (`cargo clippy --fix`)**

Also make sure if you're working on platform-specific behaviour to test it on all platforms.