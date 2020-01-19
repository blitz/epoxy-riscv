#pragma once

#include "api.hpp"
#include "types.hpp"

class thread;
class syscall_args;

class kobject {
protected:
  constexpr kobject() = default;
public:
  virtual syscall_result_t invoke(thread *thread, syscall_args const &args) = 0;
};

// A primitive logging system call.
class klog_kobject final : public kobject {
public:
  syscall_result_t invoke(thread *thread, syscall_args const &args) override;
};

// Graceful exit from a thread.
class exit_kobject final : public kobject {
public:
  [[noreturn]] syscall_result_t invoke(thread *thread, syscall_args const &args) override;
};
