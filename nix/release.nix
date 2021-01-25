{ sources ? import ./sources.nix
, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { }
}:
let
  dependencies = import ./dependencies.nix { inherit sources nixpkgs pkgs; };

  riscvPkgs = dependencies.riscvPkgs;

  naersk = pkgs.callPackage sources.naersk { };
in
{
  # This is for convenience to build RISC-V apps from the CLI with nix-build.
  inherit riscvPkgs;

  shellDependencies = {
    inherit (dependencies)
      epoxy-qemu-boot qemuHeadless pprintpp range-v3;
    inherit (pkgs) clang-tools niv;
  };

  # This is the playground for the new Rust-based harden implementation.
  new-harden =
    let hardenCmd = "harden -r ${../config} -s ulx3s-saxonsoc-fbdemo -vvvv";
    in
    rec {
      # This is the new harden binary that needs quite a bit of work to be useful.
      new-harden = naersk.buildPackage { root = ../harden; };

      new-harden-test = pkgs.runCommandNoCC "new-harden-verify-test"
        {
          nativeBuildInputs = [ new-harden ];
        } "${hardenCmd} verify 2>&1 | tee $out";

      new-harden-api = riscvPkgs.callPackage ./epoxy-api.nix { };

      new-harden-hello = riscvPkgs.callPackage ./epoxy-hello.nix { };

      new-harden-fbdemo = riscvPkgs.callPackage ./epoxy-fbdemo.nix {
        resourceHeader = pkgs.runCommandNoCC "fbdemo-resources.hpp"
          {
            nativeBuildInputs = [ new-harden ];
          } "${hardenCmd} configure-process fbdemo > $out";
      };

      new-harden-user-binaries = riscvPkgs.symlinkJoin {
        name = "user-binaries";
        paths = [ new-harden-fbdemo new-harden-hello ];
      };

      new-harden-kern-state = pkgs.runCommandNoCC "kern-state"
        {
          nativeBuildInputs = [ new-harden ];
        } ''
        mkdir -p $out
        ${hardenCmd} configure-kernel --header ${new-harden-user-binaries} > $out/state.hpp
        ${hardenCmd} configure-kernel ${new-harden-user-binaries} > $out/state.cpp
      '';

      new-harden-kern = riscvPkgs.callPackage ./epoxy-kern-new.nix {
        epoxy-api = new-harden-api;
        epoxy-kern-state = new-harden-kern-state;
      };

      # This is only convenience for developing the Rust harden code. Remove later.
      new-harden-boot-image-input = riscvPkgs.symlinkJoin {
        name = "all-binaries";
        paths = [ new-harden-user-binaries new-harden-kern ];
      };

      new-harden-boot-image = pkgs.runCommandNoCC "boot-image"
        {
          nativeBuildInputs = [ new-harden ];
        } ''
        mkdir -p $out/bin

        ${hardenCmd} boot-image ${new-harden-kern}/bin/epoxy-kern ${new-harden-user-binaries} > $out/bin/epoxy-boot
      '';
    };

  # TODO Use test.nix
}
