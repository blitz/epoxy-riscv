#pragma once

#include "api.hpp"
#include "kobject.hpp"
#include "types.hpp"

struct capability_set {
  cap_t length;
  kobject *const *object;
};

using processor_id_t = int;
