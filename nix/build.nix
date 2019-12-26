{ stdenv, cmake, epoxyHarden }:

stdenv.mkDerivation {
  pname = "epoxy";
  version = "0.0.0";

  nativeBuildInputs = [ epoxyHarden ];

  src = ../src;

  makeFlags = [ "TARGET=$(out)"];
  hardeningDisable = [ "all" ];
}
