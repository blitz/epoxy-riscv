let Epoxy = ../types/Epoxy.dhall

in    { name = "fbdemo"
      , needs = [ { name : "fb0", type : Epoxy.ResourceType.Framebuffer } ]
      }
    : Epoxy.Application
