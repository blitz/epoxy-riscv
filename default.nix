{ sources ? import ./nix/sources.nix, nixpkgs ? sources.nixpkgs
, pkgs ? import nixpkgs { }, lib ? pkgs.lib }:

rec {
  inherit pkgs;

  riscvPkgs =
    import nixpkgs { crossSystem = lib.systems.examples.riscv64-embedded; };

  kernel = riscvPkgs.callPackage ./build.nix { };

  bootScript = pkgs.writeShellScriptBin "boot" ''
    exec ${pkgs.qemu}/bin/qemu-system-riscv64 -M virt -m 256M -serial stdio \
         -bios default $*
  '';
}

