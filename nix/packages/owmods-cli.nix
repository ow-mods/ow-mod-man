{
  lib,
  pkg-config,
  openssl,
  libsoup_3,
  installShellFiles,
  rustPlatform,
  makeWrapper,
  mono,
  wrapWithMono ? true,
}:
rustPlatform.buildRustPackage rec {
  pname = "owmods-cli";
  version = "0.15.2";

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

  doCheck = false;

  nativeBuildInputs =
    [
      pkg-config
      installShellFiles
    ]
    ++ lib.optional wrapWithMono makeWrapper;

  buildInputs = [
    openssl
    libsoup_3
  ];

  buildAndTestSubdir = "owmods_cli";

  postInstall = ''
    cargo xtask dist_cli
    installManPage dist/cli/man/*
    installShellCompletion --cmd owmods \
    dist/cli/completions/owmods.{bash,fish,zsh}
    ${lib.optionalString wrapWithMono "wrapProgram $out/bin/${meta.mainProgram} --prefix PATH : '${mono}/bin'"}
  '';

  meta = with lib; {
    description = "CLI version of the mod manager for Outer Wilds Mod Loader";
    homepage = "https://github.com/ow-mods/ow-mod-man/tree/main/owmods_cli";
    downloadPage = "https://github.com/ow-mods/ow-mod-man/releases/tag/cli_v${version}";
    changelog = "https://github.com/ow-mods/ow-mod-man/releases/tag/cli_v${version}";
    mainProgram = "owmods";
    license = licenses.gpl3;
    maintainers = with maintainers; [
      bwc9876
      locochoco
    ];
  };
}
