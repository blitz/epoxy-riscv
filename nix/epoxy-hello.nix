{ stdenv, cmake, ninja }:

stdenv.mkDerivation {
  pname = "epoxy-hello";
  version = "1.0.0";

  src = ../src/user-hello;

  nativeBuildInputs = [
    cmake
    ninja
  ];

  buildInputs = [];
}
