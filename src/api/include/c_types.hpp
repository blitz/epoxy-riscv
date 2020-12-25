#pragma once

#if __STDC_HOSTED__

#include <cstddef>
#include <cstdint>

#else

using int8_t = signed char;
using uint8_t = unsigned char;

using int16_t = signed short;
using uint16_t = unsigned short;

using int32_t = signed int;
using uint32_t = unsigned int;

using int64_t = long long;
using uint64_t = unsigned long long;

#ifdef __riscv
# if __riscv_xlen == 32
using size_t = unsigned int;
# else
using size_t = unsigned long;
# endif
#else
# error Unknown platform
#endif

#define offsetof(type, member) __builtin_offsetof(type, member)

inline void *operator new(size_t, void *p)
{
  return p;
}

static_assert(sizeof(void *) == sizeof(size_t));
using uintptr_t = size_t;

#endif
