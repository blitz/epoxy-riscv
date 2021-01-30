let Epoxy = ../types/Epoxy.dhall

in    { name = "kern"
      , needs = [ { name = "plic", type = Epoxy.ResourceType.SiFivePLIC } ]
      , binary = "bin/epoxy-kern"
      }
    : Epoxy.Application
