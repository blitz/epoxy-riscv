#pragma once

// External API

#include "types.hpp"

using cap_t = int;
const cap_t invalid_capability = -1;

enum class syscall_result_t {
  OK,
  NOCAP,
};

inline syscall_result_t invoke(cap_t cap,
                               mword_t arg0 = 0,
                               mword_t arg1 = 0,
                               mword_t arg2 = 0,
                               mword_t arg3 = 0)
{
  register mword_t result asm("a0") = static_cast<mword_t>(cap);
  register mword_t arg0_ asm("a1") = arg0;
  register mword_t arg1_ asm("a2") = arg1;
  register mword_t arg2_ asm("a3") = arg2;
  register mword_t arg3_ asm("a4") = arg3;

  asm volatile("ecall" : "+r"(result) : "r"(arg0_), "r"(arg1_), "r"(arg2_), "r"(arg3_) : "memory");

  return static_cast<syscall_result_t>(result);
}
