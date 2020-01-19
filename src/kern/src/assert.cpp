#include "assert.hpp"
#include "asm.hpp"
#include "io.hpp"

void assert_failed(char const *file, int line)
{
  panic("!! Assertion failed at ", file, ":", line, "\n");
}
