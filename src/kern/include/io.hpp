#pragma once

#include "types.hpp"
#include "spinlock.hpp"

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

extern spinlock format_lock;

template <typename... T>
void format(T... args)
{
  spinlock_guard g { format_lock };
  format_unlocked(args...);
}
