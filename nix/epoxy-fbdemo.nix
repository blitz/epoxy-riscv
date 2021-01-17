{ stdenv, lib, cmake, ninja,
  resourceHeader ? null }:

stdenv.mkDerivation {
  pname = "epoxy-fbdemo";
  version = "1.0.0";

  src = ../src/user-fbdemo;

  nativeBuildInputs = [
    cmake
    ninja
  ];

  buildInputs = [];

  cmakeFlags = lib.optional (resourceHeader != null)
    "-DRESOURCE_HEADER=${resourceHeader}";
}
