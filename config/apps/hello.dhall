let Epoxy = ../types/Epoxy.dhall

in    { name = "hello", needs = [] : List Epoxy.NamedResourceType }
    : Epoxy.Application
