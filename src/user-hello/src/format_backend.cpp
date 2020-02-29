#include "capabilities.hpp"
#include "format.hpp"

void put_char(char c)
{
  invoke(CAP_KLOG, c);
}

void do_panic()
{
  format("!! Panic! Thread exits.\n");

  invoke(CAP_EXIT);
  __builtin_trap();
}
