let Epoxy = ../types/Epoxy.dhall

in    { name = "Framebuffer Demo"
      , machine = "ulx3s-saxonsoc"
      , kernel = "kern"
      , processes =
        [ { program = "fbdemo", name = "fbdemo" }
        , { program = "blink", name = "blink" }
        ]
      , mappings =
        [ { from = "hdmi-fb", to = "fbdemo.fb0" }
        , { from = "gpio", to = "blink.gpio0" }
        , { from = "plic", to = "kern.plic" }
        , { from = "sbitimer", to = "kern.sbitimer" }
        ]
      }
    : Epoxy.System
