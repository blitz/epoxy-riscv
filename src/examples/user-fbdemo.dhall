-- An application description is a set of processes and their capabilities.
--
-- TODO Store type definitions in epoxy-harden instead of here.

let SharedMemorySource
    : Type
    = < NamedSharedMemory : { sharedMemKey : Text }
      | AnonymousMemory : { sharedMemSize : Natural }
      >

let SharedMemoryPermissions
    : Type
    = < R | RW | RX >

let AddressSpaceElem
    : Type
    = < ELF : { binary : Text }
      | SharedMemory :
          { source : SharedMemorySource
          , vaDestination : Natural
          , permissions : SharedMemoryPermissions
          }
      >

let AddressSpace
    : Type
    = List AddressSpaceElem

let ThreadStack
    : Type
    = < Auto | Fixed : { vaInitStackPtr : Natural } >

let KObjectImpl
    : Type
    = < Exit
      | KLog : { prefix : Text }
      | Process :
          { pid : Natural
          , addressSpace : AddressSpace
          , capabilities : List Text
          }
      | Thread : { process : Text, stack : ThreadStack }
      >

let KObject
    : Type
    = { gid : Text, impl : KObjectImpl }

let ApplicationDescription
    : Type
    = { kobjects : List KObject }

let fbdemoHiAddressSpace =
      [ AddressSpaceElem.ELF { binary = "fbdemo.user.elf" }
      , AddressSpaceElem.SharedMemory
          { source =
              SharedMemorySource.NamedSharedMemory
                { sharedMemKey = "framebuffer hi" }
          , vaDestination = 0x10000000
          , permissions = SharedMemoryPermissions.RW
          }
      ]

let fbdemoLoAddressSpace =
      [ AddressSpaceElem.ELF { binary = "fbdemo.user.elf" }
      , AddressSpaceElem.SharedMemory
          { source =
              SharedMemorySource.NamedSharedMemory
                { sharedMemKey = "framebuffer lo" }
          , vaDestination = 0x10000000
          , permissions = SharedMemoryPermissions.RW
          }
      ]

in    { kobjects =
        [ { gid = "exit", impl = KObjectImpl.Exit }
        , { gid = "klog_u2", impl = KObjectImpl.KLog { prefix = "fb lo" } }
        , { gid = "klog_u3", impl = KObjectImpl.KLog { prefix = "fb hi" } }
        , { gid = "process_u2"
          , impl =
              KObjectImpl.Process
                { pid = 0
                , addressSpace = fbdemoLoAddressSpace
                , capabilities = [ "exit", "klog_u2" ]
                }
          }
        , { gid = "thread_u2"
          , impl =
              KObjectImpl.Thread
                { process = "process_u2", stack = ThreadStack.Auto }
          }
        , { gid = "process_u3"
          , impl =
              KObjectImpl.Process
                { pid = 1
                , addressSpace = fbdemoHiAddressSpace
                , capabilities = [ "exit", "klog_u3" ]
                }
          }
        , { gid = "thread_u3"
          , impl =
              KObjectImpl.Thread
                { process = "process_u3", stack = ThreadStack.Auto }
          }
        ]
      }
    : ApplicationDescription
