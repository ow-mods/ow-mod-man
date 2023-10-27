{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    name = "owmods_gui-shell";
    buildInputs = [
        pkgs.rustc
        pkgs.cargo
        pkgs.clippy
        pkgs.rustfmt
        pkgs.nodejs
        pkgs.nodePackages.pnpm
        pkgs.gcc
        pkgs.webkitgtk
        pkgs.pkg-config
        pkgs.libnotify
        pkgs.gtk3
        pkgs.openssl_1_1
        pkgs.libsoup
    ];
}
