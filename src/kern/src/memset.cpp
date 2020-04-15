#include <types.hpp>

// This memset implementation is only here to make the compiler
// happy. It should not be used by kernel code directly.

extern "C" void *memset(void *s, int c, size_t n);

__attribute__((used)) void *memset(void *s, int c, size_t n)
{
  for (size_t cur = 0; cur < n; cur++) {
    static_cast<char *>(s)[cur] = c;
  }

  return s;
}
