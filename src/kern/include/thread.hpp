#pragma once

#include <epoxy-api/api.hpp>

#include "exception_frame.hpp"

class process;

enum class thread_state : uint8_t { RUNNABLE, BLOCKED, EXITED };

class thread : private exception_frame
{
  // The current thread. Maybe nullptr, if the CPU is idle.
  static thread *active_;

  // The process this thread belongs to.
  process *const process_;

  thread_state state_;

  // Exit to userspace via SRET.
  [[noreturn]] void exit_from_preemption();

public:
  static thread *active() { return active_; }

  exception_frame *frame() { return this; }
  process *get_process() { return process_; }

  bool is_runnable() const { return state_ == thread_state::RUNNABLE; }

  void exit() { state_ = thread_state::EXITED; }
  void block() { state_ = thread_state::BLOCKED; }
  void unblock() { state_ = thread_state::RUNNABLE; }

  [[noreturn]] void finish_syscall(syscall_result_t ret)
  {
    regs_[10] = static_cast<mword_t>(ret);
    activate();
  }

  [[noreturn]] void activate();

  thread(process *process, mword_t sp, mword_t a0, mword_t a1);
};
