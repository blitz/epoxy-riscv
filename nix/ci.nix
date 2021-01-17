let
  release = import ./release.nix {};
in
release.newWorld // release.new-harden
