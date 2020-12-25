#include "thread.hpp"

#include "asm.hpp"
#include "csr.hpp"
#include "process.hpp"
#include "state.hpp"

thread *thread::active_;

namespace
{
// Clear outstanding load-reserved / store-conditional reservations.
void clear_lrsc_reservation()
{
  mword_t sc_dummy;

#if __riscv_xlen == 64
  asm volatile("sc.d zero, zero, (%[mem])" : "=m"(sc_dummy) : [ mem ] "r"(&sc_dummy));
#elif __riscv_xlen == 32
  asm volatile("sc.w zero, zero, (%[mem])" : "=m"(sc_dummy) : [ mem ] "r"(&sc_dummy));
#else
# error Unknown platform
#endif
}

}  // namespace

void thread::exit_from_preemption()
{
  exception_frame *const frame {this};
  process_->activate();

  clear_lrsc_reservation();

  // Clear SPP to return to usermode.
  csr_rc<csr::SSTATUS>(SSTATUS_SPP);
  csr_w<csr::SSCRATCH>(reinterpret_cast<uintptr_t>(frame));
  csr_w<csr::SEPC>(pc_);

  asm_exc_ret(frame);
}

void thread::activate()
{
  active_ = this;
  exit_from_preemption();
}
