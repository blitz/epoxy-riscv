let Epoxy = ../types/Epoxy.dhall

in    { name = "ulx3s-saxonsoc"
      , available_memory = [ { start = 0x81000000, size = 0x1000000 } ]
      , devices =
        [ { name = "hdmi-fb"
          , resource =
              Epoxy.Resource.Framebuffer
                { format =
                  { height = 480
                  , width = 640
                  , stride = 1280
                  , pixel = Epoxy.PixelFormat.R5G6B5
                  }
                , region = { start = 0x80e00000, size = 0x96000 }
                }
          }
        , { name = "plic"
          , resource =
              Epoxy.Resource.SiFivePLIC
                { ndev = 0x20
                , region = { start = 0x10c00000, size = 0x400000 }
                }
          }
        , { name = "sbi-timer"
          , resource = Epoxy.Resource.SBITimer { freq_hz = 52000000 }
          }
        ]
      }
    : Epoxy.Machine
