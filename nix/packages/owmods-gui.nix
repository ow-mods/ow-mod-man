{
  jq,
  lib,
  libsoup_3,
  dbus,
  glib,
  glib-networking,
  librsvg,
  webkitgtk_4_1,
  pkg-config,
  wrapGAppsHook3,
  rustPlatform,
  makeBinaryWrapper,
  buildNpmPackage,
  importNpmLock,
  cargo-tauri,
  stdenv,
  mono,
  wrapWithMono ? true,
}: let
  version = "0.15.7";
  frontend = let
    src = ../../owmods_gui/frontend;
  in
    buildNpmPackage {
      inherit src version;
      VITE_VERSION_SUFFIX = "-nix";
      pname = "owmods_gui-ui";

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
in
  rustPlatform.buildRustPackage rec {
    pname = "owmods-gui";
    inherit version;

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

    buildNoDefaultFeatures = true;
    buildFeatures = [
      "custom-protocol"
    ];

    doCheck = false;

    nativeBuildInputs =
      [
        cargo-tauri.hook
      ]
      ++ lib.optionals stdenv.hostPlatform.isLinux [
        pkg-config
        wrapGAppsHook3
      ]
      ++ lib.optionals stdenv.hostPlatform.isDarwin [
        makeBinaryWrapper
      ];

    buildInputs = lib.optionals stdenv.hostPlatform.isLinux [
      dbus
      libsoup_3
      glib
      librsvg
      glib-networking
      webkitgtk_4_1
    ];

    buildAndTestSubdir = "owmods_gui/backend";

    preFixup = lib.optionalString (
      stdenv.hostPlatform.isLinux && wrapWithMono
    ) "gappsWrapperArgs+=(--prefix PATH : '${mono}/bin')";

    postPatch = ''
      ${lib.getExe jq} \
        'del(.plugins.tauri.updater) | .build.frontendDist = "${frontend}" | del(.build.beforeBuildCommand) | .bundle.createUpdaterArtifacts = false' owmods_gui/backend/tauri.conf.json > owmods_gui/backend/new.tauri.conf.json;
      mv owmods_gui/backend/new.tauri.conf.json owmods_gui/backend/tauri.conf.json
    '';

    postInstall = lib.optionalString stdenv.hostPlatform.isDarwin ''
      mkdir -p "$out/bin"
      makeWrapper "$out/Applications/Outer Wilds Mod Manager.app/Contents/MacOS/owmods_gui" "$out/bin/owmods_gui" ${lib.optionalString wrapWithMono "--set MONO_BINARY ${lib.getExe mono}"}
    '';

    passthru = {
      inherit frontend;
    };

    meta = with lib; {
      description = "GUI version of the mod manager for Outer Wilds Mod Loader";
      homepage = "https://github.com/ow-mods/ow-mod-man/tree/main/owmods_gui";
      downloadPage = "https://github.com/ow-mods/ow-mod-man/releases/tag/gui_v${version}";
      changelog = "https://github.com/ow-mods/ow-mod-man/releases/tag/gui_v${version}";
      mainProgram = "owmods_gui";
      platforms = platforms.linux ++ platforms.darwin;
      license = licenses.gpl3;
      maintainers = with maintainers; [
        bwc9876
        locochoco
      ];
    };
  }
