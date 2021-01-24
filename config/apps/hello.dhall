let Epoxy = ../types/Epoxy.dhall

in    { name = "hello"
      , needs = [] : List Epoxy.NamedResourceType
      , binary = "bin/epoxy-hello"
      }
    : Epoxy.Application
