{inputs, ...}: {
  imports = [
    # module configuration options
    ./options.nix
    # acceptxmr config
    ./config.nix
  ];
}
