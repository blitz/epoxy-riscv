#include "assert.hpp"
#include "format.hpp"

void assert_failed(char const *file, int line)
{
  panic("!! Assertion failed at ", file, ":", line, "\n");
}
