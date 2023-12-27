{
  stdenv,
  lib,
  libsoup,
  dbus,
  dpkg,
  fetchurl,
  autoPatchelfHook,
  glib,
  glib-networking,
  webkitgtk,
  pkg-config,
  openssl,
  wrapGAppsHook,
  makeDesktopItem,
  copyDesktopItems,
  rustPlatform,
  mkPnpmPackage,
}:
rustPlatform.buildRustPackage rec {
  pname = "owmods-gui";
  version = "0.12.0";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
    outputHashes = {"tauri-plugin-window-state-0.1.0" = "sha256-3lFd3Wx+xglRX/Xy3zW8yBOwX7pYlFEwVxvPqYA9ewI=";};
  };

  nativeBuildInputs = [
    pkg-config
    copyDesktopItems
  ];

  buildInputs = [
    openssl
    dbus
    libsoup
    glib
    glib-networking
    webkitgtk
    wrapGAppsHook
  ];

  buildAndTestSubdir = "owmods_gui/backend";

  postPatch = let
    frontend-drv = mkPnpmPackage rec {
      src = ../owmods_gui/frontend;
      installInPlace = true;
      distDir = "../dist";
    };
    frontend = frontend-drv; # assert lib.assertMsg (owmods-gui-frontend.version == version) "Frontend (${owmods-gui-frontend.version}) and backend (${version}) Version Mismatch"; owmods-gui-frontend;
  in ''
    substituteInPlace owmods_gui/backend/tauri.conf.json \
    --replace '"distDir": "../dist"' '"distDir": "${frontend}"'
  '';

  postInstall = ''
    install -DT owmods_gui/backend/icons/128x128@2x.png $out/share/icons/hicolor/256x256@2/apps/outer-wilds-mod-manager.png
    install -DT owmods_gui/backend/icons/128x128.png $out/share/icons/hicolor/128x128/apps/outer-wilds-mod-manager.png
    install -DT owmods_gui/backend/icons/32x32.png $out/share/icons/hicolor/32x32/apps/outer-wilds-mod-manager.png
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
    sourceProvenance = with sourceTypes; [binaryNativeCode];
    platforms = platforms.linux;
    license = licenses.gpl3;
    maintainers = with maintainers; [locochoco];
  };
}
