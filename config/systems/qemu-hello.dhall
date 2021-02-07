let Epoxy = ../types/Epoxy.dhall

in    { name = "Qemu Hello World"
      , machine = "qemu"
      , kernel = "kern"
      , processes =
        [ { program = "hello", name = "h1" }
        , { program = "hello", name = "h2" }
        , { program = "hello", name = "h3" }
        , { program = "hello", name = "h4" }
        ]
      , mappings =
        [ { from = "plic", to = "kern.plic" }
        , { from = "sbitimer", to = "kern.sbitimer" }
        ]
      }
    : Epoxy.System
