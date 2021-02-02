#pragma once

#include "kobject.hpp"
#include "state.hpp"  // for the number of threads
#include "util.hpp"
#include "vector.hpp"

/// A virtual interrupt.
class virq
{
  bool triggered {false};

  /// The list of all threads that are currently blocked on this event
  /// source.
  vector<thread *, array_size(threads)> blocked_threads;

public:
  /// Enqueue a thread to the wait list.
  void enqueue_waiter(thread *thread)
  {
    assert(thread->is_runnable());

    thread->block();
    blocked_threads.push_back(thread);
  }

  /// Return the last value of the trigger and reset it to zero
  /// internally.
  bool consume_value()
  {
    bool old {triggered};

    triggered = false;
    return old;
  }
};

class virq_wait_kobject final : public kobject
{
  virq *const virq_;

public:
  syscall_result_t invoke(thread *thread, syscall_args const &args) override;

  constexpr virq_wait_kobject(virq *virq) : virq_ {virq} {}
};
