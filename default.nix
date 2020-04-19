{ sources ? import ./nix/sources.nix, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { } }:

let
  lib = pkgs.lib;
  epoxyHardenSrc = import sources.epoxy-harden { };

  newlibOverlay = self: super: {
    newlibCross = super.newlibCross.overrideAttrs (attrs: {
      version = "epoxy";
      src = sources.epoxy-newlib;
    });
  };
in rec {
  riscvPkgs = (import nixpkgs {
    overlays = [ newlibOverlay ];
  }).pkgsCross.riscv64-embedded;

  shellDependencies = rec {
    inherit (epoxyHardenSrc) dhall;

    # Use a ncurses-only qemu to reduce closure size.
    qemuHeadless = pkgs.qemu.override {
      gtkSupport = false;
      vncSupport = false;
      sdlSupport = false;
      spiceSupport = false;
      pulseSupport = false;
      smartcardSupport = false;
      hostCpuTargets = [ "riscv64-softmmu" ];
    };

    bootScript = pkgs.writeShellScriptBin "boot" ''
      exec ${qemuHeadless}/bin/qemu-system-riscv64 -M virt -m 256M -serial stdio -bios default $*
    '';

    inherit (pkgs) clang-tools niv nixfmt;
  };

  dependencies = {
    inherit (epoxyHardenSrc) epoxyHarden;

    pprintpp = riscvPkgs.callPackage ./nix/pprintpp.nix { };
    range-v3 = riscvPkgs.callPackage ./nix/range-v3.nix { };
  };

  kernel = riscvPkgs.callPackage ./nix/build.nix dependencies;

  kernelGcc8 = kernel.override { stdenv = riscvPkgs.gcc8Stdenv; };

  test = pkgs.callPackage ./nix/test.nix {
    inherit (shellDependencies) bootScript;
    qemuBootImage = "${kernel}/qemu-example-hello.elf";
  };
}
