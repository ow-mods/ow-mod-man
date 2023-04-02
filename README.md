<!-- markdownlint-disable MD030 MD033 -->

# Outer Wilds Mod Manager

> **CURRENTLY A WORK IN PROGRESS!!**

<p align="center">
<img src="https://raw.githubusercontent.com/Bwc9876/ow-mod-man/main/owmods_gui/frontend/src/assets/images/logo.png" alt="OWMM Logo"/><br/>
This is the monorepo for the new <a href="https://www.mobiusdigitalgames.com/outer-wilds.html">Outer Wilds</a> Mod Manager.<br/>
<a href="owmods_core">Core</a><b> |</b>
<a href="owmods_cli">CLI</a><b> |</b>
<a href="owmods_gui">GUI</a>
</p>

## Packages

- [owmods_core](owmods_core): The core library, shared between the CLI and the GUI
- [owmods_cli](owmods_cli): The CLI interface for the manager, made with clap
- [owmods_gui](owmods_gui): The GUI interface for the manager, made with tauri

## Platform Support

| **Platform** |  **Supported** |
|:------------:|:--------------:|
| **Windows**  | ✅             |
| **Linux**    | ✅*            |
| **Deck**     | 🔜*            |

\* Quantum Space Buddies Currently Has Issues

## Contributing

Each package has different prerequisites for compiling/building, please look at the README.md and CONTRIBUTING.md files in each package for more info.

### Commit Names

When performing changes on a specific package, try to prefix the name with the package you're changin, see below:

- For the core package: `[CORE] Made changes to core`
- For the cli package: `[CLI] Made changes to the CLI`
- For the gui package: `[GUI] Made changes to the GUI`
- For multiple packages: `[CORE/GUI] Made changes to both the core and GUI`
- For all packages: `[ALL] Made changes to everything`
- For chores such as updating deps: `[CHORE] Update Deps`
- For meta-related changes such as editing actions or updating READMEs: `[META] Changed CD/CI pipeline`

Also try to tag issues and PRs with the appropriate tags.
