#include "asm.hpp"

#include "csr.hpp"

void reset_stack_and_wait_for_interrupt()
{
  asm volatile(
      "mv sp, %0\n"
      "csrrs zero, sstatus, %1\n"
      "wfi"
      :
      : "r"(kern_stack_end), "r"(SSTATUS_SIE)
      : "memory");

  // We always return directly to userspace after an interrupt in the
  // kernel, so we never come back here.
  __builtin_trap();
}
