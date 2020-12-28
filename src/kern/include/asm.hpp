#pragma once

#include <epoxy-api/c_types.hpp>
#include <epoxy-api/types.hpp>

struct exception_frame;

// The entry point for the C++ part of the kernel.
extern "C" [[noreturn]] void start();

// The exception/interrupt entry point in assembly.
extern "C" char asm_exc_entry[];

// Restores the given state and executes an sret.
extern "C" [[noreturn]] void asm_exc_ret(exception_frame const *frame);

// The initial kernel stack pointer.
extern "C" char kern_stack_end[];

// Entrypoint for interrupts/exceptions from userspace.
extern "C" [[noreturn]] void user_exc_entry(exception_frame *frame);

// Entrypoint for interrupts/exceptions from the kernel..
extern "C" [[noreturn]] void kern_exc_entry();

inline void wait_for_interrupt()
{
  asm volatile("wfi");
}

[[noreturn]] void reset_stack_and_wait_for_interrupt();

[[noreturn]] inline void wait_forever()
{
  while (true) {
    wait_for_interrupt();
  }
}

// Return the value of the current wall-clock time.
inline uint64_t rdtime()
{
#if __riscv_xlen == 32
  mword_t time_lo;
  mword_t time_hi_before;
  mword_t time_hi_after;

  // Repeat reading the two parts of the timer register if the low
  // part wrapped.
  do {
    asm volatile("rdtimeh %0\n"
		 "rdtime %1\n"
		 "rdtimeh %2\n"
		 : "=r" (time_hi_before), "=r" (time_lo), "=r" (time_hi_after));
  } while (time_hi_after != time_hi_before);

  return static_cast<uint64_t>(time_hi_before) << 32 | time_lo;

#else
  mword_t time;
  asm volatile("rdtime %0" : "=r"(time));
  return time;
#endif
}
