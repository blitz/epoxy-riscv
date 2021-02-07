{ sources ? import ./sources.nix
, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { }
}:
let
  dependencies = import ./dependencies.nix { inherit sources nixpkgs pkgs; };

  riscvPkgs = dependencies.riscvPkgs;

  naersk = pkgs.callPackage sources.naersk { };
in
rec {
  # This is for convenience to build RISC-V apps from the CLI with nix-build.
  inherit riscvPkgs;

  shellDependencies = {
    inherit (dependencies)
      epoxy-qemu-boot qemuHeadless pprintpp range-v3;
    inherit (pkgs) clang-tools niv dhall;
  };

  # This is the new harden binary that needs quite a bit of work to be useful.
  new-harden = naersk.buildPackage { root = ../harden; };

  api = riscvPkgs.callPackage ./epoxy-api.nix { };

  systems =
    let
      buildSystem = system:
        let
          hardenCmd = "${new-harden}/bin/harden -r ${../config} -s ${system} -vvvv";

          mkKernState = user-binaries: pkgs.runCommandNoCC "${system}-kern-state" { } ''
            mkdir -p $out
            ${hardenCmd} configure-kernel state-hpp ${user-binaries} > $out/state.hpp
            ${hardenCmd} configure-kernel state-cpp ${user-binaries} > $out/state.cpp
            ${hardenCmd} configure-kernel resources ${user-binaries} > $out/resources.hpp
          '';

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
          buildProcess = {name, program}: riscvPkgs.callPackage (./epoxy- + "${program}.nix") {
            resourceHeader = mkResourceHeader name;
            outputName = name;
          };

        in
        rec {
          # All user binaries symlinked into one derivation.
          user-binaries = riscvPkgs.symlinkJoin {
            name = "${system}-user-binaries";
            paths =
              builtins.map buildProcess processes;
          };

          kern-state = mkKernState user-binaries;

          kern = riscvPkgs.callPackage ./epoxy-kern.nix {
            epoxy-api = api;
            epoxy-kern-state = kern-state;
          };

          boot-image-input = riscvPkgs.symlinkJoin {
            name = "all-binaries";
            paths = [ user-binaries kern ];
          };

          boot-image = mkBootImage boot-image-input;
        };
    in
    {
      # TODO Use readDir to automate this.
      qemu-hello = buildSystem "qemu-hello";
      ulx3s-saxonsoc-fbdemo = buildSystem "ulx3s-saxonsoc-fbdemo";
    };

  tests = {
    qemu-hello = riscvPkgs.callPackage ./test-hello.nix {
      inherit (dependencies) epoxy-qemu-boot;

      bootElf = systems.qemu-hello.boot-image;
    };
  };

  # TODO Use test.nix
}
