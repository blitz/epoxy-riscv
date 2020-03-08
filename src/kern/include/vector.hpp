#pragma once

#include "types.hpp"

// A vector with statically allocating backing store and a compile-time maximum
// length.
template <typename T, size_t CAPACITY>
class vector
{
  size_t length {};
  alignas(T) char backing[sizeof(T[CAPACITY])] {};

public:
  T &operator[](size_t i) { return reinterpret_cast<T *>(backing)[i]; }
  T &operator[](size_t i) const { return reinterpret_cast<T const *>(backing)[i]; }

  T *begin() { return &(*this)[0]; }
  T const *begin() const { return &(*this)[0]; }

  T *end() { return &(*this)[length]; }
  T const *end() const { return &(*this)[length]; }

  size_t size() const { return length; }
  size_t capacity_left() const { return CAPACITY - size(); }

  void reset()
  {
    for (size_t i {0}; i < length; i++) {
      (*this)[i].~T();
    }

    length = 0;
  }

  void push_back(T const &value)
  {
    if (unlikely(length >= CAPACITY)) {
      __builtin_trap();
    }

    new (&(*this)[length++])(T) {value};
  }

  constexpr vector() {}

  // TODO We have to prevent destructor calls from being generatoed.
  // ~vector()
  // {
  //   reset();
  // }
};
