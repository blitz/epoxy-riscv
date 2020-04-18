#pragma once

template <typename T, size_t SIZE>
constexpr size_t array_size(T (&)[SIZE])
{
  return SIZE;
}
