let
  release = import ./release.nix {};
in
release.test // release.newWorld
