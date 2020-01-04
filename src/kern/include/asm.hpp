#pragma once

#include <types.hpp>

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

[[noreturn]] inline void wait_forever()
{
  while (true) {
    asm volatile ("wfi");
  }
}

// Return the value of the current wall-clock time.
inline mword_t rdtime()
{
  mword_t time;

  asm volatile ("rdtime %0" : "=r" (time));

  return time;
}
