{ runCommandNoCC, epoxy-qemu-boot, bootElf }:

runCommandNoCC "epoxy-qemu-hello-test"
  {
    nativeBuildInputs = [ epoxy-qemu-boot ];
  } ''
   epoxy-qemu-boot \
                    -display none \
                    -device loader,file=${bootElf} | tee run.log

   set -x
   grep -q "hello | Hello World" run.log
   grep -q "Last thread is gone" run.log
   set +x
   
   cp run.log $out
  ''
