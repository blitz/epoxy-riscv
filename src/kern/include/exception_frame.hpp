#pragma once

#include <types.hpp>

// General-purpose registers.
//
// Registers are offset by one, i.e. x1 is stored at x[0].
struct gp_regs {
  mword_t x[31] {};
};

class exception_frame {
public:
  gp_regs regs_;
  mword_t pc_;

  mword_t a0() const { return regs_.x[9]; }
  mword_t a1() const { return regs_.x[10]; }
  mword_t a2() const { return regs_.x[11]; }
  mword_t a3() const { return regs_.x[12]; }
  mword_t a4() const { return regs_.x[13]; }

protected:

  explicit constexpr exception_frame(mword_t pc)
    : pc_ {pc}
  {}
};

  // These offsets are used from assembly (see exc_entry.S).
static_assert(offsetof(exception_frame, regs_) == 0);
static_assert(offsetof(exception_frame, pc_)   == 31*8);
