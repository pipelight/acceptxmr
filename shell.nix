{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs.buildPackages; [
    openssl
    pkg-config

    # Lsp
    # rust-analyzer
    # typos-lsp
    # alejandra

    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
  ];

  shellHook = ''
    alias clippy="cargo +nightly clippy --all-targets --all-features"
    alias test="cargo +nightly test --all-targets --all-features"
  '';
}
