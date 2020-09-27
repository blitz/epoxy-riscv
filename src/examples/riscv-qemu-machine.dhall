-- Dhall 13.0.0 can do hexadecimal, but the formatter is only fixed in 14.0.0.
--
-- XXX Large device mappings are currently very inefficient to generate (probably because of our n^2
-- page table deduplication).
let MemoryType
    : Type
    = < Available | Device : { key : Text } >

let RID
    : Type
    = { bus : Natural, device : Natural, function : Natural }

let cfgSpaceBase = 0x30000000

let cfgSpaceAddress =
      λ(bdf : RID) →
        cfgSpaceBase + 4096 * (bdf.bus * 256 + bdf.device * 8 + bdf.function)

let cfgSpaceEntry =
      λ(rid : RID) →
      λ(key : Text) →
        { baseAddress = cfgSpaceAddress rid
        , memoryLength = 4096
        , memoryType = MemoryType.Device { key }
        }

let virtioNetRid
    : RID
    = { bus = 0, device = 1, function = 0 }

let virtioNetBar4Base = 0x40000000

let virtioNetBar4Size = 0x00010000

in  { memoryMap =
      [ { baseAddress = 0x80200000
        , memoryLength = 0x2000000
        , memoryType = MemoryType.Available
        }
      , { baseAddress = 0x82200000
        , memoryLength = 0x100000
        , memoryType = MemoryType.Device { key = "virtio-net dma buffer" }
        }
      , cfgSpaceEntry virtioNetRid "virtio-net pci-cfg"
      , { baseAddress = virtioNetBar4Base
        , memoryLength = virtioNetBar4Size
        , memoryType = MemoryType.Device { key = "virtio-net bar4" }
        }
      ]
    }
