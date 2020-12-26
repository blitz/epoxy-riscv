# Epoxy RISC-V

[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental)
![GitHub](https://img.shields.io/github/license/blitz/epoxy-riscv.svg)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/blitz/epoxy-riscv)

🚧 **Nothing to see here yet.** 🚧

This code compiles using [Nix](https://nixos.org). The first compilation is going to take a long
time, because it compiles a RISC-V toolchain. You can use my [cachix](https://cachix.org/) repo to
fetch precompiled dependencies:

```
% cachix use blitz # optional
```

To build the demo application enter a Nix shell to get the right dependencies and use the demo make
target. This will build and execute the demo.

```
% nix-shell
% make qemu-boot-hello
```

If everything goes well it looks like this:

[![asciicast](https://asciinema.org/a/381274.svg)](https://asciinema.org/a/381274)

# Resources

- https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc
- https://riscv.org/wp-content/uploads/2015/01/riscv-calling.pdf
- https://wiki.qemu.org/Documentation/Platforms/RISCV
