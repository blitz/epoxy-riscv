#pragma once

#include <optional>
#include <utility>
#include <vector>

template <typename T>
std::optional<T> try_pop_back(std::vector<T> &v)
{
  if (v.empty()) {
    return std::nullopt;
  }

  auto val {std::move(v.back())};
  v.pop_back();
  return val;
}
