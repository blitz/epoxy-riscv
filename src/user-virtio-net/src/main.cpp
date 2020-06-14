#include <cassert>
#include <cstdio>
#include <iomanip>
#include <optional>
#include <pprintpp/pprintpp.hpp>
#include <range/v3/view/filter.hpp>
#include <range/v3/view/transform.hpp>
#include <sstream>
#include <string>

#include "pci_device.hpp"
#include "virtio-spec.hpp"

extern "C" int main();

namespace
{
// TODO We have to hardcode the virtual addresses here for now. In
// the future epoxy-harden should generate a nice header with
// virtual addresses of shared memory regions.
//
// See virtioNetAddressSpace in user-hello.dhall.
auto const virtio_net_pci_cfg {reinterpret_cast<uint32_t volatile *>(0x10000000)};

uint32_t const virtio_net_pci_bar4_phys {0x40000000};
auto const virtio_net_bar4 {reinterpret_cast<uint32_t volatile *>(0x11000000)};

struct mac_address {
  std::array<uint8_t, 6> raw {};

  std::string to_string() const
  {
    std::stringstream ss;

    for (size_t i = 0; i < raw.size(); i++) {
      if (i != 0) {
        ss << ":";
      }

      ss << std::hex << std::setw(2) << std::setfill('0') << static_cast<unsigned>(raw[i]);
    }

    return ss.str();
  }

  template <typename T>
  mac_address(T (&bytes)[6])
  {
    static_assert(sizeof(T) == 1);
    std::copy_n(bytes, raw.size(), raw.begin());
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

class virtio_net_device : public pci_device
{
private:
  // The device and vendor ID of a virtio-net device.
  uint32_t const VIRTIO_NET_ID {0x10001af4};

  // MMIO registers for the PCI generic part.
  virtio::pci_common_cfg volatile *mmio_pci_common;

  // MMIO registers for the network specific part.
  virtio::virtio_net_config volatile *mmio_net_config;

  void initialize_bars()
  {
    // TODO This should be autoconfigured.
    set_bar(4, virtio_net_pci_bar4_phys);

    // Failure to enable memory decoding leads to load access faults
    // from the CPU (at least on QEMU).
    enable_mem_decoding();
  }

  std::optional<virtio_vendor_pci_cap> find_mmio_region(virtio::pci_vendor_cap_type type)
  {
    using namespace ranges;

    auto caps {get_caps()};
    auto matching {
        caps | views::filter(virtio_vendor_pci_cap::converts_from) |
        views::transform([](pci_device::pci_cap const &cap) {
          return static_cast<virtio_vendor_pci_cap>(cap);
        }) |
        views::filter([type](virtio_vendor_pci_cap cap) { return cap.get_cfg_type() == type; })};

    if (matching.begin() != matching.end()) {
      return *matching.begin();
    } else {
      return std::nullopt;
    }
  }

  void volatile *mmio_region_from_cap(virtio_vendor_pci_cap const &cap)
  {
    // TODO This should be more sophisticated...
    assert(cap.get_bar_no() == 4);

    return reinterpret_cast<char volatile *>(virtio_net_bar4) + cap.get_bar_offset();
  }

  void discover_mmio_regions()
  {
    using namespace ranges;

    if (auto opt_cap {find_mmio_region(virtio::PCI_CAP_COMMON_CFG)}; opt_cap) {
      mmio_pci_common =
          static_cast<virtio::pci_common_cfg volatile *>(mmio_region_from_cap(*opt_cap));
    }

    if (auto opt_cap {find_mmio_region(virtio::PCI_CAP_DEVICE_CFG)}; opt_cap) {
      mmio_net_config =
          static_cast<virtio::virtio_net_config volatile *>(mmio_region_from_cap(*opt_cap));
    }

    if (not mmio_pci_common or not mmio_net_config) {
      pprintf("Failed to find MMIO regions?!\n");
      abort();
    }
  }

  mac_address get_mac() const { return mmio_net_config->mac; }

public:
  void print_device_info()
  {
    pprintf("virtio-net {s}: device_features={#x} num_queues={}\n", get_mac().to_string().c_str(),
            mmio_pci_common->device_feature, mmio_pci_common->num_queues);
  }

  virtio_net_device(uint32_t volatile *cfg_space) : pci_device {cfg_space}
  {
    // This is only an assertion, because we are guaranteed that we
    // get a correct device by construction. The assertion is just a
    // desaster fail-safe when you configured the system wrong.
    assert(get_vendor_device_id() == VIRTIO_NET_ID);

    initialize_bars();
    discover_mmio_regions();

    enable_bus_master();  // Allow the device to access memory via DMA.
  }
};

}  // anonymous namespace

int main()
{
  using namespace ranges;

  pprintf("Hello from virtio-io!\n");

  virtio_net_device virtio_net {virtio_net_pci_cfg};
  virtio_net.print_device_info();

  // TODO Set up DMA queues.

  return 0;
}
