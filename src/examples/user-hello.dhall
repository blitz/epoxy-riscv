-- An application description is a set of processes and their capabilities.
{ kobjects =
    [ { gid = 0, kobjType = "EXIT" },
      { gid = 1, kobjType = "KLOG" }]
, processes =
    [ { pid = 0, binary = "hello.user.elf", capabilities = [ 0, 1 ] },
      { pid = 1, binary = "hello.user.elf", capabilities = [ 0, 1 ] }]
}
