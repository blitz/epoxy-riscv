let Epoxy = ../types/Epoxy.dhall

in    { name = "hello", heap_kb = 8, needs = [] : List Epoxy.NamedResourceType }
    : Epoxy.Application
