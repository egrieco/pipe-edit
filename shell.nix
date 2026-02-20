{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy

    # Required for linking
    gcc

    # Useful development tools
    rust-analyzer
  ];

  # Set up environment variables
  RUST_BACKTRACE = 1;

  shellHook = ''
    echo "Rust TUI Editor development environment"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
  '';
}
