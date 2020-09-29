{ stdenv, qemuBootImage, bootScript, curl }:

stdenv.mkDerivation {
  pname = "epoxy-qemu-test";
  version = "0.0.0";

  nativeBuildInputs = [ bootScript curl ];
  src = null;

  phases = [ "buildPhase" "checkPhase" "installPhase" ];

  doCheck = true;

  # Execute the system in qemu, retrieve a website and terminate everything.
  #
  # TODO This should probably be an expect script.
  buildPhase = ''
    timeout 20 boot -display none \
                    -netdev user,id=u,hostfwd=tcp::8000-10.0.2.4:80 \
                    -device virtio-net-pci,netdev=u \
                    -device loader,file=${qemuBootImage} &

    # Give the system a bit to boot.
    sleep 2

    curl -s http://localhost:8000/ > index.html

    echo "Terminating QEMU."
    kill %1

    echo "Retrieved the following HTML file:"
    cat index.html
  '';

  checkPhase = ''
    sha256sum -c ${./test.sha256}
  '';

  installPhase = ''
    cp index.html $out
  '';
}
