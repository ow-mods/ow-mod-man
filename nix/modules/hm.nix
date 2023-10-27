{ lib, pkgs, config, ... }:
with lib;                      
let
  cli-cfg = config.programs.owmods-cli;
  gui-cfg = config.programs.owmods-gui;
in {
  options.programs = {
    owmods-cli = {
      enable = mkEnableOption "owmods-cli program";
    };
    owmods-gui = {
      enable = mkEnableOption "owmods-gui program";
    };
  };

  config = mkIf (cli-cfg.enable || gui-cfg.enable) (
  	mkMerge [{
          home.packages = [
            (mkIf cli-cfg.enable (pkgs.owmods-cli))
            (mkIf gui-cfg.enable (pkgs.owmods-gui))
	    pkgs.mono
          ];
	}]
  );
}

