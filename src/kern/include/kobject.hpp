#pragma once

#include "api.hpp"
#include "types.hpp"

class process;

using kobj_id_t = int;

enum class kobject_type {
  // A primitive logging system call.
  KLOG,

  // System call to yield the current time slice.
  YIELD,
};

struct kobject {
  kobject_type type;

  syscall_result_t invoke(uint64_t w0, uint64_t w1, uint64_t w2);
};
