final: prev: {
  owmods-cli = final.pkgs.callPackage ./owmods-cli.nix {};
  owmods-gui = final.pkgs.callPackage ./owmods-gui.nix {};
}
