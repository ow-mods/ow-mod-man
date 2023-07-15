<!-- markdownlint-disable MD030 MD033 -->

# Outer Wilds Mod Manager

> **CURRENTLY A WORK IN PROGRESS!!**

<p align="center">
<a href="https://github.com/Bwc9876/ow-mod-man"><img src="https://raw.githubusercontent.com/Bwc9876/ow-mod-man/main/.github/assets/logo.png" alt="OWMM Logo"/></a><br/>
This is the monorepo for the new <a href="https://www.mobiusdigitalgames.com/outer-wilds.html">Outer Wilds</a> Mod Manager.<br/>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/main/owmods_core">Core</a><b> |</b>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/main/owmods_cli">CLI</a><b> |</b>
<a href="https://github.com/Bwc9876/ow-mod-man/tree/main/owmods_gui">GUI</a>
</p>

## Packages

- [owmods_core](owmods_core): The core library, shared between the CLI and the GUI
- [owmods_cli](owmods_cli): The CLI interface for the manager, made with clap
- [owmods_gui](owmods_gui): The GUI interface for the manager, made with tauri

## Platform Support

| **Platform** |  **Supported**  |
|:------------:|:---------------:|
| **Windows**  | ✅              |
| **Linux**    | ✅              |
| **Deck**     | ✅*             |

\* The manager itself can only be launched in desktop mode, but mods will persist if you switch to game mode and launch the game itself.

## Credits

- Main Author: [Bwc9876](https://github.com/Bwc9876)
- Old Mod Manager and the rest of the OW mods infrastructure author: [Raicuparta](https://github.com/Raicuparta)
- Contributors:
  - [Raicuparta](https://github.com/Raicuparta)
  - [JohnCorby](https://github.com/JohnCorby)
  - [dgarroDC](https://github.com/dgarroDC)
- Testing:
  - [ShoosGun](https://github.com/ShoosGun)
  - [JohnCorby](https://github.com/JohnCorby)
- Help with Linux Support:
  - [JohnCorby](https://github.com/JohnCorby)
  - [BUNN1E5](https://github.com/BUNN1E5)
  - [ShoosGun](https://github.com/ShoosGun)

And the support of the [Outer Wilds Modding Discord](https://discord.com/invite/wusTQYbYTc)
