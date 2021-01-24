let Epoxy = ../types/Epoxy.dhall

in    { name = "Framebuffer Demo"
      , machine = "ulx3s-saxonsoc"
      , processes =
        [ { program = "fbdemo", name = "fbdemo" }
        , { program = "hello", name = "hello" }
        ]
      , mappings = [ { from = "hdmi-fb", to = "fbdemo.fb0" } ]
      }
    : Epoxy.System
