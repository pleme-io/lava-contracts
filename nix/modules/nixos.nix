# nix/modules/nixos.nix — auto-generated from lava-contracts.caixa.lisp
# description: "Typed result contracts for lava architectures: NetworkResult / IamResult / ClusterResult / etc. Cross-architecture composition validates at compile time, not apply time. Pangea::Contracts::*Result analog."
{ config, lib, pkgs, ... }:
let
  cfg = config.services.lava-contracts;
in {
  options.services.lava-contracts = {
    enable = lib.mkEnableOption "lava-contracts";
    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.lava-contracts or null;
    };
  };
  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}
