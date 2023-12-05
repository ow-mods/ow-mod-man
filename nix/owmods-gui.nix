{ stdenv
, lib
, dpkg
, fetchurl
, autoPatchelfHook
, glib-networking
, openssl_1_1
, webkitgtk
, wrapGAppsHook
}:

stdenv.mkDerivation rec {
  pname = "owmods-gui";
  version = "0.12.0";

  src = fetchurl {
    url = "https://github.com/ow-mods/ow-mod-man/releases/download/gui_v${version}/outer-wilds-mod-manager_${version}_amd64.deb";
    hash = "sha256-D+rnF40e1xpwKeMvyiOuhMSJ1+JlDKq5c195iam3d2Y=";
  };

  nativeBuildInputs = [
    autoPatchelfHook
    dpkg
  ];

  buildInputs = [
    glib-networking
    webkitgtk
    wrapGAppsHook
  ];

  unpackCmd = "dpkg-deb -x $curSrc source";

  installPhase = "mv usr $out";
  meta = with lib; {
    description = "GUI version of the mod manager for Outer Wilds Mod Loader";
    homepage = "https://github.com/ow-mods/ow-mod-man/tree/main/owmods_gui";
    downloadPage = "https://github.com/ow-mods/ow-mod-man/releases/tag/gui_v${version}";
    changelog = "https://github.com/ow-mods/ow-mod-man/releases/tag/gui_v${version}";
    mainProgram = "outer-wilds-mod-manager";
    sourceProvenance = with sourceTypes; [ binaryNativeCode ];
    platforms = platforms.linux;
    license = licenses.gpl3;
    maintainers = with maintainers; [ locochoco ];
  };
}
