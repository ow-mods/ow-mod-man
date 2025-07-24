{
  lib,
  libsoup_3,
  dbus,
  glib,
  glib-networking,
  librsvg,
  webkitgtk_4_1,
  pkg-config,
  wrapGAppsHook,
  makeDesktopItem,
  copyDesktopItems,
  rustPlatform,
  buildNpmPackage,
  importNpmLock,
  mono,
  wrapWithMono ? true,
}:
rustPlatform.buildRustPackage rec {
  pname = "owmods-gui";
  version = "0.15.2";

  VITE_VERSION_SUFFIX = "-nix";

  # Prevent unneeded rebuilds
  src = with lib.fileset;
    toSource {
      root = ../../.;
      fileset = unions [
        ../../.cargo
        ../../owmods_gui
        ../../owmods_cli
        ../../owmods_core
        ../../xtask
        ../../Cargo.toml
        ../../Cargo.lock
      ];
    };

  cargoLock = {
    lockFile = ../../Cargo.lock;
  };

  buildFeatures = [
    "tauri/custom-protocol"
  ];

  doCheck = false;

  nativeBuildInputs = [
    pkg-config
    copyDesktopItems
    wrapGAppsHook
  ];

  buildInputs = [
    dbus
    libsoup_3
    glib
    librsvg
    glib-networking
    webkitgtk_4_1
  ];

  buildAndTestSubdir = "owmods_gui/backend";

  preFixup = lib.optionalString wrapWithMono "gappsWrapperArgs+=(--prefix PATH : '${mono}/bin')";

  postPatch = let
    frontend = let
      src = ../../owmods_gui/frontend;
    in
      buildNpmPackage {
        inherit version VITE_VERSION_SUFFIX;
        pname = "owmods_gui-ui";

        inherit src;

        packageJSON = ../../owmods_gui/frontend/package.json;
        npmDeps = importNpmLock {
          npmRoot = src;
        };

        npmConfigHook = importNpmLock.npmConfigHook;

        postBuild = ''
          cp -r ../dist/ $out
        '';
        distPhase = "true";
        dontInstall = true;
        installInPlace = true;
        distDir = "../dist";
      };
  in ''
    substituteInPlace owmods_gui/backend/tauri.conf.json \
    --replace '"frontendDist": "../dist"' '"frontendDist": "${frontend}"'
  '';

  postInstall = ''
    install -DT owmods_gui/backend/icons/128x128@2x.png $out/share/icons/hicolor/256x256@2/apps/outer-wilds-mod-manager.png
    install -DT owmods_gui/backend/icons/128x128.png $out/share/icons/hicolor/128x128/apps/outer-wilds-mod-manager.png
    install -DT owmods_gui/backend/icons/32x32.png $out/share/icons/hicolor/32x32/apps/outer-wilds-mod-manager.png

    mv $out/bin/owmods_gui $out/bin/outer-wilds-mod-manager
  '';

  desktopItems = [
    (makeDesktopItem {
      name = "outer-wilds-mod-manager";
      exec = "outer-wilds-mod-manager %u";
      icon = "outer-wilds-mod-manager";
      desktopName = "Outer Wilds Mod Manager";
      categories = ["Game"];
      comment = meta.description;
      mimeTypes = ["x-scheme-handler/owmods"];
    })
  ];

  meta = with lib; {
    description = "GUI version of the mod manager for Outer Wilds Mod Loader";
    homepage = "https://github.com/ow-mods/ow-mod-man/tree/main/owmods_gui";
    downloadPage = "https://github.com/ow-mods/ow-mod-man/releases/tag/gui_v${version}";
    changelog = "https://github.com/ow-mods/ow-mod-man/releases/tag/gui_v${version}";
    mainProgram = "outer-wilds-mod-manager";
    platforms = platforms.linux;
    license = licenses.gpl3;
    maintainers = with maintainers; [bwc9876 locochoco];
  };
}
