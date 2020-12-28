#pragma once

#include <epoxy-api/c_types.hpp>

// The backend interface that needs to be provided for format
// functions below to work.
void put_char(char c);
[[noreturn]] void do_panic();

// The frontend functionality for formatting.

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

template <typename... T>
[[noreturn]] void panic(T... args)
{
  format(args...);
  do_panic();
}
