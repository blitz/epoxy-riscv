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
  }).overrideAttrs (old : {
    # Fix a bug that the SBI triggers. This should be fixed after 5.1.0.
    patches = old.patches ++ [ ./0001-riscv-sifive_test-Allow-16-bit-writes-to-memory-regi.patch ];
  });

  # All nixpkgs built for RISC-V with our patched newlib.
  riscvPkgs = (import nixpkgs {
    # Patch the libc to do system calls the epoxy way.
    overlays = [ newlibOverlay ];

    # Disabled floating point and compressed instructions to make SaxonSoc happy.
    crossSystem = pkgs.lib.recursiveUpdate pkgs.lib.systems.examples.riscv32-embedded {
      platform = {
        gcc = {
          arch = "rv32ima";
        };
      };
    };
  });

  pprintpp = riscvPkgs.callPackage ./pprintpp.nix { };
  range-v3 = riscvPkgs.callPackage ./range-v3.nix { };
}
