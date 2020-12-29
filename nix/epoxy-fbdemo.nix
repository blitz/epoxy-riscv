{ stdenv, cmake, ninja }:

stdenv.mkDerivation {
  pname = "epoxy-fbdemo";
  version = "1.0.0";

  src = ../src/user-fbdemo;

  nativeBuildInputs = [
    cmake
    ninja
  ];

  buildInputs = [];
}
