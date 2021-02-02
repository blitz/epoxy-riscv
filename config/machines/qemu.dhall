let Epoxy = ../types/Epoxy.dhall

in    { name = "qemu"
      , available_memory =
        [ { start = 0x80400000, size = 0x1000000 }
        , { start = 0x82200000, size = 0x100000 }
        ]
      , devices =
        [ { name = "plic"
          , resource =
              Epoxy.Resource.SiFivePLIC
                { ndev = 0x20
                , region = { start = 0x0c000000, size = 0x400000 }
                }
          }
        , { name = "sbitimer"
          , resource = Epoxy.Resource.SBITimer { freq_hz = 1000000 }
          }
        ]
      }
    : Epoxy.Machine
