let Epoxy = ../types/Epoxy.dhall

in    { name = "Qemu Hello World"
      , machine = "qemu"
      , kernel = "kern"
      , processes = [ { program = "hello", name = "hello" } ]
      , mappings = [ { from = "plic", to = "kern.plic" } ]
      }
    : Epoxy.System
