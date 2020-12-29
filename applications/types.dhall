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

in  { SharedMemorySource
    , SharedMemoryPermissions
    , AddressSpaceElem
    , AddressSpace
    , ThreadStack
    , KObjectImpl
    , KObject
    , ApplicationDescription
    }
