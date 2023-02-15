{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlays.default];
        };

        libsqlite3-sys = pkgs.rustBuilder.rustLib.makeOverride {
          name = "libsqlite3-sys";
          overrideAttrs = drv: {
            propagatedNativeBuildInputs = drv.propagatedNativeBuildInputs or [ ] ++ [ pkgs.sqlite ];
          };
        };
        overrides = pkgs.rustBuilder.overrides.all ++ [ libsqlite3-sys ];

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustChannel = "nightly";
          packageOverrides = pkgs: overrides; 
          packageFun = import ./Cargo.nix;
        };

      in rec {
        packages = {
          # replace hello-world with your package name
          recentf = (rustPkgs.workspace.recentf {}).bin;
          default = packages.recentf;
        };
      }
    );
}
