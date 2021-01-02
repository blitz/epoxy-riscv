let ResourceType
    : Type
    = < Framebuffer >

let MemoryRegion
    : Type
    = { start : Natural, size : Natural }

let PixelFormat
    : Type
    = < R5G6B5 >

let Resource
    : Type
    = < Framebuffer :
          { height : Natural
          , width : Natural
          , stride : Natural
          , format : PixelFormat
          , region : MemoryRegion
          }
      >

let NamedResource
    : Type
    = { name : Text, resource : Resource }

let NamedResourceType
    : Type
    = { name : Text, type : ResourceType }

let Application
    : Type
    = { -- | The name of the application as displayed by tools.
        name : Text
      , -- | A list of resources that an application needs to run.
        needs : List NamedResourceType
      , -- | The Nix expression that builds this application.
        program : Text
      }

let Machine
    : Type
    = { name : Text
      , availableMemory : List MemoryRegion
      , devices : List NamedResource
      }

let System
    : Type
    = { name : Text
      , machine : Text
      , processes : List { proc_name : Text, program : Text }
      , mappings : List { from : Text, to : Text }
      }

in  { ResourceType
    , MemoryRegion
    , PixelFormat
    , Resource
    , NamedResource
    , NamedResourceType
    , Application
    , Machine
    , System
    }
