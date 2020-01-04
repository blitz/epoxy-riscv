#pragma once

#include "types.hpp"
#include "kobject.hpp"

using cap_t = int;
const cap_t invalid_capability = -1;

struct capability_set {
  cap_t length;
  kobj_id_t const *object;
};

using processor_id_t = int;
