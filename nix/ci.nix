let
  release = import ./release.nix {};
  recurse = release.pkgs.recurseIntoAttrs;
in {
  test = recurse release.test;
  newWorld = recurse release.newWorld;
}
