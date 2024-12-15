{
  description = "Flake for owmods-cli and owmods-gui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed;
    pkgsFor = system:
      (import nixpkgs) {
        inherit system;
        overlays = [self.overlays.default];
      };
  in {
    packages = forAllSystems (system: with pkgsFor system; {inherit owmods-cli owmods-gui;});
    overlays.default = import ./nix/overlay.nix;

    formatter = forAllSystems (system: (pkgsFor system).alejandra);
    devShells = forAllSystems (system: {default = import ./nix/shell.nix {pkgs = pkgsFor system;};});
  };

  # nixConfig = {
  #   extra-substituters = ["https://ow-mods.cachix.org"];
  #   extra-trusted-public-keys = ["ow-mods.cachix.org-1:6RTOd1dSRibA2W0MpZHxzT0tw1RzyhKObTPKQJpcrZo="];
  # };
}
