#include "asm.hpp"
#include "format.hpp"
#include "sbi.hpp"

void put_char(char c)
{
  sbi_putc(c);
}

void do_panic()
{
  format("!! Panic! System reset...\n");

  sbi_shutdown();
  wait_forever();
}
