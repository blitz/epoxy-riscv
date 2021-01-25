let Epoxy = ../types/Epoxy.dhall

in    { name = "Qemu Hello World"
      , machine = "qemu"
      , processes = [ { program = "hello", name = "hello" } ]
      , mappings = [] : List { from : Text, to : Text }
      }
    : Epoxy.System
