#pragma once

#include <cstdint>
#include <vector>

class pci_device
{
  union {
    uint8_t volatile *const u8_cfg_;
    uint16_t volatile *const u16_cfg_;
    uint32_t volatile *const u32_cfg_;
  };

public:
  static uint8_t const CAP_PTR_MASK {static_cast<uint8_t>(~0b11U)};

  // Accessor functions
  uint8_t get_u8(uint8_t byte_offset) const { return u8_cfg_[byte_offset]; }

  uint32_t get_u32(uint8_t byte_offset) const
  {
    assert((byte_offset & (sizeof(uint32_t) - 1)) == 0);

    return u32_cfg_[byte_offset / sizeof(uint32_t)];
  }

  uint32_t get_vendor_device_id() const { return u32_cfg_[0]; }
  uint16_t get_status() const { return u16_cfg_[3]; }
  uint8_t get_cap_ptr() const { return get_u8(0x34) & CAP_PTR_MASK; };

  bool has_cap_list() const { return get_status() & (1U << 4 /* Capability List */); }

  class pci_cap
  {
    pci_device *dev_;
    uint8_t offset_;

  public:
    uint8_t get_u8(uint8_t offset) const
    {
      // TODO Pretty slow, because the compiler can't cache the
      // length.
      assert(offset < get_len());

      return dev_->get_u8(offset_ + offset);
    }

    uint32_t get_u32(uint8_t offset) const { return dev_->get_u32(offset_ + offset); }

    uint8_t get_id() const { return dev_->get_u8(offset_); }

    uint8_t get_next_cap_ptr() const { return dev_->get_u8(offset_ + 1) & CAP_PTR_MASK; }

    uint8_t get_len() const { return dev_->get_u8(offset_ + 2); }

    pci_cap(pci_device *dev, uint8_t offset) : dev_ {dev}, offset_ {offset} {}
  };

  std::vector<pci_cap> get_caps()
  {
    std::vector<pci_cap> caps;

    if (has_cap_list()) {
      for (uint8_t off = get_cap_ptr(); off != 0;) {
        caps.emplace_back(this, off);
        off = caps.back().get_next_cap_ptr();
      }
    }

    return caps;
  }

  explicit pci_device(uint32_t volatile *cfg_space) : u32_cfg_ {cfg_space} {}
};
