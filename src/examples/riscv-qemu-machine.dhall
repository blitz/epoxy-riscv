-- Dhall 13.0.0 can do hexadecimal.
let MemoryType : Type = < Available | Reserved >

in  { memoryMap =
        [ { baseAddress =
              2149580800
          , memoryLength =
              33554432
          , memoryType =
              MemoryType.Available
          }
        ]
    }
