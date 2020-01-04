#pragma once

#include <types.hpp>

// General-purpose registers.
//
// Registers are offset by one, i.e. x1 is stored at x[0].
struct gp_regs {
  mword_t x[31];
};

struct exception_frame {
  gp_regs regs;
};

static_assert(offsetof(exception_frame, regs) == 0);
