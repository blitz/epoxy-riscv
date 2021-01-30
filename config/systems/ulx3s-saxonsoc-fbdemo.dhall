let Epoxy = ../types/Epoxy.dhall

in    { name = "Framebuffer Demo"
      , machine = "ulx3s-saxonsoc"
      , kernel = "kern"
      , processes =
        [ { program = "fbdemo", name = "fbdemo" }
        , { program = "hello", name = "hello" }
        ]
      , mappings =
        [ { from = "hdmi-fb", to = "fbdemo.fb0" }
        , { from = "plic", to = "kern.plic" }
        ]
      }
    : Epoxy.System
