#pragma once

#include "types.hpp"

#ifndef NDEBUG

#define assert(x)                        \
  do {                                   \
    if (unlikely(!(x))) {                \
      assert_failed(__FILE__, __LINE__); \
    }                                    \
  } while (0)

#else
#define assert(x) \
  do {            \
  } while (0)
#endif

[[noreturn]] void assert_failed(char const *file, int line);
