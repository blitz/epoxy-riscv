#pragma once

constexpr inline bool likely(bool b)
{
  return __builtin_expect(b, 1);
}

constexpr inline bool unlikely(bool b)
{
  return __builtin_expect(b, 0);
}
