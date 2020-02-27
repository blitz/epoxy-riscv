-- Dhall 13.0.0 can do hexadecimal.
let MemoryType
    : Type
    = < Available | Device : { key : Text } >

in  { memoryMap =
      [ { baseAddress = 2149580800
        , memoryLength = 33554432
        , memoryType = MemoryType.Available
        }
      , { baseAddress = 805306368
        , memoryLength = 268435456
        , memoryType = MemoryType.Device { key = "PCIe ECAM" }
        }
      ]
    }
