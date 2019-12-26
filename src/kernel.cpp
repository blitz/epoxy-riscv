extern "C" void sbi_putchar(int c);
extern "C" [[noreturn]] void start();

namespace {

  void print(const char *str)
  {
    for (;*str;str++) {
      sbi_putchar(*str);
    }
  }

}

void start()
{
  print("\nHello World!\n");

  while (true) {
    asm volatile ("wfi");
  }
}
