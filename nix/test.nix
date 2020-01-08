{ stdenv, qemuBootImage, bootScript }:

stdenv.mkDerivation {
  pname = "epoxy-qemu-test";
  version = "0.0.0";

  nativeBuildInputs = [ bootScript ];
  src = null;

  phases = [ "buildPhase" "installPhase" ];

  buildPhase = ''
    timeout 10 boot -display none -device loader,file=${qemuBootImage} | tee > output.log
    cat output.log
    grep -q "Epoxy.*RISC-V" output.log
  '';

  installPhase = ''
    cp output.log $out
  '';
}
