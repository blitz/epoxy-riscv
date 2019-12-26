#include "asm.hpp"
#include "io.hpp"
#include "sbi.hpp"

spinlock format_lock;

namespace {
  void putc(char c)
  {
    sbi_putc(c);
  }
}

void put(const char *str)
{
  for (;*str != 0; str++)
    putc(*str);
}

void put(uint64_t v)
{
  static const char hex[] = "0123456789abcdef";
  bool skip_leading_zeroes = true;

  putc('0'); putc('x');

  for (int i = sizeof(v)*8 - 4; i >= 0; i -= 4) {
    int nibble = (v >> i) & 0xF;

    // Don't print leading zeroes.
    if (nibble == 0 and i != 0 and skip_leading_zeroes)
      continue;
    else
      skip_leading_zeroes = false;

    putc(hex[nibble]);
  }
}
