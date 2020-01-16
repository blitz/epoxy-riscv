{ stdenv, dhall-json, cmake, epoxyHarden, python3, python3Packages }:

stdenv.mkDerivation {
  pname = "epoxy";
  version = "0.0.0";

  nativeBuildInputs = [
    epoxyHarden
    dhall-json
  ];

  # This is not useful and patchelf also segfaults on our crude ELFs.
  dontPatchELF = true;
  dontStrip = true;

  src = ../src;

  makeFlags = [ "TARGET=$(out)"];
  hardeningDisable = [ "all" ];
}
