let
  release = import ./release.nix {};
in
release.new-harden
