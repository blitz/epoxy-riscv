#pragma once

#include "types.hpp"

class pci_device {
  union {
    uint8_t  volatile * const u8_cfg_;
    uint16_t volatile * const u16_cfg_;
    uint32_t volatile * const u32_cfg_;
  };

public:
  static uint8_t const CAP_PTR_MASK {static_cast<uint8_t>(~0b11U)};

  // Accessor functions
  uint8_t  get_byte(uint8_t byte_offset) const { return u8_cfg_[byte_offset]; }

  uint32_t get_vendor_device_id() const { return u32_cfg_[0]; }
  uint16_t get_status()           const { return u16_cfg_[3]; }
  uint8_t  get_cap_ptr()          const { return get_byte(0x34) & CAP_PTR_MASK; };

  bool has_cap_list() const
  {
    return get_status() & (1U << 4 /* Capability List */);
  }

  class pci_cap {
    pci_device *dev_;
    uint8_t offset_;

  public:

    uint8_t get_id() const
    {
      return dev_->get_byte(offset_);
    }

    uint8_t get_next_cap_ptr() const
    {
      return dev_->get_byte(offset_ + 1) & CAP_PTR_MASK;
    }

    pci_cap(pci_device *dev, uint8_t offset)
      : dev_ {dev}, offset_{offset}
    {}
  };

  class pci_cap_iterator {
    pci_device *dev_;
    uint8_t offset_;

  public:

    pci_cap operator*() const
    {
      return {dev_, offset_};
    }

    bool operator==(pci_cap_iterator const &other) const
    {
      // It's broken to compare iterators from different PCI
      // devices.
      assert(dev_ == other.dev_);

      return offset_ == other.offset_;
    }

    bool operator!=(pci_cap_iterator const &other) const
    {
      return not (*this == other);
    }

    pci_cap_iterator &operator++()
    {
      assert(offset_ != 0);

      offset_ = (**this).get_next_cap_ptr();
      return *this;
    }

    pci_cap_iterator(pci_device *dev, uint8_t offset)
      : dev_ {dev}, offset_{offset}
    {}
  };

  class pci_cap_list {
    pci_device *dev_;

  public:

    pci_cap_iterator begin()
    {
      if (dev_->has_cap_list()) {
	return {dev_, dev_->get_cap_ptr()};
      } else {
	return end();
      }
    }

    pci_cap_iterator end() {
      return { dev_, 0};
    }

    explicit pci_cap_list(pci_device *dev)
      : dev_ {dev}
    {}
  };

  pci_cap_list get_cap_list()
  {
    return pci_cap_list {this};
  }

  explicit pci_device(uint32_t volatile *cfg_space)
    : u32_cfg_ {cfg_space}
  {}
};
