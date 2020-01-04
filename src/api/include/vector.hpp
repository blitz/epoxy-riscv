#pragma once

#include "types.hpp"

// A vector with statically allocating backing store and a compile-time maximum
// length.
template <typename T, size_t CAPACITY>
class vector {
  size_t length;
  alignas(T) char backing[sizeof(T[CAPACITY])];

public:

  T &operator[](size_t i)       { return reinterpret_cast<T       *>(backing)[i]; }
  T &operator[](size_t i) const { return reinterpret_cast<T const *>(backing)[i]; }

  T       *begin()       { return (*this)[0]; }
  T const *begin() const { return (*this)[0]; }

  T       *end()       { return (*this)[length]; }
  T const *end() const { return (*this)[length]; }

  size_t size() const { return length; }

  T &front() { return (*this)[0]; }

  void remove_at(size_t p)
  {
    length -= 1;

    // TODO Call destructor of removed element and use move constructor to move
    // remaining elements.
    for (size_t i = p; i < length; i++)
      (*this)[i] = (*this)[i + 1];
  }

  void pop_front()
  {
    remove_at(0);
  }

  void push_back(T const &value)
  {
    if (length >= CAPACITY)
      __builtin_trap();

    new (&(*this)[length++]) (T) {value};
  }

  vector()
    : length(0)
  {}
};
