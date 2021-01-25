let
  release = import ./release.nix { };

  lib = release.riscvPkgs.lib;

  bootImages = lib.mapAttrs (k: v: v.boot-image) release.systems;
in
# TODO Add tests. See test.nix.
bootImages
