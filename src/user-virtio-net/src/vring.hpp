#pragma once

#include <cassert>
#include <optional>

#include "util.hpp"
#include "virtio-spec.hpp"

struct dma_buf {
  void *data;
  uint32_t length;

  virtio::virtq_desc to_virtq_desc(uint16_t flags = 0) const
  {
    return {reinterpret_cast<uintptr_t>(data), length, flags, 0};
  }
};

template <size_t QUEUE_SIZE>
class vring
{
  virtio::virtq<QUEUE_SIZE> *const vq;

  std::vector<uint16_t> free_descs {ranges::views::iota(static_cast<uint16_t>(0), QUEUE_SIZE) |
                                    ranges::to<std::vector>()};

  uint16_t last_used {0};

public:
  bool add_buf(dma_buf const &buf, uint16_t virtio_desc_flags = 0)
  {
    if (auto maybe_free_desc {try_pop_back(free_descs)}; maybe_free_desc) {
      uint16_t const new_desc {*maybe_free_desc};

      assert(new_desc < QUEUE_SIZE);

      vq->desc[new_desc] = buf.to_virtq_desc(virtio_desc_flags);

      vq->avail.ring[vq->avail.idx % QUEUE_SIZE] = new_desc;

      // The device only _reads_ this value concurrently, so we can
      // modify it without atomics, but we must make sure that earlier
      // memory is written back and that _writing_ the index is
      // atomic.
      //
      // TODO: Validate that we do the correct memory ordering.
      __atomic_store_n(&vq->avail.idx, vq->avail.idx + 1, __ATOMIC_RELEASE);

      return true;
    } else {
      return false;
    }
  }

  std::optional<dma_buf> get_buf()
  {
    auto device_used {__atomic_load_n(&vq->used.idx, __ATOMIC_ACQUIRE)};

    if (device_used == last_used) {
      return std::nullopt;
    }

    uint16_t const used_idx = last_used++;
    virtio::virtq_used_elem const used {vq->used.ring[used_idx % QUEUE_SIZE]};

    assert(used.id < QUEUE_SIZE);
    virtio::virtq_desc const used_desc {vq->desc[used.id]};

    // We can reuse the descriptor now.
    free_descs.push_back(used.id);

    // We assume that the whole buffer is contiguous. This is not
    // true, if we start negotiating more features with the virtio
    // device.
    assert(used.len <= used_desc.len);

    return dma_buf {reinterpret_cast<void *>(used_desc.addr), used.len};
  }

  explicit vring(virtio::virtq<QUEUE_SIZE> *virtq) : vq {virtq} {}
};
