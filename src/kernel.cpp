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
