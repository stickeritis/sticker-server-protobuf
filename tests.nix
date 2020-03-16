{ pkgs ? import (import ./nix/sources.nix).nixpkgs {} }:

let
  sources = import ./nix/sources.nix;
  crateTools = pkgs.callPackage "${sources.crate2nix}/tools.nix" {};
  cargoNix = pkgs.callPackage (crateTools.generatedCargoNix {
    name = "sticker-server-protobuf";
    src = pkgs.nix-gitignore.gitignoreSource [ ".git/" ] ./.;
  }) { inherit buildRustCrate; };
  crateOverrides = with pkgs; defaultCrateOverrides // {
    sticker-server-protobuf = attr: {
      nativeBuildInputs = [ protobuf ];
    };
  };
  buildRustCrate = pkgs.buildRustCrate.override {
    defaultCrateOverrides = crateOverrides;
  };
in cargoNix.rootCrate.build
