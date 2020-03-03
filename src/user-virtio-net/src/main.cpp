#include "assert.hpp"
#include "format.hpp"
#include "pci_device.hpp"

extern "C" void main();

namespace {

  // TODO We have to hardcode the virtual addresses here for now. In
  // the future epoxy-harden should generate a nice header with
  // virtual addresses of shared memory regions.
  auto const virtio_net_pci_cfg {reinterpret_cast<uint32_t volatile *>(0x10000000)};

  class virtio_net_device : public pci_device {
  private:

    // The device and vendor ID of a virtio-net device.
    uint32_t const VIRTIO_NET_ID {0x10001af4};

  public:

    virtio_net_device(uint32_t volatile *cfg_space)
      : pci_device {cfg_space}
    {
      // This is only an assertion, because we are guaranteed that we
      // get a correct device by construction. The assertion is just a
      // desaster fail-safe when you configured the system wrong.
      assert(get_vendor_device_id() == VIRTIO_NET_ID);
    }
  };

} // anonymous namespace

void main()
{
  format("Hello from virtio-io!\n");

  virtio_net_device virtio_net {virtio_net_pci_cfg};

  for (auto const pci_cap : virtio_net.get_cap_list()) {
    format("CAP ID ", pci_cap.get_id(), "\n");
  }
}
