#include "assert.hpp"
#include "asm.hpp"
#include "io.hpp"

void assert_failed(char const *file, int line)
{
  format("!! Assertion failed at ", file, ":", line, "\n");
  wait_forever();
}
