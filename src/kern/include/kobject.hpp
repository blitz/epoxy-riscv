#pragma once

#include "api.hpp"
#include "types.hpp"

class thread;

using kobj_id_t = int;

enum class kobject_type {
  // A primitive logging system call.
  KLOG,

  // Graceful exit from a thread.
  EXIT,
};

class syscall_args;

struct kobject {
  kobject_type type;
  syscall_result_t invoke(thread *thread, syscall_args const &args);
};

struct klog_kobject : public kobject {
  syscall_result_t invoke(thread *thread, syscall_args const &args);
};
static_assert(sizeof(klog_kobject) == sizeof(kobject));

struct exit_kobject : public kobject {
  [[noreturn]] syscall_result_t invoke(thread *thread, syscall_args const &args);
};
static_assert(sizeof(exit_kobject) == sizeof(kobject));
