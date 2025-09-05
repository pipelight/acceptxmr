{
  lib,
  config,
  inputs,
  pkgs,
  ...
}: let
  moduleName = "acceptxmr";
  cfg = config.services.${moduleName};
  package = inputs.acceptxmr.packages.${pkgs.system}.default;
in
  lib.mkIf cfg.enable
  {
    ## Working dir
    systemd.tmpfiles.rules = [
      "Z '/var/lib/${moduleName}' 760 ${cfg.user} root - -"
      "d '/var/lib/${moduleName}' 760 ${cfg.user} root - -"
    ];

    ## Systemd unit file
    systemd.services.${moduleName} = {
      enable = true;
      description = "Acceptxmr - A monero payment gateway server";
      documentation = [
        "https://github.com/busyboredom/acceptxmr"
      ];
      after = [
        "network.target"
      ];
      wantedBy = ["multi-user.target"];
      serviceConfig = let
        verbosity =
          {
            "error" = "RUST_LOG=error";
            "warn" = "RUST_LOG=warn";
            "info" = "RUST_LOG=info";
            "debug" = "RUST_LOG=debug";
            "trace" = "RUST_LOG=trace";
          }.${
            cfg.logLevel
          };
      in {
        Type = "simple";
        User = cfg.user;
        Group = "root";
        Environment = [
          "PRIVATE_VIEWKEY=ad2093a5705b9f33e6f0f0c1bc1f5f639c756cdfc168c8f2ac6127ccbdab3a03"
          "INTERNAL_API_TOKEN=supersecrettoken"
          "CONFIG_FILE=/etc/${moduleName}/acceptxmr.yaml"
          verbosity
        ];

        ExecStart = " ${package}/bin/${moduleName}-server";

        StandardInput = "null";
        # StandardOutput = "journal+console";
        # StandardError = "journal+console";

        # StateDirectory = "/var/lib/${moduleName}";
        LogsDirectory = "/var/lib/${moduleName}";

        AmbientCapabilities = [
          "CAP_NET_BIND_SERVICE"
        ];
      };
    };

    environment.systemPackages = [
      package
    ];
  }
