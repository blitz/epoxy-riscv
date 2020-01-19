#pragma once

#include "types.hpp"

void put_char(char c);
void put(const char *str);
void put(uint64_t v);

template <typename T>
void format_unlocked(T first)
{
  put(first);
}

template <typename T, typename... R>
void format_unlocked(T first, R... rest)
{
  format_unlocked(first);
  format_unlocked(rest...);
}

template <typename... T>
void format(T... args)
{
  format_unlocked(args...);
}

[[noreturn]] void do_panic();

template <typename... T>
[[noreturn]] void panic(T... args)
{
  format(args...);
  do_panic();
}
