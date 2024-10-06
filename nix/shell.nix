{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  name = "owmods-shell";
  buildInputs = with pkgs; [
    rustc
    cargo
    clippy
    rustfmt
    nodejs
    libnotify
    typeshare
    at-spi2-atk
    atkmm
    cairo
    gdk-pixbuf
    glib
    gobject-introspection
    gobject-introspection.dev
    gtk3
    harfbuzz
    librsvg
    libsoup_3
    (callPackage ./tauri-cli.nix {})
    pango
    webkitgtk
  ];
  PKG_CONFIG_PATH = with pkgs; "${glib.dev}/lib/pkgconfig:${libsoup_3.dev}/lib/pkgconfig:${webkitgtk_4_1.dev}/lib/pkgconfig:${at-spi2-atk.dev}/lib/pkgconfig:${gtk3.dev}/lib/pkgconfig:${gdk-pixbuf.dev}/lib/pkgconfig:${cairo.dev}/lib/pkgconfig:${pango.dev}/lib/pkgconfig:${harfbuzz.dev}/lib/pkgconfig";
  OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
  shellHook = ''
    export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules/
  '';
}
