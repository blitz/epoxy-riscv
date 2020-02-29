-- An application description is a set of processes and their capabilities.
--
-- TODO Store type definitions in epoxy-harden instead of here.

let AddressSpaceElem
    : Type
    = < ELF : { binary : Text }
      | SharedMemory : { key : Text, vaDestination : Natural }
      >

let AddressSpace
    : Type
    = List AddressSpaceElem

let KObjectImpl
    : Type
    = < Exit
      | KLog : { prefix : Text }
      | Process :
          { pid : Natural
          , addressSpace : AddressSpace
          , capabilities : List Text
          }
      | Thread : { process : Text }
      >

let KObject
    : Type
    = { gid : Text, impl : KObjectImpl }

let ApplicationDescription
    : Type
    = { kobjects : List KObject }

let helloAddressSpace =
      [ AddressSpaceElem.ELF { binary = "hello.user.elf" }
      , AddressSpaceElem.SharedMemory
          { key = "virtio-net pci-cfg", vaDestination = 268435456 }
      ]

in    { kobjects =
        [ { gid = "exit", impl = KObjectImpl.Exit }
        , { gid = "klog_u1", impl = KObjectImpl.KLog { prefix = "U1" } }
        , { gid = "klog_u2", impl = KObjectImpl.KLog { prefix = "U2" } }
        , { gid = "process_u1"
          , impl =
              KObjectImpl.Process
                { pid = 0
                , addressSpace = helloAddressSpace
                , capabilities = [ "exit", "klog_u1" ]
                }
          }
        , { gid = "process_u2"
          , impl =
              KObjectImpl.Process
                { pid = 1
                , addressSpace = helloAddressSpace
                , capabilities = [ "exit", "klog_u2" ]
                }
          }
        , { gid = "thread_u1"
          , impl = KObjectImpl.Thread { process = "process_u1" }
          }
        , { gid = "thread_u2"
          , impl = KObjectImpl.Thread { process = "process_u2" }
          }
        ]
      }
    : ApplicationDescription
