#include "api.hpp"

extern "C" void main();

namespace {

  const cap_t klog_cap {0};

  void klog_msg(char const *msg)
  {
    char c;
    while (c = *(msg++)) {
      invoke(klog_cap, c);
    }
  }

} // anonymous namespace

void main()
{
  klog_msg("Hello from userspace!\n");
}
