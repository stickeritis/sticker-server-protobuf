{ pkgs ? import (import ./nix/sources.nix).nixpkgs {} }:

let
  sources = import ./nix/sources.nix;
  crateTools = pkgs.callPackage "${sources.crate2nix}/tools.nix" {};
  cargoNix = (crateTools.appliedCargoNix {
    name = "sticker-server-protobuf";
    src = pkgs.nix-gitignore.gitignoreSource [ ".git/" ] ./.;
  });
in cargoNix.rootCrate.build.override (attr: {
  runTests = true;
})
