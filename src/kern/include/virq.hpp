#pragma once

#include "kobject.hpp"
#include "state.hpp"  // for the number of threads
#include "util.hpp"
#include "vector.hpp"

class plic_irq_link;

/// A virtual interrupt.
class virq
{
  bool triggered_ {false};

  plic_irq_link *const irq_link_ {nullptr};

  /// The list of all threads that are currently blocked on this event
  /// source.
  vector<thread *, array_size(threads)> blocked_threads_;

public:
  /// A default constructed vIRQ that doesn't connect to a real interrupt source.
  virq() = default;

  /// A vIRQ that is wired to a real interrupt source.
  explicit virq(plic_irq_link *irq_link);

  /// Enqueue a thread to the wait list.
  void enqueue_waiter(thread *thread)
  {
    assert(thread->is_runnable());

    thread->block();
    blocked_threads_.push_back(thread);
  }

  /// Return the last value of the trigger and reset it to zero
  /// internally.
  bool consume_value();

  void trigger()
  {
    triggered_ |= true;

    if (blocked_threads_.size() > 0) {
      thread *t = blocked_threads_.pop_front();

      assert(not t->is_runnable());
      t->unblock();

      // TODO We might want to expedite scheduling here.
    }
  }
};

class virq_trigger_kobject final : public kobject
{
  virq *const virq_;

public:
  syscall_result_t invoke(thread *thread, syscall_args const &args) override;

  constexpr virq_trigger_kobject(virq *virq) : virq_ {virq} { assert(virq); }
};

class virq_wait_kobject final : public kobject
{
  virq *const virq_;

public:
  syscall_result_t invoke(thread *thread, syscall_args const &args) override;

  constexpr virq_wait_kobject(virq *virq) : virq_ {virq} { assert(virq); }
};
