#pragma once

#define STR_(x) #x
#define STR(x) STR_(x)

template <typename T, size_t SIZE>
constexpr size_t array_size(T (&)[SIZE])
{
  return SIZE;
}
