#pragma once

#include <epoxy-api/likely.hpp>
#include <epoxy-api/types.hpp>

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

  T pop_at(size_t pos)
  {
    if (unlikely(pos >= length)) {
      __builtin_trap();
    }

    T ret {(*this)[pos]};

    // Move all elements after the removal point one to the front.
    for (size_t i = pos + 1; i < length; i++) {
      (*this)[i - 1] = (*this)[i];
    }

    // Shrink the vector by one element.
    (*this)[length - 1].~T();
    length--;

    return ret;
  }

  T pop_front() { return pop_at(0); }

  T pop_back() { return pop_at(length - 1); }

  constexpr vector() {}

  // TODO We have to prevent destructor calls from being generated.
  // ~vector()
  // {
  //   reset();
  // }
};
