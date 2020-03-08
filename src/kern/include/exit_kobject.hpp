#pragma once

#include "kobject.hpp"

// Graceful exit from a thread.
class exit_kobject final : public kobject
{
public:
  [[noreturn]] syscall_result_t invoke(thread *thread, syscall_args const &args) override;
};
