#pragma once

#include <types.hpp>

class spinlock {
public:
  uint8_t value = 0;

public:
  void acquire()
  {
    while (__atomic_test_and_set(&value, __ATOMIC_ACQUIRE)) {
      // Do nothing.
      // TODO Is there a RISC-V equivalent of "pause"?
    }
  }

  void release()
  {
    __atomic_clear(&value, __ATOMIC_RELEASE);
  }
};

class spinlock_guard {
  spinlock &lock;

public:

  spinlock_guard(spinlock &lock_)
    : lock(lock_)
  {
    lock.acquire();
  }

  spinlock_guard(spinlock_guard const &) = delete;

  ~spinlock_guard()
  {
    lock.release();
  }
};
