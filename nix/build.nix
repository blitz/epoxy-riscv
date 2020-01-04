{ stdenv, cmake, epoxyHarden, python3, python3Packages }:

stdenv.mkDerivation {
  pname = "epoxy";
  version = "0.0.0";

  nativeBuildInputs = [ epoxyHarden python3 python3Packages.pyelftools];

  postPatch = ''
    patchShebangs scripts/config
  '';

  src = ../src;

  makeFlags = [ "TARGET=$(out)"];
  hardeningDisable = [ "all" ];
}
