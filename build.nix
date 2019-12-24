{ stdenv, cmake }:

stdenv.mkDerivation {
  pname = "riscv-hello";
  version = "0.0.0";

  src = ./src;

  makeFlags = [ "TARGET=$(out)"];  
  hardeningDisable = [ "all" ];
}
