{ sources ? import ./sources.nix
, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { }
}:
let
  naersk = pkgs.callPackage sources.naersk { };
  dependencies = import ./dependencies.nix { inherit sources nixpkgs pkgs; };

  inherit (dependencies) rv32 rv64;
  inherit (import sources."gitignore.nix" { inherit (pkgs) lib; }) gitignoreSource;
in
rec {
  inherit pkgs;

  # This is for convenience to build RISC-V apps from the CLI with nix-build.
  rv32Pkgs = rv32.pkgs;
  rv64Pkgs = rv64.pkgs;

  shellDependencies = {
    inherit (dependencies)
      epoxy-qemu-boot-64 qemuHeadless;
    inherit (pkgs) clang-tools niv dhall;
  };

  # This is the new harden binary that needs quite a bit of work to be useful.
  new-harden = naersk.buildPackage {
    root = gitignoreSource ../harden;
  };

  api = rv32.pkgs.callPackage ./epoxy-api.nix { };

  systems =
    let
      buildSystem = crossPkgs: system:
        let
          hardenCmd = "${new-harden}/bin/harden -r ${../config} -s ${system} -vvvv";

          mkResourceHeader = procName: pkgs.runCommandNoCC "${system}-${procName}-resources.hpp" { }
            "${hardenCmd} configure-process ${procName} > $out";

          mkBootImage = user-binaries: pkgs.runCommandNoCC "${system}-boot-image" { } ''
            ${hardenCmd} boot-image ${user-binaries} > $out
          '';

          # A list of name/program sets that describe all processes that we need to build
          processes = builtins.fromJSON (builtins.readFile (pkgs.runCommandNoCC "${system}-processes" {
            nativeBuildInputs = [ pkgs.dhall-json pkgs.jq ];
          } ''
            export XDG_CACHE_HOME=/tmp
            dhall-to-json --file ${../config}/systems/${system}.dhall | jq .processes > $out
          ''));

          # Return a derivation for the process from the system description.
          buildProcess = {name, program}: crossPkgs.pkgs.callPackage (./epoxy- + "${program}.nix") {
            resourceHeader = mkResourceHeader name;
            outputName = name;
          };

        in
        rec {
          kern-state = pkgs.runCommandNoCC "${system}-kern-state" { } ''
            mkdir -p $out
            ${hardenCmd} configure-kernel state-hpp > $out/state.hpp
            ${hardenCmd} configure-kernel state-cpp > $out/state.cpp
            ${hardenCmd} configure-kernel resources > $out/resources.hpp
          '';

          kern = crossPkgs.pkgs.callPackage ./epoxy-kern.nix {
            epoxy-api = api;
            epoxy-kern-state = kern-state;
          };

          boot-image-input = crossPkgs.pkgs.symlinkJoin {
            name = "all-binaries";
            paths = [ kern ] ++ builtins.map buildProcess processes;
          };

          boot-image = mkBootImage boot-image-input;
        };
    in
    {
      # TODO Use readDir to automate this.
      qemu-hello-32 = buildSystem rv32 "qemu-hello";
      qemu-hello-64 = buildSystem rv64 "qemu-hello";
     };

  tests = {
    qemu-hello-32 = pkgs.callPackage ./test-hello.nix {
      epoxy-qemu-boot = dependencies.epoxy-qemu-boot-32;
      bootElf = systems.qemu-hello-32.boot-image;
    };
  };
}
