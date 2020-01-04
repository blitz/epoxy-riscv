#pragma once

#include <types.hpp>
#include "exception_frame.hpp"

class process;

enum class thread_state : uint8_t { RUNNABLE, BLOCKED };

class thread : private exception_frame {
  // The current thread. Maybe nullptr, if the CPU is idle.
  static thread *active_;

  // The process this thread belongs to.
  process * const process_;

  thread_state state_;

  // Exit to userspace via SRET.
  [[noreturn]] void exit_from_preemption();

public:

  static thread *active() { return active_; }

  bool is_runnable() const { return state_ == thread_state::RUNNABLE; }

  [[noreturn]] void activate();

  constexpr thread(process *process, mword_t user_entry)
    : exception_frame {user_entry}, process_ {process}, state_ {thread_state::RUNNABLE}
  {}
};
