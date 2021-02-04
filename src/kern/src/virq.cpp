#include "virq.hpp"

#include "plic.hpp"
#include "scheduler.hpp"

bool virq::consume_value()
{
  bool old = triggered_;

  // If we set triggered_ back to false, we need to reenable the
  // interrupt source.
  if (irq_link_ and old) {
    irq_link_->unmask();
  }

  triggered_ = false;
  return old;
}

syscall_result_t virq_trigger_kobject::invoke([[maybe_unused]] thread *thread,
                                              [[maybe_unused]] syscall_args const &args)
{
  virq_->trigger();
  return syscall_result_t::OK;
}

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
