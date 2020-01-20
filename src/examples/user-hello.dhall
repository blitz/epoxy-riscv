-- An application description is a set of processes and their capabilities.
{ kobjects =
    [ { gid = 0, kobjType = "exit" },
      { gid = 1, kobjType = "klog" },
      { gid = 2, kobjType = "klog" }]
, processes =
    [ { pid = 0, binary = "hello.user.elf", capabilities = [ 0, 1 ] },
      { pid = 1, binary = "hello.user.elf", capabilities = [ 0, 2 ] }]
}
