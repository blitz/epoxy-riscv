#include "asm.hpp"
#include "csr.hpp"
#include "thread.hpp"

thread *thread::active_ = nullptr;

namespace {

// Clear outstanding load-reserved / store-conditional reservations.
void clear_lrsc_reservation()
{
  mword_t sc_dummy;

  asm volatile ("sc.d zero, zero, (%[mem])"
                : "=m" (sc_dummy)
                : [mem] "r" (&sc_dummy));
}

}

void thread::exit_from_syscall()
{
  clear_lrsc_reservation();

  // Clear SPP to return to usermode.
  csr_rc<csr::SSTATUS>(SSTATUS_SPP);
  csr_w<csr::SSCRATCH>(reinterpret_cast<uintptr_t>(&frame_));
  csr_w<csr::SEPC>(frame_.pc);

  asm_exc_ret(&frame_);
}
