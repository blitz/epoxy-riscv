{ stdenv, dhall-json, epoxyHarden, pprintpp }:

stdenv.mkDerivation {
  pname = "epoxy";
  version = "0.0.0";

  nativeBuildInputs = [
    epoxyHarden
    dhall-json
  ];

  buildInputs = [
    pprintpp
  ];

  # This is not useful and patchelf also segfaults on our crude ELFs.
  dontPatchELF = true;
  dontStrip = true;

  src = ../src;

  makeFlags = [ "TARGET=$(out)"];
  hardeningDisable = [ "all" ];
}
