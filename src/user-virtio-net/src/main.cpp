#include <cassert>
#include <cstdio>
#include <pprintpp/pprintpp.hpp>
#include <range/v3/view/filter.hpp>
#include <range/v3/view/transform.hpp>

#include "pci_device.hpp"
#include "virtio-spec.hpp"

extern "C" int main();

namespace
{
// TODO We have to hardcode the virtual addresses here for now. In
// the future epoxy-harden should generate a nice header with
// virtual addresses of shared memory regions.
auto const virtio_net_pci_cfg {reinterpret_cast<uint32_t volatile *>(0x10000000)};

class virtio_net_device : public pci_device
{
private:
  // The device and vendor ID of a virtio-net device.
  uint32_t const VIRTIO_NET_ID {0x10001af4};

public:
  virtio_net_device(uint32_t volatile *cfg_space) : pci_device {cfg_space}
  {
    // This is only an assertion, because we are guaranteed that we
    // get a correct device by construction. The assertion is just a
    // desaster fail-safe when you configured the system wrong.
    assert(get_vendor_device_id() == VIRTIO_NET_ID);
  }
};

class virtio_vendor_pci_cap : public pci_device::pci_cap
{
  static uint8_t const ID {0x9};

public:
  // This type means that bar_no, bar_offset, and bar_length are
  // writable to select location in one of the BARs and there is an
  // additional 32-bit field at offset 16 to read/write data from.
  static const uint8_t CFG_TYPE_PCI_CFG {5};

  uint8_t get_cfg_type() const { return get_u8(3); }
  uint8_t get_bar_no() const { return get_u8(4); }
  uint32_t get_bar_offset() const { return get_u32(8); }
  uint32_t get_bar_length() const { return get_u32(12); }

  // If this is true, you can static_cast a pci_cap to this type.
  static bool converts_from(pci_device::pci_cap const &cap)
  {
    return cap.get_id() == ID and cap.get_len() >= 16;
  }

  explicit virtio_vendor_pci_cap(pci_device::pci_cap const &other) : pci_device::pci_cap {other}
  {
    assert(converts_from(other));
  }
};

}  // anonymous namespace

int main()
{
  using namespace ranges;

  pprintf("Hello from virtio-io!\n");

  virtio_net_device virtio_net {virtio_net_pci_cfg};

  auto caps {virtio_net.get_caps()};

  for (auto const vendor_cap : caps | views::filter(virtio_vendor_pci_cap::converts_from) |
                                   views::transform([](pci_device::pci_cap const &cap) {
                                     return static_cast<virtio_vendor_pci_cap>(cap);
                                   })) {
    pprintf("cfg_types={#x} bar={} offset={#x} length={#x}\n", vendor_cap.get_cfg_type(),
            vendor_cap.get_bar_no(), vendor_cap.get_bar_offset(), vendor_cap.get_bar_length());
  }

  return 0;
}
