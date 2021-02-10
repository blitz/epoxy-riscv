let Epoxy = ../types/Epoxy.dhall

in    { name = "kern"
      , heap_kb = 0            -- The kernel needs no heap. That's the whole point!
      , needs =
        [ { name = "plic", type = Epoxy.ResourceType.SiFivePLIC }
        , { name = "sbitimer", type = Epoxy.ResourceType.SBITimer }
        ]
      }
    : Epoxy.Application
