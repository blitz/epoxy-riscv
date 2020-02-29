-- Dhall 13.0.0 can do hexadecimal, but the formatter is only fixed in 14.0.0.
--
-- XXX Large device mappings are currently very inefficient to
-- generate (probably because of our page table deduplication).
let MemoryType
    : Type
    = < Available | Device : { key : Text } >

in  { memoryMap =
      [ { baseAddress = 2149580800
        , memoryLength = 33554432
        , memoryType = MemoryType.Available
        }
      , { baseAddress = 805306368
        , memoryLength = 256 * 4096
        , memoryType = MemoryType.Device { key = "PCIe ECAM" }
        }
      ]
    }
