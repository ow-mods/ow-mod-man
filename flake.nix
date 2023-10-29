{
  description = "Flake for owmods-cli and owmods-gui";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/master";
  };

  outputs = { self, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
	  overlays = [ self.overlay.owmods ];
        };
      in rec {
        # For `nix build` & `nix run`:
        packages = rec {
          owmods-cli = pkgs.owmods-cli;
          owmods-gui = pkgs.owmods-gui;
          default = pkgs.owmods-cli;
        };        
        # For `nix develop`:
        #devShell = pkgs.mkShell {
        #  nativeBuildInputs = with pkgs; [ rustc cargo openssl libsoup ];
        #};
      }
    ) // {
        overlay.owmods = import ./nix/overlay.nix;
	nixosModules.owmods = import ./nix/modules/nixos.nix;
	homeManagerModules.owmods = import ./nix/modules/hm.nix;
    };
  
  nixConfig = {
    extra-substituters = [ "https://ow-mods.cachix.org" ];
    extra-trusted-public-keys = [ "ow-mods.cachix.org-1:6RTOd1dSRibA2W0MpZHxzT0tw1RzyhKObTPKQJpcrZo=" ];
  };
}
