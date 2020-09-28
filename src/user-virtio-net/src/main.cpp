#include <array>
#include <cassert>
#include <cstdio>
#include <cstring>
#include <iomanip>
#include <numeric>
#include <optional>
#include <pprintpp/pprintpp.hpp>
#include <range/v3/range/conversion.hpp>
#include <range/v3/view/filter.hpp>
#include <range/v3/view/generate.hpp>
#include <range/v3/view/iota.hpp>
#include <range/v3/view/take.hpp>
#include <range/v3/view/transform.hpp>
#include <sstream>

#include "pci_device.hpp"
#include "virtio-spec.hpp"
#include "vring.hpp"

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

// See user-hello.dhall for this. These are identity mapped DMA buffers.
uint32_t const virtio_net_dma_phys {0x82200000};
size_t const virtio_net_dma_len {0x100000};
void *const virtio_net_dma_virt {reinterpret_cast<void *>(virtio_net_dma_phys)};

bool is_power_of_two(uint64_t x)
{
  return x == 0 ? false : ((x & (x - 1)) == 0);
}

// A simple bump-pointer allocator for DMA-able memory.
class dma_allocator
{
  uintptr_t const start_phys;
  size_t const dma_length;

  // The position of free memory.
  size_t cur {0};

  void align_to(size_t alignment)
  {
    assert(is_power_of_two(alignment));

    size_t mask = alignment - 1;

    // We could handle misaligned DMA areas, but currently we don't.
    assert((start_phys & mask) == 0);

    cur = (cur + mask) & ~mask;

    if (cur >= dma_length) {
      pprintf("Ran out of DMA space!");
      abort();
    }
  }

  // Allocate zero-initialized memory.
  void *allocate(size_t length, size_t alignment)
  {
    align_to(alignment);

    void *ptr = reinterpret_cast<void *>(start_phys + cur);
    cur += length;

    if (cur >= dma_length) {
      pprintf("Not enough DMA memory left to satisfy allocation of {} bytes.\n", length);
      abort();
    }

    memset(ptr, 0, length);
    return ptr;
  }

public:
  // Allocate backing store for a DMA-able data structure.
  template <typename T>
  T *allocate()
  {
    return new (reinterpret_cast<void *>(allocate(sizeof(T), alignof(T)))) T;
  }

  dma_allocator(uintptr_t dma_start_phys, void *dma_start_virt, size_t length)
      : start_phys {dma_start_phys}, dma_length {length}
  {
    assert(dma_start_phys == reinterpret_cast<uintptr_t>(dma_start_virt));
  }
};

template <typename CONTAINER>
uint64_t bits_to_word(CONTAINER bits)
{
  return std::accumulate(
      std::begin(bits), std::end(bits), static_cast<uint64_t>(0),
      [](uint64_t accum, int bit) { return accum | (static_cast<uint64_t>(1) << bit); });
}

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

  // Finds the first PCI virtio vendor capability that matches the
  // given type.
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

  uint64_t get_device_features() const
  {
    uint32_t lo, hi;

    mmio_pci_common->device_feature_select = 0;
    lo = mmio_pci_common->device_feature;

    mmio_pci_common->device_feature_select = 1;
    hi = mmio_pci_common->device_feature;

    return static_cast<uint64_t>(hi) << 32 | lo;
  }

  void set_driver_features(uint64_t driver_features)
  {
    mmio_pci_common->driver_feature_select = 0;
    mmio_pci_common->driver_feature = static_cast<uint32_t>(driver_features);

    mmio_pci_common->driver_feature_select = 1;
    mmio_pci_common->driver_feature = static_cast<uint32_t>(driver_features >> 32);
  }

  void negotiate_features()
  {
    auto const device_features {get_device_features()};
    auto required_features {bits_to_word(std::initializer_list {virtio::VIRTIO_F_VERSION_1})};

    if ((device_features & required_features) != required_features) {
      pprintf("Missing required features: {x} vs {x}\n", device_features, required_features);
      abort();
    }

    set_driver_features(required_features);

    mmio_pci_common->device_status |= virtio::FEATURES_OK;

    if (not(mmio_pci_common->device_status & virtio::FEATURES_OK)) {
      pprintf("Device did not accept our feature selection.\n");
      abort();
    }
  }

  template <size_t QUEUE_SIZE>
  void setup_queue(size_t queue_no, virtio::virtq<QUEUE_SIZE> *vq)
  {
    assert(queue_no < mmio_pci_common->num_queues);

    mmio_pci_common->queue_select = queue_no;

    if (mmio_pci_common->queue_size < QUEUE_SIZE) {
      pprintf("Trying to setup queue {} with size {}, but it only supports {} elements.\n",
              queue_no, QUEUE_SIZE, mmio_pci_common->queue_size);
      abort();
    }

    mmio_pci_common->queue_size = QUEUE_SIZE;

    // TODO We assume 1:1 mapping of the DMA area.
    mmio_pci_common->queue_desc = reinterpret_cast<uintptr_t>(&vq->desc);
    mmio_pci_common->queue_driver = reinterpret_cast<uintptr_t>(&vq->avail);
    mmio_pci_common->queue_device = reinterpret_cast<uintptr_t>(&vq->used);

    pprintf("Enabling queue{}: vq={}\n", queue_no, vq);

    mmio_pci_common->queue_enable = 1;
  }

  using rx_virtq = virtio::virtq<32>;
  using tx_virtq = virtio::virtq<32>;

  rx_virtq *rx;
  tx_virtq *tx;

  vring<rx_virtq::queue_size> rx_ring {rx};

  using packet_buffer = std::array<uint8_t, 2048>;

public:
  void print_device_info()
  {
    pprintf("virtio-net {s}: device_features={#x} num_queues={}\n", get_mac().to_string().c_str(),
            mmio_pci_common->device_feature, mmio_pci_common->num_queues);
  }

  virtio_net_device(uint32_t volatile *cfg_space, dma_allocator *dma_alloc)
      : pci_device {cfg_space},
        rx {dma_alloc->allocate<rx_virtq>()},
        tx {dma_alloc->allocate<tx_virtq>()}
  {
    // This is only an assertion, because we are guaranteed that we
    // get a correct device by construction. The assertion is just a
    // desaster fail-safe when you configured the system wrong.
    assert(get_vendor_device_id() == VIRTIO_NET_ID);

    initialize_bars();
    discover_mmio_regions();
    enable_bus_master();  // Allow the device to access memory via DMA.

    mmio_pci_common->device_status |= virtio::ACKNOWLEDGE | virtio::DRIVER;

    negotiate_features();

    // Queue Setup
    size_t nqueues = mmio_pci_common->num_queues;
    if (nqueues < 3) {
      pprintf("virtio device has too few queues ({}) to be a network device?\n", nqueues);
      abort();
    }

    // Dump some queue statistics.
    for (size_t i = 0; i < nqueues; i++) {
      mmio_pci_common->queue_select = i;

      auto size {mmio_pci_common->queue_size};
      pprintf("queue{}: max_size={}\n", i, size);
    }

    // Start configuring queues.
    size_t const rx_queue_no = 0;
    size_t const tx_queue_no = 1;
    // size_t const ctrl_queue_no = nqueues - 1;

    setup_queue(rx_queue_no, rx);
    setup_queue(tx_queue_no, tx);

    mmio_pci_common->device_status |= virtio::DRIVER_OK;

    // Populate the receive queue with buffers.
    for (packet_buffer *packet :
         (ranges::views::generate([dma_alloc] { return dma_alloc->allocate<packet_buffer>(); }) |
          ranges::views::take(rx_virtq::queue_size - 1))) {
      rx_ring.add_buf({packet->data(), static_cast<uint32_t>(packet->size())},
                      virtio::VIRTQ_DESC_F_WRITE);
    }
  }
};

}  // anonymous namespace

extern "C" {
#include <uip.h>
}

int main()
{
  using namespace ranges;

  pprintf("Hello from virtio-io!\n");

  static dma_allocator dma_allocator {virtio_net_dma_phys, virtio_net_dma_virt, virtio_net_dma_len};

  static virtio_net_device virtio_net {virtio_net_pci_cfg, &dma_allocator};
  virtio_net.print_device_info();

  pprintf("Initializing uIP...\n");

  uip_init();

  uip_ipaddr_t ipaddr;

  // The 10.0.2.x addresses are Qemu's default (see -netdev user).
  uip_ipaddr(ipaddr, 10, 0, 2, 4);
  uip_sethostaddr(ipaddr);

  // The default router. This is also the Qemu default.
  uip_ipaddr(ipaddr, 10, 0, 2, 2);
  uip_setdraddr(ipaddr);

  uip_ipaddr(ipaddr, 255, 255, 255, 0);
  uip_setnetmask(ipaddr);

  hello_world_init();

  // TODO Implement me
  // pprintf("Starting packet loop.\n");

  return 0;
}
