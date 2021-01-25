{ stdenv, epoxy-api, epoxy-kern-state }:

stdenv.mkDerivation {
  pname = "epoxy-kern";
  version = "1.0.0";

  src = ../src/kern;

  makeFlags = [
    "-f" "Makefile.new"
    "EPOXY_API=${epoxy-api}/include"
    "KERN_STATE=${epoxy-kern-state}"
    "PREFIX=$(out)"
  ];

  # This is not useful.
  dontPatchELF = true;
  dontStrip = true;
  hardeningDisable = [ "all" ];
}
