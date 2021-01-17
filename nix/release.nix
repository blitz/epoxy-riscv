{ sources ? import ./sources.nix, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { } }:
let
  dependencies = import ./dependencies.nix { inherit sources nixpkgs pkgs; };

  riscvPkgs = dependencies.riscvPkgs;

  mkEpoxyBoot = { userBinaries, epoxy-api, applicationDesc, machineDesc }:
    riscvPkgs.callPackage ./epoxy-kern.nix {
      inherit epoxy-api applicationDesc machineDesc;
      inherit (dependencies) epoxy-harden;

      # TODO We should only join paths that are actually referenced in the application description.
      userBinaries = riscvPkgs.symlinkJoin {
        name = "user-binaries";
        paths = userBinaries;
      };
    };

  naersk = pkgs.callPackage sources.naersk {};
in {
  # This is for convenience to build RISC-V apps from the CLI with nix-build.
  inherit riscvPkgs;

  shellDependencies = {
    inherit (dependencies) dhall epoxy-dtb epoxy-qemu-boot qemuHeadless pprintpp range-v3;
    inherit (pkgs) clang-tools niv nixfmt;
  };

  newWorld = rec {
    epoxy-api = riscvPkgs.callPackage ./epoxy-api.nix {};
    epoxy-hello = riscvPkgs.callPackage ./epoxy-hello.nix {};
    epoxy-fbdemo = riscvPkgs.callPackage ./epoxy-fbdemo.nix {};

    epoxy-virtio-net = riscvPkgs.callPackage ./epoxy-virtio-net.nix {
      inherit (dependencies) pprintpp range-v3;
    };

    epoxy-boot-hello = mkEpoxyBoot {
      inherit epoxy-api;

      applicationDesc = "${../applications}/hello.dhall";
      machineDesc = ../machines/qemu-riscv32.dhall;

      userBinaries = [ epoxy-hello ];
    };

    epoxy-boot-virtio-net = mkEpoxyBoot {
      inherit epoxy-api;

      applicationDesc = "${../applications}/virtio-net.dhall";
      machineDesc = ../machines/qemu-riscv32.dhall;

      userBinaries = [ epoxy-virtio-net ];
    };

    test = pkgs.callPackage ./test.nix {
      inherit (dependencies) epoxy-qemu-boot;
      bootElf = "${epoxy-boot-virtio-net}/bin/epoxy-boot";
    };
  };

  # This is the playground for the new Rust-based harden implementation.
  new-harden =
    let
      hardenCmd = "harden -r ${../config} -s ulx3s-saxonsoc-fbdemo -vvvv";
    in rec {
      # This is the new harden binary that needs quite a bit of work to be useful.
      new-harden = naersk.buildPackage {
        root = ../harden;
      };

      new-harden-test = pkgs.runCommandNoCC "new-harden-verify-test"
        { nativeBuildInputs = [ new-harden ]; }
        "${hardenCmd} verify 2>&1 | tee $out";

      new-harden-fbdemo = riscvPkgs.callPackage ./epoxy-fbdemo.nix {
        resourceHeader = pkgs.runCommandNoCC "fbdemo-resources.hpp"
          { nativeBuildInputs = [ new-harden ]; }
          "${hardenCmd} configure-process fbdemo > $out";
      };
    };

}
