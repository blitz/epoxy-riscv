#pragma once

inline bool likely(bool b)
{
  return __builtin_expect(b, 1);
}

inline bool unlikely(bool b)
{
  return __builtin_expect(b, 0);
}
