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

  bootScript = pkgs.writeShellScriptBin "boot" ''
    exec ${pkgs.qemu}/bin/qemu-system-riscv64 -M virt -m 256M -serial stdio \
         -bios default $*
  '';
}

