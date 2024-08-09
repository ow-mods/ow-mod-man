{pkgs ? import <nixpkgs> {}}:
# NOTE(Spoonbaker): This doesn't include the overlay when using `nix-shell` instead of `nix shell`
pkgs.mkShell {
  name = "owmods-shell";
  buildInputs = with pkgs; [
    rustc
    cargo
    clippy
    rustfmt
    nodejs
    gcc
    webkitgtk_4_1
    glib-networking
    pkg-config
    libnotify
    gtk3
    libsoup
    librsvg
    (pkgs.callPackage ./tauri-cli.nix {})
    typeshare
  ];
  OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
  shellHook = ''
    export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules/
  '';
}
