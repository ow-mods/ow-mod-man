# Help

This file contains common questions for the manager.

## Table of Contents

- [Help](#help)
  - [Table of Contents](#table-of-contents)
  - [How do I use this?](#how-do-i-use-this)
  - [How do I use this on Linux?](#how-do-i-use-this-on-linux)
    - [What About Steam Deck?](#what-about-steam-deck)
  - [How do I uninstall it?](#how-do-i-uninstall-it)
  - [How do I update it?](#how-do-i-update-it)
  - [The game won't launch on Linux](#the-game-wont-launch-on-linux)
    - [On Flatpak](#on-flatpak)
  - [I made a mod! How do I submit it?](#i-made-a-mod-how-do-i-submit-it)
  - [I have a question about modding](#i-have-a-question-about-modding)
  - [I want to contribute to the manager](#i-want-to-contribute-to-the-manager)
  - [My issue isn't listed here](#my-issue-isnt-listed-here)

## How do I use this?

- Download the Outer Wilds Mod Manager installer [from the latest release](https://github.com/ow-mods/ow-mod-man/releases/latest);
- Run the downloaded .msi (you might need to ignore some Chrome / Windows warnings);
- Shortcuts are added to the desktop and start menu, use them to run the manager;
- Run the manager;
- Install OWML;
- Head to the "Get Mods" tab to install any mods you want;
- Press the top right play button to launch the game;
- You won't believe what happens next.

## How do I use this on Linux?

Using the manager on Linux should be easy **proton and wine are not required**. What is required is [Mono](https://www.mono-project.com/). If you're using the Flatpak, AUR, or Nix versions, Mono will be installed automatically.

### What About Steam Deck?

The manager can be installed on Steam Deck in Desktop Mode in the Discover app.

Note that you won't be able to launch the game from the manager when in steam deck's game mode. But so long as you've *pressed* "Launch Game" at least once, you can launch the game directly and it will still be modded.

## How do I uninstall it?

You can uninstall the Mod Manager by searching for "Add or remove programs" in the start menu (or in the control panel) and then finding "Outer Wilds Mod Manager" in the list. However, this won't uninstall your mods.

To revert the game to its original state, verify the game files' integrity:

- **Steam**: Library > Right-click Outer Wilds > Properties > Local Files > Verify integrity of game files.
- **Epic**: Library > Click three dots under Outer Wilds > Verify.

## How do I update it?

The manager will automatically check for updates when you launch it. If an update is available, you will be prompted to download it. On the Flatpak, AUR, and Nix versions, your system's package manager will handle updates.

## The game won't launch on Linux

Please ensure you have [Mono](https://www.mono-project.com/) installed and available on your PATH.

To debug issues with mono set the `MONO_LOG_LEVEL` variable to `debug`. and look at the logs.

On steam deck, keep in mind the game won't launch if you click Launch Game from the manager while in game mode. You need to launch the game directly from the steam deck's game mode.

### On Flatpak

Also, ensure the game is located in `~/.steam/steam/steamapps/common/Outer Wilds`, otherwise the manager won't be able to find the game as it only has access to that folder.

## I made a mod! How do I submit it?

The mod database is stored in a separate repository. [Go here to find out how to add your mod to the list](https://github.com/ow-mods/ow-mod-db#readme).

## I have a question about modding

Please refer to the [OWML documentation](https://owml.outerwildsmods.com) for information about creating mods.

## I want to contribute to the manager

Depending on which package you want to contribute to, please refer to the following:

- [The main CONTRIBUTING (read this first)](https://github.com/ow-mods/ow-mod-man/blob/main/CONTRIBUTING.md)
- [The CONTRIBUTING for the Core package](https://github.com/ow-mods/ow-mod-man/blob/main/owmods_core/CONTRIBUTING.md)
- [The CONTRIBUTING for the GUI](https://github.com/ow-mods/ow-mod-man/blob/main/owmods_gui/CONTRIBUTING.md)
- [The CONTRIBUTING for the CLI](https://github.com/ow-mods/ow-mod-man/blob/main/owmods_cli/CONTRIBUTING.md)

You can also take a look at [The Architecture Doc](https://github.com/ow-mods/ow-mod-man/blob/main/ARCHITECTURE.md) for more info above the manager's behavior.

## My issue isn't listed here

If you're encountering issues or have questions, please [open an Issue](https://github.com/ow-mods/ow-mod-man/issues/new/choose). You can also [view closed issues](https://github.com/ow-mods/ow-mod-man/issues?q=is%3Aissue+is%3Aclosed) to see if your issue has already been resolved.

We also have [a Discord server](https://discord.com/invite/wusTQYbYTc) available if you want to chat.

You can also email `bwc9876@outerwildsmods.com` if your issue is sensitive / security related.
