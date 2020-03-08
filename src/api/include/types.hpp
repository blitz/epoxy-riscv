#pragma once

using int8_t = signed char;
using uint8_t = unsigned char;

using int16_t = signed short;
using uint16_t = unsigned short;

using int32_t = signed int;
using uint32_t = unsigned int;

using int64_t = long long;
using uint64_t = unsigned long long;

#ifdef __riscv
using size_t = unsigned long;
#else
#error Unknown platform
#endif

static_assert(sizeof(void *) == sizeof(size_t));
using uintptr_t = size_t;
using mword_t = size_t;

#define __packed __attribute__((packed))

inline bool likely(bool b)
{
  return __builtin_expect(b, 1);
}
inline bool unlikely(bool b)
{
  return __builtin_expect(b, 0);
}

template <typename T, size_t SIZE>
constexpr size_t array_size(T (&)[SIZE])
{
  return SIZE;
}

#define offsetof(type, member) __builtin_offsetof(type, member)

inline void *operator new(size_t, void *p)
{
  return p;
}
