{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs.buildPackages; [
    openssl
    pkg-config

    clang
    llvmPackages.libclang

    gcc
    # rustup
    pkg-config
    # rust-analyzer
    openssl.dev
    # typos-lsp

    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
  ];

  shellHook = ''
    alias clippy="cargo +nightly clippy --all-targets --all-features"
    alias test="cargo +nightly test --all-targets --all-features"
  '';

  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
}
