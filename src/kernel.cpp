#include <types.hpp>

#include "asm.hpp"
#include "io.hpp"

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

  while (true) {
    asm volatile ("wfi");
  }
}
