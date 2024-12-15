<!-- markdownlint-disable MD030 MD033 -->

# Outer Wilds Mod Manager

<p align="center">
<a href="https://github.com/ow-mods/ow-mod-man"><img src="https://raw.githubusercontent.com/ow-mods/ow-mod-man/main/.github/assets/logo.png" alt="OWMM Logo"/></a><br/>
This is the monorepo for the new <a href="https://www.mobiusdigitalgames.com/outer-wilds.html">Outer Wilds</a> Mod Manager.<br/>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_core">Core</a><b> |</b>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_cli">CLI</a><b> |</b>
<a href="https://github.com/ow-mods/ow-mod-man/tree/main/owmods_gui">GUI</a>
</p>

## Packages

- [owmods_core](owmods_core): The core library, shared between the CLI and the GUI
- [owmods_cli](owmods_cli): The CLI interface for the manager, made with clap
- [owmods_gui](owmods_gui): The GUI interface for the manager, made with tauri

## Platform Support

| **Platform** | **Supported**  |
| :----------: |  :-----------: |
| **Windows**  |      ✅        |
|  **Linux**   |      ✅        |
|   **Deck**   |      ✅        |
|   **Mac**    |      ❌*       |

You'll want to check out the [Help document](owmods_gui/HELP.md) for platform-specific instruction and caveats.

\* Currently not native, but you can [check out this Reddit post](https://www.reddit.com/r/outerwilds/comments/1b9ysbm/outer_wilds_is_playable_on_macos_with_an_m1/) by u/FoxGray for a way to run it.

## Related Repos

- [Flatpak Repo](https://github.com/flathub/com.outerwildsmods.owmods_gui)

## Credits

- Main Author: [Bwc9876](https://github.com/Bwc9876)
- Old Mod Manager and the rest of the OW mods infrastructure author: [Raicuparta](https://github.com/Raicuparta)
- Contributors:
  - [Raicuparta](https://github.com/Raicuparta)
  - [JohnCorby](https://github.com/JohnCorby)
  - [dgarroDC](https://github.com/dgarroDC)
  - [MegaPiggy](https://github.com/MegaPiggy)
- Testing:
  - [Locochoco](https://github.com/Locochoco)
  - [JohnCorby](https://github.com/JohnCorby)
- Help with Linux Support:
  - [JohnCorby](https://github.com/JohnCorby)
  - [BUNN1E5](https://github.com/BUNN1E5)
  - [Locochoco](https://github.com/Locochoco)
  - [JSpoonBaker](https://github.com/Spoonbaker)
- Translations:
  - Chinese: [SmallGarfield](https://github.com/xiaojiafei520)
  - Vietnamese: [KNNFx](https://github.com/KNNFx)
  - Japanese: [orclecle](https://github.com/TRSasasusu)
  - French: [xen-42](https://github.com/xen-42)

And the support of the [Outer Wilds Modding Discord](https://discord.com/invite/wusTQYbYTc)
