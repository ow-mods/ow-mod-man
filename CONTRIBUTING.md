# Contributing

Each package has different prerequisites for compiling/building, please look at the README.md and CONTRIBUTING.md files in each package for more info.  
This file is for general information about contributing to the project.

## Useful Tips

### Use the CLI

When testing core functionality, it's much easier to use the CLI than the GUI, you can use the CLI to test the core functionality without having to worry about the GUI being weird. Just do `cargo run -p owmods_cli -- <args>` to run the CLI.  
The core also has a test suite that you can run with `cargo test -p owmods_core`.

### log_client

You can use the `log_client` xtask for testing how the manager receives logs from the game:

```sh
cargo run -p owmods_cli -- log_server
```

Then in another terminal:

```sh
cargo xtask log_client 12345
```

Replace 12345 with the port you see in the first terminal (or in the GUI if you're testing that).

Then simply enter a message and press enter to send it to the manager. This doesn't support sending different types of messages yet, but it's good enough for testing. You can always edit `xtask/log_client.rs` to add more functionality / just change it.

### log_spammer

You can use the log_spammer xtask for testing how the manager handles a lot of logs:

```sh
cargo run -p owmods_cli -- log_server
```

Then in another terminal:

```sh
cargo xtask log_spammer 12345 0.001
```

Replace 12345 with the port you see in the first terminal (or in the GUI if you're testing that).
Replace 0.001 with the interval between each log in seconds.

Just a warning, on the GUI don't set the interval too low because it might die.

### NO_GAME env var

You can set `NO_GAME` equal to `TRUE` and then compile to skip the game launching code, this is useful for testing the GUI without having to launch the game.

## Commit Names

When performing changes on a specific package, try to prefix the name with the package you're changing, see below:

- For the core package: `[CORE] Made changes to core`
- For the CLI package: `[CLI] Made changes to the CLI`
- For the GUI package: `[GUI] Made changes to the GUI`
- For multiple packages: `[CORE/GUI] Made changes to both the core and GUI`
- For all packages: `[ALL] Made changes to everything`
- For chores such as updating deps: `[CHORE] Update Deps`
- For meta-related changes such as editing actions or updating READMEs: `[META] Changed CD/CI pipeline`

Also, could you try to tag issues and PRs with the appropriate tags?

## Creating Releases

I tried to automate this process in a way that makes sense but it still needs some manual input

1. **Ensure all versions are up to date** Make sure these are bumped to the versions
   1. `owmods_core/Cargo.toml`
   2. `owmods_cli/Cargo.toml` - **also update the reference to owmods_core if it was updated**
   3. `owmods_gui/backend/Cargo.toml`  - **also update the reference to owmods_core if it was updated**
   4. `xtask/Cargo.toml` - not the end of the world, if you don't update this, determines the version shown in man files for the CLI
   5. `owmods_gui/backend/tauri.conf.json` - **PLEASE UPDATE THIS ALONG WITH THE GUI, IT'S WHAT MAKES THE UPDATER WORK!**
2. Merge `dev` to `main` (assuming you have a PR going)
3. Run the "Create Core Release" action
4. After the action completes go to Releases and fill out the draft release with the core's changelog, **do not set this as the latest release**, then publish
5. Then another action will run once you publish the release, wait for that to finish
6. Now you can run both the Release CLI and Release GUI workflows, the CLI will probably finish first
7. After the CLI finishes, go to releases and fill out the draft release, **do not set this as the latest release**, then publish
8. After the GUI is finished, go to releases and fill out the draft release, **set this as latest**, then publish
9. An action will run after each release, they don't require any further input
10. Finally, [the flatpak repo](https://github.com/flathub/com.outerwildsmods.owmods_gui) should automatically create a PR for updating after about an hour. Only certain people have access to the repo and can merge, so if you don't, @ me on Discord and I'll handle it.
