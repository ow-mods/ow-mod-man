{ lib
, pkg-config
, openssl
, libsoup
, fetchFromGitHub
, installShellFiles
, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "owmods-cli";
  version = "0.12.0";

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
    outputHashes = { "tauri-plugin-window-state-0.1.0" = "sha256-wAlwiC8a21R0jyfIkOfmdjm82VHaS07eCq30L7jGVis=";};
  };

  nativeBuildInputs = [
    pkg-config
    installShellFiles
  ];

  buildInputs = [
    openssl
    libsoup
  ];

  buildAndTestSubdir = "owmods_cli";

  postInstall = ''
    cargo xtask dist_cli
    installManPage dist/cli/man/*
    installShellCompletion --cmd owmods \
    dist/cli/completions/owmods.{bash,fish,zsh}
  '';

  meta = with lib; {
    description = "CLI version of the mod manager for Outer Wilds Mod Loader";
    homepage = "https://github.com/ow-mods/ow-mod-man/tree/main/owmods_cli";
    downloadPage = "https://github.com/ow-mods/ow-mod-man/releases/tag/cli_v${version}";
    changelog = "https://github.com/ow-mods/ow-mod-man/releases/tag/cli_v${version}";
    mainProgram = "owmods";
    license = licenses.gpl3;
    maintainers = with maintainers; [ locochoco ];
  };
}
