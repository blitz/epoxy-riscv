let Epoxy = ../types/Epoxy.dhall

in    { name = "qemu"
      , available_memory =
        [ { start = 0x80400000, size = 0x1000000 }
        , { start = 0x82200000, size = 0x100000 }
        ]
      , devices = [] : List Epoxy.NamedResource
      }
    : Epoxy.Machine
