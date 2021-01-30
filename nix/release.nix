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

          mkBootImage = kern: user-binaries: pkgs.runCommandNoCC "${system}-boot-image" { } ''
            ${hardenCmd} boot-image ${kern}/bin/epoxy-kern ${user-binaries} > $out
          '';

          # A file that contains all processes that are necessary to build the system.
          list-processes-output = pkgs.runCommandNoCC "${system}-processes" { } ''
            ${hardenCmd} list-processes > $out
          '';

          # Return a derivation for the process from the system description.
          buildProcess = procName: riscvPkgs.callPackage (./epoxy- + "${procName}.nix") {
            resourceHeader = mkResourceHeader procName;
          };

        in
        rec {
          # All user binaries symlinked into one derivation.
          user-binaries = riscvPkgs.symlinkJoin {
            name = "${system}-user-binaries";
            paths =
              let
                procs = builtins.filter (n: n != "") (pkgs.lib.splitString "\n" (builtins.readFile list-processes-output));
              in
              builtins.map buildProcess procs;
          };

          kern-state = mkKernState user-binaries;

          kern = riscvPkgs.callPackage ./epoxy-kern.nix {
            epoxy-api = api;
            epoxy-kern-state = kern-state;
          };

          # This is only convenience for developing the Rust harden code.
          boot-image-input = riscvPkgs.symlinkJoin {
            name = "all-binaries";
            paths = [ user-binaries kern ];
          };

          boot-image = mkBootImage kern user-binaries;
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
