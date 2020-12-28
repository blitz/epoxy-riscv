#pragma once

#include <epoxy-api/api.hpp>
#include <epoxy-api/likely.hpp>

class thread;
class syscall_args;

class kobject
{
protected:
  constexpr kobject() = default;

public:
  virtual syscall_result_t invoke(thread *thread, syscall_args const &args) = 0;
};
