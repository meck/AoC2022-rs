{
  description = "Advent of code";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let

        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rust-bin = pkgs.rust-bin.nightly.latest;
        rust-platform = pkgs.makeRustPlatform {
          cargo = rust-bin.minimal;
          rustc = rust-bin.minimal;
        };

        aoc-cli = with pkgs; rust-platform.buildRustPackage rec {
          pname = "aoc-cli";
          version = "0.5.0";
          src = fetchFromGitHub {
            owner = "scarvalhojr";
            repo = "aoc-cli";
            rev = "0b4e9723a85264e091e0ef9b77ec475fbe4ad2d7";
            sha256 = "0bzncwsmkr50xykhn4qgrv91yph5mvhgwx03mxxgh8kdmd3jhb1i";
          };
          cargoHash = "sha256-15yn+298qKwXSgs4umie/aiFMdNH2/3R9d7dhYrS9jw=";
          nativeBuildInputs = [ pkg-config ];
          buildInputs = [ openssl ];
        };

        cargoConfig = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        name = "${cargoConfig.package.name}";

        buildInputs = with pkgs; [ ];
        nativeBuildInputs = with pkgs; [ ];

      in
      {
        # nix develop
        devShells.default = pkgs.mkShell {
          name = "${name}-devshell";
          buildInputs =
            with pkgs; [
              aoc-cli
              cargo-edit
              rust-analyzer
              rust-bin.default
              rustfmt
            ] ++ buildInputs;
          inherit nativeBuildInputs;
        };
      }
    );
}
