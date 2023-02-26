#pragma once

#include <epoxy-api/c_types.hpp>
#include <epoxy-api/types.hpp>

#include "assert.hpp"

// General-purpose registers.
class gp_regs
{
private:
  // Registers are offset by one, i.e. x1 is stored at reg[0].
  mword_t reg[31] {};

public:
  constexpr mword_t &operator[](size_t i)
  {
    assert(i > 0 and i < 32);
    return reg[i - 1];
  }

  mword_t const &operator[](size_t i) const
  {
    assert(i > 0 and i < 32);
    return reg[i - 1];
  }
};

class exception_frame
{
public:
  gp_regs regs_;
  mword_t pc_;

  mword_t a0() const { return regs_[10]; }
  mword_t a1() const { return regs_[11]; }
  mword_t a2() const { return regs_[12]; }
  mword_t a3() const { return regs_[13]; }
  mword_t a4() const { return regs_[14]; }

protected:
  explicit constexpr exception_frame(mword_t pc, mword_t sp, mword_t a0, mword_t a1) : pc_ {pc}
  {
    regs_[2] = sp;
    regs_[10] = a0;
    regs_[11] = a1;
  }
};

// These offsets are used from assembly (see exc_entry.S).
static_assert(offsetof(exception_frame, regs_) == 0);
static_assert(offsetof(exception_frame, pc_) == 31 * __SIZEOF_POINTER__);
