# This is a collection of dependencies that we use.
{ sources, nixpkgs, pkgs }:
let
  newlibOverlay = self: super: {
    newlibCross = super.newlibCross.overrideAttrs (attrs: {
      version = "epoxy";
      src = sources.epoxy-newlib;
    });
  };
in rec {
  epoxy-qemu-boot = pkgs.writeShellScriptBin "epoxy-qemu-boot" ''
                      exec ${qemuHeadless}/bin/qemu-system-riscv32 -M virt -m 256M -serial stdio -bios default $*
                    '';

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


  rv32 = {
    # All nixpkgs built for RISC-V with our patched newlib.
    pkgs = (import nixpkgs {
      # Patch the libc to do system calls the epoxy way.
      overlays = [ newlibOverlay ];

      # Disabled floating point and compressed instructions to make SaxonSoc happy.
      crossSystem = pkgs.lib.recursiveUpdate pkgs.lib.systems.examples.riscv32-embedded {
        gcc = {
          arch = "rv32ima";
        };
      };
    });

    pprintpp = pkgs.callPackage ./pprintpp.nix { };
    range-v3 = pkgs.callPackage ./range-v3.nix { };
  };
}
