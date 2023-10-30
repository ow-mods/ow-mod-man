# NixOS Installation

You can either install using the package in [nixpkgs](https://search.nixos.org/packages?channel=unstable&show=owmods-cli&from=0&size=50&sort=relevance&type=packages&query=owmods) or by getting it from the flake.
## Without Flakes

This flake comes with [flake-compat](https://github.com/edolstra/flake-compat), which makes it usable in systems that use flakes, or not.

To install it, edit you NixOS/Home Manager configuration with:
```nix
{ pkgs, lib, ... }:
let
  ow-mod-man = import (builtins.fetchGit {
    url = "https://github.com/ow-mods/ow-mod-man.git";
    # You can choose which version by changing the ref
    ref = "dev";
  });
in
{
  imports = [
    # For NixOS
    ow-mod-man.nixosModules.owmods
    # For Home Manager
    ow-mod-man.homeManagerModules.owmods
  ];
  
  # Unsafe dependency of the gui version
  permittedInsecurePackages = [ "openssl-1.1.1w" ];

  
  # To enable the cli version
  programs.owmods-cli.enable = true;
  # To enable the gui version
  programs.owmods-gui.enable = true;
}
```

## Using Flakes

If you are already using flakes, this is the recommended method, what you need to add is:

- In your flake.nix:
```nix
{
  inputs.ow-mod-man = {
    url = "github:ow-mods/ow-mod-man/dev";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = { ow-mod-man }:
  # You need to add the ow-mod-man overlay to your packages
  let pkgs = import nixpkgs {
      inherit system;
      config = {
        # Needed for the gui version
        permittedInsecurePackages = [ "openssl-1.1.1w" ];
      };
      # Needed to be able to use the packages
      overlays = [ ow-mod-man.overlay.owmods ];
  };
  in {
      homeConfigurations = {
      userA = home-manager.lib.homeManagerConfiguration {
        inherit pkgs;
        # For your home.nix
        modules = [ ow-mod-man.homeManagerModules.owmods ];
      };
    };

    nixosConfigurations = {
      systemA = lib.nixosSystem {
        inherit system;
        inherit pkgs;
        # For your configuration.nix
        modules = [ ow-mod-man.nixosModules.owmods ];
      };
  };
}
```
- In your configuration.nix:
```nix
{ pkgs, inputs, ... }:
{
  # To enable the cli version
  programs.owmods-cli.enable = true;
  # To enable the gui version
  programs.owmods-gui.enable = true;
}
```
- In your home.nix:
```nix
{ pkgs, inputs, ... }:
{
  # To enable the cli version
  programs.owmods-cli.enable = true;
  # To enable the gui version
  programs.owmods-gui.enable = true;
}
```
