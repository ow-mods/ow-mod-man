final: prev: {
  owmods-cli = final.callPackage ./owmods-cli.nix {};
  owmods-gui = final.callPackage ./owmods-gui.nix {};
}
