#pragma once

#include "api.hpp"
#include "types.hpp"

class thread;
class syscall_args;

class kobject {
public:
  virtual syscall_result_t invoke(thread *thread, syscall_args const &args) = 0;
};

// A primitive logging system call.
class klog_kobject final : public kobject {
  syscall_result_t invoke(thread *thread, syscall_args const &args) override;
};
static_assert(sizeof(klog_kobject) == sizeof(kobject));

// Graceful exit from a thread.
class exit_kobject final : public kobject {
  [[noreturn]] syscall_result_t invoke(thread *thread, syscall_args const &args) override;
};
static_assert(sizeof(exit_kobject) == sizeof(kobject));
