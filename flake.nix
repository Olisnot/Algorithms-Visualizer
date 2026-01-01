{
  description = "Rust Development Shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    nixvimConfig.url = "git+https://codeberg.org/Olisnot/NixVimConfig";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, nixvimConfig, fenix }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages."${system}";
      fpkgs = with fenix.packages."${system}";
        combine [
          stable.toolchain
          # add additional compilation targets
          #targets.wasm32-unknown-unknown.stable.rust-std
        ];
      nvim = nixvimConfig.packages."${system}".default.extend {
        plugins = {
          rustaceanvim.enable = true;
          dap.enable = true;
          dap-ui.enable = true;
          dap-lldb.enable = true;
        };
      };
    in {
      devShells."${system}".default = pkgs.mkShell {
        buildInputs = # bash
          [ nvim ];
        nativeBuildInputs = with pkgs; [
          rustc
          fpkgs
          cargo
          gcc
          rustfmt
          rustup
          clippy
        ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        shellHook = ''
          export SHELL=/run/current-system/sw/bin/bash
        '';
      };
    };
}
