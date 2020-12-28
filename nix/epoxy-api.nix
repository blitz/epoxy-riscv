{ stdenv, cmake, ninja }:

stdenv.mkDerivation {
  pname = "epoxy-api";
  version = "1.0.0";

  src = ../src/api;

  nativeBuildInputs = [
    cmake
    ninja
  ];

  buildInputs = [
  ];

  # This is not useful and patchelf also segfaults on our crude ELFs.
  dontPatchELF = true;
  dontStrip = true;
  hardeningDisable = [ "all" ];
}
