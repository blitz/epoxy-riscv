{ sources ? import ./nix/sources.nix
, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { }}:

let
  lib = pkgs.lib;
in rec {
  inherit pkgs;

  epoxyHarden = (import sources.epoxy-harden {}).epoxy-harden;

  riscvPkgs =
    import nixpkgs { crossSystem = lib.systems.examples.riscv64-embedded; };

  kernel = riscvPkgs.callPackage ./nix/build.nix { inherit epoxyHarden; };

  kernelGcc9 = kernel.override { stdenv = riscvPkgs.gcc9Stdenv; };

  # This doesn't work, because it results in "Package gcc-debug-8.3.0
  # is not supported in riscv64-none".
  #
  # kernelClang = kernel.override { stdenv = riscvPkgs.llvmPackages_latest.stdenv; };

  bootScript = pkgs.writeShellScriptBin "boot" ''
    exec ${pkgs.qemu}/bin/qemu-system-riscv64 -M virt -m 256M -serial stdio \
         -bios default $*
  '';
}

