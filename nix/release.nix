{ sources ? import ./sources.nix, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { } }:

let
  lib = pkgs.lib;
  epoxyHardenSrc = import "${sources.epoxy-harden}/nix/ci.nix" {};

  newlibOverlay = self: super: {
    newlibCross = super.newlibCross.overrideAttrs (attrs: {
      version = "epoxy";
      src = sources.epoxy-newlib;
    });
  };

  riscvPkgs = (import nixpkgs {
    overlays = [ newlibOverlay ];
  }).pkgsCross.riscv32-embedded;

  testConfigurations = {
    "gcc8" = { stdenv = riscvPkgs.gcc8Stdenv; };
    "gcc9" = { stdenv = riscvPkgs.gcc9Stdenv; };
    "gcc10" = { stdenv = riscvPkgs.gcc10Stdenv; };
  };

  gitignoreSource = (import sources.gitignore { inherit (pkgs) lib; }).gitignoreSource;
  cleanSrc = gitignoreSource ../src;

in rec {
  inherit riscvPkgs;

  shellDependencies = rec {
    inherit (epoxyHardenSrc) dhall;

    # Use a ncurses-only qemu to reduce closure size.
    qemuHeadless = (pkgs.qemu.override {
      gtkSupport = false;
      vncSupport = false;
      sdlSupport = false;
      spiceSupport = false;
      pulseSupport = false;
      smartcardSupport = false;
      hostCpuTargets = [ "riscv32-softmmu" "riscv64-softmmu" ];
    }).overrideAttrs (old : {
      # Fix a bug that the SBI triggers. This should be fixed after 5.1.0.
      patches = old.patches ++ [ ./0001-riscv-sifive_test-Allow-16-bit-writes-to-memory-regi.patch ];
    });

    bootScript = pkgs.writeShellScriptBin "boot" ''
      exec ${qemuHeadless}/bin/qemu-system-riscv32 -M virt -m 256M -serial stdio -bios default $*
    '';

    inherit (pkgs) clang-tools niv nixfmt;
  };

  dependencies = {
    inherit (epoxyHardenSrc) epoxy-harden;

    pprintpp = riscvPkgs.callPackage ./pprintpp.nix { };
    range-v3 = riscvPkgs.callPackage ./range-v3.nix { };

    src = cleanSrc;
  };

  kernel = let kernelDrv = riscvPkgs.callPackage ./build.nix dependencies;
  in builtins.mapAttrs (_: overrides: kernelDrv.override overrides)
  testConfigurations;

  test = builtins.mapAttrs (_: kernel:
    pkgs.callPackage ./test.nix {
      inherit (shellDependencies) bootScript;
      qemuBootImage = "${kernel}/qemu-example-hello.elf";
    }) kernel;
}
