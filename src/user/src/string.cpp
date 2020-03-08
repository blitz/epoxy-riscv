#include <types.hpp>

extern "C" void *memcpy(void *dest, const void *src, size_t n);

__attribute__((used)) void *memcpy(void *dst, const void *src, size_t n)
{
  auto cdst {static_cast<char       *>(dst)};
  auto csrc {static_cast<char const *>(src)};

  while (n--) {
    *(cdst++) = *(csrc++);
  }

  return dst;
}
