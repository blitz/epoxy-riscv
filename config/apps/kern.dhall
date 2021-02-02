let Epoxy = ../types/Epoxy.dhall

in    { name = "kern"
      , needs =
        [ { name = "plic", type = Epoxy.ResourceType.SiFivePLIC }
        , { name = "sbitimer", type = Epoxy.ResourceType.SBITimer }
        ]
      , binary = "bin/epoxy-kern"
      }
    : Epoxy.Application
