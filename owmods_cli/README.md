# owmods-cli

The CLI interface for the mod manager, built using clap.

## Usage

This is just the output of `owmods help`, but im guessing if you're here you can't use it for some reason.

```txt
A CLI Tool To Manage OWML Mods

Usage: owmods [OPTIONS] <COMMAND>

Commands:
  version      Print Version
  setup        Install/Update OWML (default installs to %APPDATA%/ow-mod-man/OWML)
  alert        View the current database alert (if there is one)
  update       Updates all mods
  list         List local (installed) or remote (in the database) mods
  info         View info about a specific mod
  enable       Enable a mod (use -r to enable dependencies too)
  disable      Disable a mod (use -r to disable dependencies too)
  install      Install a mod (use -r to auto-install dependencies)
  install-zip  Install a mod from a .zip file (-r not supported)
  install-url  Install a mod from a URL (-r not supported)
  uninstall    Uninstall a mod (use -r to uninstall dependencies too)
  export       Export enabled mods to stdout as JSON
  import       Import mods from a .json file (installs if not there, enables if already installed)
  run          Run the game
  open         Quickly open something
  readme       Open the readme for a mod
  validate     Validate local mods for missing dependencies and conflicts
  help         Print this message or the help of the given subcommand(s)

Options:
  -r, --recursive                 Apply the action recursively (to all dependencies)
      --settings <SETTINGS_PATH>  Override the settings file
      --debug                     Enable debug output
  -h, --help                      Print help
  -V, --version                   Print version
```

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
