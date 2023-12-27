{nixpkgs}: let
  pkgs = nixpkgs.legacyPackages."x86_64-linux";
in
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
      typeshare
    ];
    shellHook = ''
      export GIO_MODULE_DIR=${pkgs.glib-networking}/lib/gio/modules/
    '';
  }
