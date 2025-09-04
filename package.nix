{
  pkgs ? import <nixpkgs> {},
  lib,
  ...
}:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "acceptxmr";
  version = (builtins.fromTOML (lib.readFile ./server/Cargo.toml)).package.version;

  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  # disable tests
  checkType = "debug";
  doCheck = false;

  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    openssl
    pkg-config

    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
  ];
}
