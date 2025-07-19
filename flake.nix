{
  description = "Flake for owmods-cli and owmods-gui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flakelight.url = "github:nix-community/flakelight";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {flakelight, ...} @ inputs:
    flakelight ./. {
      inherit inputs;

      withOverlays = [inputs.rust-overlay.overlays.default];

      flakelight.builtinFormatters = false;
      formatters = pkgs: let
        prettier = "${pkgs.prettier}/bin/prettier --write .";
        alejandra = "${pkgs.alejandra}/bin/alejandra .";
        rustfmt = "${pkgs.rustfmt}/bin/rustfmt";
        just = "${pkgs.just}/bin/just --fmt --unstable";
      in {
        "justfile" = just;
        "*.nix" = alejandra;
        "*.js" = prettier;
        "*.ts" = prettier;
        "*.jsx" = prettier;
        "*.tsx" = prettier;
        "*.md" = prettier;
        "*.json" = prettier;
        "*.rs" = rustfmt;
      };
    };
}
