#include <types.hpp>

#include "asm.hpp"
#include "io.hpp"

void start()
{
  format("\nHello World!\n");

  while (true) {
    asm volatile ("wfi");
  }
}
