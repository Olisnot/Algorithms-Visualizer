{
  description = "Rust Development Shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages."${system}";
      fpkgs = with fenix.packages."${system}";
        combine [
          stable.toolchain
          # add additional compilation targets
          targets.wasm32-unknown-unknown.stable.rust-std
        ];
    in {
      devShells."${system}".default = pkgs.mkShell {
        buildInputs = # bash
          [ ];
        nativeBuildInputs = with pkgs; [
          fpkgs
          cargo
          gcc
          rustfmt
          rustup
          clippy
          lld
          trunk
          leptosfmt
        ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        shellHook = ''
          export SHELL=/run/current-system/sw/bin/bash
        '';
      };
    };
}
