#include "assert.hpp"
#include "io.hpp"
#include "kobject.hpp"
#include "process.hpp"
#include "scheduler.hpp"
#include "syscall_args.hpp"
#include "thread.hpp"

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
