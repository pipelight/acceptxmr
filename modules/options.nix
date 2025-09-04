{
  lib,
  config,
  inputs,
  pkgs,
  ...
}:
with lib; let
  moduleName = "acceptxmr";
in {
  ## Options
  options.services.${moduleName} = {
    enable = mkEnableOption "Enable ${moduleName}.";
    logLevel = mkOption {
      default = "info";
      type = types.enum ["error" "warn" "info" "debug" "trace"];
    };
    user = mkOption {
      default = "root";
      type = types.str;
    };
  };
}
