#pragma once

#include "config_types.hpp"
#include "exception_frame.hpp"

// System call arguments roughly follow the System-V ABI.
class syscall_args {
public:

  cap_t   cap_idx;

  mword_t arg0;
  mword_t arg1;
  mword_t arg2;
  mword_t arg3;

  syscall_args() = delete;

  explicit syscall_args(exception_frame const &frame)
    : cap_idx {static_cast<cap_t>(frame.a0())},
      arg0 {frame.a1()},
      arg1 {frame.a2()},
      arg2 {frame.a3()},
      arg3 {frame.a4()}
  {}
};
