#pragma once

#include <types.hpp>
#include "exception_frame.hpp"

class process;

class thread {
  // The current thread. Maybe nullptr, if the CPU is idle.
  static thread *active_;

  // The process this thread belongs to.
  process * const process_;

  exception_frame frame_;

public:

  // Exit to userspace via SRET.
  [[noreturn]] void exit_from_syscall();

  // XXX Implement me
  constexpr thread(process *process, mword_t user_entry)
    : process_ {process}, frame_ {user_entry}
  {}
};
