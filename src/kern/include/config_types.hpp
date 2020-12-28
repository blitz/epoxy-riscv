#pragma once

#include <epoxy-api/api.hpp>
#include "kobject.hpp"

struct capability_set {
  cap_t length;
  kobject *const *object;
};

using processor_id_t = int;
