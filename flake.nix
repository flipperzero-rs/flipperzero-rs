{
  description = "Rust on the Flipper Zero";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      perSystem =
        { system, pkgs, ... }:
        let
          rust = pkgs.rust-bin.fromRustupToolchainFile ./crates/rust-toolchain.toml;
        in
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          formatter = pkgs.nixfmt-rfc-style;
          devShells = {
            default = pkgs.mkShell {
              packages = with pkgs; [
                rust
                python3
                pkg-config
                systemd
              ];
            };
            github-actions = pkgs.mkShell {
              nativeBuildInputs = with pkgs; [
                act
                actionlint
                pinact
              ];
            };
          };
        };
    };
}
