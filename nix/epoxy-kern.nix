{ stdenv, epoxy-harden, epoxy-api, applicationDesc, machineDesc, userBinaries }:

stdenv.mkDerivation {
  pname = "epoxy-kern";
  version = "1.0.0";

  src = ../src/kern;

  nativeBuildInputs = [
    epoxy-harden
  ];

  makeFlags = [
    "APPLICATION_DESC=${applicationDesc}"
    "MACHINE_DESC=${machineDesc}"
    "EPOXY_API=${epoxy-api}/include"
    "USER_BINARIES=${userBinaries}"
    "PREFIX=$(out)"
  ];

  # This is not useful.
  dontPatchELF = true;
  dontStrip = true;
  hardeningDisable = [ "all" ];

  # This makes dhall happy, because it wants to write a cache file.
  XDG_CACHE_HOME = "./.";
}
