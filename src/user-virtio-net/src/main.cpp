#include "format.hpp"

extern "C" void main();

namespace {
  // TODO We have to hardcode the virtual addresses here for now. In
  // the future epoxy-harden should generate a nice header with
  // virtual addresses of shared memory regions.
  uint32_t volatile * const virtio_net_pci_cfg = reinterpret_cast<uint32_t *>(0x10000000);

} // anonymous namespace

void main()
{
  format("Hello from virtio-io!\n");

  // XXX This is just to check whether we can access the memory.
  for (size_t i = 0; i < (64 / sizeof(*virtio_net_pci_cfg)); i++) {
    format("PCI[", i * sizeof(*virtio_net_pci_cfg), "] = ", virtio_net_pci_cfg[i], "\n");
  }
}
