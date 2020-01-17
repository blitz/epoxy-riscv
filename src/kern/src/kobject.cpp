#include "assert.hpp"
#include "io.hpp"
#include "kobject.hpp"
#include "process.hpp"
#include "scheduler.hpp"
#include "syscall_args.hpp"
#include "thread.hpp"

syscall_result_t kobject::invoke(thread *thread, syscall_args const &args)
{
  switch (type) {
  case kobject_type::KLOG:
    return static_cast<klog_kobject *>(this)->invoke(thread, args);
  case kobject_type::EXIT:
    return static_cast<exit_kobject *>(this)->invoke(thread, args);
  }

  assert(false);
  __builtin_unreachable();
}

syscall_result_t klog_kobject::invoke(thread *, syscall_args const &args)
{
  put_char(args.arg0 & 0xFF);
  return syscall_result_t::OK;
}

syscall_result_t exit_kobject::invoke(thread *thread, syscall_args const &args)
{
  format(">> Thread of process ", thread->get_process()->pid(), " is done.\n");
  thread->exit();
  schedule();
}
