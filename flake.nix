{
  description = "Flake for owmods-cli and owmods-gui";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    pnpm2nix.url = "github:nzbr/pnpm2nix-nzbr";
    nixpkgs.url = "github:NixOS/nixpkgs/master";
  };

  outputs = {
    self,
    flake-utils,
    flake-compat,
    nixpkgs,
    pnpm2nix,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [self.overlay.owmods pnpm2nix.overlays.default];
        };
      in rec {
        # For `nix build` & `nix run`:
        packages = rec {
          owmods-cli = pkgs.owmods-cli;
          owmods-gui = pkgs.owmods-gui;
          default = pkgs.owmods-cli;
        };
      }
    )
    // {
      formatter."x86_64-linux" = nixpkgs.legacyPackages."x86_64-linux".alejandra;
      devShell = import ./nix/shell.nix;
      overlay.owmods = import ./nix/overlay.nix;
      nixosModules.owmods = import ./nix/modules/nixos.nix;
      homeManagerModules.owmods = import ./nix/modules/hm.nix;
    };

  nixConfig = {
    extra-substituters = ["https://ow-mods.cachix.org"];
    extra-trusted-public-keys = ["ow-mods.cachix.org-1:6RTOd1dSRibA2W0MpZHxzT0tw1RzyhKObTPKQJpcrZo="];
  };
}
