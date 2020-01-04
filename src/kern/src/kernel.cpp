#include <types.hpp>

#include "asm.hpp"
#include "csr.hpp"
#include "exception_frame.hpp"
#include "io.hpp"
#include "scheduler.hpp"

namespace {

void arch_init()
{
  // Prevent executable memory from being automatically readable and disable interrupts while we are in supervisor mode.
  csr_rc<csr::SSTATUS>(SSTATUS_MXR | SSTATUS_SIE);

  // Prevent touching user memory unintentionally.
  csr_rs<csr::SSTATUS>(SSTATUS_SUM);

  // Set up exception and interrupt entry. We use direct mode where all interrupts and exceptions enter at the same
  // location.
  csr_w<csr::STVEC>(reinterpret_cast<uintptr_t>(asm_exc_entry));

  // A scratch register containing zero means we are in the kernel.
  csr_w<csr::SSCRATCH>(0);
}

[[noreturn]] void die_on_exception_from(char const *from)
{
  mword_t const sepc   {csr_r<csr::SEPC>()};
  mword_t const scause {csr_r<csr::SCAUSE>()};
  mword_t const stval  {csr_r<csr::STVAL>()};

  format("!! Exception from ", from, "!\n"
         "!! SCAUSE ", scause, "\n"
         "!! SEPC   ", sepc, "\n"
         "!! STVAL  ", stval, "\n");

  wait_forever();
}

} // anonymous namespace

void user_exc_entry([[maybe_unused]] exception_frame *frame)
{
  die_on_exception_from("user");
}

void kern_exc_entry()
{
  die_on_exception_from("kernel");
}

void start()
{
  format("\n"
         ">> Epoxy (RISC-V 64-bit, "
#ifdef __clang__
         "clang " __clang_version__
#elif __GNUC__
         "gcc " __VERSION__
#else
         "unknown compiler"
#endif
         ")\n");

  arch_init();

  schedule();
}
