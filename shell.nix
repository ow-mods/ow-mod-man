{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    name = "owmods-shell";
    buildInputs = [
        pkgs.rustc
        pkgs.cargo
        pkgs.clippy
        pkgs.rustfmt
        pkgs.nodejs
        pkgs.nodePackages.pnpm
        pkgs.gcc
        pkgs.webkitgtk_4_1
        pkgs.pkg-config
        pkgs.libnotify
        pkgs.gtk3
        pkgs.libsoup_3
    ];
}
