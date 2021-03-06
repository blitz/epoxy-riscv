#pragma once

#include "kobject.hpp"
#include "vector.hpp"

// A primitive logging system call.
//
// Messages are line-buffered and printed with a prefix indicating the
// process.
class klog_kobject final : public kobject
{
  char const *const prefix_;

  vector<char, 80> line_buffer_;

public:
  syscall_result_t invoke(thread *thread, syscall_args const &args) override;

  constexpr klog_kobject(char const *prefix) : prefix_ {prefix} {}
};
