{
  description = "signal-lojix — the ordinary Lojix Datom and bound-wire contract";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = {
      url = "github:LiGoldragon/rust-build";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-build }:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = rust-build.lib.${system}.fromPkgs pkgs;
        inherit (rust) craneLib toolchain;
        src = rust.cleanSource { root = ./.; };
        common = { inherit src; strictDeps = true; };
        cargoArtifacts = craneLib.buildDepsOnly common;
      in {
        packages.default = craneLib.buildPackage (common // { inherit cargoArtifacts; });
        checks = {
          build = craneLib.cargoBuild (common // { inherit cargoArtifacts; });
          test = craneLib.cargoTest (common // { inherit cargoArtifacts; });
          fmt = craneLib.cargoFmt common;
          clippy = craneLib.cargoClippy (common // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- -D warnings";
          });
        };
        devShells.default = pkgs.mkShell { packages = [ pkgs.jujutsu toolchain ]; };
      });
}
