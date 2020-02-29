-- Dhall 13.0.0 can do hexadecimal, but the formatter is only fixed in 14.0.0.
--
-- XXX Large device mappings are currently very inefficient to
-- generate (probably because of our page table deduplication).
let MemoryType
    : Type
    = < Available | Device : { key : Text } >

let RID
    : Type
    = { bus : Natural, device : Natural, function : Natural }

let cfgSpaceBase = 805306368

let cfgSpaceAddress =
        λ(bdf : RID)
      → cfgSpaceBase + 4096 * (bdf.bus * 256 + bdf.device * 8 + bdf.function)

let cfgSpaceEntry =
        λ(rid : RID)
      → λ(key : Text)
      → { baseAddress = cfgSpaceAddress rid
        , memoryLength = 4096
        , memoryType = MemoryType.Device { key = key }
        }

let virtioNetRid
    : RID
    = { bus = 0, device = 1, function = 0 }

in  { memoryMap =
      [ { baseAddress = 2149580800
        , memoryLength = 33554432
        , memoryType = MemoryType.Available
        }
      , cfgSpaceEntry virtioNetRid "virtio-net pci-cfg"
      ]
    }
