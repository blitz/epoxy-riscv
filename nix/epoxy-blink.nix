{ stdenv, lib, cmake, ninja, outputName ? null, resourceHeader ? null }:

stdenv.mkDerivation {
  pname = "epoxy-blink";
  version = "1.0.0";

  src = ../src/user-blink;

  nativeBuildInputs = [
    cmake
    ninja
  ];

  buildInputs = [ ];

  cmakeFlags =
    lib.optional (outputName != null) "-DTARGET_BIN_NAME=${outputName}"
    ++ lib.optional (resourceHeader != null) "-DRESOURCE_HEADER=${resourceHeader}";
}
