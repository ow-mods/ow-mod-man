# Usage with Nix

Currently, `owmods-cli` is in nixpkgs. ([Nixpkgs status](https://search.nixos.org/packages?channel=unstable&type=packages&query=owmod))

Alternatively, you can get the latest version from this repo.

## Flakes

The flake provides an overlay and the packages `owmods-cli` and `owmods-gui`.
```nix
ow-mod-man = {
  url = "github:ow-mods/ow-mod-man/dev";
  inputs.nixpkgs.follows = "nixpkgs";
};
```
You can then reference `ow-mod-man.packages.<system>.owmods-<gui/cli>`, or use the overlay, for example:
```nix
nixpkgs.overlays = [ inputs.ow-mod-man.overlays.default ]
```

## Without flakes
If you can't or don't want to use flakes, you can use [flake-compat](https://github.com/edolstra/flake-compat).

```nix
let
  flake-compat = import (fetchTarball "https://github.com/edolstra/flake-compat/archive/master.tar.gz");
   src = fetchGit {
    url = "https://github.com/ow-mods/ow-mod-man.git";
    ref = "dev";
  };
  ow-mod-man = (flake-compat { inherit src; }).defaultNix;
in ow-mod-man.packages.<system>.owmods-<gui/cli>

```