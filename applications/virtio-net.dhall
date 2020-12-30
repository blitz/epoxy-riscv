let t = ./types.dhall

let virtioNetAddressSpace =
      [ t.AddressSpaceElem.ELF { binary = "bin/epoxy-virtio-net" }
      , t.AddressSpaceElem.SharedMemory
          { source =
              t.SharedMemorySource.NamedSharedMemory
                { sharedMemKey = "virtio-net pci-cfg" }
          , vaDestination = 0x10000000
          , permissions = t.SharedMemoryPermissions.RW
          }
      , t.AddressSpaceElem.SharedMemory
          { source =
              t.SharedMemorySource.NamedSharedMemory
                { sharedMemKey = "virtio-net bar4" }
          , vaDestination = 0x11000000
          , permissions = t.SharedMemoryPermissions.RW
          }
      , t.AddressSpaceElem.SharedMemory
          { source =
              t.SharedMemorySource.NamedSharedMemory
                { sharedMemKey = "virtio-net dma buffer" }
          , vaDestination = 0x82200000
          , permissions = t.SharedMemoryPermissions.RW
          }
      ]

in    { kobjects =
        [ { gid = "exit", impl = t.KObjectImpl.Exit }
        , { gid = "klog_u1", impl = t.KObjectImpl.KLog { prefix = "vnet" } }
        , { gid = "process_u1"
          , impl =
              t.KObjectImpl.Process
                { pid = 0
                , addressSpace = virtioNetAddressSpace
                , capabilities = [ "exit", "klog_u1" ]
                }
          }
        , { gid = "thread_u1"
          , impl =
              t.KObjectImpl.Thread
                { process = "process_u1", stack = t.ThreadStack.Auto }
          }
        ]
      }
    : t.ApplicationDescription
