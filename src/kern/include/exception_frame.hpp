#pragma once

#include <types.hpp>

// General-purpose registers.
//
// Registers are offset by one, i.e. x1 is stored at x[0].
struct gp_regs {
  mword_t x[31] {};
};

struct exception_frame {
  gp_regs regs;
  mword_t pc;

  explicit constexpr exception_frame(mword_t initial_pc)
    : pc {initial_pc}
  {}
};

// These offsets are used from assembly (see exc_entry.S).
static_assert(offsetof(exception_frame, regs) == 0);
static_assert(offsetof(exception_frame, pc)   == 31*8);
