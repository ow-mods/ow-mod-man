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
    openssl
    nodePackages.pnpm
    gcc
    webkitgtk
    glib-networking
    pkg-config
    libnotify
    gtk3
    libsoup
    librsvg
    cargo-tauri
    typeshare
  ];
  shellHook = ''
    export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules/
  '';
}
