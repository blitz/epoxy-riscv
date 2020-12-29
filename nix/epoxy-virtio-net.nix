{ stdenv, cmake, ninja, pprintpp, range-v3 }:

stdenv.mkDerivation {
  pname = "epoxy-virtio-net";
  version = "1.0.0";

  src = ../src/user-virtio-net;

  nativeBuildInputs = [
    cmake
    ninja
  ];

  buildInputs = [
    pprintpp
    range-v3
  ];
}
