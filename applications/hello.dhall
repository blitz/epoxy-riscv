let t = ./types.dhall

let helloAddressSpace = [ t.AddressSpaceElem.ELF { binary = "bin/epoxy-hello" } ]

in    { kobjects =
        [ { gid = "exit", impl = t.KObjectImpl.Exit }
        , { gid = "klog_u1", impl = t.KObjectImpl.KLog { prefix = "hello" } }
        , { gid = "process_u1"
          , impl =
              t.KObjectImpl.Process
                { pid = 0
                , addressSpace = helloAddressSpace
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
