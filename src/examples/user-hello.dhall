-- An application description is a set of processes and their capabilities.
--
-- TODO Store type definitions in epoxy-harden instead of here.

let KObjectImpl
    : Type
    = < Exit
      | KLog : { prefix : Text }
      | Process : { pid : Natural, binary : Text, capabilities : List Natural }
      | Thread : { process : Natural }
      >

let KObject
    : Type
    = { gid : Natural, impl : KObjectImpl }

let ApplicationDescription
    : Type
    = { kobjects : List KObject }

in    { kobjects =
        [ { gid = 0, impl = KObjectImpl.Exit }
        , { gid = 1, impl = KObjectImpl.KLog { prefix = "U1" } }
        , { gid = 2, impl = KObjectImpl.KLog { prefix = "U2" } }
        , { gid = 3
          , impl =
              KObjectImpl.Process
                { pid = 0, binary = "hello.user.elf", capabilities = [ 0, 1 ] }
          }
        , { gid = 4
          , impl =
              KObjectImpl.Process
                { pid = 1, binary = "hello.user.elf", capabilities = [ 0, 2 ] }
          }
        , { gid = 5, impl = KObjectImpl.Thread { process = 3 } }
        , { gid = 6, impl = KObjectImpl.Thread { process = 4 } }
        ]
      }
    : ApplicationDescription
