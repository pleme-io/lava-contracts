# nix/modules/darwin.nix — auto-generated from lava-contracts.caixa.lisp
{ config, lib, pkgs, ... }:
let cfg = config.services.lava-contracts; in {
  options.services.lava-contracts = {
    enable = lib.mkEnableOption "lava-contracts";
    package = lib.mkOption { type = lib.types.package; default = pkgs.lava-contracts or null; };
  };
  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}
