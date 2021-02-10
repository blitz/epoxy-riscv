let Epoxy = ../types/Epoxy.dhall

in    { name = "blink"
      , heap_kb = 8
      , needs = [ { name = "gpio0", type = Epoxy.ResourceType.SpinalGPIO } ]
      }
    : Epoxy.Application
