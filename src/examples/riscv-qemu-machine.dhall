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

in  { memoryMap =
      [ { baseAddress = 0x80200000
        , memoryLength = 0x2000000
        , memoryType = MemoryType.Available
        }
      , cfgSpaceEntry virtioNetRid "virtio-net pci-cfg"
      ]
    }
