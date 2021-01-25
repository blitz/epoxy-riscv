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
    inherit (pkgs) clang-tools niv;
  };

  # This is the new harden binary that needs quite a bit of work to be useful.
  new-harden = naersk.buildPackage { root = ../harden; };

  # This is the playground for the new Rust-based harden implementation.
  ulx3s-fbdemo =
    let
      hardenCmd = "harden -r ${../config} -s ulx3s-saxonsoc-fbdemo -vvvv";

      mkKernState = user-binaries: pkgs.runCommandNoCC "kern-state"
        {
          nativeBuildInputs = [ new-harden ];
        } ''
        mkdir -p $out
        ${hardenCmd} configure-kernel --header ${user-binaries} > $out/state.hpp
        ${hardenCmd} configure-kernel ${user-binaries} > $out/state.cpp
      '';

      mkResourceHeader = procName: pkgs.runCommandNoCC "${procName}-resources.hpp"
        {
          nativeBuildInputs = [ new-harden ];
        } "${hardenCmd} configure-process ${procName} > $out";

      mkBootImage = kern: user-binaries: pkgs.runCommandNoCC "boot-image"
        {
          nativeBuildInputs = [ new-harden ];
        } ''
        mkdir -p $out/bin

        ${hardenCmd} boot-image ${kern}/bin/epoxy-kern ${user-binaries} > $out/bin/epoxy-boot
      '';

    in
    rec {
      api = riscvPkgs.callPackage ./epoxy-api.nix { };

      # TODO Use harden list-processes to figure out what user processes to build.

      hello = riscvPkgs.callPackage ./epoxy-hello.nix { };

      fbdemo = riscvPkgs.callPackage ./epoxy-fbdemo.nix {
        resourceHeader = mkResourceHeader "fbdemo";
      };

      user-binaries = riscvPkgs.symlinkJoin {
        name = "user-binaries";
        paths = [ fbdemo hello ];
      };

      kern-state = mkKernState user-binaries;

      kern = riscvPkgs.callPackage ./epoxy-kern-new.nix {
        epoxy-api = api;
        epoxy-kern-state = kern-state;
      };

      # This is only convenience for developing the Rust harden code. Remove later.
      boot-image-input = riscvPkgs.symlinkJoin {
        name = "all-binaries";
        paths = [ user-binaries kern ];
      };

      boot-image = mkBootImage kern user-binaries;
    };

  # TODO Use test.nix
}
