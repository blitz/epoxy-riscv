#include "assert.hpp"
#include "io.hpp"
#include "kobject.hpp"
#include "syscall_args.hpp"

syscall_result_t kobject::invoke(syscall_args const &args)
{
  switch (type) {
  case kobject_type::KLOG:
    return static_cast<klog_kobject *>(this)->invoke(args);
  }

  assert(false);
  __builtin_unreachable();
}

syscall_result_t klog_kobject::invoke(syscall_args const &args)
{
  put_char(args.arg0 & 0xFF);
  return syscall_result_t::OK;
}
