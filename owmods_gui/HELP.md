# Help

This file contains common questions for the manager.

## Table of Contents

- [Help](#help)
  - [Table of Contents](#table-of-contents)
  - [How do I use this?](#how-do-i-use-this)
  - [How do I uninstall it?](#how-do-i-uninstall-it)
  - [The game won't launch on Linux](#the-game-wont-launch-on-linux)
    - [On Flatpak](#on-flatpak)
  - [Steam deck game mode won't work](#steam-deck-game-mode-wont-work)
  - [I made a mod! How do I submit it?](#i-made-a-mod-how-do-i-submit-it)
  - [My issue isn't listed here](#my-issue-isnt-listed-here)

## How do I use this?

- Download the Outer Wilds Mod Manager installer [from the latest release](https://github.com/Bwc9876/ow-mod-man/releases/latest);
- Run the downloaded .msi (you might need to ignore some Chrome / Windows warnings);
- Shortcuts are added to the desktop and start menu, use them to run the manager;
- Run the manager;
- Install OWML;
- Head to the "Get Mods" tab to install any mods you want;
- Press the top right play button to launch the game;
- You won't believe what happens next.

## How do I uninstall it?

You can uninstall the Mod Manager by searching for "Add or remove programs" in the start menu (or in the control panel) and then finding "Outer Wilds Mod Manager" in the list. However, this won't uninstall your mods.

To revert the game to its original state, verify the game files' integrity:

- **Steam**: Library > Right-click Outer Wilds > Properties > Local Files > Verify integrity of game files.
- **Epic**: Library > Click three dots under Outer Wilds > Verify.

## The game won't launch on Linux

Please ensure you have [Mono](https://www.mono-project.com/) installed and available on your PATH.

To debug issues with mono set the `MONO_LOG_LEVEL` variable to `debug`. and look at the logs.

### On Flatpak

Also, ensure the game is located in `~/.steam/steam/steamapps/common/Outer Wilds`, otherwise the manager won't be able to find the game as it only has access to that folder.

## Steam deck game mode won't work

The manager itself can only be launched in desktop mode, but mods will persist after the first run of the game from the manager if you switch to game mode and launch the game.

## I made a mod! How do I submit it?

The mod database is stored in a separate repository. [Go here to find out how to add your mod to the list](https://github.com/ow-mods/ow-mod-db#readme).

## My issue isn't listed here

If you're encountering issues or have questions, please [open an Issue](https://github.com/Bwc9876/ow-mod-man/issues/new/choose).

We also have [a Discord server](https://discord.com/invite/wusTQYbYTc) available if you want to chat.
