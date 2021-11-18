# This is a collection of dependencies that we use.
{ sources, nixpkgs, pkgs }:
let
  newlibOverlay = self: super: {
    newlibCross = super.newlibCross.overrideAttrs (attrs: {
      version = "epoxy";
      src = sources.epoxy-newlib;
    });
  };

  # Use a ncurses-only qemu to reduce closure size.
  qemuHeadless = (pkgs.qemu.override {
    gtkSupport = false;
    vncSupport = false;
    sdlSupport = false;
    spiceSupport = false;
    pulseSupport = false;
    smartcardSupport = false;
    hostCpuTargets = [ "riscv32-softmmu" "riscv64-softmmu" ];
  });

  mkBoot = bits: pkgs.writeShellScriptBin "epoxy-qemu-boot" ''
    exec ${qemuHeadless}/bin/qemu-system-riscv${toString bits} -M virt -m 256M -serial stdio -bios default $*
  '';

  # All nixpkgs built for RISC-V with our patched newlib.
  mkCrossPkgs = crossSystem: import nixpkgs {
    # Patch the libc to do system calls the epoxy way.
    overlays = [ newlibOverlay ];

    # Disabled floating point.
    inherit crossSystem;
  };
in rec {
  epoxy-qemu-boot-32 = mkBoot 32;
  epoxy-qemu-boot-64 = mkBoot 64;

  rv32 = {
    pkgs = mkCrossPkgs (pkgs.lib.recursiveUpdate pkgs.lib.systems.examples.riscv32-embedded {
      gcc = {
        # Disable floating point.
        arch = "rv32imac";
      };
    });

    pprintpp = pkgs.callPackage ./pprintpp.nix { };
    range-v3 = pkgs.callPackage ./range-v3.nix { };
  };

  rv64 = {
    pkgs = mkCrossPkgs (pkgs.lib.recursiveUpdate pkgs.lib.systems.examples.riscv32-embedded {
      gcc = {
        # Disable floating point.
        arch = "rv64imac";
      };
    });

    pprintpp = pkgs.callPackage ./pprintpp.nix { };
    range-v3 = pkgs.callPackage ./range-v3.nix { };
  };
}
