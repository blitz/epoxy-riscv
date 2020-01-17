#pragma once

#include "api.hpp"
#include "types.hpp"
#include "kobject.hpp"

struct capability_set {
  cap_t length;
  kobj_id_t const *object;
};

using processor_id_t = int;
