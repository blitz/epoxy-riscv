let
  release = import ./release.nix {};
in {
  ulx3s-fbdemo = release.ulx3s-fbdemo.boot-image;
}
