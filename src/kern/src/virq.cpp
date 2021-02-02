#include "virq.hpp"

#include "scheduler.hpp"

syscall_result_t virq_wait_kobject::invoke(thread *thread,
                                           [[maybe_unused]] syscall_args const &args)
{
  if (virq_->consume_value()) {
    // We consumed one event.
    return syscall_result_t::OK;
  }

  // We need to wait for an event.
  virq_->enqueue_waiter(thread);
  schedule();
}
