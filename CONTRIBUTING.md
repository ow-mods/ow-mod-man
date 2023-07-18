# Contributing

Each package has different prerequisites for compiling/building, please look at the README.md and CONTRIBUTING.md files in each package for more info.

## Commit Names

When performing changes on a specific package, try to prefix the name with the package you're changing, see below:

- For the core package: `[CORE] Made changes to core`
- For the cli package: `[CLI] Made changes to the CLI`
- For the gui package: `[GUI] Made changes to the GUI`
- For multiple packages: `[CORE/GUI] Made changes to both the core and GUI`
- For all packages: `[ALL] Made changes to everything`
- For chores such as updating deps: `[CHORE] Update Deps`
- For meta-related changes such as editing actions or updating READMEs: `[META] Changed CD/CI pipeline`

Also try to tag issues and PRs with the appropriate tags.

## Creating Releases

I tried to automate this process in a way that makes sense but it still needs some manual input

1. **Ensure all versions a up to date** Make sure these are bumped to the versions
   1. `owmods_core/Cargo.toml`
   2. `owmods_cli/Cargo.toml` - **also update the reference to owmods_core if it was updated**
   3. `owmods_gui/backend/Cargo.toml`  - **also update the reference to owmods_core if it was updated**
   4. `xtask/Cargo.toml` - not the end of the world if you don't update thi,s just determines the version shown in man files for the CLI
   5. `owmods_gui/backend/tauri.conf.json` - **PLEASE UPDATE THIS ALONG WITH THE GUI, IT'S WHAT MAKES THE UPDATER WORK!**
2. Merge `dev` to `main` (assuming you have a PR going)
3. Run the "Create Core Release" action
4. After the action completes go to Releases and fill out the draft release with the core's changelog, **do not set this as latest release**, then publish
5. Then another action will run once you publish the release, wait for that to finish
6. Now you can run both the Release CLI and Release GUI workflows, the CLI will probably finish first
7. After the CLI finishes, go to releases and fill out the draft release, **do not set this as latest release**, then publish
8. After the GUI finished, go to releases and fill out the draft release, **set this as latest**, then publish
9. An action will run after each release, they don't require any further input
10. Finally, [the flatpak repo](https://github.com/flathub/com.outerwildsmods.owmods_gui) should automatically create a PR for updating after about an hour, sadly because of how flathub is setup you can't merge this! @ me on discord and I'll handle it.
