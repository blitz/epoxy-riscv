#include "api.hpp"
#include "format.hpp"

extern "C" void main();

namespace {
  // TODO We have to hardcode the virtual addresses here for now. In
  // the future epoxy-harden should generate a nice header with
  // virtual addresses of shared memory regions.
  uint32_t volatile * const pci_ecam = reinterpret_cast<uint32_t *>(0x10000000);

} // anonymous namespace

void main()
{
  format("Hello from userspace!\n");

  // XXX This is just to check whether we can access the memory.
  pci_ecam[0];
}
