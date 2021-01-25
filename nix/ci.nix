let
  release = import ./release.nix { };
in
{
  qemu-hello = release.systems.qemu-hello.boot-image;
  ulx3s-fbdemo = release.systems.ulx3s-saxonsoc-fbdemo.boot-image;
}
