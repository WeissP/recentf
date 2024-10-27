{
  description = "A devShell example";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    {
      self,
      nixpkgs,
      naersk,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        toolchain = pkgs.rust-bin.stable.latest.default;
        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

      in
      {
        packages = rec {
          default =
            with pkgs;
            naersk'.buildPackage {
              src = ./.;
              nativeBuildInputs =
                [ ] ++ (lib.optional stdenv.isDarwin (with darwin.apple_sdk.frameworks; [ SystemConfiguration ]));
            };
          recentf = default;
        };
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              openssl
              pkg-config
              bacon
              sqlx-cli
              (lib.hiPrio (
                rust-bin.stable.latest.minimal.override {
                  extensions = [
                    "rust-docs"
                    "rust-src"
                    "rust-analyzer"
                    "clippy"
                  ];
                }
              ))
              (rust-bin.selectLatestNightlyWith (
                toolchain:
                toolchain.minimal.override {
                  extensions = [
                    "rustfmt"
                  ];
                }
              ))
            ];
          };
      }
    );
}
