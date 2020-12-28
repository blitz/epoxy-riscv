let MemoryType
    : Type
    = < Available | Device : { key : Text } >

in
{ memoryMap =
  [ { baseAddress = 0x81000000
    , memoryLength = 0x1000000
    , memoryType = MemoryType.Available
    }
  , { baseAddress = 0x80e00000
    , memoryLength = 0x4b000
    , memoryType = MemoryType.Device { key = "framebuffer hi" }
    }
  , { baseAddress = 0x80e4b000
    , memoryLength = 0x4b000
    , memoryType = MemoryType.Device { key = "framebuffer lo" }
    }
  ]
}
