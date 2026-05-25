# nix/modules/home-manager.nix — auto-generated from lava-contracts.caixa.lisp
{ config, lib, pkgs, ... }:
let cfg = config.programs.lava-contracts; in {
  options.programs.lava-contracts = {
    enable = lib.mkEnableOption "lava-contracts";
    package = lib.mkOption { type = lib.types.package; default = pkgs.lava-contracts or null; };
  };
  config = lib.mkIf cfg.enable { home.packages = [ cfg.package ]; };
}
