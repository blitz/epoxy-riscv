let Epoxy = ../types/Epoxy.dhall

in    { name = "fbdemo"
      , heap_kb = 8
      , needs = [ { name = "fb0", type = Epoxy.ResourceType.Framebuffer } ]
      }
    : Epoxy.Application
