let
  release = import ./release.nix { };

  lib = release.pkgs.lib;

  bootImages = lib.attrsets.mapAttrs' (k: v: lib.attrsets.nameValuePair "boot-${k}" v.boot-image) release.systems;

  tests = lib.attrsets.mapAttrs' (k: v: lib.attrsets.nameValuePair "test-${k}" v) release.tests;
in
bootImages // tests
