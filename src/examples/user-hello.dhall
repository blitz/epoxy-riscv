-- An application description is a set of processes and their capabilities.
--
-- TODO Store type definitions in epoxy-harden instead of here.

let KObjectImpl
    : Type
    = < Exit
      | KLog : { prefix : Text }
      | Process : { pid : Natural, binary : Text, capabilities : List Text }
      | Thread : { process : Text }
      >

let KObject
    : Type
    = { gid : Text, impl : KObjectImpl }

let ApplicationDescription
    : Type
    = { kobjects : List KObject }

in    { kobjects =
        [ { gid = "exit", impl = KObjectImpl.Exit }
        , { gid = "klog_u1", impl = KObjectImpl.KLog { prefix = "U1" } }
        , { gid = "klog_u2", impl = KObjectImpl.KLog { prefix = "U2" } }
        , { gid = "process_u1"
          , impl =
              KObjectImpl.Process
                { pid = 0
                , binary = "hello.user.elf"
                , capabilities = [ "exit", "klog_u1" ]
                }
          }
        , { gid = "process_u2"
          , impl =
              KObjectImpl.Process
                { pid = 1
                , binary = "hello.user.elf"
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
