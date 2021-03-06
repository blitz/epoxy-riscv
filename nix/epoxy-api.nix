{ stdenv, cmake, ninja }:

stdenv.mkDerivation {
  pname = "epoxy-api";
  version = "1.0.0";

  src = ../src/api;

  nativeBuildInputs = [
    cmake
    ninja
  ];

  buildInputs = [];
}
