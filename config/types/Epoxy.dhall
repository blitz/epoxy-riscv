let ResourceType
    : Type
    = < Framebuffer | SiFivePLIC | SBITimer | SpinalGPIO >

let MemoryRegion
    : Type
    = { start : Natural, size : Natural }

let PixelFormat
    : Type
    = < R5G6B5 >

let FramebufferFormat
    : Type
    = { height : Natural
      , width : Natural
      , stride : Natural
      , pixel : PixelFormat
      }

let Resource
    : Type
    = < Framebuffer : { format : FramebufferFormat, region : MemoryRegion }
      | SiFivePLIC : { ndev : Natural, region : MemoryRegion }
      | SBITimer : { freq_hz : Natural }
      | SpinalGPIO : { ngpio : Natural, region : MemoryRegion }
      >

let NamedResource
    : Type
    = { name : Text, resource : Resource }

let NamedResourceType
    : Type
    = { name : Text, type : ResourceType }

let Application
    : Type
    = { name : Text, heap_kb : Natural, needs : List NamedResourceType }

let Machine
    : Type
    = { name : Text
      , available_memory : List MemoryRegion
      , devices : List NamedResource
      }

let System
    : Type
    = { name : Text
      , machine : Text
      , kernel : Text
      , processes : List { name : Text, program : Text }
      , mappings : List { from : Text, to : Text }
      }

in  { ResourceType
    , MemoryRegion
    , FramebufferFormat
    , PixelFormat
    , Resource
    , NamedResource
    , NamedResourceType
    , Application
    , Machine
    , System
    }
