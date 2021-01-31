#pragma once

#include <epoxy-api/c_types.hpp>

#include "assert.hpp"


/// An abstraction around the SiFice Platform-Level Interrupt
/// Controller.
class plic
{
  uint32_t volatile * const reg_;
  uint16_t const ndev_;

  using int_no = int;
  using int_threshold = int;

  /// The number of different threshold levels.
  static const int_threshold NTHRESHOLD = 8;

  static constexpr size_t bits_per_word() { return sizeof(reg_[0] * 8); }

  class in_place_bitfield
  {
    uint32_t volatile * const base_;

    size_t bits_per_word() const { return sizeof(reg_[0]) * 8; }

  public:
    in_place_bitfield(uint32_t volatile *reg_, size_t base_offset)
      : base_{&reg_[base_offset / sizeof(base_[0])]}
    {}

    bool get(int_no i) const
    {
      return base_[i / bits_per_word()] & (static_cast<uint32_t>(1) << (i % bits_per_word()));
    }

    void set(int_no i, bool v) const
    {
      uint32_t bit {(static_cast<uint32_t>(1) << (i % bits_per_word()))};

      if (v) {
	base_[i / bits_per_word()] |= bit;
      } else {
	base_[i / bits_per_word()] &= ~bit;
      }
    }
  };

  in_place_bitfield pending_bits() const { return in_place_bitfield { reg_, 0x1000 }; }
  in_place_bitfield enable_bits() const { return in_place_bitfield { reg_, 0x2000 }; }

  /// Return a pointer for the priority register of an interrupt source.
  uint32_t volatile *priority_reg(int_no src) const
  {
    assert(src > 0 and src < ndev_);
    return &reg_[src];
  }

public:

  /// Return the global instances of the PLIC.
  static plic const& global();

  /// The number of interrupts supported by this PLIC.
  uint16_t ndev() const
  {
    return ndev_;
  }

  /// Return the interrupt number of the next pending interrupt. This
  /// "claims" the interrupt.
  int_no claim() const
  {
    return reg_[0x200004 / sizeof(reg_[0])];
  }

  /// Mark an interrupt as being handled. This is the same as EOI on
  /// x86 interrupt controllers.
  void complete(int_no src) const
  {
    assert(src > 0 and src < ndev_);
    reg_[0x200004 / sizeof(reg_[0])] = src;
  }

  /// Set the threshold at which the hart operates.
  ///
  /// A threshold of X masks all interrupts with priority <=
  /// X. Threshold 0 will thus allow all interrupts to pass, threshold
  /// NTHRESHOLD - 1 will mask all external interrupts.
  void set_hart_threshold(int_threshold t) const
  {
    assert(t >= 0 and t < NTHRESHOLD);
    reg_[0x200000 / sizeof(reg_[0])] = t;
  }

  /// Return the current threshold at which the hart operates.
  int_threshold hart_threshold() const
  {
    return reg_[0x200000 / sizeof(reg_[0])];
  }

  /// Set the interrupt's priority.
  void set_interupt_prio(int_no src, int_threshold t) const
  {
    assert(src > 0 and src < ndev_);
    assert(t >= 0 and t < NTHRESHOLD);

    *priority_reg(src) = t;
  }

  /// Returns true of the given interrupt is pending.
  bool is_pending(int_no src) const
  {
    assert(src > 0 and src < ndev_);
    return pending_bits().get(src);
  }

  /// Mask the given interrupt.
  void mask(int_no src) const
  {
    assert(src > 0 and src < ndev_);
    enable_bits().set(src, true);
  }

  /// Mask all interrupts.
  void mask_all() const
  {
    for (size_t i = 0; i < ((ndev_ + bits_per_word() - 1) % bits_per_word()); i++) {
      reg_[0x2000 / sizeof(reg_[0]) + i] = 0;
    }
  }

  /// Unmask the given interrupt.
  void unmask(int_no src) const
  {
    assert(src > 0 and src < ndev_);
    enable_bits().set(src, false);
  }

  /// Construct a PLIC object with a pointer to its registers and the
  /// number of supported interrupts.
  constexpr plic(uint32_t volatile *reg, uint16_t ndev)
    : reg_{reg}, ndev_{ndev}
  {}
};
