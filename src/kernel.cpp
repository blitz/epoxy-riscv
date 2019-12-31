#include <types.hpp>

#include "asm.hpp"
#include "csr.hpp"
#include "io.hpp"

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
}

}

void exc_entry()
{
  mword_t const sepc   {csr_r<csr::SEPC>()};
  mword_t const scause {csr_r<csr::SCAUSE>()};
  mword_t const stval  {csr_r<csr::STVAL>()};

  format("Exception!\n"
         "SCAUSE ", scause, "\n"
         "SEPC   ", sepc, "\n"
         "STVAL  ", stval, "\n");

  while (true) {
    asm volatile ("wfi");
  }
}

void start()
{
  format("\n"
         ">> Epoxy (RISC-V 64-bit)\n"
         ">>  compiled with "
#ifdef __clang__
         "clang " __clang_version__
#elif __GNUC__
         "gcc " __VERSION__
#else
         "unknown compiler"
#endif
         "\n\n");

  arch_init();

  while (true) {
    asm volatile ("wfi");
  }
}
